use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    env, fs,
    io::{self, Read, Write},
    os::windows::fs::OpenOptionsExt,
    path::{Path, PathBuf},
    process::Command,
    ptr,
    time::{Duration, Instant},
};
use tempfile::{Builder, NamedTempFile, TempPath};
use windows_sys::Win32::{Networking::WinHttp::*, Storage::FileSystem::FILE_SHARE_READ};

const REPO: &str = "werton-core/PanopticonHub";
const ASSET: &str = "PanopticonHubSetup.exe";
const MAX_SETUP: u64 = 512 * 1024 * 1024;
const MAX_METADATA: usize = 256 * 1024;

#[derive(Clone, Copy, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    #[default]
    Ask,
    Auto,
    Manual,
}

fn state_dir() -> Result<PathBuf, String> {
    env::var_os("LOCALAPPDATA")
        .map(|path| PathBuf::from(path).join("PanopticonHub").join("updates"))
        .ok_or_else(|| "Папка LOCALAPPDATA недоступна.".into())
}

pub fn mode() -> Result<Mode, String> {
    let path = state_dir()?.join("mode.json");
    match fs::File::open(path) {
        Ok(file) => serde_json::from_reader(file.take(64))
            .map_err(|_| "Не удалось прочитать настройки обновлений.".into()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Mode::default()),
        Err(error) => Err(error.to_string()),
    }
}

pub fn set_mode(mode: Mode) -> Result<(), String> {
    let directory = state_dir()?;
    fs::create_dir_all(&directory).map_err(io_error)?;
    let mut file = NamedTempFile::new_in(&directory).map_err(io_error)?;
    serde_json::to_writer(file.as_file_mut(), &mode).map_err(|error| error.to_string())?;
    file.as_file().sync_all().map_err(io_error)?;
    file.persist(directory.join("mode.json"))
        .map_err(|error| error.to_string())?;
    Ok(())
}

fn io_error(error: io::Error) -> String {
    error.to_string()
}

struct Handle(*mut std::ffi::c_void);

impl Handle {
    fn new(value: *mut std::ffi::c_void) -> Result<Self, String> {
        if value.is_null() {
            return Err(network_error());
        }
        Ok(Self(value))
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { WinHttpCloseHandle(self.0) };
    }
}

fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(Some(0)).collect()
}

fn network_error() -> String {
    format!("GitHub недоступен: {}", io::Error::last_os_error())
}

fn response(host: &str, path: &str, metadata: bool) -> Result<(HandleSet, u32), String> {
    let agent = wide(concat!("PanopticonHub/", env!("CARGO_PKG_VERSION")));
    let accept = wide(if metadata {
        "Accept: application/vnd.github+json\r\n"
    } else {
        "Accept: application/octet-stream\r\n"
    });
    unsafe {
        let session = Handle::new(WinHttpOpen(
            agent.as_ptr(),
            WINHTTP_ACCESS_TYPE_AUTOMATIC_PROXY,
            ptr::null(),
            ptr::null(),
            0,
        ))?;
        if WinHttpSetTimeouts(session.0, 4000, 5000, 5000, 10000) == 0 {
            return Err(network_error());
        }
        let connection = Handle::new(WinHttpConnect(session.0, wide(host).as_ptr(), 443, 0))?;
        let request = Handle::new(WinHttpOpenRequest(
            connection.0,
            wide("GET").as_ptr(),
            wide(path).as_ptr(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
            WINHTTP_FLAG_SECURE,
        ))?;
        let policy = WINHTTP_OPTION_REDIRECT_POLICY_NEVER;
        if WinHttpSetOption(
            request.0,
            WINHTTP_OPTION_REDIRECT_POLICY,
            (&policy as *const u32).cast(),
            std::mem::size_of_val(&policy) as u32,
        ) == 0
            || WinHttpSendRequest(
                request.0,
                accept.as_ptr(),
                u32::MAX,
                ptr::null_mut(),
                0,
                0,
                0,
            ) == 0
            || WinHttpReceiveResponse(request.0, ptr::null_mut()) == 0
        {
            return Err(network_error());
        }
        let mut status = 0u32;
        let mut bytes = 4u32;
        if WinHttpQueryHeaders(
            request.0,
            WINHTTP_QUERY_STATUS_CODE | WINHTTP_QUERY_FLAG_NUMBER,
            ptr::null(),
            (&mut status as *mut u32).cast(),
            &mut bytes,
            ptr::null_mut(),
        ) == 0
        {
            return Err(network_error());
        }
        // The request must close before its connection and session.
        Ok((
            HandleSet {
                request,
                _connection: connection,
                _session: session,
            },
            status,
        ))
    }
}

struct HandleSet {
    request: Handle,
    _connection: Handle,
    _session: Handle,
}

impl Read for HandleSet {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut read = 0u32;
        let ok = unsafe {
            WinHttpReadData(
                self.request.0,
                buf.as_mut_ptr().cast(),
                buf.len().min(u32::MAX as usize) as u32,
                &mut read,
            )
        };
        if ok == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(read as usize)
    }
}

fn asset_redirect(response: &HandleSet) -> Result<String, String> {
    let mut buffer = [0u16; 8192];
    let mut bytes = std::mem::size_of_val(&buffer) as u32;
    let ok = unsafe {
        WinHttpQueryHeaders(
            response.request.0,
            WINHTTP_QUERY_LOCATION,
            ptr::null(),
            buffer.as_mut_ptr().cast(),
            &mut bytes,
            ptr::null_mut(),
        )
    };
    if ok == 0 {
        return Err(network_error());
    }
    String::from_utf16(&buffer[..bytes as usize / 2]).map_err(|error| error.to_string())
}

fn redirected_asset(url: &str) -> Option<(&str, &str)> {
    let tail = url.strip_prefix("https://")?;
    let (host, path) = tail.split_at(tail.find('/')?);
    if !matches!(
        host,
        "release-assets.githubusercontent.com" | "objects.githubusercontent.com"
    ) || path.starts_with("//")
        || path.contains(['#', '\\'])
        || !path.bytes().all(|byte| byte.is_ascii_graphic())
    {
        return None;
    }
    Some((host, path))
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    body: Option<String>,
    draft: bool,
    prerelease: bool,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    state: String,
    size: u64,
    digest: Option<String>,
    browser_download_url: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Update {
    version: String,
    body: String,
    html_url: String,
    #[serde(skip)]
    asset_path: String,
    #[serde(skip)]
    size: u64,
    #[serde(skip)]
    digest: String,
}

fn version(value: &str) -> Option<(u32, u32, u32)> {
    let mut parts = value.split('.');
    fn number(value: &str) -> Option<u32> {
        if value.is_empty()
            || !value.bytes().all(|b| b.is_ascii_digit())
            || value.len() > 1 && value.starts_with('0')
        {
            return None;
        }
        value.parse().ok()
    }
    let result = (
        number(parts.next()?)?,
        number(parts.next()?)?,
        number(parts.next()?)?,
    );
    parts.next().is_none().then_some(result)
}

fn select_release(release: Release, current: &str) -> Result<Option<Update>, String> {
    let Some(tag_version) = release.tag_name.strip_prefix('v').and_then(version) else {
        return Ok(None);
    };
    let current = version(current).ok_or("Некорректная установленная версия Panopticon Hub.")?;
    if release.draft || release.prerelease || tag_version <= current {
        return Ok(None);
    }
    let asset_path = format!("/{REPO}/releases/download/{}/{ASSET}", release.tag_name);
    let Some(asset) = release.assets.into_iter().find(|asset| {
        asset.name == ASSET
            && asset.state == "uploaded"
            && asset.browser_download_url == format!("https://github.com{asset_path}")
    }) else {
        return Ok(None);
    };
    let digest = asset
        .digest
        .as_deref()
        .and_then(|d| d.strip_prefix("sha256:"))
        .filter(|d| d.len() == 64 && d.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .ok_or("GitHub не предоставил SHA-256 установщика.")?;
    if !(2..=MAX_SETUP).contains(&asset.size) {
        return Err("Размер установщика GitHub выходит за допустимый диапазон.".into());
    }
    Ok(Some(Update {
        version: release.tag_name[1..].into(),
        body: release
            .body
            .unwrap_or_default()
            .chars()
            .take(4000)
            .collect(),
        html_url: format!(
            "https://github.com/{REPO}/releases/tag/{}",
            release.tag_name
        ),
        asset_path,
        size: asset.size,
        digest: digest.to_ascii_lowercase(),
    }))
}

pub fn installed_version(receipt: &serde_json::Value) -> &str {
    let host_version = env!("CARGO_PKG_VERSION");
    receipt["version"]
        .as_str()
        .filter(|value| version(value) > version(host_version))
        .unwrap_or(host_version)
}

pub fn check(current: &str) -> Result<Option<Update>, String> {
    let (response, status) = response(
        "api.github.com",
        &format!("/repos/{REPO}/releases/latest"),
        true,
    )?;
    if status == 404 {
        return Ok(None);
    }
    if status != 200 {
        return Err(format!("GitHub вернул HTTP {status}."));
    }
    let deadline = Instant::now() + Duration::from_secs(15);
    let mut bytes = Vec::new();
    read_bounded(response, &mut bytes, MAX_METADATA as u64, deadline)?;
    let release = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    select_release(release, current)
}

fn read_bounded(
    mut input: impl Read,
    mut output: impl Write,
    limit: u64,
    deadline: Instant,
) -> Result<(u64, String), String> {
    let mut hash = Sha256::new();
    let mut written = 0u64;
    let mut buffer = [0u8; 64 * 1024];
    loop {
        if Instant::now() >= deadline {
            return Err("Истекло время загрузки с GitHub.".into());
        }
        let count = input.read(&mut buffer).map_err(io_error)?;
        if count == 0 {
            break;
        }
        written += count as u64;
        if written > limit {
            return Err("GitHub вернул больше данных, чем ожидалось.".into());
        }
        hash.update(&buffer[..count]);
        output.write_all(&buffer[..count]).map_err(io_error)?;
    }
    Ok((written, format!("{:x}", hash.finalize())))
}

pub struct VerifiedSetup {
    file: fs::File,
    path: TempPath,
}

pub fn download(update: &Update) -> Result<VerifiedSetup, String> {
    let (mut stream, mut status) = response("github.com", &update.asset_path, false)?;
    if matches!(status, 301 | 302 | 303 | 307 | 308) {
        let location = asset_redirect(&stream)?;
        let (host, path) =
            redirected_asset(&location).ok_or("GitHub вернул недопустимый адрес загрузки.")?;
        (stream, status) = response(host, path, false)?;
    }
    if status != 200 {
        return Err(format!(
            "GitHub вернул HTTP {status} при загрузке установщика."
        ));
    }
    let directory = state_dir()?;
    fs::create_dir_all(&directory).map_err(io_error)?;
    remove_stale_installers(&directory);
    let mut file = Builder::new()
        .prefix("setup-")
        .suffix(".exe")
        .tempfile_in(&directory)
        .map_err(io_error)?;
    let deadline = Instant::now() + Duration::from_secs(15 * 60);
    let (size, digest) = read_bounded(stream, file.as_file_mut(), update.size, deadline)?;
    if size != update.size || digest != update.digest {
        return Err("Размер или SHA-256 установщика не совпадает с опубликованными GitHub.".into());
    }
    file.as_file().sync_all().map_err(io_error)?;
    let path = file.into_temp_path();
    // Keep the validated file locked against writes and deletion until CreateProcess opens it.
    let mut file = fs::OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ)
        .open(&path)
        .map_err(io_error)?;
    let (size, digest) = read_bounded(&mut file, io::sink(), update.size, deadline)?;
    if size != update.size || digest != update.digest {
        return Err("SHA-256 проверенного установщика изменился перед запуском.".into());
    }
    Ok(VerifiedSetup { file, path })
}

fn remove_stale_installers(directory: &std::path::Path) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if !name.starts_with("setup-") || !name.ends_with(".exe") {
            continue;
        }
        let Ok(metadata) = entry.metadata() else {
            continue;
        };
        if metadata.is_file()
            && metadata
                .modified()
                .ok()
                .and_then(|time| time.elapsed().ok())
                .is_some_and(|age| age > Duration::from_secs(24 * 60 * 60))
        {
            let _ = fs::remove_file(entry.path());
        }
    }
}

pub struct Installation {
    dir: PathBuf,
    scope_arg: &'static str,
}

impl Installation {
    pub fn from_receipt(receipt: &serde_json::Value) -> Option<Self> {
        let executable = env::current_exe().ok()?;
        let expected_name = executable.file_name()?.to_str()?;
        if !expected_name.eq_ignore_ascii_case("PanopticonHub.exe")
            || receipt["files"]["PanopticonHub.exe"].as_str().is_none()
        {
            return None;
        }
        let dir = PathBuf::from(receipt["directory"].as_str()?);
        if !dir.is_absolute()
            || fs::canonicalize(&dir).ok()? != fs::canonicalize(executable.parent()?).ok()?
        {
            return None;
        }
        let scope_arg = match receipt["scope"].as_str()? {
            "machine" => "--all-users",
            "user" => "--current-user",
            _ => return None,
        };
        Some(Self { dir, scope_arg })
    }

    pub fn start(&self, setup: VerifiedSetup) -> Result<(), String> {
        let path: &Path = setup.path.as_ref();
        Command::new(path)
            .args(["--silent", self.scope_arg, "--dir"])
            .arg(&self.dir)
            .args([
                "--from-hub-pid",
                &std::process::id().to_string(),
                "--relaunch",
            ])
            .spawn()
            .map_err(io_error)?;
        drop(setup.file);
        let _ = setup.path.keep();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn release(tag: &str) -> Release {
        Release {
            tag_name: tag.into(),
            body: Some("Changes".into()),
            draft: false,
            prerelease: false,
            assets: vec![Asset {
                name: ASSET.into(),
                size: 128,
                state: "uploaded".into(),
                digest: Some(format!("sha256:{}", "A".repeat(64))),
                browser_download_url: format!(
                    "https://github.com/{REPO}/releases/download/{tag}/{ASSET}"
                ),
            }],
        }
    }

    #[test]
    fn selects_only_a_new_stable_version_with_an_official_asset() {
        let update = select_release(release("v0.2.10"), "0.2.9")
            .unwrap()
            .unwrap();
        assert_eq!(update.version, "0.2.10");
        assert_eq!(update.digest, "a".repeat(64));
        assert!(select_release(release("v0.2.10"), "0.2.10")
            .unwrap()
            .is_none());
        assert!(select_release(release("v0.2.9"), "0.2.10")
            .unwrap()
            .is_none());
        assert!(select_release(release("v0.3.0-rc.1"), "0.2.10")
            .unwrap()
            .is_none());
        let mut draft = release("v0.2.11");
        draft.draft = true;
        assert!(select_release(draft, "0.2.10").unwrap().is_none());
        let mut foreign = release("v0.2.11");
        foreign.assets[0].browser_download_url = "https://github.com/foreign/setup.exe".into();
        assert!(select_release(foreign, "0.2.10").unwrap().is_none());
        let mut unverified = release("v0.2.11");
        unverified.assets[0].digest = None;
        assert!(select_release(unverified, "0.2.10").is_err());
    }

    #[test]
    fn remembers_a_newer_installer_even_when_the_hub_binary_was_unchanged() {
        let receipt = serde_json::json!({"version": "4294967295.0.0"});
        assert_eq!(installed_version(&receipt), "4294967295.0.0");
        let invalid = serde_json::json!({"version": "0.050.0"});
        assert_eq!(installed_version(&invalid), env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn disallows_untrusted_redirects_and_excess_download_bytes() {
        assert!(redirected_asset("https://release-assets.githubusercontent.com/a?a=b").is_some());
        for url in [
            "http://release-assets.githubusercontent.com/a",
            "https://release-assets.githubusercontent.com.mirror.example/a",
            "https://release-assets.githubusercontent.com@mirror.example/a",
            "https://release-assets.githubusercontent.com//mirror.example/a",
            "https://release-assets.githubusercontent.com/a#other",
        ] {
            assert!(redirected_asset(url).is_none());
        }
        let deadline = Instant::now() + Duration::from_secs(1);
        let mut output = Vec::new();
        assert!(read_bounded(&b"oversized"[..], &mut output, 4, deadline).is_err());
        let (length, digest) = read_bounded(&b"abc"[..], &mut output, 3, deadline).unwrap();
        assert_eq!(length, 3);
        assert_eq!(digest, format!("{:x}", Sha256::digest(b"abc")));
    }

    #[test]
    #[ignore = "Downloads the published Windows installer over HTTPS"]
    fn downloads_published_installer_through_winhttp() {
        let update = check("0.0.0")
            .unwrap()
            .expect("a stable release with setup");
        let setup = download(&update).unwrap();
        assert_eq!(fs::metadata(&setup.path).unwrap().len(), update.size);
        let path: &Path = setup.path.as_ref();
        let output = Command::new(path).arg("--check-payload").output().unwrap();
        assert!(output.status.success());
        assert!(String::from_utf8_lossy(&output.stdout).contains("fixture=false"));
        assert!(check(&update.version).unwrap().is_none());
    }
}
