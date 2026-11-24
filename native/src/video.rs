use super::{symbol, voice::module_of, Interface, Library};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::ptr::null_mut;
use windows_sys::Win32::Foundation::HMODULE;

type Factory = unsafe extern "C" fn(*const c_char, *mut c_int) -> Interface;
type GetVideoModes = unsafe extern "system" fn(Interface, *mut *const VideoMode, *mut c_int);
type GetVideoConfig = unsafe extern "system" fn(Interface) -> *const VideoConfig;
type ClientCommand = unsafe extern "system" fn(Interface, *const c_char);

// These are the slots used by the installed x64 GameUI for these interface versions.
const GET_VIDEO_MODES: usize = 3;
const GET_VIDEO_CONFIG: usize = 21;
const CLIENT_COMMAND: usize = 106;
const WINDOWED: u32 = 1;
const NO_WINDOW_BORDER: u32 = 1 << 16;

#[repr(C)]
struct VideoMode {
    width: c_int,
    height: c_int,
    bits_per_pixel: c_int,
    refresh_rate: c_int,
}

// GameUI reads m_VideoMode first and m_Flags at offset 0x34 in VMaterialSystem080.
#[repr(C)]
struct VideoConfig {
    width: c_int,
    height: c_int,
    _before_flags: [u8; 0x34 - 8],
    flags: u32,
}

pub(super) struct EngineObject {
    _library: Library,
    pub(super) interface: Interface,
    vtable: *const *const c_void,
    module: HMODULE,
}

impl EngineObject {
    pub(super) fn get(dll: &str, version: &CStr) -> Option<Self> {
        let library = Library::open_already_loaded(dll).ok()?;
        let factory: Factory = unsafe { symbol(&library, b"CreateInterface")? };
        let module = module_of(factory as *const c_void)?;
        let interface = unsafe { factory(version.as_ptr(), null_mut()) };
        if module_of(interface) != Some(module) {
            return None;
        }
        let vtable = unsafe { *interface.cast::<*const *const c_void>() };
        if module_of(vtable.cast()) != Some(module) {
            return None;
        }
        Some(Self {
            _library: library,
            interface,
            vtable,
            module,
        })
    }

    pub(super) fn method(&self, index: usize) -> Option<*const c_void> {
        let method = unsafe { *self.vtable.add(index) };
        (module_of(method) == Some(self.module)).then_some(method)
    }
}

pub(super) fn client_command(command: &CStr) -> Option<()> {
    let engine = EngineObject::get("engine.dll", c"VEngineClient015")?;
    let run: ClientCommand = unsafe { std::mem::transmute(engine.method(CLIENT_COMMAND)?) };
    unsafe { run(engine.interface, command.as_ptr()) };
    Some(())
}

fn sensible_size(width: c_int, height: c_int) -> bool {
    (640..=16384).contains(&width) && (480..=16384).contains(&height)
}

fn available() -> Option<BTreeSet<(c_int, c_int)>> {
    let engine = EngineObject::get("engine.dll", c"VENGINE_GAMEUIFUNCS_VERSION005")?;
    let get_video_modes: GetVideoModes =
        unsafe { std::mem::transmute(engine.method(GET_VIDEO_MODES)?) };
    let mut modes = std::ptr::null();
    let mut count = 0;
    unsafe { get_video_modes(engine.interface, &mut modes, &mut count) };
    if !(1..=4096).contains(&count) || modes.is_null() || !modes.is_aligned() {
        return None;
    }
    let modes = unsafe { std::slice::from_raw_parts(modes, count as usize) };
    Some(
        modes
            .iter()
            .filter(|mode| sensible_size(mode.width, mode.height))
            .map(|mode| (mode.width, mode.height))
            .collect(),
    )
}

fn current() -> Option<(c_int, c_int, bool, bool)> {
    let engine = EngineObject::get("materialsystem.dll", c"VMaterialSystem080")?;
    let get_config: GetVideoConfig =
        unsafe { std::mem::transmute(engine.method(GET_VIDEO_CONFIG)?) };
    let result = unsafe { get_config(engine.interface) };
    if module_of(result.cast()) != Some(engine.module) {
        return None;
    }
    let config = unsafe { &*result };
    let windowed = config.flags & WINDOWED != 0;
    sensible_size(config.width, config.height).then_some((
        config.width,
        config.height,
        windowed,
        windowed && config.flags & NO_WINDOW_BORDER != 0,
    ))
}

pub fn modes() -> Option<Value> {
    let mut available = available()?;
    let (width, height, windowed, borderless) = current()?;
    available.insert((width, height));
    let choices: Vec<_> = available
        .into_iter()
        .map(|(width, height)| json!({"w": width, "h": height}))
        .collect();
    Some(json!({
        "modes": choices,
        "current": {"w": width, "h": height, "windowed": windowed, "borderless": borderless}
    }))
}

pub fn apply(width: c_int, height: c_int, mode: &[u8]) -> Option<()> {
    let (windowed, borderless) = match mode {
        b"fullscreen" => (0, 0),
        b"windowed" => (1, 0),
        b"borderless" => (1, 1),
        _ => return None,
    };
    if !sensible_size(width, height) {
        return None;
    }
    let active = current()?;
    if !available()?.contains(&(width, height)) && (active.0, active.1) != (width, height) {
        return None;
    }
    let command = CString::new(format!(
        "mat_setvideomode {width} {height} {windowed} {borderless}\nmat_savechanges\n"
    ))
    .ok()?;
    client_command(&command)
}
