#![cfg(windows)]

use libloading::os::windows::Library;
use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::sync::OnceLock;

mod client_settings;
mod directory;
mod loading_dialog;
mod shared_settings;
mod video;
mod voice;
mod window;

type Lua = *mut c_void;
type Interface = *mut c_void;
type LuaCallback = unsafe extern "C" fn(Lua) -> c_int;

const LUA_GLOBALS_INDEX: c_int = -10002;
const LUA_TBOOLEAN: c_int = 1;
const SUPPORT: &CStr = c"https://panopticon-trial.neocities.org/";

unsafe fn symbol<T: Copy>(library: &Library, name: &[u8]) -> Option<T> {
    unsafe { library.get::<T>(name).ok().map(|item| *item) }
}

struct LuaApi {
    push_boolean: unsafe extern "C" fn(Lua, c_int),
    push_string: unsafe extern "C" fn(Lua, *const c_char, usize),
    read_string: unsafe extern "C" fn(Lua, c_int, *mut usize) -> *const c_char,
    push_function: unsafe extern "C" fn(Lua, LuaCallback, c_int),
    get_field: unsafe extern "C" fn(Lua, c_int, *const c_char),
    set_field: unsafe extern "C" fn(Lua, c_int, *const c_char),
    new_table: unsafe extern "C" fn(Lua, c_int, c_int),
    call: unsafe extern "C" fn(Lua, c_int, c_int, c_int) -> c_int,
    read_boolean: unsafe extern "C" fn(Lua, c_int) -> c_int,
    lua_type: unsafe extern "C" fn(Lua, c_int) -> c_int,
    push_number: unsafe extern "C" fn(Lua, f64),
    read_number: unsafe extern "C" fn(Lua, c_int) -> f64,
    is_number: unsafe extern "C" fn(Lua, c_int) -> c_int,
    set_top: unsafe extern "C" fn(Lua, c_int),
}

impl LuaApi {
    fn get() -> Option<&'static Self> {
        static API: OnceLock<Option<LuaApi>> = OnceLock::new();
        API.get_or_init(|| {
            let lib = Library::open_already_loaded("lua_shared.dll").ok()?;
            unsafe {
                Some(Self {
                    push_boolean: symbol(&lib, b"lua_pushboolean")?,
                    push_string: symbol(&lib, b"lua_pushlstring")?,
                    read_string: symbol(&lib, b"lua_tolstring")?,
                    push_function: symbol(&lib, b"lua_pushcclosure")?,
                    get_field: symbol(&lib, b"lua_getfield")?,
                    set_field: symbol(&lib, b"lua_setfield")?,
                    new_table: symbol(&lib, b"lua_createtable")?,
                    call: symbol(&lib, b"lua_pcall")?,
                    read_boolean: symbol(&lib, b"lua_toboolean")?,
                    lua_type: symbol(&lib, b"lua_type")?,
                    push_number: symbol(&lib, b"lua_pushnumber")?,
                    read_number: symbol(&lib, b"lua_tonumber")?,
                    is_number: symbol(&lib, b"lua_isnumber")?,
                    set_top: symbol(&lib, b"lua_settop")?,
                })
            }
        })
        .as_ref()
    }

    unsafe fn json(&self, lua: Lua, value: Option<serde_json::Value>) -> c_int {
        let Some(value) = value else { return 0 };
        let json = value.to_string();
        unsafe { (self.push_string)(lua, json.as_ptr().cast(), json.len()) };
        1
    }

    unsafe fn game_flag(&self, lua: Lua, name: &CStr) -> Option<bool> {
        unsafe {
            (self.get_field)(lua, LUA_GLOBALS_INDEX, name.as_ptr());
            let success = (self.call)(lua, 0, 1, 0) == 0;
            let enabled = success && (self.read_boolean)(lua, -1) != 0;
            (self.set_top)(lua, -2);
            success.then_some(enabled)
        }
    }
}

unsafe extern "C" fn directory_plan(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    // Copy each small selector before making another call into Lua.
    let read = |index| {
        let mut length = 0;
        let value = unsafe { (api.read_string)(lua, index, &mut length) };
        if value.is_null() || length > 16 {
            return None;
        }
        let bytes = unsafe { std::slice::from_raw_parts(value.cast::<u8>(), length) };
        std::str::from_utf8(bytes).ok().map(str::to_owned)
    };
    let (Some(project), Some(region), Some(category)) = (read(1), read(2), read(3)) else {
        return 0;
    };
    unsafe { api.json(lua, directory::plan(&project, &region, &category)) }
}

unsafe extern "C" fn hide_loading_dialog(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    let hidden = unsafe { api.game_flag(lua, c"IsInLoading") } == Some(true)
        && loading_dialog::hide();
    unsafe { (api.push_boolean)(lua, i32::from(hidden)) };
    1
}

unsafe extern "C" fn directory_response(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    let mut length = 0;
    let body = unsafe { (api.read_string)(lua, 1, &mut length) };
    if body.is_null() {
        return 0;
    }
    let body = unsafe { std::slice::from_raw_parts(body.cast::<u8>(), length) };
    unsafe { api.json(lua, directory::parse(body)) }
}

unsafe extern "C" fn game_action(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    let mut length = 0;
    let action = unsafe { (api.read_string)(lua, 1, &mut length) };
    if action.is_null() || length > 16 {
        return 0;
    }
    let command: &[u8] = match unsafe { std::slice::from_raw_parts(action.cast(), length) } {
        b"OpenOptions" => b"OpenOptionsDialog",
        b"Resume" => b"ResumeGame",
        b"Disconnect" => b"Disconnect",
        b"Quit" => b"QuitNoConfirm",
        _ => return 0,
    };

    if command == b"QuitNoConfirm" {
        voice::stop();
    }
    unsafe {
        (api.get_field)(lua, LUA_GLOBALS_INDEX, c"RunGameUICommand".as_ptr());
        (api.push_string)(lua, command.as_ptr().cast(), command.len());
        let success = (api.call)(lua, 1, 0, 0) == 0;
        (api.push_boolean)(lua, i32::from(success));
    }
    1
}

unsafe extern "C" fn game_state(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    unsafe {
        let in_game = api.game_flag(lua, c"IsInGame").unwrap_or(false);
        let loading = api.game_flag(lua, c"IsInLoading").unwrap_or(false);
        (api.push_boolean)(lua, i32::from(in_game));
        (api.push_boolean)(lua, i32::from(loading));
    }
    2
}

unsafe fn request_open_url(api: &LuaApi, lua: Lua, url: &CStr) -> bool {
    // Штатное подтверждение GMod вызывает gui.OpenURL после согласия игрока.
    unsafe {
        (api.get_field)(lua, LUA_GLOBALS_INDEX, c"RequestOpenURL".as_ptr());
        (api.push_string)(lua, url.as_ptr(), url.to_bytes().len());
        let opened = (api.call)(lua, 1, 0, 0) == 0;
        if !opened {
            (api.set_top)(lua, -2);
        }
        opened
    }
}

unsafe extern "C" fn support(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    let opened = unsafe { request_open_url(api, lua, SUPPORT) };
    unsafe { (api.push_boolean)(lua, i32::from(opened)) };
    1
}

unsafe extern "C" fn open_url(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    let mut length = 0;
    let value = unsafe { (api.read_string)(lua, 1, &mut length) };
    if value.is_null() || length > 4096 {
        return 0;
    }
    let bytes = unsafe { std::slice::from_raw_parts(value.cast::<u8>(), length) };
    let Some(url) = std::str::from_utf8(bytes).ok().and_then(web_url) else {
        return 0;
    };
    let opened = unsafe { request_open_url(api, lua, &url) };
    unsafe { (api.push_boolean)(lua, i32::from(opened)) };
    1
}

fn steam_overlay(url: &CStr) -> Option<()> {
    let sdk = Library::open_already_loaded("steam_api64.dll").ok()?;
    unsafe {
        let utils: unsafe extern "C" fn() -> Interface = symbol(&sdk, b"SteamAPI_SteamUtils_v010")?;
        let friends: unsafe extern "C" fn() -> Interface =
            symbol(&sdk, b"SteamAPI_SteamFriends_v018")?;
        let enabled: unsafe extern "C" fn(Interface) -> bool =
            symbol(&sdk, b"SteamAPI_ISteamUtils_IsOverlayEnabled")?;
        let activate: unsafe extern "C" fn(Interface, *const c_char, i32) =
            symbol(&sdk, b"SteamAPI_ISteamFriends_ActivateGameOverlayToWebPage")?;
        let utils = utils();
        let friends = friends();
        if utils.is_null() || friends.is_null() || !enabled(utils) {
            return None;
        }
        activate(friends, url.as_ptr(), 0);
    }
    Some(())
}

unsafe extern "C" fn open_confirmed_url(lua: Lua) -> c_int {
    use windows_sys::Win32::UI::{Shell::ShellExecuteW, WindowsAndMessaging::SW_SHOWNORMAL};
    let Some(api) = LuaApi::get() else { return 0 };
    let mut length = 0;
    let value = unsafe { (api.read_string)(lua, 1, &mut length) };
    if value.is_null() || length > 4096 {
        return 0;
    }
    let bytes = unsafe { std::slice::from_raw_parts(value.cast::<u8>(), length) };
    let Some(url) = std::str::from_utf8(bytes).ok().and_then(web_url) else {
        return 0;
    };
    let opened = steam_overlay(&url).is_some() || {
        let wide: Vec<u16> = url
            .to_str()
            .unwrap_or_default()
            .encode_utf16()
            .chain([0])
            .collect();
        let operation = [b'o' as u16, b'p' as u16, b'e' as u16, b'n' as u16, 0];
        let result = unsafe {
            ShellExecuteW(
                std::ptr::null_mut(),
                operation.as_ptr(),
                wide.as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                SW_SHOWNORMAL,
            )
        };
        result as isize > 32
    };
    unsafe { (api.push_boolean)(lua, i32::from(opened)) };
    1
}

fn web_url(value: &str) -> Option<CString> {
    let rest = value
        .strip_prefix("https://")
        .or_else(|| value.strip_prefix("http://"))?;
    let authority = rest.split(['/', '?', '#']).next()?;
    if authority.is_empty()
        || authority.contains(['@', '\\'])
        || value.chars().any(|c| c.is_whitespace() || c.is_control())
    {
        return None;
    }
    CString::new(value).ok()
}

unsafe fn offline_menu(api: &LuaApi, lua: Lua) -> bool {
    unsafe {
        api.game_flag(lua, c"IsInGame") == Some(false)
            && api.game_flag(lua, c"IsInLoading") == Some(false)
    }
}

unsafe extern "C" fn voice_start(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    let started = unsafe { offline_menu(api, lua) } && voice::start();
    unsafe { (api.push_boolean)(lua, i32::from(started)) };
    1
}

unsafe extern "C" fn voice_stop(_: Lua) -> c_int {
    voice::stop();
    0
}

unsafe extern "C" fn voice_poll(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    if !unsafe { offline_menu(api, lua) } {
        voice::stop();
        return 0;
    }
    let Some(level) = voice::poll() else {
        return 0;
    };
    unsafe { (api.push_number)(lua, level) };
    1
}

unsafe extern "C" fn voice_gain(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    let Some(gain) = voice::gain() else { return 0 };
    unsafe { (api.push_number)(lua, gain) };
    1
}

unsafe extern "C" fn voice_set_gain(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    if !unsafe { offline_menu(api, lua) } {
        return 0;
    }
    let value = unsafe { (api.read_number)(lua, 1) };
    let Some(gain) = voice::set_gain(value) else {
        return 0;
    };
    unsafe { (api.push_number)(lua, gain) };
    1
}

unsafe extern "C" fn apply_window_identity(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    unsafe { (api.push_boolean)(lua, i32::from(window::apply())) };
    1
}

unsafe extern "C" fn video_modes(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    unsafe { api.json(lua, video::modes()) }
}

unsafe extern "C" fn voice_settings(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    unsafe { api.json(lua, voice::settings()) }
}

unsafe extern "C" fn set_voice_setting(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    let mut length = 0;
    let key = unsafe { (api.read_string)(lua, 1, &mut length) };
    let value = unsafe { (api.read_number)(lua, 2) };
    let applied = !key.is_null()
        && length <= 15
        && unsafe { (api.is_number)(lua, 2) } != 0
        && voice::set_setting(
            unsafe { std::slice::from_raw_parts(key.cast(), length) },
            value,
        )
        .is_some();
    unsafe { (api.push_boolean)(lua, i32::from(applied)) };
    1
}

unsafe extern "C" fn apply_video_mode(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    let width = unsafe { (api.read_number)(lua, 1) };
    let height = unsafe { (api.read_number)(lua, 2) };
    let mut length = 0;
    let mode = unsafe { (api.read_string)(lua, 3, &mut length) };
    let applied = width == f64::from(width as c_int)
        && height == f64::from(height as c_int)
        && !mode.is_null()
        && length <= 10
        && video::apply(width as c_int, height as c_int, unsafe {
            std::slice::from_raw_parts(mode.cast(), length)
        })
        .is_some();
    unsafe { (api.push_boolean)(lua, i32::from(applied)) };
    1
}

unsafe extern "C" fn read_shared_settings(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    let mut stored = shared_settings::read().ok();
    if unsafe { api.game_flag(lua, c"IsInGame") } == Some(true) {
        if let (Some(serde_json::Value::Object(disk)), Some(serde_json::Value::Object(active))) =
            (&mut stored, client_settings::read())
        {
            disk.extend(active);
        }
    }
    unsafe { api.json(lua, stored) }
}

unsafe extern "C" fn apply_shared_settings(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    let mut length = 0;
    let raw = unsafe { (api.read_string)(lua, 1, &mut length) };
    if raw.is_null() || length > shared_settings::MAX_PATCH_BYTES {
        unsafe { (api.push_boolean)(lua, 0) };
        return 1;
    }
    let raw = unsafe { std::slice::from_raw_parts(raw.cast(), length) };
    if unsafe { api.game_flag(lua, c"IsInGame") } == Some(true) {
        let applied = shared_settings::read().is_ok()
            && serde_json::from_slice(raw)
                .ok()
                .and_then(|patch| client_settings::apply(&patch))
                .is_some();
        unsafe {
            (api.push_boolean)(lua, i32::from(applied));
            (api.push_boolean)(lua, 1);
        }
        return 2;
    }
    unsafe { (api.push_boolean)(lua, i32::from(shared_settings::apply(raw).is_ok())) };
    1
}

unsafe extern "C" fn reset_shared_settings(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    let offline = unsafe { api.game_flag(lua, c"IsInGame") } == Some(false);
    unsafe { (api.push_boolean)(lua, i32::from(offline && shared_settings::reset().is_ok())) };
    1
}

unsafe extern "C" fn get_server_loading_screens(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    let Some(enabled) = shared_settings::server_loading_screens() else { return 0 };
    unsafe { (api.push_boolean)(lua, i32::from(enabled)) };
    1
}

unsafe extern "C" fn set_server_loading_screens(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    let applied = unsafe { (api.lua_type)(lua, 1) } == LUA_TBOOLEAN
        && shared_settings::set_server_loading_screens(unsafe { (api.read_boolean)(lua, 1) != 0 }).is_ok();
    unsafe { (api.push_boolean)(lua, i32::from(applied)) };
    1
}

unsafe extern "C" fn get_loading_indicator(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    let Some(enabled) = shared_settings::loading_indicator() else { return 0 };
    unsafe { (api.push_boolean)(lua, i32::from(enabled)) };
    1
}

unsafe extern "C" fn set_loading_indicator(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    let applied = unsafe { (api.lua_type)(lua, 1) } == LUA_TBOOLEAN
        && shared_settings::set_loading_indicator(unsafe { (api.read_boolean)(lua, 1) != 0 }).is_ok();
    unsafe { (api.push_boolean)(lua, i32::from(applied)) };
    1
}

/// # Safety
/// Garry's Mod passes its active LuaJIT state through its public module API.
#[no_mangle]
pub unsafe extern "C" fn gmod13_open(lua: Lua) -> c_int {
    let Some(api) = LuaApi::get() else { return 0 };
    let methods: &[(&CStr, LuaCallback)] = &[
        (c"DirectoryPlan", directory_plan),
        (c"HideLoadingDialog", hide_loading_dialog),
        (c"DirectoryResponse", directory_response),
        (c"OpenSupport", support),
        (c"OpenURL", open_url),
        (c"OpenConfirmedURL", open_confirmed_url),
        (c"GameAction", game_action),
        (c"GameState", game_state),
        (c"VoiceStart", voice_start),
        (c"VoiceStop", voice_stop),
        (c"VoicePoll", voice_poll),
        (c"VoiceGain", voice_gain),
        (c"VoiceSetGain", voice_set_gain),
        (c"ApplyWindowIdentity", apply_window_identity),
        (c"VideoModes", video_modes),
        (c"ApplyVideoMode", apply_video_mode),
        (c"GetVoiceSettings", voice_settings),
        (c"SetVoiceSetting", set_voice_setting),
        (c"ReadSharedSettings", read_shared_settings),
        (c"ApplySharedSettings", apply_shared_settings),
        (c"ResetSharedSettings", reset_shared_settings),
        (c"GetServerLoadingScreens", get_server_loading_screens),
        (c"SetServerLoadingScreens", set_server_loading_screens),
        (c"GetLoadingIndicator", get_loading_indicator),
        (c"SetLoadingIndicator", set_loading_indicator),
    ];
    unsafe {
        (api.new_table)(lua, 0, methods.len() as c_int);
        for &(name, method) in methods {
            (api.push_function)(lua, method, 0);
            (api.set_field)(lua, -2, name.as_ptr());
        }
        (api.set_field)(lua, LUA_GLOBALS_INDEX, c"PanopticonNative".as_ptr());
    }
    0
}

/// # Safety
/// Garry's Mod calls this when destroying the Lua realm.
#[no_mangle]
pub unsafe extern "C" fn gmod13_close(_: Lua) -> c_int {
    voice::stop();
    0
}

