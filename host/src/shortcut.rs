use std::{
    ffi::OsStr,
    io,
    os::windows::ffi::OsStrExt,
    path::{Path, PathBuf},
};
use windows::{
    core::{Interface, PCWSTR},
    Win32::{
        System::Com::{
            CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize, IPersistFile,
            CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, STGM_READ,
        },
        UI::Shell::{
            FOLDERID_Desktop, IShellLinkW, SHChangeNotify, SHGetKnownFolderPath, ShellLink,
            KF_FLAG_DEFAULT, SHCNE_UPDATEITEM, SHCNF_FLUSHNOWAIT, SHCNF_PATHW, SLGP_RAWPATH,
        },
    },
};

const ICON_INDEX: i32 = -204;

fn wide(value: &OsStr) -> Vec<u16> {
    value.encode_wide().chain([0]).collect()
}

fn windows_error(error: windows::core::Error) -> io::Error {
    io::Error::other(error.to_string())
}

fn notify_shortcut(path: &Path) {
    unsafe {
        SHChangeNotify(
            SHCNE_UPDATEITEM,
            SHCNF_PATHW | SHCNF_FLUSHNOWAIT,
            Some(wide(path.as_os_str()).as_ptr().cast()),
            None,
        );
    }
}

struct ComApartment;

impl ComApartment {
    fn enter() -> io::Result<Self> {
        unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok() }.map_err(windows_error)?;
        Ok(Self)
    }
}

impl Drop for ComApartment {
    fn drop(&mut self) {
        unsafe { CoUninitialize() }
    }
}

pub fn desktop_shortcut() -> io::Result<PathBuf> {
    let _com = ComApartment::enter()?;
    unsafe {
        let desktop = SHGetKnownFolderPath(&FOLDERID_Desktop, KF_FLAG_DEFAULT, None)
            .map_err(windows_error)?;
        let directory = desktop
            .to_string()
            .map(PathBuf::from)
            .map_err(io::Error::other);
        CoTaskMemFree(Some(desktop.0.cast()));
        Ok(directory?.join("Panopticon Hub.lnk"))
    }
}

fn new_link() -> io::Result<IShellLinkW> {
    unsafe { CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER) }.map_err(windows_error)
}

fn link_file(link: &IShellLinkW) -> io::Result<IPersistFile> {
    link.cast().map_err(windows_error)
}

fn apply(link: &IShellLinkW, exe: &Path, path: &Path) -> io::Result<()> {
    let parent = exe
        .parent()
        .ok_or_else(|| io::Error::other("Не найден каталог лаунчера."))?;
    unsafe {
        link.SetPath(PCWSTR(wide(exe.as_os_str()).as_ptr()))
            .map_err(windows_error)?;
        link.SetWorkingDirectory(PCWSTR(wide(parent.as_os_str()).as_ptr()))
            .map_err(windows_error)?;
        link.SetIconLocation(PCWSTR(wide(exe.as_os_str()).as_ptr()), ICON_INDEX)
            .map_err(windows_error)?;
        link_file(link)?
            .Save(PCWSTR(wide(path.as_os_str()).as_ptr()), true)
            .map_err(windows_error)?;
    }
    Ok(())
}

fn load_link(path: &Path) -> io::Result<IShellLinkW> {
    let link = new_link()?;
    unsafe { link_file(&link)?.Load(PCWSTR(wide(path.as_os_str()).as_ptr()), STGM_READ) }
        .map_err(windows_error)?;
    Ok(link)
}

fn same_path(buffer: &[u16], path: &Path) -> bool {
    let end = buffer
        .iter()
        .position(|code| *code == 0)
        .unwrap_or(buffer.len());
    String::from_utf16_lossy(&buffer[..end]).eq_ignore_ascii_case(&path.to_string_lossy())
}

fn target_matches(link: &IShellLinkW, path: &Path) -> io::Result<bool> {
    let mut target = vec![0; 32768];
    unsafe { link.GetPath(&mut target, std::ptr::null_mut(), SLGP_RAWPATH.0 as u32) }
        .map_err(windows_error)?;
    Ok(same_path(&target, path))
}

#[allow(dead_code)]
pub fn upgraded(path: &Path, exe: &Path, core: &Path) -> io::Result<Option<Vec<u8>>> {
    let _com = ComApartment::enter()?;
    let link = load_link(path)?;
    let correct_target = target_matches(&link, exe)?;
    if !correct_target && !target_matches(&link, core)? {
        return Ok(None);
    }
    let mut icon = vec![0; 32768];
    let mut index = 0;
    unsafe { link.GetIconLocation(&mut icon, &mut index) }.map_err(windows_error)?;
    if correct_target && index == ICON_INDEX && same_path(&icon, exe) {
        return Ok(None);
    }
    let temp = tempfile::tempdir()?;
    let updated = temp.path().join("Panopticon Hub.lnk");
    apply(&link, exe, &updated)?;
    std::fs::read(updated).map(Some)
}

#[allow(dead_code)]
pub fn create(exe: &Path, core: &Path) -> io::Result<()> {
    let shortcut = desktop_shortcut()?;
    if shortcut.exists() {
        if let Some(bytes) = upgraded(&shortcut, exe, core)? {
            let parent = shortcut
                .parent()
                .ok_or_else(|| io::Error::other("Не найдена папка ярлыка."))?;
            let mut temp = tempfile::NamedTempFile::new_in(parent)?;
            std::io::Write::write_all(&mut temp, &bytes)?;
            temp.persist(&shortcut)?;
            notify_shortcut(&shortcut);
        } else {
            let _com = ComApartment::enter()?;
            if !target_matches(&load_link(&shortcut)?, exe)? {
                return Err(io::Error::other(
                    "Этот ярлык принадлежит другому приложению.",
                ));
            }
        }
        return Ok(());
    }
    let _com = ComApartment::enter()?;
    apply(&new_link()?, exe, &shortcut)?;
    notify_shortcut(&shortcut);
    Ok(())
}
