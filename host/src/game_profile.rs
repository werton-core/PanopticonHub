use std::{
    fs, io,
    os::windows::{
        io::{AsRawHandle, FromRawHandle, OwnedHandle},
        process::ExitStatusExt,
    },
    path::Path,
    process::ExitStatus,
};
use windows_sys::Win32::{
    Foundation::{BOOL, HWND, LPARAM, WAIT_OBJECT_0},
    System::Threading::{
        GetExitCodeProcess, OpenProcess, QueryFullProcessImageNameW, WaitForSingleObject,
        INFINITE, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SYNCHRONIZE,
    },
    UI::WindowsAndMessaging::{
        EnumWindows, GetClassNameW, GetWindow, GetWindowThreadProcessId, IsWindowVisible, GW_OWNER,
    },
};

pub fn check_running_at(root: &Path) -> Result<(), String> {
    if game_running(root)? {
        return Err("GMOD_STOCK_RUNNING: закройте Garry’s Mod перед запуском Panopticon".into());
    }
    if let Some(common) = root.parent() {
        if game_running(&common.join("Shinri Trial Host"))? {
            return Err(
                "GMOD_STOCK_RUNNING: закройте Shinri Trial перед запуском Panopticon".into(),
            );
        }
    }
    Ok(())
}

pub fn game_running(root: &Path) -> Result<bool, String> {
    use std::ffi::c_void;
    use std::io::Error;
    type Handle = *mut c_void;
    #[repr(C)]
    struct Entry {
        size: u32,
        usage: u32,
        pid: u32,
        heap: usize,
        module: u32,
        threads: u32,
        parent: u32,
        priority: i32,
        flags: u32,
        name: [u16; 260],
    }
    #[link(name = "kernel32")]
    extern "system" {
        fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> Handle;
        fn Process32FirstW(handle: Handle, entry: *mut Entry) -> i32;
        fn Process32NextW(handle: Handle, entry: *mut Entry) -> i32;
    }
    let expected = match fs::canonicalize(root) {
        Ok(root) => normalize_windows_path(&root.to_string_lossy()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(format!("Не определён каталог игры: {error}")),
    };
    let scan_error = |error| format!("Не проверен список игровых процессов: {error}");
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(2, 0);
        if snapshot as isize == -1 {
            return Err(scan_error(Error::last_os_error()));
        }
        let snapshot = OwnedHandle::from_raw_handle(snapshot);
        let mut entry: Entry = std::mem::zeroed();
        entry.size = std::mem::size_of::<Entry>() as u32;
        if Process32FirstW(snapshot.as_raw_handle(), &mut entry) == 0 {
            return Err(scan_error(Error::last_os_error()));
        }
        loop {
            let end = entry.name.iter().position(|c| *c == 0).unwrap_or(260);
            let name = String::from_utf16_lossy(&entry.name[..end]).to_ascii_lowercase();
            if matches!(
                name.as_str(),
                "gmod.exe" | "gmod_win64.exe" | "hl2.exe" | "shinri-trial.exe"
            ) {
                let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, entry.pid);
                if process.is_null() {
                    let error = Error::last_os_error();
                    // Процесс из снимка может завершиться до открытия его PID.
                    if error.raw_os_error() != Some(87) {
                        return Err(scan_error(error));
                    }
                } else {
                    let process = OwnedHandle::from_raw_handle(process);
                    let image = process_image(&process).map_err(scan_error)?;
                    if belongs_to_game(&expected, &image) {
                        return Ok(true);
                    }
                }
            }
            if Process32NextW(snapshot.as_raw_handle(), &mut entry) == 0 {
                let error = Error::last_os_error();
                if error.raw_os_error() == Some(18) {
                    return Ok(false);
                }
                return Err(scan_error(error));
            }
        }
    }
}

fn process_image(process: &OwnedHandle) -> io::Result<String> {
    let mut text = [0u16; 32768];
    let mut length = text.len() as u32;
    if unsafe {
        QueryFullProcessImageNameW(process.as_raw_handle(), 0, text.as_mut_ptr(), &mut length)
    } == 0
    {
        return Err(io::Error::last_os_error());
    }
    Ok(String::from_utf16_lossy(&text[..length as usize]))
}

pub struct Client {
    pub pid: u32,
    handle: OwnedHandle,
}

impl Client {
    pub fn wait(self) -> io::Result<ExitStatus> {
        let handle = self.handle.as_raw_handle();
        if unsafe { WaitForSingleObject(handle, INFINITE) } != WAIT_OBJECT_0 {
            return Err(io::Error::last_os_error());
        }
        let mut status = 0;
        if unsafe { GetExitCodeProcess(handle, &mut status) } == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(ExitStatus::from_raw(status))
    }
}

pub fn find_client(executable: &Path) -> io::Result<Option<Client>> {
    struct Search<'a> {
        executable: &'a str,
        client: Option<Client>,
    }

    unsafe extern "system" fn find_window(window: HWND, context: LPARAM) -> BOOL {
        if unsafe { IsWindowVisible(window) } == 0 || !unsafe { GetWindow(window, GW_OWNER) }.is_null() {
            return 1;
        }
        // Служебные процессы Chromium могут иметь то же имя файла.
        let mut class = [0u16; 32];
        let length = unsafe { GetClassNameW(window, class.as_mut_ptr(), class.len() as i32) };
        if !class[..length as usize].iter().copied().eq("Valve001".encode_utf16()) {
            return 1;
        }

        let mut pid = 0;
        if unsafe { GetWindowThreadProcessId(window, &mut pid) } == 0 {
            return 1;
        }
        let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_SYNCHRONIZE, 0, pid) };
        if handle.is_null() {
            return 1;
        }
        let handle = unsafe { OwnedHandle::from_raw_handle(handle) };
        let search = unsafe { &mut *(context as *mut Search<'_>) };
        if process_image(&handle).is_ok_and(|image| normalize_windows_path(&image) == search.executable) {
            search.client = Some(Client { pid, handle });
            return 0;
        }
        1
    }

    let executable = normalize_windows_path(&fs::canonicalize(executable)?.to_string_lossy());
    let mut search = Search { executable: &executable, client: None };
    unsafe { EnumWindows(Some(find_window), (&mut search as *mut Search<'_>) as LPARAM) };
    Ok(search.client)
}

fn normalize_windows_path(path: &str) -> String {
    path.trim_start_matches("\\\\?\\")
        .replace('/', "\\")
        .to_lowercase()
}

#[cfg(windows)]
fn belongs_to_game(root: &str, image: &str) -> bool {
    let image = normalize_windows_path(image);
    image
        .strip_prefix(root)
        .is_some_and(|path| path.starts_with('\\'))
}
