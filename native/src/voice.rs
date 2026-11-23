use super::{symbol, Interface, Library};
use std::{
    cell::Cell,
    ffi::{c_char, c_int, c_void},
    ptr::null_mut,
};
use windows_sys::Win32::System::LibraryLoader::{
    GetModuleHandleExW, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
    GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
};

type Factory = unsafe extern "C" fn(*const c_char, *mut c_int) -> Interface;
type GetVoiceTweak = unsafe extern "system" fn(Interface) -> *const VoiceTweak;

// IVEngineClient013::GetVoiceTweakAPI в открытом Source SDK для Windows x64.
// До него в объявлении 53 виртуальных метода, деструктора в таблице нет.
const GET_VOICE_TWEAK_SLOT: usize = 53;
const MICROPHONE_VOLUME: c_int = 0;
const MIC_BOOST: c_int = 2;
const SPEAKING_VOLUME: c_int = 3;

// C-структура IVoiceTweak из public/ivoicetweak.h.
#[repr(C)]
struct VoiceTweak {
    start: unsafe extern "C" fn() -> c_int,
    end: unsafe extern "C" fn(),
    set: unsafe extern "C" fn(c_int, f32),
    get: unsafe extern "C" fn(c_int) -> f32,
    active: unsafe extern "C" fn() -> bool,
}

pub(super) fn module_of(address: *const c_void) -> Option<*mut c_void> {
    let mut module = null_mut();
    let found = unsafe {
        GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            address.cast(),
            &mut module,
        )
    };
    (found != 0).then_some(module)
}

fn engine_api() -> Option<&'static VoiceTweak> {
    let engine = Library::open_already_loaded("engine.dll").ok()?;
    unsafe {
        let factory: Factory = symbol(&engine, b"CreateInterface")?;
        let module = module_of(factory as *const c_void)?;
        let client = factory(c"VEngineClient013".as_ptr(), null_mut());
        if client.is_null() {
            return None;
        }
        let table = *client.cast::<*const *const c_void>();
        if module_of(table.cast()) != Some(module) {
            return None;
        }
        let getter = *table.add(GET_VOICE_TWEAK_SLOT);
        if module_of(getter) != Some(module) {
            return None;
        }
        let get_voice_tweak: GetVoiceTweak = std::mem::transmute(getter);
        let api = get_voice_tweak(client);
        if module_of(api.cast()) != Some(module) {
            return None;
        }
        for index in 0..5 {
            if module_of(*api.cast::<*const c_void>().add(index)) != Some(module) {
                return None;
            }
        }
        api.as_ref()
    }
}

thread_local! {
    static ACTIVE: Cell<Option<&'static VoiceTweak>> = const { Cell::new(None) };
}

pub fn start() -> bool {
    if ACTIVE.with(|state| state.get().is_some()) {
        return true;
    }
    let Some(api) = engine_api() else {
        return false;
    };
    // Независимо открытый тест GameUI принадлежит движку, а не нашему меню.
    if unsafe { (api.active)() || (api.start)() == 0 } {
        return false;
    }
    ACTIVE.with(|state| state.set(Some(api)));
    true
}

pub fn poll() -> Option<f64> {
    let api = ACTIVE.with(Cell::get)?;
    if !unsafe { (api.active)() } {
        ACTIVE.with(|state| state.set(None));
        return None;
    }
    let level = unsafe { (api.get)(SPEAKING_VOLUME) };
    Some(if level.is_finite() {
        f64::from(level.clamp(0.0, 1.0))
    } else {
        0.0
    })
}

pub fn gain() -> Option<f64> {
    let api = ACTIVE.with(Cell::get)?;
    let value = unsafe { (api.get)(MICROPHONE_VOLUME) };
    (0.0..=1.0).contains(&value).then_some(f64::from(value))
}

pub fn set_gain(gain: f64) -> Option<f64> {
    if !(0.0..=1.0).contains(&gain) {
        return None;
    }
    let api = ACTIVE.with(Cell::get)?;
    self::gain()?;
    unsafe { (api.set)(MICROPHONE_VOLUME, gain as f32) };
    self::gain()
}

fn settings_api() -> Option<&'static VoiceTweak> {
    ACTIVE.with(Cell::get).or_else(engine_api)
}

fn control_value(api: &VoiceTweak, control: c_int) -> Option<f64> {
    let value = unsafe { (api.get)(control) };
    (0.0..=1.0).contains(&value).then_some(f64::from(value))
}

pub fn settings() -> Option<serde_json::Value> {
    let api = settings_api()?;
    let mut result = serde_json::Map::new();
    if let Some(gain) = control_value(api, MICROPHONE_VOLUME) {
        result.insert("voiceMicrophone".into(), gain.into());
    }
    if let Some(boost) = control_value(api, MIC_BOOST) {
        result.insert("voiceBoost".into(), i32::from(boost > 0.0).into());
    }
    (!result.is_empty()).then_some(serde_json::Value::Object(result))
}

pub fn set_setting(key: &[u8], value: f64) -> Option<()> {
    let control = match key {
        b"voiceMicrophone" if (0.0..=1.0).contains(&value) => MICROPHONE_VOLUME,
        b"voiceBoost" if value == 0.0 || value == 1.0 => MIC_BOOST,
        _ => return None,
    };
    let api = settings_api()?;
    control_value(api, control)?;
    unsafe { (api.set)(control, value as f32) };
    Some(())
}

pub fn stop() {
    if let Some(api) = ACTIVE.with(Cell::take) {
        unsafe { (api.end)() };
    }
}
