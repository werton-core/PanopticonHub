use crate::{InstallOptions, Receipt, Scope, HOST, PRODUCT, UNINSTALL};
use std::{
    ffi::OsString,
    fs::{self, OpenOptions},
    io::Error,
    num::NonZeroU32,
    os::windows::{
        ffi::{OsStrExt, OsStringExt},
        fs::{MetadataExt, OpenOptionsExt},
        io::{AsRawHandle, FromRawHandle, OwnedHandle},
    },
    path::{Path, PathBuf},
};
use windows::{
    core::{Interface, PCWSTR},
    Win32::{
        System::Com::{
            CoCreateInstance, CoInitializeEx, CoUninitialize, IPersistFile, CLSCTX_INPROC_SERVER,
            COINIT_APARTMENTTHREADED, STGM_READ,
        },
        UI::Shell::{
            FOLDERID_CommonPrograms, FOLDERID_Desktop, FOLDERID_Programs, FOLDERID_PublicDesktop,
            IShellLinkW, SHGetKnownFolderPath, ShellLink, KF_FLAG_DEFAULT, SLGP_RAWPATH,
        },
    },
};
use windows_sys::Win32::{
    Foundation::*,
    Globalization::{CompareStringOrdinal, GetUserDefaultUILanguage, CSTR_EQUAL},
    Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
    Storage::FileSystem::{
        FileDispositionInfo, MoveFileExW, SetFileInformationByHandle, DELETE,
        FILE_DISPOSITION_INFO, FILE_FLAG_OPEN_REPARSE_POINT, FILE_SHARE_READ,
        MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH,
    },
    System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
            TH32CS_SNAPPROCESS,
        },
        Registry::*,
        Threading::{
            CreateMutexW, GetCurrentProcess, GetExitCodeProcess, OpenProcess, OpenProcessToken,
            QueryFullProcessImageNameW, ReleaseMutex, WaitForSingleObject,
            INFINITE, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SYNCHRONIZE,
        },
    },
    UI::{
        Shell::{
            SHChangeNotify, ShellExecuteExW, SEE_MASK_NOCLOSEPROCESS, SHCNE_UPDATEITEM,
            SHCNF_FLUSHNOWAIT, SHCNF_PATHW, SHELLEXECUTEINFOW,
        },
        WindowsAndMessaging::SW_SHOWNORMAL,
    },
};

const PRODUCT_KEY: &str = "Software\\PanopticonTrial\\Wizard62Sandbox";
const UNINSTALL_KEY: &str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\PanopticonTrial.Wizard62Sandbox";
// Явный ID из host/build.rs обходит старую запись Shell для индекса 0.
const SHORTCUT_ICON_INDEX: i32 = -204;
pub fn wide(value: impl AsRef<std::ffi::OsStr>) -> Vec<u16> {
    value.as_ref().encode_wide().chain([0]).collect()
}

pub fn reject_links(path: &Path) -> Result<(), String> {
    let mut cursor = Some(path);
    while let Some(path) = cursor {
        match fs::symlink_metadata(path) {
            Ok(metadata) if metadata.file_attributes() & 0x400 != 0 => {
                return Err(format!(
                    "Ссылка или reparse point запрещены в пути установки: {}",
                    path.display()
                ))
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("Путь установки недоступен: {error}")),
        }
        cursor = path.parent();
    }
    Ok(())
}

pub fn launch_hub(executable: &Path, directory: &Path) -> Result<(), String> {
    reject_links(executable)?;
    std::process::Command::new(executable)
        .current_dir(directory)
        .spawn()
        .map_err(|error| format!("Не удалось запустить Panopticon Hub: {error}"))?;
    Ok(())
}

pub(crate) fn lock_verified_hub(path: &Path, expected: &str) -> Result<fs::File, String> {
    reject_links(path)?;
    let mut file = OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
        .map_err(|error| format!("Не удалось открыть прежний Hub: {error}"))?;
    let metadata = file.metadata().map_err(|error| error.to_string())?;
    if !metadata.is_file() || metadata.file_attributes() & 0x400 != 0
        || crate::digest_reader(&mut file)? != expected
    {
        return Err("Файл прежнего Hub отличается от манифеста установки.".into());
    }
    Ok(file)
}

fn process_image(handle: &OwnedHandle, buffer: &mut [u16]) -> Result<PathBuf, Error> {
    let mut length = buffer.len() as u32;
    if unsafe {
        QueryFullProcessImageNameW(handle.as_raw_handle(), 0, buffer.as_mut_ptr(), &mut length)
    } == 0
    {
        return Err(Error::last_os_error());
    }
    Ok(PathBuf::from(OsString::from_wide(&buffer[..length as usize])))
}

pub fn wait_for_hub(pid: NonZeroU32, expected: &Path, timeout_ms: u32) -> Result<(), String> {
    let handle = unsafe {
        OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_SYNCHRONIZE, 0, pid.get())
    };
    if handle.is_null() {
        let error = Error::last_os_error();
        if error.raw_os_error() == Some(ERROR_INVALID_PARAMETER as i32) {
            return Ok(());
        }
        return Err(format!("Не удалось открыть запущенный Hub: {error}"));
    }
    let handle = unsafe { OwnedHandle::from_raw_handle(handle) };
    let mut buffer = [0u16; 32768];
    let image = match process_image(&handle, &mut buffer) {
        Ok(path) => path,
        Err(_) if unsafe { WaitForSingleObject(handle.as_raw_handle(), 0) } == WAIT_OBJECT_0 => {
            return Ok(());
        }
        Err(error) => return Err(format!("Не удалось определить путь запущенного Hub: {error}")),
    };
    reject_links(expected)?;
    let expected = expected.canonicalize().map_err(|error| {
        format!("Не удалось проверить путь установленного Hub: {error}")
    })?;
    let image = image.canonicalize().map_err(|error| {
        format!("Не удалось проверить путь запущенного Hub: {error}")
    })?;
    if expected != image {
        return Err("PID принадлежит процессу вне выбранной установки Panopticon Hub.".into());
    }
    match unsafe { WaitForSingleObject(handle.as_raw_handle(), timeout_ms) } {
        WAIT_OBJECT_0 => Ok(()),
        WAIT_TIMEOUT => Err("Hub не завершился за отведённое время. Повторите обновление.".into()),
        _ => Err(format!(
            "Не удалось дождаться выхода Hub: {}",
            Error::last_os_error()
        )),
    }
}

pub(crate) fn is_executable_running(path: &Path) -> Result<bool, String> {
    let expected = match path.canonicalize() {
        Ok(expected) => expected,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(format!("Путь к Panopticon Hub недоступен: {error}")),
    };
    let Some(name) = expected.file_name() else {
        return Ok(false);
    };
    let expected_path = wide(&expected);
    let scan_error = |error| format!("Не удалось проверить запущенные процессы: {error}");
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            return Err(scan_error(Error::last_os_error()));
        }
        let snapshot = OwnedHandle::from_raw_handle(snapshot);
        let mut entry: PROCESSENTRY32W = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
        let mut image = [0u16; 32768];
        let mut found = Process32FirstW(snapshot.as_raw_handle(), &mut entry);
        while found != 0 {
            let end = entry.szExeFile.iter().position(|&c| c == 0).unwrap_or(entry.szExeFile.len());
            if name.eq_ignore_ascii_case(OsString::from_wide(&entry.szExeFile[..end])) {
                let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, entry.th32ProcessID);
                // Процесс мог завершиться после снимка. Без полного пути совпадение не подтверждено.
                if !handle.is_null() {
                    let handle = OwnedHandle::from_raw_handle(handle);
                    if let Ok(image_path) =
                        process_image(&handle, &mut image).and_then(|path| path.canonicalize())
                    {
                        let image_path = wide(image_path);
                        if CompareStringOrdinal(expected_path.as_ptr(), -1, image_path.as_ptr(), -1, 1) == CSTR_EQUAL {
                            return Ok(true);
                        }
                    }
                }
            }
            found = Process32NextW(snapshot.as_raw_handle(), &mut entry);
        }
        let error = Error::last_os_error();
        if error.raw_os_error() == Some(ERROR_NO_MORE_FILES as i32) {
            Ok(false)
        } else {
            Err(scan_error(error))
        }
    }
}

pub(crate) fn close_hub_message() -> &'static str {
    match unsafe { GetUserDefaultUILanguage() } & 0x03ff {
        0x19 => "Panopticon Hub сейчас запущен. Закройте хаб и повторите удаление.",
        0x0c => "Panopticon Hub est en cours d’exécution. Fermez-le et relancez la désinstallation.",
        _ => "Panopticon Hub is running. Close it and retry the uninstall.",
    }
}

pub fn move_file(source: &Path, target: &Path, replace: bool) -> Result<(), String> {
    reject_links(source)?;
    reject_links(target)?;
    let flags = MOVEFILE_WRITE_THROUGH
        | if replace {
            MOVEFILE_REPLACE_EXISTING
        } else {
            0
        };
    if unsafe { MoveFileExW(wide(source).as_ptr(), wide(target).as_ptr(), flags) } == 0 {
        Err(format!(
            "Не удалось заменить {}: {}. Закройте Panopticon Hub и нажмите «Повторить».",
            target.display(),
            std::io::Error::last_os_error()
        ))
    } else {
        Ok(())
    }
}

pub fn delete_matching(path: &Path, hashes: &[&str]) -> Result<bool, String> {
    reject_links(path)?;
    let mut file = match OpenOptions::new()
        .access_mode(GENERIC_READ | DELETE)
        .share_mode(FILE_SHARE_READ)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
    {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => {
            return Err(format!(
                "Не удалось открыть {} для удаления: {error}",
                path.display()
            ))
        }
    };
    let metadata = file.metadata().map_err(|e| e.to_string())?;
    if !metadata.is_file() || metadata.file_attributes() & 0x400 != 0 {
        return Ok(false);
    }
    let hash = crate::digest_reader(&mut file)?;
    if !hashes.contains(&hash.as_str()) {
        return Ok(false);
    }
    // Хеш и удаление относятся к одному открытому файлу. Запись и переименование закрыты.
    let info = FILE_DISPOSITION_INFO { DeleteFile: 1 };
    if unsafe {
        SetFileInformationByHandle(
            file.as_raw_handle(),
            FileDispositionInfo,
            (&info as *const FILE_DISPOSITION_INFO).cast(),
            std::mem::size_of::<FILE_DISPOSITION_INFO>() as u32,
        )
    } == 0
    {
        return Err(format!(
            "Не удалось удалить {}: {}. Закройте Panopticon Hub и повторите действие.",
            path.display(),
            std::io::Error::last_os_error(),
        ));
    }
    Ok(true)
}

pub struct InstallLock(HANDLE);
impl InstallLock {
    pub fn take() -> Result<Self, String> {
        let name = wide(r"Global\PanopticonTrial.Wizard62Sandbox.Setup");
        unsafe {
            let handle = CreateMutexW(std::ptr::null(), 0, name.as_ptr());
            if handle.is_null() {
                return Err(format!(
                    "Не удалось начать установку: {}",
                    std::io::Error::last_os_error()
                ));
            }
            if matches!(
                WaitForSingleObject(handle, 5_000),
                WAIT_OBJECT_0 | WAIT_ABANDONED
            ) {
                return Ok(Self(handle));
            }
            CloseHandle(handle);
        }
        Err("Другая установка Panopticon Hub уже выполняется. Повторите действие.".into())
    }
}
impl Drop for InstallLock {
    fn drop(&mut self) {
        unsafe {
            ReleaseMutex(self.0);
            CloseHandle(self.0);
        }
    }
}

pub fn is_admin() -> bool {
    unsafe {
        let mut token = std::ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
            return false;
        }
        let mut elevation: TOKEN_ELEVATION = std::mem::zeroed();
        let mut length = 0;
        let ok = GetTokenInformation(
            token,
            TokenElevation,
            (&mut elevation as *mut TOKEN_ELEVATION).cast(),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut length,
        ) != 0;
        CloseHandle(token);
        ok && elevation.TokenIsElevated != 0
    }
}

fn folder(scope: Scope, desktop: bool) -> Result<PathBuf, String> {
    let id = match (scope, desktop) {
        (Scope::User, true) => FOLDERID_Desktop,
        (Scope::Machine, true) => FOLDERID_PublicDesktop,
        (Scope::User, false) => FOLDERID_Programs,
        (Scope::Machine, false) => FOLDERID_CommonPrograms,
    };
    unsafe {
        let path = SHGetKnownFolderPath(&id, KF_FLAG_DEFAULT, None).map_err(|e| e.to_string())?;
        let result = PathBuf::from(path.to_string().map_err(|e| e.to_string())?);
        windows::Win32::System::Com::CoTaskMemFree(Some(path.0.cast()));
        Ok(result)
    }
}

pub fn shortcut_paths(options: &InstallOptions) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    if options.desktop_shortcut {
        paths.push(folder(options.scope, true)?.join("Panopticon Hub.lnk"));
    }
    if options.start_menu_shortcut {
        paths.push(folder(options.scope, false)?.join("Panopticon Hub.lnk"));
    }
    Ok(paths)
}

pub fn allowed_shortcut(path: &Path, scope: Scope) -> Result<bool, String> {
    let key = |path: &Path| path.to_string_lossy().to_lowercase();
    Ok([folder(scope, true)?, folder(scope, false)?]
        .into_iter()
        .any(|base| key(path) == key(&base.join("Panopticon Hub.lnk"))))
}

fn with_shell_link<T>(action: impl FnOnce(IShellLinkW) -> Result<T, String>) -> Result<T, String> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED)
            .ok()
            .map_err(|e| e.to_string())?;
        let result = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)
            .map_err(|e| e.to_string())
            .and_then(action);
        CoUninitialize();
        result
    }
}

pub fn create_shortcut(path: &Path, target: &Path, working_directory: &Path) -> Result<(), String> {
    reject_links(path)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    with_shell_link(|link| unsafe {
        link.SetPath(PCWSTR(wide(target).as_ptr()))
            .map_err(|e| e.to_string())?;
        link.SetWorkingDirectory(PCWSTR(wide(working_directory).as_ptr()))
            .map_err(|e| e.to_string())?;
        link.SetDescription(PCWSTR(wide(PRODUCT).as_ptr()))
            .map_err(|e| e.to_string())?;
        link.SetIconLocation(PCWSTR(wide(target).as_ptr()), SHORTCUT_ICON_INDEX)
            .map_err(|e| e.to_string())?;
        use windows::Win32::{
            Foundation::PROPERTYKEY, System::Com::StructuredStorage::PROPVARIANT,
            UI::Shell::PropertiesSystem::IPropertyStore,
        };
        let store: IPropertyStore = link.cast().map_err(|e| e.to_string())?;
        let key = PROPERTYKEY {
            fmtid: windows::core::GUID::from_u128(0x9f4c2855_9f79_4b39_a8d0_e1d42de1d5f3),
            pid: 5,
        };
        store
            .SetValue(&key, &PROPVARIANT::from("org.panopticon.hub"))
            .map_err(|e| e.to_string())?;
        store.Commit().map_err(|e| e.to_string())?;
        let persist: IPersistFile = link.cast().map_err(|e| e.to_string())?;
        persist
            .Save(PCWSTR(wide(path).as_ptr()), true)
            .map_err(|e| e.to_string())
    })
}

pub fn shortcut_is_current(path: &Path, target: &Path) -> Result<bool, String> {
    reject_links(path)?;
    with_shell_link(|link| unsafe {
        let persist: IPersistFile = link.cast().map_err(|e| e.to_string())?;
        persist
            .Load(PCWSTR(wide(path).as_ptr()), STGM_READ)
            .map_err(|e| e.to_string())?;
        let matches_target = |buffer: &[u16]| {
            let value = String::from_utf16_lossy(buffer);
            crate::path_key(Path::new(value.trim_end_matches('\0'))) == crate::path_key(target)
        };
        let mut buffer = vec![0; 32768];
        let mut index = 0;
        link.GetIconLocation(&mut buffer, &mut index)
            .map_err(|e| e.to_string())?;
        if index != SHORTCUT_ICON_INDEX || !matches_target(&buffer) {
            return Ok(false);
        }
        buffer.fill(0);
        link.GetPath(&mut buffer, std::ptr::null_mut(), SLGP_RAWPATH.0 as u32)
            .map_err(|e| e.to_string())?;
        Ok(matches_target(&buffer))
    })
}

pub fn notify_shortcut(path: &Path) {
    unsafe {
        SHChangeNotify(
            SHCNE_UPDATEITEM as i32,
            SHCNF_PATHW | SHCNF_FLUSHNOWAIT,
            wide(path).as_ptr().cast(),
            std::ptr::null(),
        );
    }
}

pub fn choose_directory(
    hwnd: usize,
    title: &str,
    initial: &Path,
) -> Result<Option<PathBuf>, String> {
    use windows_sys::Win32::UI::{Shell::*, WindowsAndMessaging::SendMessageW};
    unsafe extern "system" fn select_initial(
        dialog: HWND,
        message: u32,
        _: LPARAM,
        path: LPARAM,
    ) -> i32 {
        if message == BFFM_INITIALIZED {
            SendMessageW(dialog, BFFM_SETSELECTIONW, 1, path);
        }
        0
    }
    let title = wide(title);
    let initial = wide(initial);
    let mut display = [0u16; 260];
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED)
            .ok()
            .map_err(|e| e.to_string())?;
        let info = BROWSEINFOW {
            hwndOwner: hwnd as HWND,
            pszDisplayName: display.as_mut_ptr(),
            lpszTitle: title.as_ptr(),
            ulFlags: BIF_RETURNONLYFSDIRS | BIF_NEWDIALOGSTYLE,
            lpfn: Some(select_initial),
            lParam: initial.as_ptr() as LPARAM,
            ..std::mem::zeroed()
        };
        let item = SHBrowseForFolderW(&info);
        let mut buffer = [0u16; 260];
        let selected = !item.is_null() && SHGetPathFromIDListW(item, buffer.as_mut_ptr()) != 0;
        windows::Win32::System::Com::CoTaskMemFree(Some(item.cast()));
        CoUninitialize();
        use std::os::windows::ffi::OsStringExt;
        Ok(selected.then(|| {
            PathBuf::from(std::ffi::OsString::from_wide(
                &buffer[..buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len())],
            ))
        }))
    }
}
pub fn default_directory(scope: Scope) -> Result<PathBuf, String> {
    if let Some(path) = registered_location(scope) {
        return Ok(path);
    }
    let key = if scope == Scope::Machine {
        "ProgramW6432"
    } else {
        "LOCALAPPDATA"
    };
    let base = std::env::var_os(key)
        .or_else(|| {
            std::env::var_os(if scope == Scope::Machine {
                "ProgramFiles"
            } else {
                "USERPROFILE"
            })
        })
        .ok_or("Windows не определила папку установки.")?;
    Ok(PathBuf::from(base).join(if scope == Scope::Machine {
        "Panopticon Hub"
    } else {
        "Programs\\Panopticon Hub"
    }))
}

fn hive(scope: Scope) -> HKEY {
    if scope == Scope::Machine {
        HKEY_LOCAL_MACHINE
    } else {
        HKEY_CURRENT_USER
    }
}
struct Key(HKEY);
impl Drop for Key {
    fn drop(&mut self) {
        unsafe {
            RegCloseKey(self.0);
        }
    }
}
fn open_key(scope: Scope, path: &str, write: bool) -> Result<Key, String> {
    unsafe {
        let mut key = std::ptr::null_mut();
        let result = if write {
            RegCreateKeyExW(
                hive(scope),
                wide(path).as_ptr(),
                0,
                std::ptr::null_mut(),
                REG_OPTION_NON_VOLATILE,
                KEY_WRITE | KEY_WOW64_64KEY,
                std::ptr::null(),
                &mut key,
                std::ptr::null_mut(),
            )
        } else {
            RegOpenKeyExW(
                hive(scope),
                wide(path).as_ptr(),
                0,
                KEY_READ | KEY_WOW64_64KEY,
                &mut key,
            )
        };
        if result != 0 {
            Err(format!("Ошибка реестра Windows: {result}"))
        } else {
            Ok(Key(key))
        }
    }
}
fn write_string(key: &Key, name: &str, value: &str) -> Result<(), String> {
    let value = wide(value);
    let result = unsafe {
        RegSetValueExW(
            key.0,
            wide(name).as_ptr(),
            0,
            REG_SZ,
            value.as_ptr().cast(),
            (value.len() * 2) as u32,
        )
    };
    if result != 0 {
        Err(format!("Не удалось зарегистрировать приложение: {result}"))
    } else {
        Ok(())
    }
}
fn write_dword(key: &Key, name: &str, value: u32) -> Result<(), String> {
    let result = unsafe {
        RegSetValueExW(
            key.0,
            wide(name).as_ptr(),
            0,
            REG_DWORD,
            (&value as *const u32).cast(),
            4,
        )
    };
    if result != 0 {
        Err(format!("Не удалось зарегистрировать приложение: {result}"))
    } else {
        Ok(())
    }
}
fn read_string(scope: Scope, path: &str, name: &str) -> Option<String> {
    let key = open_key(scope, path, false).ok()?;
    let mut data = vec![0u16; 32768];
    let mut size = (data.len() * 2) as u32;
    let mut kind = 0;
    let status = unsafe {
        RegQueryValueExW(
            key.0,
            wide(name).as_ptr(),
            std::ptr::null(),
            &mut kind,
            data.as_mut_ptr().cast(),
            &mut size,
        )
    };
    if status != 0 || kind != REG_SZ {
        return None;
    }
    let length = (size as usize / 2).min(data.len());
    Some(
        String::from_utf16_lossy(&data[..length])
            .trim_end_matches('\0')
            .to_owned(),
    )
}
pub fn registration_locations(scope: Scope) -> impl Iterator<Item = PathBuf> {
    [PRODUCT_KEY, UNINSTALL_KEY]
        .into_iter()
        .filter_map(move |key| read_string(scope, key, "InstallLocation").map(PathBuf::from))
}

pub fn registered_location(scope: Scope) -> Option<PathBuf> {
    registration_locations(scope).next()
}

pub fn register(receipt: &Receipt) -> Result<(), String> {
    let key = open_key(receipt.scope, PRODUCT_KEY, true)?;
    write_string(
        &key,
        "InstallLocation",
        &receipt.directory.to_string_lossy(),
    )?;
    let key = open_key(receipt.scope, UNINSTALL_KEY, true)?;
    let uninstall = receipt.directory.join(UNINSTALL);
    let scope = if receipt.scope == Scope::Machine {
        "--all-users"
    } else {
        "--current-user"
    };
    for (name, value) in [
        ("DisplayName", PRODUCT.to_owned()),
        ("Publisher", "Panopticon Trial".into()),
        ("DisplayVersion", receipt.version.clone()),
        (
            "InstallLocation",
            receipt.directory.to_string_lossy().into_owned(),
        ),
        (
            "DisplayIcon",
            receipt.directory.join(HOST).to_string_lossy().into_owned(),
        ),
        (
            "UninstallString",
            format!("\"{}\" {scope}", uninstall.display()),
        ),
        (
            "QuietUninstallString",
            format!("\"{}\" {scope} --silent", uninstall.display()),
        ),
    ] {
        write_string(&key, name, &value)?;
    }
    write_dword(&key, "NoModify", 1)?;
    write_dword(&key, "NoRepair", 1)?;
    write_dword(
        &key,
        "EstimatedSize",
        (receipt
            .binaries()
            .map(|(name, _)| receipt.directory.join(name))
            .filter_map(|path| fs::metadata(path).ok().map(|metadata| metadata.len()))
            .sum::<u64>()
            / 1024)
            .min(u32::MAX as u64) as u32,
    )?;
    Ok(())
}

pub fn unregister(receipt: &Receipt) -> Result<(), String> {
    // Каждый ключ проверяется отдельно: повтор безопасен после частичного удаления.
    let expected = crate::path_key(&receipt.directory);
    for path in [UNINSTALL_KEY, PRODUCT_KEY] {
        if read_string(receipt.scope, path, "InstallLocation")
            .map(|location| crate::path_key(Path::new(&location)))
            .as_ref()
            != Some(&expected)
        {
            continue;
        }
        let status = unsafe {
            RegDeleteKeyExW(hive(receipt.scope), wide(path).as_ptr(), KEY_WOW64_64KEY, 0)
        };
        if status != 0 && status != ERROR_FILE_NOT_FOUND {
            return Err(format!("Не удалось удалить регистрацию: {status}"));
        }
    }
    Ok(())
}

pub fn elevate(args: &[String], wait_for_install: bool) -> Result<(), String> {
    let executable = std::env::current_exe().map_err(|e| e.to_string())?;
    let executable = wide(executable);
    let verb = wide("runas");
    let parameters = wide(
        args.iter()
            .map(|arg| quote(arg))
            .collect::<Vec<_>>()
            .join(" "),
    );
    unsafe {
        let mut info: SHELLEXECUTEINFOW = std::mem::zeroed();
        info.cbSize = std::mem::size_of::<SHELLEXECUTEINFOW>() as u32;
        info.fMask = SEE_MASK_NOCLOSEPROCESS;
        info.lpVerb = verb.as_ptr();
        info.lpFile = executable.as_ptr();
        info.lpParameters = parameters.as_ptr();
        info.nShow = SW_SHOWNORMAL;
        if ShellExecuteExW(&mut info) == 0 {
            return Err(format!(
                "Повышение прав не выполнено: {}",
                std::io::Error::last_os_error()
            ));
        }
        if info.hProcess.is_null() {
            return if wait_for_install {
                Err("Не удалось получить процесс установки с повышенными правами.".into())
            } else {
                Ok(())
            };
        }
        let process = OwnedHandle::from_raw_handle(info.hProcess);
        if wait_for_install {
            let mut exit_code = 0;
            if WaitForSingleObject(process.as_raw_handle(), INFINITE) != WAIT_OBJECT_0
                || GetExitCodeProcess(process.as_raw_handle(), &mut exit_code) == 0
            {
                return Err(format!(
                    "Не удалось проверить результат установки: {}",
                    Error::last_os_error()
                ));
            }
            if exit_code != 0 {
                return Err(format!("Установка не завершилась (код {exit_code})."));
            }
        }
        Ok(())
    }
}
pub fn quote(value: &str) -> String {
    let mut result = String::from("\"");
    let mut slashes = 0;
    for character in value.chars() {
        if character == '\\' {
            slashes += 1;
        } else {
            if character == '"' {
                result.push_str(&"\\".repeat(slashes * 2 + 1));
            } else {
                result.push_str(&"\\".repeat(slashes));
            }
            slashes = 0;
            result.push(character);
        }
    }
    result.push_str(&"\\".repeat(slashes * 2));
    result.push('"');
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn updater_waits_only_for_the_hub_in_the_selected_directory() {
        use std::{
            os::windows::process::CommandExt,
            process::{Command, Stdio},
        };
        use windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;

        let root = std::env::temp_dir().join(format!("panopticon-wait-test-{}", std::process::id()));
        let second = root.join("other");
        fs::create_dir_all(&second).unwrap();
        let cmd = PathBuf::from(std::env::var_os("SystemRoot").unwrap()).join("System32/cmd.exe");
        let expected = root.join(HOST);
        let other = second.join(HOST);
        fs::copy(&cmd, &expected).unwrap();
        fs::copy(cmd, &other).unwrap();
        let mut child = Command::new(&expected)
            .args(["/d", "/q"])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .unwrap();
        let pid = NonZeroU32::new(child.id()).unwrap();

        let error = wait_for_hub(pid, &other, 0).unwrap_err();
        assert!(error.contains("вне выбранной установки"), "{error}");
        let equivalent = root.join(HOST.to_ascii_lowercase());
        let error = wait_for_hub(pid, &equivalent, 0).unwrap_err();
        assert!(error.contains("не завершился"), "{error}");

        drop(child.stdin.take());
        wait_for_hub(pid, &expected, 5_000).unwrap();
        assert!(child.wait().unwrap().success());
        drop(child);
        wait_for_hub(NonZeroU32::new(u32::MAX).unwrap(), &expected, 0).unwrap();

        fs::remove_file(expected).unwrap();
        fs::remove_file(other).unwrap();
        fs::remove_dir(second).unwrap();
        fs::remove_dir(root).unwrap();
    }

    #[test]
    fn shortcut_has_real_target_and_working_directory() {
        let root =
            std::env::temp_dir().join(format!("panopticon-shortcut-test-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let target = root.join("PanopticonHub.exe");
        let shortcut = root.join("Panopticon Hub.lnk");
        create_shortcut(&shortcut, &target, &root).unwrap();
        fs::write(&target, b"MZfixture").unwrap();
        assert!(shortcut_is_current(&shortcut, &target).unwrap());
        unsafe {
            CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok().unwrap();
            let link: IShellLinkW =
                CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER).unwrap();
            let persist: IPersistFile = link.cast().unwrap();
            persist
                .Load(
                    PCWSTR(wide(&shortcut).as_ptr()),
                    windows::Win32::System::Com::STGM_READ,
                )
                .unwrap();
            let mut path = [0u16; 32768];
            link.GetPath(
                &mut path,
                std::ptr::null_mut(),
                windows::Win32::UI::Shell::SLGP_RAWPATH.0 as u32,
            )
            .unwrap();
            assert_eq!(
                PathBuf::from(String::from_utf16_lossy(&path).trim_end_matches('\0'))
                    .canonicalize()
                    .unwrap(),
                target.canonicalize().unwrap()
            );
            let mut work = [0u16; 32768];
            link.GetWorkingDirectory(&mut work).unwrap();
            assert_eq!(
                PathBuf::from(String::from_utf16_lossy(&work).trim_end_matches('\0'))
                    .canonicalize()
                    .unwrap(),
                root.canonicalize().unwrap()
            );
            link.SetIconLocation(PCWSTR(wide(&target).as_ptr()), 0).unwrap();
            persist
                .Save(PCWSTR(wide(&shortcut).as_ptr()), true)
                .unwrap();
            drop(persist);
            drop(link);
            CoUninitialize();
        }
        assert!(!shortcut_is_current(&shortcut, &target).unwrap());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn registry_round_trip_uses_isolated_test_key() {
        let path = format!(
            "Software\\PanopticonTrial\\SetupTest-{}",
            std::process::id()
        );
        let key = open_key(Scope::User, &path, true).unwrap();
        write_string(&key, "InstallLocation", "C:\\Тест\\Panopticon Hub").unwrap();
        drop(key);
        assert_eq!(
            read_string(Scope::User, &path, "InstallLocation").as_deref(),
            Some("C:\\Тест\\Panopticon Hub")
        );
        unsafe {
            assert_eq!(
                RegDeleteKeyExW(HKEY_CURRENT_USER, wide(&path).as_ptr(), KEY_WOW64_64KEY, 0),
                0
            );
        }
    }
}
