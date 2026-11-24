use serde_json::Value;
use std::{
    cell::{Cell, RefCell},
    fs::{self, File},
    io::Read,
    os::windows::{ffi::OsStrExt, fs::MetadataExt},
    path::{Path, PathBuf},
    ptr,
};
use windows_sys::Win32::{
    Foundation::{BOOL, HMODULE, HWND, LPARAM},
    System::{
        LibraryLoader::{
            GetModuleHandleExW, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
            GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
        },
        Threading::GetCurrentProcessId,
    },
    UI::WindowsAndMessaging::{
        DestroyIcon, EnumWindows, GetClassNameW, GetSystemMetrics, GetWindow, GetWindowTextW,
        GetWindowThreadProcessId, IsIconic, IsWindow, IsWindowVisible, LoadImageW, SendMessageW, SetWindowTextW,
        GW_OWNER, HICON, ICON_BIG, ICON_SMALL, IMAGE_ICON, LR_LOADFROMFILE, SM_CXICON, SM_CXSMICON,
        SM_CYICON, SM_CYSMICON, WM_GETICON, WM_SETICON,
    },
};

const TITLE: &str = "Panopticon Trial (x86-64)";
const WINDOW_CLASS: &str = "Valve001";
const ICON_RESOURCE: *const u16 = 1 as *const u16;

struct WindowSearch {
    pid: u32,
    window: HWND,
}

unsafe extern "system" fn find_window(window: HWND, context: LPARAM) -> BOOL {
    let search = unsafe { &mut *(context as *mut WindowSearch) };
    let mut pid = 0;
    unsafe { GetWindowThreadProcessId(window, &mut pid) };
    if pid != search.pid
        || unsafe { IsWindowVisible(window) } == 0
        || !unsafe { GetWindow(window, GW_OWNER) }.is_null()
    {
        return 1;
    }
    let mut class = [0; 32];
    let length = unsafe { GetClassNameW(window, class.as_mut_ptr(), class.len() as i32) };
    if !class[..length as usize]
        .iter()
        .copied()
        .eq(WINDOW_CLASS.encode_utf16())
    {
        return 1;
    }
    search.window = window;
    0
}

struct WindowIcons {
    large: HICON,
    small: HICON,
}

impl WindowIcons {
    fn default() -> Option<Self> {
        let mut module: HMODULE = ptr::null_mut();
        let found = unsafe {
            GetModuleHandleExW(
                GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS
                    | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                apply as *const () as *const u16,
                &mut module,
            )
        };
        if found == 0 {
            return None;
        }
        Self::load(module, ICON_RESOURCE, 0)
    }

    fn from_file(path: &Path) -> Option<Self> {
        let path: Vec<_> = path.as_os_str().encode_wide().chain(Some(0)).collect();
        Self::load(ptr::null_mut(), path.as_ptr(), LR_LOADFROMFILE)
    }

    fn load(module: HMODULE, name: *const u16, flags: u32) -> Option<Self> {
        let load = |width, height| unsafe {
            LoadImageW(
                module,
                name,
                IMAGE_ICON,
                GetSystemMetrics(width),
                GetSystemMetrics(height),
                flags,
            )
        };
        let large = load(SM_CXICON, SM_CYICON);
        if large.is_null() {
            return None;
        }
        let small = load(SM_CXSMICON, SM_CYSMICON);
        if small.is_null() {
            unsafe { DestroyIcon(large) };
            return None;
        }
        Some(Self { large, small })
    }

    fn apply(&self, window: HWND) -> bool {
        let mut applied = true;
        for (size, icon) in [(ICON_BIG, self.large), (ICON_SMALL, self.small)] {
            let size = size as usize;
            let icon = icon as isize;
            if unsafe { SendMessageW(window, WM_GETICON, size, 0) } == icon {
                continue;
            }
            unsafe { SendMessageW(window, WM_SETICON, size, icon) };
            applied &= unsafe { SendMessageW(window, WM_GETICON, size, 0) } == icon;
        }
        applied
    }
}

fn ordinary_file(path: &Path, limit: u64) -> bool {
    fs::symlink_metadata(path).is_ok_and(|meta| meta.is_file() && meta.len() <= limit)
        && path.ancestors().all(|part| {
            fs::symlink_metadata(part).is_ok_and(|meta| meta.file_attributes() & 0x400 == 0)
        })
}

fn game_design_directory() -> Option<PathBuf> {
    let config = crate::shared_settings::config().ok()?;
    let game = config.parent()?.parent()?;
    Some(game.join("html/panopticon"))
}

fn valid_icon_key(key: &str) -> bool {
    key.strip_suffix(".ico").is_some_and(|hash| {
        hash.len() == 64
            && hash
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    })
}

struct WindowIdentity {
    caption: Vec<u16>,
    icons: Option<WindowIcons>,
}

impl WindowIdentity {
    fn load() -> Self {
        let folder = game_design_directory();
        let config = folder.as_ref().and_then(|folder| {
            let path = folder.join("design-identity.json");
            if !ordinary_file(&path, 4096) {
                return None;
            }
            serde_json::from_reader::<_, Value>(File::open(path).ok()?.take(4096)).ok()
        });
        let caption = config
            .as_ref()
            .and_then(|config| config["name"].as_str())
            .filter(|name| {
                !name.trim().is_empty()
                    && name.encode_utf16().count() <= 64
                    && !name.chars().any(char::is_control)
            })
            .unwrap_or(TITLE)
            .encode_utf16()
            .chain(Some(0))
            .collect();
        let icons = config
            .as_ref()
            .and_then(|config| config["icon"].as_str())
            .filter(|key| valid_icon_key(key))
            .and_then(|key| {
                let path = folder?.join("design-media").join(key);
                if ordinary_file(&path, 1024 * 1024) {
                    WindowIcons::from_file(&path)
                } else {
                    None
                }
            })
            .or_else(WindowIcons::default);
        Self { caption, icons }
    }

    fn apply(&self, window: HWND) -> bool {
        let mut title = [0; 256];
        let length = unsafe { GetWindowTextW(window, title.as_mut_ptr(), title.len() as i32) };
        let title_applied = title[..length as usize] == self.caption[..self.caption.len() - 1]
            || unsafe { SetWindowTextW(window, self.caption.as_ptr()) != 0 };
        let icons_applied = self.icons.as_ref().is_some_and(|icons| icons.apply(window));
        title_applied && icons_applied
    }
}

thread_local! {
    // Окно держит HICON до завершения игрового процесса.
    static IDENTITY: RefCell<Option<WindowIdentity>> = const { RefCell::new(None) };
    static WINDOW: Cell<HWND> = const { Cell::new(ptr::null_mut()) };
}

fn game_window() -> Option<HWND> {
    WINDOW.with(|cached| {
        let window = cached.get();
        if !window.is_null() && unsafe { IsWindow(window) } != 0 {
            return Some(window);
        }
        let mut search = WindowSearch {
            pid: unsafe { GetCurrentProcessId() },
            window: ptr::null_mut(),
        };
        unsafe {
            EnumWindows(Some(find_window), (&mut search as *mut WindowSearch) as LPARAM);
        }
        cached.set(search.window);
        (!search.window.is_null()).then_some(search.window)
    })
}

pub fn visible() -> Option<bool> {
    let window = game_window()?;
    Some(unsafe { IsWindowVisible(window) != 0 && IsIconic(window) == 0 })
}

pub fn apply() -> bool {
    let Some(window) = game_window() else {
        return false;
    };

    IDENTITY.with(|cache| {
        let Ok(mut cache) = cache.try_borrow_mut() else {
            return false;
        };
        cache.get_or_insert_with(WindowIdentity::load).apply(window)
    })
}

#[cfg(test)]
mod tests {
    use super::valid_icon_key;

    #[test]
    fn accepts_only_owned_icon_filenames() {
        let hash = "a".repeat(64);
        assert!(valid_icon_key(&format!("{hash}.ico")));
        assert!(!valid_icon_key(&format!("../{hash}.ico")));
        assert!(!valid_icon_key(&format!("{hash}.ICO")));
        assert!(!valid_icon_key(&format!("{hash}.png")));
    }
}
