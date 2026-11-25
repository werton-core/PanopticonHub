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
    Foundation::{BOOL, HWND, LPARAM, WAIT_OBJECT_0, WAIT_TIMEOUT},
    System::Threading::{
        GetExitCodeProcess, OpenProcess, QueryFullProcessImageNameW, TerminateProcess,
        WaitForSingleObject, INFINITE, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SYNCHRONIZE,
        PROCESS_TERMINATE,
    },
    UI::WindowsAndMessaging::{
        EnumWindows, GetClassNameW, GetWindow, GetWindowThreadProcessId, IsWindowVisible,
        PostMessageW, GW_OWNER, WM_CLOSE,
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

pub const CLOSED_FOR_UPDATE: u32 = 0x5048_0001;

pub struct GameProcess {
    pid: u32,
    handle: OwnedHandle,
}

pub fn game_running(root: &Path) -> Result<bool, String> {
    Ok(!processes_at(&[root], false, true)?.is_empty())
}

pub fn update_processes(game_root: &Path, closing: bool) -> Result<Vec<GameProcess>, String> {
    let shinri = game_root.parent().map(|common| common.join("Shinri Trial Host"));
    let roots: Vec<&Path> = std::iter::once(game_root).chain(shinri.as_deref()).collect();
    processes_at(&roots, closing, !closing)
}

fn processes_at(
    roots: &[&Path],
    closing: bool,
    stop_at_first: bool,
) -> Result<Vec<GameProcess>, String> {
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
    let mut paths = Vec::new();
    let mut problem = None;
    for root in roots {
        match fs::canonicalize(root) {
            Ok(path) => paths.push(normalize_windows_path(&path.to_string_lossy())),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => problem = Some(format!("Не определён каталог игры: {error}")),
        }
    }
    if paths.is_empty() {
        return problem.map_or_else(|| Ok(Vec::new()), Err);
    }
    let scan_error = |error| format!("Не проверен список игровых процессов: {error}");
    let mut found = Vec::new();
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
                let access = PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_SYNCHRONIZE;
                let mut process = OpenProcess(
                    if closing { access | PROCESS_TERMINATE } else { access },
                    0,
                    entry.pid,
                );
                if process.is_null() && closing {
                    process = OpenProcess(access, 0, entry.pid);
                }
                if process.is_null() {
                    let error = Error::last_os_error();
                    // Процесс из снимка может завершиться до открытия его PID.
                    if error.raw_os_error() != Some(87) {
                        problem = Some(scan_error(error));
                    }
                } else {
                    let candidate = GameProcess {
                        pid: entry.pid,
                        handle: OwnedHandle::from_raw_handle(process),
                    };
                    match process_image(&candidate.handle) {
                        Ok(image) if is_game_image(&paths, &image) => {
                            if candidate.running()? {
                                found.push(candidate);
                                if stop_at_first {
                                    return Ok(found);
                                }
                            }
                        }
                        Err(error) if candidate.running()? => problem = Some(scan_error(error)),
                        _ => {}
                    }
                }
            }
            if Process32NextW(snapshot.as_raw_handle(), &mut entry) == 0 {
                let error = Error::last_os_error();
                if error.raw_os_error() == Some(18) {
                    return problem.map_or_else(|| Ok(found), Err);
                }
                return Err(scan_error(error));
            }
        }
    }
}

impl GameProcess {
    fn running(&self) -> Result<bool, String> {
        match unsafe { WaitForSingleObject(self.handle.as_raw_handle(), 0) } {
            WAIT_OBJECT_0 => Ok(false),
            WAIT_TIMEOUT => Ok(true),
            _ => Err(format!(
                "Не удалось проверить процесс {} — {}.",
                self.pid, io::Error::last_os_error(),
            )),
        }
    }
}

pub fn close(
    mut processes: Vec<GameProcess>,
    force: bool,
    timeout: std::time::Duration,
) -> Result<Vec<GameProcess>, String> {
    unsafe extern "system" fn ask(window: HWND, context: LPARAM) -> BOOL {
        let processes = unsafe { &*(context as *const Vec<GameProcess>) };
        let mut pid = 0;
        if unsafe { GetWindowThreadProcessId(window, &mut pid) } != 0
            && unsafe { IsWindowVisible(window) } != 0
            && unsafe { GetWindow(window, GW_OWNER) }.is_null()
            && processes.iter().any(|process| process.pid == pid && process.running() == Ok(true))
        {
            unsafe { PostMessageW(window, WM_CLOSE, 0, 0) };
        }
        1
    }
    if force {
        for process in &processes {
            if process.running()?
                && unsafe { TerminateProcess(process.handle.as_raw_handle(), CLOSED_FOR_UPDATE) } == 0
            {
                let error = std::io::Error::last_os_error();
                if process.running()? {
                    return Err(format!("Не удалось завершить процесс {} — {error}.", process.pid));
                }
            }
        }
    } else {
        unsafe { EnumWindows(Some(ask), (&processes as *const Vec<GameProcess>) as LPARAM) };
    }
    let deadline = std::time::Instant::now() + timeout;
    loop {
        for index in (0..processes.len()).rev() {
            if !processes[index].running()? {
                processes.swap_remove(index);
            }
        }
        if processes.is_empty() || std::time::Instant::now() >= deadline {
            return Ok(processes);
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
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

fn is_game_image(roots: &[String], image: &str) -> bool {
    let matches_root = |image: &str| roots.iter().any(|root| belongs_to_game(root, image));
    // Windows can report the short path used to start the process.
    matches_root(image)
        || fs::canonicalize(image).is_ok_and(|path| matches_root(&path.to_string_lossy()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::windows::process::CommandExt;
    use std::process::{Command, Stdio};
    use std::time::Duration;

    #[test]
    fn close_timeout_keeps_only_verified_processes_for_force() {
        let temp = tempfile::tempdir().unwrap();
        let source = Path::new(&std::env::var_os("SystemRoot").unwrap()).join("System32/ping.exe");
        let root = temp.path().join("GarrysMod");
        let other_root = temp.path().join("GarrysMod-other");
        let start = |dir: &Path| {
            fs::create_dir(dir).unwrap();
            let exe = dir.join("gmod.exe");
            fs::copy(&source, &exe).unwrap();
            Command::new(exe)
                .args(["-n", "10", "127.0.0.1"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .creation_flags(0x0800_0000)
                .spawn()
                .unwrap()
        };
        let mut game = start(&root);
        let mut other = start(&other_root);
        let processes = update_processes(&root, true).unwrap();
        assert_eq!(processes.iter().map(|p| p.pid).collect::<Vec<_>>(), [game.id()]);
        let remaining = close(processes, false, Duration::ZERO).unwrap();
        assert_eq!(remaining.len(), 1);
        assert!(close(remaining, true, Duration::from_secs(2)).unwrap().is_empty());
        assert_eq!(game.wait().unwrap().code(), Some(CLOSED_FOR_UPDATE as i32));
        assert!(update_processes(&root, false).unwrap().is_empty());
        assert!(other.try_wait().unwrap().is_none());
        other.kill().unwrap();
        other.wait().unwrap();
    }
}
