use super::{shared_settings, symbol, Interface, Library, Lua, LUA_GLOBALS_INDEX};
use serde_json::{Map, Value};
use std::{
    ffi::{c_char, c_int, c_void, CStr, CString},
    ptr::null_mut,
};
use windows_sys::Win32::System::LibraryLoader::{
    GetModuleHandleExW, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
    GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
};

const CLIENT: u8 = 0;
const GET_LUA_INTERFACE: usize = 6;
const LUA_TOP: usize = 0;
const GET_CLIENT_ABI: [u8; 9] = [0x0f, 0xb6, 0xc2, 0x48, 0x8b, 0x44, 0xc1, 0x78, 0xc3];
const TOP_ABI: [u8; 5] = [0x48, 0x8b, 0x49, 0x08, 0xe9];
const LAST_BUTTON_CODE: i32 = 171;
const CLEARED_BINDING: i32 = -1;
const CUSTOM_BINDINGS: &[(&str, &CStr)] = &[
    ("+fast_actions", c"fast_actions"),
    ("+interact", c"interact"),
    ("+closelook", c"closelook"),
    ("+inspect", c"inspect"),
    ("+openinv", c"inventory"),
    ("+dro_graffiti_place", c"graffiti"),
    ("+dro_thirdperson_toggle", c"thirdperson"),
    ("+dro_music_player", c"music_player"),
];

type Factory = unsafe extern "C" fn(*const c_char, *mut c_int) -> Interface;
type GetLuaInterface = unsafe extern "system" fn(Interface, u8) -> Interface;
type GetTop = unsafe extern "C" fn(Lua) -> c_int;

fn module_of(address: *const c_void) -> Option<Interface> {
    if address.is_null() {
        return None;
    }
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

unsafe fn method(object: Interface, index: usize, module: Interface) -> Option<*const u8> {
    if object.is_null() {
        return None;
    }
    let table = unsafe { *object.cast::<*const *const c_void>() };
    if module_of(table.cast()) != Some(module) {
        return None;
    }
    let address = unsafe { *table.add(index) };
    (module_of(address) == Some(module)).then_some(address.cast())
}

fn live_state(lib: &Library, get_top: GetTop) -> Option<Lua> {
    unsafe {
        let factory: Factory = symbol(lib, b"CreateInterface")?;
        let module = module_of(factory as *const c_void)?;
        let shared = factory(c"LUASHARED003".as_ptr(), null_mut());
        let get_interface = method(shared, GET_LUA_INTERFACE, module)?;
        // Проверены GetLuaInterface и переход Top именно установленного x64 Lua.
        if std::slice::from_raw_parts(get_interface, GET_CLIENT_ABI.len()) != GET_CLIENT_ABI {
            return None;
        }
        let get_interface: GetLuaInterface = std::mem::transmute(get_interface);
        let client = get_interface(shared, CLIENT);
        let top = method(client, LUA_TOP, module)?;
        if std::slice::from_raw_parts(top, TOP_ABI.len()) != TOP_ABI {
            return None;
        }
        let displacement = top.add(TOP_ABI.len()).cast::<i32>().read_unaligned() as isize;
        let target = (top as usize)
            .wrapping_add(9)
            .wrapping_add_signed(displacement);
        if target != get_top as usize {
            return None;
        }
        let state = *client.cast::<Lua>().add(1);
        // Не входить из меню в другую Lua-функцию, находящуюся на стеке клиента.
        (!state.is_null() && get_top(state) == 0).then_some(state)
    }
}

struct Client {
    _lib: Library,
    state: Lua,
    set_top: unsafe extern "C" fn(Lua, c_int),
    lua_type: unsafe extern "C" fn(Lua, c_int) -> c_int,
    raw_get: unsafe extern "C" fn(Lua, c_int),
    push_string: unsafe extern "C" fn(Lua, *const c_char, usize),
    push_bool: unsafe extern "C" fn(Lua, c_int),
    push_number: unsafe extern "C" fn(Lua, f64),
    get_string: unsafe extern "C" fn(Lua, c_int, *mut usize) -> *const c_char,
    get_bool: unsafe extern "C" fn(Lua, c_int) -> c_int,
    get_number: unsafe extern "C" fn(Lua, c_int) -> f64,
    pcall: unsafe extern "C" fn(Lua, c_int, c_int, c_int) -> c_int,
}

impl Client {
    fn open() -> Option<Self> {
        let lib = Library::open_already_loaded("lua_shared.dll").ok()?;
        unsafe {
            let get_top = symbol(&lib, b"lua_gettop")?;
            let state = live_state(&lib, get_top)?;
            let check_stack: unsafe extern "C" fn(Lua, c_int) -> c_int =
                symbol(&lib, b"lua_checkstack")?;
            if check_stack(state, 16) == 0 {
                return None;
            }
            Some(Self {
                state,
                set_top: symbol(&lib, b"lua_settop")?,
                lua_type: symbol(&lib, b"lua_type")?,
                raw_get: symbol(&lib, b"lua_rawget")?,
                push_string: symbol(&lib, b"lua_pushlstring")?,
                push_bool: symbol(&lib, b"lua_pushboolean")?,
                push_number: symbol(&lib, b"lua_pushnumber")?,
                get_string: symbol(&lib, b"lua_tolstring")?,
                get_bool: symbol(&lib, b"lua_toboolean")?,
                get_number: symbol(&lib, b"lua_tonumber")?,
                pcall: symbol(&lib, b"lua_pcall")?,
                _lib: lib,
            })
        }
    }

    fn field(&self, table: c_int, name: &CStr) {
        self.string(name);
        unsafe { (self.raw_get)(self.state, if table == -1 { -2 } else { table }) };
    }

    fn string(&self, name: &CStr) {
        unsafe { (self.push_string)(self.state, name.as_ptr(), name.to_bytes().len()) };
    }

    fn stack_type(&self) -> c_int {
        unsafe { (self.lua_type)(self.state, -1) }
    }

    fn function<T>(&self, fields: &[&CStr], call: impl FnOnce(&Self) -> Option<T>) -> Option<T> {
        self.field(LUA_GLOBALS_INDEX, fields[0]);
        for &name in &fields[1..] {
            if self.stack_type() != 5 {
                self.clear();
                return None;
            }
            self.field(-1, name);
        }
        let result = if self.stack_type() == 6 {
            call(self)
        } else {
            None
        };
        self.clear();
        result
    }

    fn clear(&self) {
        unsafe { (self.set_top)(self.state, 0) };
    }

    fn call(&self, arguments: c_int, results: c_int) -> Option<()> {
        (unsafe { (self.pcall)(self.state, arguments, results, 0) } == 0).then_some(())
    }

    fn gamemode(&self) -> Option<Gamemode> {
        let mode = self.function(&[c"engine", c"ActiveGamemode"], |client| {
            client.call(0, 1)?;
            if client.stack_type() != 4 {
                return None;
            }
            let mut length = 0;
            let ptr = unsafe { (client.get_string)(client.state, -1, &mut length) };
            if ptr.is_null() || length > 32 {
                return None;
            }
            match unsafe { std::slice::from_raw_parts(ptr.cast(), length) } {
                b"panopticon" => Some(Gamemode::Panopticon),
                b"shinri_trial" => Some(Gamemode::Shinri),
                _ => Some(Gamemode::Other),
            }
        })?;
        if mode == Gamemode::Shinri
            && self
                .function(&[c"ThirdPerson", c"SetEnabled"], |_| Some(()))
                .is_some()
        {
            Some(Gamemode::Panopticon)
        } else {
            Some(mode)
        }
    }

    fn get(&self, name: &CStr) -> Option<Value> {
        self.function(&[c"settings", c"GetParameter"], |client| {
            client.string(name);
            client.call(1, 1)?;
            match client.stack_type() {
                1 => Some(Value::Bool(
                    unsafe { (client.get_bool)(client.state, -1) } != 0,
                )),
                3 => serde_json::Number::from_f64(unsafe { (client.get_number)(client.state, -1) })
                    .map(Value::Number),
                _ => None,
            }
        })
    }

    fn current(&self, mode: Gamemode, name: &CStr) -> Option<Value> {
        if mode == Gamemode::Panopticon && name == c"dro_thirdperson" {
            let camera = self.function(&[c"ThirdPerson", c"IsEnabled"], |client| {
                client.call(0, 1)?;
                (client.stack_type() == 1)
                    .then(|| Value::Bool(unsafe { (client.get_bool)(client.state, -1) } != 0))
            });
            if camera.is_some() {
                return camera;
            }
        }
        self.get(name)
    }

    fn push_value(&self, value: &Value) -> Option<()> {
        unsafe {
            if let Some(flag) = value.as_bool() {
                (self.push_bool)(self.state, i32::from(flag));
            } else {
                (self.push_number)(self.state, value.as_f64()?);
            }
        }
        Some(())
    }

    fn set(&self, mode: Gamemode, name: &CStr, value: &Value) -> Option<()> {
        if mode == Gamemode::Panopticon && name == c"dro_thirdperson" {
            self.function(&[c"ThirdPerson", c"SetEnabled"], |client| {
                client.push_value(value)?;
                client.call(1, 1)?;
                (client.stack_type() != 1 || unsafe { (client.get_bool)(client.state, -1) } != 0)
                    .then_some(())
            })?;
        }
        self.function(&[c"settings", c"SetParameter"], |client| {
            client.string(name);
            client.push_value(value)?;
            client.call(2, 0)
        })?;
        if mode == Gamemode::Panopticon
            && matches!(
                name.to_bytes(),
                b"droview_bob"
                    | b"dro_thirdperson_crosshair"
                    | b"dro_thirdperson_pos_updown"
                    | b"dro_thirdperson_pos_rightleft"
                    | b"dro_thirdperson_pos_forwardback"
            )
        {
            let value = match value.as_bool() {
                Some(on) => i32::from(on).to_string(),
                None => value.as_f64()?.to_string(),
            };
            let value = CString::new(value).ok()?;
            self.function(&[c"RunConsoleCommand"], |client| {
                client.string(name);
                client.string(&value);
                client.call(2, 0)
            })?;
        }
        if mode == Gamemode::Panopticon
            && matches!(name.to_bytes(), b"musicInDances" | b"musicInOwnDances")
        {
            let _ = self.function(&[c"hook", c"Run"], |client| {
                client.string(c"DRO.SettingChanged");
                client.string(name);
                client.push_value(value)?;
                client.call(3, 0)
            });
        }
        Some(())
    }

    fn values(&self, mode: Gamemode) -> Option<Value> {
        let mut values = Map::new();
        for key in shared_settings::keys() {
            let name = CString::new(key).ok()?;
            if let Some(value) = self.current(mode, &name) {
                if shared_settings::accepts(key, &value, false) {
                    values.insert(key.to_owned(), value);
                }
            }
        }
        (!values.is_empty()).then_some(Value::Object(values))
    }

    fn binding_code(&self, getter: &CStr, action: &CStr) -> Option<Option<i32>> {
        self.function(&[c"DANGAN", c"Binds", getter], |client| {
            client.string(action);
            client.call(1, 1)?;
            match client.stack_type() {
                0 => Some(None),
                3 => {
                    let number = unsafe { (client.get_number)(client.state, -1) };
                    (number.fract() == 0.0 && (1.0..=f64::from(LAST_BUTTON_CODE)).contains(&number))
                        .then_some(Some(number as i32))
                }
                _ => None,
            }
        })
    }

    fn button_name(&self, code: i32) -> Option<String> {
        self.function(&[c"input", c"GetKeyName"], |client| {
            unsafe { (client.push_number)(client.state, f64::from(code)) };
            client.call(1, 1)?;
            client.read_key_name()
        })
    }

    fn read_key_name(&self) -> Option<String> {
        if self.stack_type() != 4 {
            return None;
        }
        let mut length = 0;
        let ptr = unsafe { (self.get_string)(self.state, -1, &mut length) };
        if ptr.is_null() || length > 64 {
            return None;
        }
        let bytes = unsafe { std::slice::from_raw_parts(ptr.cast(), length) };
        Some(std::str::from_utf8(bytes).ok()?.to_owned())
    }

    fn shinri_key(&self, command: &CStr) -> Option<String> {
        self.function(&[c"custombinds", c"getBoundKey"], |client| {
            client.string(command);
            client.call(1, 1)?;
            client.read_key_name()
        })
    }

    fn shinri_default(&self, command: &CStr) -> Option<i32> {
        let name = (|| {
            self.field(LUA_GLOBALS_INDEX, c"custombinds");
            if self.stack_type() != 5 { return None; }
            self.field(-1, c"commandIds");
            if self.stack_type() != 5 { return None; }
            self.field(-1, command);
            if self.stack_type() != 3 { return None; }
            let id = unsafe { (self.get_number)(self.state, -1) };
            self.clear();
            self.field(LUA_GLOBALS_INDEX, c"custombinds");
            self.field(-1, c"commandDefaults");
            if self.stack_type() != 5 { return None; }
            unsafe {
                (self.push_number)(self.state, id);
                (self.raw_get)(self.state, -2);
            }
            self.read_key_name()
        })();
        self.clear();
        let name = CString::new(name?).ok()?;
        self.function(&[c"input", c"GetKeyCode"], |client| {
            client.string(&name);
            client.call(1, 1)?;
            (client.stack_type() == 3).then(|| unsafe { (client.get_number)(client.state, -1) as i32 })
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Gamemode {
    Panopticon,
    Shinri,
    Other,
}

pub(super) fn has_custom_settings() -> Option<bool> {
    Some(Client::open()?.gamemode()? != Gamemode::Other)
}

pub fn read() -> Option<Value> {
    let client = Client::open()?;
    let mode = client.gamemode()?;
    (mode != Gamemode::Other).then(|| client.values(mode)).flatten()
}

pub fn apply(patch: &Value) -> Option<Value> {
    let patch = patch.as_object()?;
    if patch
        .iter()
        .any(|(key, value)| !shared_settings::accepts(key, value, false))
    {
        return None;
    }
    let client = Client::open()?;
    let mode = client.gamemode()?;
    // Отсутствие клиентского API — отдельное состояние от отклонённого значения.
    let before = client.values(mode)?;
    for (key, value) in patch {
        let name = CString::new(key.as_str()).ok()?;
        if before.get(key) == Some(value) && client.get(&name).as_ref() == Some(value) {
            continue;
        }
        // Значение на выходе читается снова, включая отклонённые игровые настройки.
        let _ = client.set(mode, &name, value);
    }
    client.values(mode)
}

pub(super) fn read_bindings() -> Option<Value> {
    let client = Client::open()?;
    let mode = client.gamemode()?;
    let mut bindings = Map::new();
    for &(command, action) in CUSTOM_BINDINGS {
        let value = match mode {
            Gamemode::Panopticon if client.binding_code(c"GetDefault", action).flatten().is_some() => {
                match client.binding_code(c"GetKey", action) {
                    Some(Some(code)) => client.button_name(code),
                    Some(None) => Some(String::new()),
                    None => None,
                }
            }
            Gamemode::Shinri => client.shinri_key(&CString::new(&command[1..]).ok()?),
            _ => None,
        };
        if let Some(value) = value {
            bindings.insert(command.to_owned(), Value::String(value));
        }
    }
    (!bindings.is_empty()).then_some(Value::Object(bindings))
}

/// A positive button code assigns a key, -1 clears it and None restores the action's default.
pub(super) fn apply_binding(command: &str, key: Option<i32>) -> Option<()> {
    let &(_, action) = CUSTOM_BINDINGS.iter().find(|(name, _)| *name == command)?;
    let client = Client::open()?;
    let mode = client.gamemode()?;
    let shinri_command = CString::new(&command[1..]).ok()?;
    let default = match mode {
        Gamemode::Panopticon => client.binding_code(c"GetDefault", action)??,
        Gamemode::Shinri => client.shinri_default(&shinri_command)?,
        Gamemode::Other => return None,
    };
    let code = key.unwrap_or(default);
    if code != CLEARED_BINDING && !(1..=LAST_BUTTON_CODE).contains(&code) {
        return None;
    }
    if mode == Gamemode::Shinri {
        client.function(&[c"custombinds", c"setBoundKey"], |client| {
            client.string(&shinri_command);
            unsafe { (client.push_number)(client.state, f64::from(code)) };
            client.call(2, 0)
        })?;
        let actual = client.shinri_key(&shinri_command)?;
        let expected = if code == CLEARED_BINDING { String::new() } else { client.button_name(code)? };
        return actual.eq_ignore_ascii_case(&expected).then_some(());
    }
    client.function(&[c"DANGAN", c"Binds", c"SetKey"], |client| {
        client.string(action);
        unsafe { (client.push_number)(client.state, f64::from(code)) };
        client.call(2, 0)
    })?;
    let expected = (code != CLEARED_BINDING).then_some(code);
    (client.binding_code(c"GetKey", action)? == expected).then_some(())
}
