//! Поддерживаемый Rust-порт действий декомпилированного NSIS, не исходный .nsi.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io::Write,
    path::{Component, Path, PathBuf, Prefix},
    sync::atomic::{AtomicBool, Ordering},
};

pub mod platform;
mod transaction;
use transaction::{Pending, Work};

pub const PRODUCT: &str = "Panopticon Hub";
pub const HOST: &str = "PanopticonHub.exe";
pub const UNINSTALL: &str = "Uninstall Panopticon Hub.exe";
pub const RECEIPT: &str = ".panopticon-hub-install.json";
const CORE: &str = "PanopticonHubCore.exe";
const PENDING: &str = ".panopticon-hub-pending.json";
const OWNER: &str = "PanopticonHubSetup/1";
pub const PAYLOAD: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/payload.exe"));
pub const PAYLOAD_SHA256: &str = env!("PANOPTICON_PAYLOAD_SHA256");
const UNINSTALL_PAYLOAD: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/uninstaller.exe"));
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    User,
    Machine,
}

#[derive(Clone)]
pub struct InstallOptions {
    pub directory: PathBuf,
    pub scope: Scope,
    pub desktop_shortcut: bool,
    pub start_menu_shortcut: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Receipt {
    owner: String,
    directory: PathBuf,
    pub scope: Scope,
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    installation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    initial_style_offer: Option<bool>,
    files: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    panopticon_core_sha256: Option<String>,
    shortcuts: Vec<PathBuf>,
    #[serde(default)]
    shortcut_hashes: BTreeMap<PathBuf, String>,
}

impl Receipt {
    pub fn check_hub_closed(&self) -> Result<(), String> {
        let pending = read_pending(&self.directory).ok().flatten();
        require_closed_launchers(
            &self.directory,
            std::iter::once(self).chain(pending.iter().map(|(p, _)| &p.receipt)),
        )
    }

    pub fn restart_previous_hub(&self) -> Result<(), String> {
        let directory = &self.directory;
        validate_directory(directory)?;
        let _lock = platform::InstallLock::take()?;
        if read_pending(directory)?.is_some()
            || read_committed(directory)?.is_none_or(|(current, _)| current != *self)
        {
            return Err("Не удалось подтвердить восстановление прежней установки.".into());
        }
        for (name, hash) in self.binaries().filter(|(name, _)| *name != HOST) {
            require_digest(&directory.join(name), hash)?;
        }
        let executable = directory.join(HOST);
        let host_hash = self.binary_hash(HOST).ok_or("В манифесте отсутствует PanopticonHub.exe.")?;
        let _verified = platform::lock_verified_hub(&executable, host_hash)?;
        if !platform::is_executable_running(&executable)? {
            platform::launch_hub(&executable, directory)?;
        }
        Ok(())
    }

    fn binary_hash(&self, name: &str) -> Option<&str> {
        if name == CORE {
            self.panopticon_core_sha256.as_deref()
        } else {
            self.files.get(name).map(String::as_str)
        }
    }

    fn binaries(&self) -> impl Iterator<Item = (&str, &str)> {
        self.files
            .iter()
            .map(|(name, hash)| (name.as_str(), hash.as_str()))
            .chain(self.panopticon_core_sha256.as_deref().map(|hash| (CORE, hash)))
    }
}

pub struct InstallResult {
    pub executable: PathBuf,
}
pub struct UninstallResult {
    pub preserved: Vec<PathBuf>,
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn file_digest(path: &Path) -> Result<String, String> {
    let mut file = fs::File::open(path)
        .map_err(|e| format!("Не удалось прочитать {}: {e}", path.display()))?;
    digest_reader(&mut file)
}

fn digest_reader(file: &mut fs::File) -> Result<String, String> {
    use std::io::Read;
    let mut hash = Sha256::new();
    let mut bytes = [0; 65536];
    loop {
        let count = file.read(&mut bytes).map_err(|e| e.to_string())?;
        if count == 0 {
            break;
        }
        hash.update(&bytes[..count]);
    }
    Ok(format!("{:x}", hash.finalize()))
}

fn gate(cancelled: &AtomicBool) -> Result<(), String> {
    if cancelled.load(Ordering::Acquire) {
        Err("Установка отменена. Повторный запуск продолжит подготовку безопасно.".into())
    } else {
        Ok(())
    }
}

pub fn validate_directory(path: &Path) -> Result<(), String> {
    if !path.is_absolute()
        || !matches!(
            path.components().next(),
            Some(Component::Prefix(prefix)) if matches!(prefix.kind(), Prefix::Disk(_))
        )
        || path
            .components()
            .filter(|c| matches!(c, Component::Normal(_)))
            .count()
            < 2
        || path.components().any(|c| match c {
            Component::ParentDir | Component::CurDir => true,
            Component::Normal(name) => {
                let name = name.to_string_lossy();
                name.ends_with([' ', '.'])
                    || name.contains([':', '\0'])
                    || name.split('.').next().is_some_and(|stem| {
                        let stem = stem.trim_end().to_ascii_uppercase();
                        matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
                            || ["COM", "LPT"].iter().any(|prefix| {
                                stem.strip_prefix(prefix).is_some_and(|suffix| {
                                    matches!(
                                        suffix,
                                        "1" | "2"
                                            | "3"
                                            | "4"
                                            | "5"
                                            | "6"
                                            | "7"
                                            | "8"
                                            | "9"
                                            | "¹"
                                            | "²"
                                            | "³"
                                    )
                                })
                            })
                    })
            }
            _ => false,
        })
    {
        return Err(
            "Выберите отдельную локальную папку приложения. Сетевые пути, корни дисков и специальные имена Windows запрещены."
                .into(),
        );
    }
    platform::reject_links(path)?;
    for key in [
        "WINDIR",
        "SystemRoot",
        "ProgramFiles",
        "ProgramFiles(x86)",
        "USERPROFILE",
        "LOCALAPPDATA",
        "APPDATA",
        "PUBLIC",
    ] {
        if let Some(protected) = std::env::var_os(key) {
            let protected = PathBuf::from(protected);
            if path_key(path) == path_key(&protected)
                || (matches!(key, "WINDIR" | "SystemRoot")
                    && path_key(path).starts_with(&(path_key(&protected) + "\\")))
            {
                return Err("Системный или общий пользовательский каталог нельзя выбрать как папку приложения.".into());
            }
        }
    }
    if path.exists() && !path.is_dir() {
        return Err("Выбранный путь занят файлом.".into());
    }
    Ok(())
}

fn path_key(path: &Path) -> String {
    let canonical = path.ancestors().find_map(|base| {
        let suffix = path.strip_prefix(base).ok()?;
        Some(base.canonicalize().ok()?.join(suffix))
    });
    canonical
        .unwrap_or_else(|| path.to_owned())
        .to_string_lossy()
        .trim_start_matches("\\\\?\\")
        .replace('/', "\\")
        .trim_end_matches('\\')
        .to_lowercase()
}

fn read_record<T: serde::de::DeserializeOwned>(
    directory: &Path,
    name: &str,
) -> Result<Option<(T, String)>, String> {
    use std::io::Read;
    let path = directory.join(name);
    platform::reject_links(&path)?;
    let file = match fs::File::open(&path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(format!("Не удалось прочитать {}: {error}", path.display())),
    };
    let mut bytes = Vec::new();
    file.take(65537)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() > 65536 {
        return Err("Повреждён манифест установки.".into());
    }
    let value =
        serde_json::from_slice(&bytes).map_err(|e| format!("Повреждён манифест установки: {e}"))?;
    Ok(Some((value, digest(&bytes))))
}

fn read_committed(directory: &Path) -> Result<Option<(Receipt, String)>, String> {
    let record = read_record::<Receipt>(directory, RECEIPT)?;
    if let Some((receipt, _)) = &record {
        validate_receipt(receipt, directory)?;
    }
    Ok(record)
}

fn read_pending(directory: &Path) -> Result<Option<(Pending, String)>, String> {
    let record = read_record::<Pending>(directory, PENDING)?;
    if let Some((pending, _)) = &record {
        pending.validate(directory)?;
    }
    Ok(record)
}

pub fn read_receipt(directory: &Path) -> Result<Receipt, String> {
    validate_directory(directory)?;
    if let Some((receipt, _)) = read_committed(directory)? {
        return Ok(receipt);
    }
    read_pending(directory)?
        .map(|(pending, _)| pending.receipt)
        .ok_or_else(|| "Не найден манифест собственной установки.".into())
}

pub fn read_update_receipt(directory: &Path, scope: Scope) -> Result<Receipt, String> {
    validate_directory(directory)?;
    let (receipt, _) = read_committed(directory)?
        .ok_or("Не найден манифест завершённой установки Panopticon Hub.")?;
    if receipt.scope != scope {
        return Err("Режим обновления отличается от существующей установки.".into());
    }
    Ok(receipt)
}

fn valid_digest(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn version_triplet(version: &str) -> Option<[u32; 3]> {
    let mut parts = version.split('.');
    let mut triplet = [0; 3];
    for component in &mut triplet {
        let raw = parts.next()?;
        *component = raw.parse().ok()?;
        if raw != component.to_string() {
            return None;
        }
    }
    parts.next().is_none().then_some(triplet)
}

fn compatible_receipt_version(version: &str, current: &str) -> bool {
    match (version_triplet(version), version_triplet(current)) {
        (Some(version), Some(current)) => {
            version <= current && (version == [0, 2, 0] || version >= [0, 2, 8])
        }
        _ => false,
    }
}

fn validate_receipt(receipt: &Receipt, directory: &Path) -> Result<(), String> {
    if receipt.owner != OWNER
        || !compatible_receipt_version(&receipt.version, VERSION)
        || path_key(&receipt.directory) != path_key(directory)
        || receipt.files.len() != 2
        || receipt
            .files
            .keys()
            .any(|name| name != HOST && name != UNINSTALL)
        || receipt.files.values().any(|value| !valid_digest(value))
        || receipt
            .panopticon_core_sha256
            .as_deref()
            .is_some_and(|hash| !valid_digest(hash))
        || receipt.shortcuts.len() > 2
        || receipt
            .shortcut_hashes
            .iter()
            .any(|(path, hash)| !receipt.shortcuts.contains(path) || !valid_digest(hash))
    {
        return Err("Каталог не принадлежит этой установке Panopticon Hub.".into());
    }
    Ok(())
}

fn require_digest(path: &Path, expected: &str) -> Result<(), String> {
    platform::reject_links(path)?;
    if file_digest(path)? != expected {
        return Err(format!("Файл {} изменился. Файл сохранён.", path.display()));
    }
    Ok(())
}

fn nonce() -> Result<u128, String> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .map_err(|e| e.to_string())
}

fn atomic_file(path: &Path, bytes: &[u8], replace: bool) -> Result<(), String> {
    platform::reject_links(path)?;
    let temp = path.with_file_name(format!(
        ".panopticon-{}-{}.tmp",
        std::process::id(),
        nonce()?
    ));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp)
        .map_err(|e| format!("Не удалось создать временный файл: {e}"))?;
    let written = file.write_all(bytes).and_then(|_| file.sync_all());
    drop(file);
    let result = written
        .map_err(|e| format!("Не удалось записать файл: {e}"))
        .and_then(|_| platform::move_file(&temp, path, replace));
    if result.is_err() {
        let _ = platform::delete_matching(&temp, &[&digest(bytes)]);
    }
    result
}

fn write_record(
    directory: &Path,
    name: &str,
    record: &impl Serialize,
    replace: bool,
) -> Result<String, String> {
    let bytes = serde_json::to_vec_pretty(record).map_err(|e| e.to_string())?;
    atomic_file(&directory.join(name), &bytes, replace)?;
    Ok(digest(&bytes))
}

pub fn install(
    options: &InstallOptions,
    cancelled: &AtomicBool,
    mut progress: impl FnMut(String),
) -> Result<InstallResult, String> {
    if env!("PANOPTICON_PAYLOAD_FIXTURE") == "true" {
        return Err(
            "Это тестовая сборка установщика. Соберите её с настоящим PANOPTICON_HOST_EXE.".into(),
        );
    }
    if env!("PANOPTICON_SETUP_UNINSTALL_ONLY") == "true" {
        return Err("Этот файл предназначен для удаления Panopticon Hub.".into());
    }
    install_payload(options, cancelled, &mut progress, PAYLOAD, UNINSTALL_PAYLOAD, true)
}

fn install_payload(
    options: &InstallOptions,
    cancelled: &AtomicBool,
    progress: &mut impl FnMut(String),
    payload: &[u8],
    setup: &[u8],
    shell: bool,
) -> Result<InstallResult, String> {
    gate(cancelled)?;
    let directory = &options.directory;
    validate_directory(directory)?;
    if shell && options.scope == Scope::Machine && !platform::is_admin() {
        return Err("Для установки всем пользователям необходимы права администратора.".into());
    }
    let _lock = platform::InstallLock::take()?;
    if shell {
        for registered in platform::registration_locations(options.scope) {
            if path_key(&registered) != path_key(directory) && registered.exists() {
                return Err(format!("Программа уже зарегистрирована в {}. Обновите её в той же папке или сначала удалите эту установку.", registered.display()));
            }
        }
    }
    fs::create_dir_all(directory)
        .map_err(|e| format!("Не удалось создать папку установки: {e}"))?;
    platform::reject_links(directory)?;
    let previous = read_committed(directory)?;
    let mut legacy = read_pending(directory)?;
    if previous
        .iter()
        .map(|(r, _)| r)
        .chain(legacy.iter().map(|(p, _)| &p.receipt))
        .any(|receipt| receipt.scope != options.scope)
    {
        return Err("Режим установки отличается от существующего. Сначала удалите собственную прежнюю установку.".into());
    }
    if let Some((pending, hash)) = &legacy {
        if pending.recovery.is_some() {
            recover_pending(directory, pending, hash, previous.as_ref(), shell)?;
            legacy = read_pending(directory)?;
        }
    }

    let known: Vec<&Receipt> = previous
        .iter()
        .map(|(r, _)| r)
        .chain(legacy.iter().map(|(p, _)| &p.receipt))
        .collect();
    let mut shortcut_hashes = BTreeMap::new();
    for receipt in &known {
        for (path, expected) in &receipt.shortcut_hashes {
            if platform::allowed_shortcut(path, options.scope)?
                && platform::reject_links(path).is_ok()
                && file_digest(path).ok().as_ref() == Some(expected)
            {
                shortcut_hashes.insert(path.clone(), expected.clone());
            }
        }
    }
    let mut receipt = Receipt {
        owner: OWNER.into(),
        directory: directory.clone(),
        scope: options.scope,
        version: VERSION.into(),
        installation_id: match known.first() {
            Some(receipt) => receipt.installation_id.clone(),
            None => {
                let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                    .map_err(|error| error.to_string())?.as_nanos();
                Some(format!("{now:024x}{:08x}", std::process::id()))
            }
        },
        initial_style_offer: known.first().map_or(Some(true), |receipt| receipt.initial_style_offer),
        files: BTreeMap::from([
            (HOST.into(), digest(payload)),
            (UNINSTALL.into(), digest(setup)),
        ]),
        panopticon_core_sha256: None,
        shortcuts: shortcut_hashes.keys().cloned().collect(),
        shortcut_hashes,
    };
    let mut work = Work::create(directory)?;
    let mut backups = BTreeMap::new();
    let mut updated_shortcuts = Vec::new();
    let mut shortcut_backups = BTreeMap::new();
    if shell {
        let mut paths = receipt.shortcuts.clone();
        for path in platform::shortcut_paths(options)? {
            if !paths.iter().any(|existing| path_key(existing) == path_key(&path)) {
                paths.push(path);
            }
        }
        let target = directory.join(HOST);
        for path in paths {
            platform::reject_links(&path)?;
            let before = receipt.shortcut_hashes.get(&path);
            if let Some(hash) = before {
                if platform::shortcut_is_current(&path, &target)? {
                    continue;
                }
                shortcut_backups.insert(path.clone(), hash.clone());
            } else if path.try_exists().map_err(|e| e.to_string())? {
                return Err(format!("Ярлык {} уже существует и не принадлежит установке. Снимите его создание или переименуйте ярлык.", path.display()));
            }
            let hash = work.stage_shortcut(
                updated_shortcuts.len(),
                directory,
                before.map(|hash| (path.as_path(), hash.as_str())),
            )?;
            receipt.shortcut_hashes.insert(path.clone(), hash);
            if !receipt.shortcuts.contains(&path) {
                receipt.shortcuts.push(path.clone());
            }
            updated_shortcuts.push(path);
        }
    }
    progress("Подготовка Panopticon Hub…".into());
    gate(cancelled)?;
    work.stage_binary(directory, HOST, payload, &known, &mut backups)?;
    work.stage_binary(directory, UNINSTALL, setup, &known, &mut backups)?;
    work.stage_legacy_core(directory, &known, &mut backups)?;
    gate(cancelled)?;

    let legacy_hash = legacy.as_ref().map(|(_, hash)| hash.clone());
    if let Some(hash) = &legacy_hash {
        require_digest(&directory.join(PENDING), hash)?;
    }
    let pending = work.journal(
        receipt,
        backups,
        updated_shortcuts.clone(),
        shortcut_backups,
        legacy.map(|(p, _)| p.receipt),
    );
    let pending_hash = match write_record(directory, PENDING, &pending, legacy_hash.is_some()) {
        Ok(hash) => hash,
        Err(error) => {
            work.clean_on_drop();
            return Err(error);
        }
    };
    let update = (|| {
        progress("Копирование Panopticon Hub…".into());
        gate(cancelled)?;
        work.publish_binary(directory, HOST, &pending)?;
        progress("Подготовка удаления приложения…".into());
        gate(cancelled)?;
        work.publish_binary(directory, UNINSTALL, &pending)?;
        if pending.removed_core_hash().is_some() {
            pending.remove_legacy_core(directory)?;
            progress("Удаление прежнего ядра…".into());
            gate(cancelled)?;
        }
        if shell {
            progress("Создание ярлыков и регистрация приложения…".into());
            gate(cancelled)?;
            for (index, path) in updated_shortcuts.iter().enumerate() {
                work.publish_shortcut(
                    index,
                    path,
                    &pending.receipt.shortcut_hashes[path],
                    pending.shortcut_before(path),
                )?;
            }
            platform::register(&pending.receipt)?;
        }
        if let Some((_, hash)) = &previous {
            require_digest(&directory.join(RECEIPT), hash)?;
        }
        write_record(directory, RECEIPT, &pending.receipt, previous.is_some())
    })();
    let recovered = transaction::recover(
        directory,
        &pending,
        &pending_hash,
        previous.as_ref().map(|(r, hash)| (r, hash.as_str())),
        shell,
        update.is_ok(),
    );
    if let Err(error) = update {
        return match recovered {
            Ok(()) => Err(error),
            Err(recovery) => Err(format!(
                "{error} Восстановление установки не завершено: {recovery}"
            )),
        };
    }
    recovered?;
    if shell {
        for path in &pending.receipt.shortcuts {
            platform::notify_shortcut(path);
        }
    }
    progress("Panopticon Hub установлен.".into());
    Ok(InstallResult {
        executable: directory.join(HOST),
    })
}

fn recover_pending(
    directory: &Path,
    pending: &Pending,
    hash: &str,
    previous: Option<&(Receipt, String)>,
    shell: bool,
) -> Result<(), String> {
    let committed = previous.is_some_and(|(receipt, _)| *receipt == pending.receipt)
        && pending
            .receipt
            .binaries()
            .all(|(name, hash)| file_digest(&directory.join(name)).ok().as_deref() == Some(hash))
        && pending.receipt.shortcut_hashes.iter().all(|(path, hash)| {
            platform::reject_links(path).is_ok() && file_digest(path).ok().as_ref() == Some(hash)
        });
    transaction::recover(
        directory,
        pending,
        hash,
        previous.map(|(receipt, hash)| (receipt, hash.as_str())),
        shell,
        committed,
    )
}

pub fn uninstall(
    directory: &Path,
    mut progress: impl FnMut(String),
) -> Result<UninstallResult, String> {
    uninstall_inner(directory, &mut progress, true)
}

fn uninstall_inner(
    directory: &Path,
    progress: &mut impl FnMut(String),
    shell: bool,
) -> Result<UninstallResult, String> {
    validate_directory(directory)?;
    let lock = platform::InstallLock::take()?;
    let previous = read_committed(directory)?;
    let mut pending = match read_pending(directory) {
        Ok(pending) => pending,
        // Посторонний файл с именем журнала не даёт права его удалить.
        Err(_) if previous.is_some() => None,
        Err(error) => return Err(error),
    };
    let receipt = previous
        .as_ref()
        .map(|(r, _)| r)
        .or(pending.as_ref().map(|(p, _)| &p.receipt))
        .cloned()
        .ok_or("Не найден манифест собственной установки.")?;
    if shell && receipt.scope == Scope::Machine && !platform::is_admin() {
        return Err("Для удаления общей установки необходимы права администратора.".into());
    }
    require_closed_launchers(
        directory,
        previous.iter().map(|(r, _)| r).chain(pending.iter().map(|(p, _)| &p.receipt)),
    )?;
    if let Some((journal, hash)) = &pending {
        if journal.receipt.scope != receipt.scope {
            return Err("Режим незавершённой установки отличается от манифеста.".into());
        }
        if journal.recovery.is_some() {
            recover_pending(directory, journal, hash, previous.as_ref(), shell)?;
            pending = read_pending(directory)?;
        }
    }
    let mut preserved = Vec::new();
    progress("Удаление собственных файлов Panopticon Hub…".into());
    let known: Vec<&Receipt> = previous
        .iter()
        .map(|(r, _)| r)
        .chain(pending.iter().map(|(p, _)| &p.receipt))
        .collect();
    for name in [HOST, CORE, UNINSTALL] {
        let path = directory.join(name);
        let hashes: Vec<_> = known.iter().filter_map(|r| r.binary_hash(name)).collect();
        if !hashes.is_empty() {
            if let Err(error) = remove_owned(&path, &hashes, &mut preserved) {
                if name != UNINSTALL {
                    require_closed_binary(&path, &hashes)?;
                }
                return Err(error);
            }
        }
    }
    let mut shortcuts = BTreeMap::<&Path, Vec<&str>>::new();
    for receipt in &known {
        for path in &receipt.shortcuts {
            let hashes = shortcuts.entry(path).or_default();
            if let Some(hash) = receipt.shortcut_hashes.get(path) {
                hashes.push(hash);
            }
        }
    }
    for (path, hashes) in shortcuts {
        if platform::allowed_shortcut(path, receipt.scope)? {
            remove_owned(path, &hashes, &mut preserved)?;
        }
    }
    if shell {
        platform::unregister(&receipt)?;
    }
    if let Some((_, hash)) = pending.as_ref() {
        remove_owned(&directory.join(PENDING), &[hash], &mut preserved)?;
    } else {
        remove_owned(&directory.join(PENDING), &[], &mut preserved)?;
    }
    if let Some((_, hash)) = previous.as_ref() {
        remove_owned(&directory.join(RECEIPT), &[hash], &mut preserved)?;
    }
    // Только пустая папка. Пользовательские и изменённые файлы никогда не удаляются рекурсивно.
    let _ = fs::remove_dir(directory);
    drop(lock);
    progress("Panopticon Hub удалён. Пользовательские данные сохранены.".into());
    Ok(UninstallResult { preserved })
}

fn require_closed_launchers<'a>(
    directory: &Path,
    receipts: impl Iterator<Item = &'a Receipt> + Clone,
) -> Result<(), String> {
    for name in [HOST, CORE] {
        let hashes: Vec<_> = receipts.clone().filter_map(|r| r.binary_hash(name)).collect();
        require_closed_binary(&directory.join(name), &hashes)?;
    }
    Ok(())
}

fn require_closed_binary(path: &Path, hashes: &[&str]) -> Result<(), String> {
    if !hashes.is_empty()
        && platform::reject_links(path).is_ok()
        && platform::is_executable_running(path)?
        && hashes.contains(&file_digest(path)?.as_str())
    {
        return Err(platform::close_hub_message().into());
    }
    Ok(())
}

fn remove_owned(path: &Path, hashes: &[&str], preserved: &mut Vec<PathBuf>) -> Result<(), String> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_file() && platform::reject_links(path).is_ok() => {
            if hashes.is_empty() || !platform::delete_matching(path, hashes)? {
                preserved.push(path.to_owned());
            }
        }
        Ok(_) => preserved.push(path.to_owned()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.to_string()),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn temp(label: &str) -> PathBuf {
        let path =
            std::env::temp_dir().join(format!("panopticon-setup-{}-{label}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        path
    }
    fn options(path: &Path) -> InstallOptions {
        InstallOptions {
            directory: path.to_owned(),
            scope: Scope::User,
            desktop_shortcut: false,
            start_menu_shortcut: false,
        }
    }
    #[test]
    fn install_update_uninstall_preserves_foreign_files() {
        let root = temp("smoke");
        let cancelled = AtomicBool::new(false);
        let mut progress = |_: String| {};
        install_payload(
            &options(&root),
            &cancelled,
            &mut progress,
            b"MZhost1",
            b"MZsetup1",
            false,
        )
        .unwrap();
        fs::write(root.join("user-project.txt"), "Сохранить").unwrap();
        install_payload(
            &options(&root),
            &cancelled,
            &mut progress,
            b"MZhost2",
            b"MZsetup2",
            false,
        )
        .unwrap();
        assert_eq!(fs::read(root.join(HOST)).unwrap(), b"MZhost2");
        uninstall_inner(&root, &mut progress, false).unwrap();
        assert_eq!(
            fs::read_to_string(root.join("user-project.txt")).unwrap(),
            "Сохранить"
        );
        assert!(!root.join(HOST).exists());
        assert!(!root.join(RECEIPT).exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn style_offer_tracks_fresh_installation_receipts() {
        let root = temp("style-offer");
        let install = |host: &[u8], setup: &[u8]| {
            install_payload(&options(&root), &AtomicBool::new(false), &mut |_| {}, host, setup, false).unwrap();
            read_receipt(&root).unwrap()
        };
        let fresh = install(b"host1", b"setup1");
        assert_eq!(fresh.initial_style_offer, Some(true));
        assert!(fresh.installation_id.as_ref().is_some_and(|id| {
            id.len() == 32 && id.bytes().all(|byte| byte.is_ascii_hexdigit())
        }));

        let repaired = install(b"host1", b"setup1");
        assert_eq!(repaired.installation_id, fresh.installation_id);
        assert_eq!(repaired.initial_style_offer, fresh.initial_style_offer);

        let mut older = repaired;
        older.version = "0.2.16".into();
        write_record(&root, RECEIPT, &older, true).unwrap();
        let upgraded = install(b"host2", b"setup2");
        assert_eq!(upgraded.version, VERSION);
        assert_eq!(upgraded.installation_id, fresh.installation_id);
        assert_eq!(upgraded.initial_style_offer, fresh.initial_style_offer);

        let mut disabled = upgraded;
        disabled.initial_style_offer = Some(false);
        write_record(&root, RECEIPT, &disabled, true).unwrap();
        let repaired = install(b"host2", b"setup2");
        assert_eq!(repaired.installation_id, fresh.installation_id);
        assert_eq!(repaired.initial_style_offer, Some(false));

        uninstall_inner(&root, &mut |_| {}, false).unwrap();
        assert!(!root.join(RECEIPT).exists());
        let reinstalled = install(b"host2", b"setup2");
        assert_eq!(reinstalled.initial_style_offer, Some(true));
        assert_ne!(reinstalled.installation_id, fresh.installation_id);
        uninstall_inner(&root, &mut |_| {}, false).unwrap();
    }

    #[test]
    fn refuses_foreign_or_modified_payload() {
        let root = temp("foreign");
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join(HOST), "чужое").unwrap();
        assert!(install_payload(
            &options(&root),
            &AtomicBool::new(false),
            &mut |_| {},
            b"host",
            b"setup",
            false
        )
        .is_err());
        assert_eq!(fs::read(root.join(HOST)).unwrap(), "чужое".as_bytes());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn cancellation_is_retryable() {
        let root = temp("cancel");
        let cancelled = AtomicBool::new(false);
        let error = install_payload(
            &options(&root),
            &cancelled,
            &mut |_| cancelled.store(true, Ordering::Release),
            b"host",
            b"setup",
            false,
        );
        assert!(error.is_err());
        cancelled.store(false, Ordering::Release);
        install_payload(
            &options(&root),
            &cancelled,
            &mut |_| {},
            b"host",
            b"setup",
            false,
        )
        .unwrap();
        uninstall_inner(&root, &mut |_| {}, false).unwrap();
        assert!(!root.exists());
    }
    #[test]
    fn uninstall_keeps_modified_file() {
        let root = temp("modified");
        install_payload(
            &options(&root),
            &AtomicBool::new(false),
            &mut |_| {},
            b"host",
            b"setup",
            false,
        )
        .unwrap();
        fs::write(root.join(HOST), "пользователь").unwrap();
        let result = uninstall_inner(&root, &mut |_| {}, false).unwrap();
        assert_eq!(result.preserved, vec![root.join(HOST)]);
        assert_eq!(
            fs::read(root.join(HOST)).unwrap(),
            "пользователь".as_bytes()
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn uninstall_removes_only_the_recorded_adjacent_core() {
        for (label, record_core, core_bytes) in [
            ("owned-core", true, b"stock".as_slice()),
            ("modified-core", true, b"foreign".as_slice()),
            ("legacy-core", false, b"stock".as_slice()),
        ] {
            let root = temp(label);
            install_payload(
                &options(&root),
                &AtomicBool::new(false),
                &mut |_| {},
                b"host",
                b"setup",
                false,
            )
            .unwrap();
            let core = root.join(CORE);
            fs::write(&core, core_bytes).unwrap();
            let mut receipt = read_receipt(&root).unwrap();
            assert!(receipt.panopticon_core_sha256.is_none());
            if record_core {
                receipt.panopticon_core_sha256 = Some(digest(b"stock"));
                write_record(&root, RECEIPT, &receipt, true).unwrap();
            }
            let result = uninstall_inner(&root, &mut |_| {}, false).unwrap();
            assert!(!root.join(HOST).exists());
            assert!(!root.join(UNINSTALL).exists());
            assert!(!root.join(RECEIPT).exists());
            if label == "owned-core" {
                assert!(result.preserved.is_empty());
                assert!(!root.exists());
            } else {
                assert_eq!(fs::read(&core).unwrap(), core_bytes);
                if record_core {
                    assert_eq!(result.preserved, [core.clone()]);
                }
                fs::remove_file(core).unwrap();
                fs::remove_dir(root).unwrap();
            }
        }
    }
    #[test]
    fn busy_payload_can_be_retried_without_corruption() {
        use std::os::windows::fs::OpenOptionsExt;
        let root = temp("busy");
        let cancel = AtomicBool::new(false);
        install_payload(
            &options(&root),
            &cancel,
            &mut |_| {},
            b"host1",
            b"setup1",
            false,
        )
        .unwrap();
        let held = OpenOptions::new()
            .read(true)
            .share_mode(0)
            .open(root.join(HOST))
            .unwrap();
        assert!(install_payload(
            &options(&root),
            &cancel,
            &mut |_| {},
            b"host2",
            b"setup2",
            false
        )
        .is_err());
        drop(held);
        assert_eq!(fs::read(root.join(HOST)).unwrap(), b"host1");
        install_payload(
            &options(&root),
            &cancel,
            &mut |_| {},
            b"host2",
            b"setup2",
            false,
        )
        .unwrap();
        uninstall_inner(&root, &mut |_| {}, false).unwrap();
        assert!(!root.exists());
    }
    #[test]
    fn failed_upgrade_restores_both_binaries_and_receipt() {
        let root = temp("rollback");
        let cancel = AtomicBool::new(false);
        install_payload(
            &options(&root),
            &cancel,
            &mut |_| {},
            b"host1",
            b"setup1",
            false,
        )
        .unwrap();
        let receipt = fs::read(root.join(RECEIPT)).unwrap();
        let error = install_payload(
            &options(&root),
            &cancel,
            &mut |stage| {
                if stage.starts_with("Подготовка удаления") {
                    assert_eq!(fs::read(root.join(HOST)).unwrap(), b"host2");
                    cancel.store(true, Ordering::Release);
                }
            },
            b"host2",
            b"setup2",
            false,
        );
        assert!(error.is_err());
        assert_eq!(fs::read(root.join(HOST)).unwrap(), b"host1");
        assert_eq!(fs::read(root.join(UNINSTALL)).unwrap(), b"setup1");
        assert_eq!(fs::read(root.join(RECEIPT)).unwrap(), receipt);
        assert!(!root.join(PENDING).exists());
        uninstall_inner(&root, &mut |_| {}, false).unwrap();
        assert!(!root.exists());
    }

    fn legacy_fixture(root: &Path, host: &[u8], uninstaller: &[u8], core: Option<&[u8]>) -> Receipt {
        fs::create_dir_all(root).unwrap();
        fs::write(root.join(HOST), host).unwrap();
        fs::write(root.join(UNINSTALL), uninstaller).unwrap();
        if let Some(core) = core {
            fs::write(root.join(CORE), core).unwrap();
        }
        let receipt = Receipt {
            owner: OWNER.into(),
            directory: root.to_owned(),
            scope: Scope::User,
            version: "0.2.0".into(),
            installation_id: None,
            initial_style_offer: None,
            files: BTreeMap::from([(HOST.into(), digest(host)), (UNINSTALL.into(), digest(uninstaller))]),
            panopticon_core_sha256: core.map(digest),
            shortcuts: Vec::new(),
            shortcut_hashes: BTreeMap::new(),
        };
        write_record(root, RECEIPT, &receipt, false).unwrap();
        receipt
    }

    #[test]
    fn uninstall_requires_only_the_recorded_running_executable_to_close() {
        use std::{
            os::windows::process::CommandExt,
            process::{Command, Stdio},
        };
        use windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;

        let cmd = PathBuf::from(std::env::var_os("SystemRoot").unwrap()).join("System32/cmd.exe");
        let executable = fs::read(cmd).unwrap();
        for name in [HOST, CORE] {
            let root = temp(&format!("running-{name}"));
            let other = temp(&format!("other-{name}"));
            let (host, core): (&[u8], &[u8]) = if name == HOST {
                (&executable, b"legacy core")
            } else {
                (b"host", &executable)
            };
            let receipt = legacy_fixture(&root, host, b"uninstaller", Some(core));
            let manifest = fs::read(root.join(RECEIPT)).unwrap();
            let mut process = Command::new(root.join(name))
                .args(["/d", "/q"])
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .creation_flags(CREATE_NO_WINDOW)
                .spawn()
                .unwrap();

            legacy_fixture(&other, host, b"uninstaller", Some(core));
            uninstall_inner(&other, &mut |_| {}, false).unwrap();
            assert!(!other.exists());
            assert_eq!(receipt.check_hub_closed().unwrap_err(), platform::close_hub_message());
            let result = uninstall_inner(&root, &mut |_| panic!("Началось удаление"), false);
            assert_eq!(result.err().as_deref(), Some(platform::close_hub_message()));
            assert_eq!(fs::read(root.join(HOST)).unwrap(), host);
            assert_eq!(fs::read(root.join(CORE)).unwrap(), core);
            assert_eq!(fs::read(root.join(UNINSTALL)).unwrap(), b"uninstaller");
            assert_eq!(fs::read(root.join(RECEIPT)).unwrap(), manifest);

            drop(process.stdin.take());
            assert!(process.wait().unwrap().success());
            uninstall_inner(&root, &mut |_| {}, false).unwrap();
            assert!(!root.exists());
        }
    }

    #[test]
    fn upgrades_previous_patch_without_resetting_installation_id() {
        let root = temp("previous-patch");
        let mut receipt = legacy_fixture(&root, b"host1", b"setup1", None);
        receipt.version = "0.2.16".into();
        receipt.installation_id = Some("same-installation".into());
        write_record(&root, RECEIPT, &receipt, true).unwrap();

        install_payload(
            &options(&root),
            &AtomicBool::new(false),
            &mut |_| {},
            b"host2",
            b"setup2",
            false,
        )
        .unwrap();

        let updated = read_receipt(&root).unwrap();
        assert_eq!(updated.version, VERSION);
        assert_eq!(updated.installation_id, receipt.installation_id);
        assert_eq!(updated.initial_style_offer, None);
        let stored: serde_json::Value = serde_json::from_slice(&fs::read(root.join(RECEIPT)).unwrap()).unwrap();
        assert!(stored.get("initial_style_offer").is_none());
        assert_eq!(fs::read(root.join(HOST)).unwrap(), b"host2");
        uninstall_inner(&root, &mut |_| {}, false).unwrap();
    }

    #[test]
    fn legacy_core_removal_is_rolled_back_then_committed() {
        let root = temp("legacy-core-rollback");
        let previous = legacy_fixture(&root, b"legacy launcher", b"legacy setup", Some(b"legacy core"));
        let cancel = AtomicBool::new(false);
        let options = options(&root);
        let update = |progress: &mut dyn FnMut(String)| {
            install_payload(&options, &cancel, &mut |stage| progress(stage), b"unified host", b"light setup", false)
        };
        let error = update(&mut |stage: String| {
            if stage.starts_with("Удаление прежнего ядра") {
                assert!(!root.join(CORE).exists());
                cancel.store(true, Ordering::Release);
            }
        });
        assert!(error.is_err());
        assert_eq!(read_receipt(&root).unwrap(), previous);
        assert_eq!(fs::read(root.join(HOST)).unwrap(), b"legacy launcher");
        assert_eq!(fs::read(root.join(UNINSTALL)).unwrap(), b"legacy setup");
        assert_eq!(fs::read(root.join(CORE)).unwrap(), b"legacy core");
        assert!(!root.join(PENDING).exists());

        cancel.store(false, Ordering::Release);
        let interruption = std::panic::catch_unwind(|| {
            let _ = update(&mut |stage| {
                if stage.starts_with("Удаление прежнего ядра") {
                    panic!("Сбой до записи финального receipt");
                }
            });
        });
        assert!(interruption.is_err());
        assert!(root.join(PENDING).is_file());
        assert!(!root.join(CORE).exists());
        update(&mut |_| {}).unwrap();
        let receipt = read_receipt(&root).unwrap();
        assert_eq!(receipt.version, VERSION);
        assert_eq!(receipt.installation_id, None);
        assert_eq!(receipt.initial_style_offer, None);
        assert_eq!(receipt.files.len(), 2);
        assert!(receipt.binary_hash(CORE).is_none());
        assert!(!root.join(CORE).exists());
        assert_eq!(fs::read_dir(&root).unwrap().count(), 3);
        uninstall_inner(&root, &mut |_| {}, false).unwrap();
        assert!(!root.exists());
    }

    #[test]
    fn unified_install_leaves_modified_adjacent_core() {
        let root = temp("foreign-adjacent-core");
        legacy_fixture(&root, b"old host", b"old setup", Some(b"owned core"));
        fs::write(root.join(CORE), b"foreign core").unwrap();
        install_payload(
            &options(&root), &AtomicBool::new(false), &mut |_| {}, b"unified host", b"light setup", false,
        )
        .unwrap();
        uninstall_inner(&root, &mut |_| {}, false).unwrap();
        assert_eq!(fs::read(root.join(CORE)).unwrap(), b"foreign core");
        fs::remove_file(root.join(CORE)).unwrap();
        fs::remove_dir(root).unwrap();
    }

    #[test]
    #[ignore = "Нужны копии релизов 0.2.0 и 0.2.7 в PANOPTICON_SETUP_TEST_RELEASES"]
    fn upgrades_verified_original_and_stock_releases() {
        let fixtures = PathBuf::from(std::env::var_os("PANOPTICON_SETUP_TEST_RELEASES").unwrap());
        for (version, expected_host, expected_setup, expected_core) in [
            ("0.2.0", "457ed6b487c414e98ff4f1177b14294fc0ea3a9263644904813af20b1eeba680", "ca37b49b3f12d42ddccfaa52263b42ac5c09cbef36df46f854a473f5cae2849d", None),
            ("0.2.7", "88d46e0891369cc986b3bc4f22ac00fb686be6fa07a2b735de5457109bdf2ad5", "9e2db1031f8834b5b971311595b8e03befccd9ddfdf6209a1e9657e0ced39bb2", Some("4800de9910069c9ca846c9c7179347af87efcbda98460ef0af25e71c8d303c8d")),
        ] {
            let dir = fixtures.join(version);
            let host = fs::read(dir.join(HOST)).unwrap();
            let setup = fs::read(dir.join(UNINSTALL)).unwrap();
            let core = expected_core.map(|_| fs::read(dir.join(CORE)).unwrap());
            assert_eq!(digest(&host), expected_host);
            assert_eq!(digest(&setup), expected_setup);
            assert_eq!(core.as_deref().map(digest).as_deref(), expected_core);
            let root = temp(version);
            legacy_fixture(&root, &host, &setup, core.as_deref());
            let payload = if PAYLOAD.is_empty() { b"unified host" } else { PAYLOAD };
            let uninstaller = if UNINSTALL_PAYLOAD.is_empty() { b"light setup" } else { UNINSTALL_PAYLOAD };
            install_payload(&options(&root), &AtomicBool::new(false), &mut |_| {}, payload, uninstaller, false).unwrap();
            let receipt = read_receipt(&root).unwrap();
            assert_eq!(receipt.version, VERSION);
            assert_eq!(receipt.files.len(), 2);
            assert!(receipt.binary_hash(CORE).is_none());
            assert_eq!(fs::read(root.join(HOST)).unwrap(), payload);
            assert_eq!(fs::read(root.join(UNINSTALL)).unwrap(), uninstaller);
            assert!(!root.join(CORE).exists());
            uninstall_inner(&root, &mut |_| {}, false).unwrap();
            assert!(!root.exists());
        }
    }

    #[test]
    fn interrupted_upgrade_recovers_before_different_payload() {
        let root = temp("interrupted");
        let cancel = AtomicBool::new(false);
        install_payload(
            &options(&root),
            &cancel,
            &mut |_| {},
            b"host1",
            b"setup1",
            false,
        )
        .unwrap();
        let interruption = std::panic::catch_unwind(|| {
            let _ = install_payload(
                &options(&root),
                &cancel,
                &mut |stage| {
                    if stage.starts_with("Подготовка удаления") {
                        panic!("simulated interruption");
                    }
                },
                b"host2",
                b"setup2",
                false,
            );
        });
        assert!(interruption.is_err());
        assert_eq!(fs::read(root.join(HOST)).unwrap(), b"host2");
        assert!(root.join(PENDING).exists());
        install_payload(
            &options(&root),
            &cancel,
            &mut |_| {},
            b"host3",
            b"setup3",
            false,
        )
        .unwrap();
        assert_eq!(fs::read(root.join(HOST)).unwrap(), b"host3");
        assert_eq!(fs::read(root.join(UNINSTALL)).unwrap(), b"setup3");
        assert!(!root.join(PENDING).exists());
        uninstall_inner(&root, &mut |_| {}, false).unwrap();
        assert!(!root.exists());
    }

    #[test]
    fn legacy_interrupted_upgrade_removes_only_recorded_payload() {
        let root = temp("legacy");
        let cancel = AtomicBool::new(false);
        install_payload(
            &options(&root),
            &cancel,
            &mut |_| {},
            b"host1",
            b"setup1",
            false,
        )
        .unwrap();
        let mut pending = read_receipt(&root).unwrap();
        pending.files.insert(HOST.into(), digest(b"host2"));
        pending.files.insert(UNINSTALL.into(), digest(b"setup2"));
        write_record(&root, PENDING, &pending, false).unwrap();
        fs::write(root.join(HOST), b"host2").unwrap();
        fs::write(root.join(".panopticon-setup.lock"), b"foreign").unwrap();
        let result = uninstall_inner(&root, &mut |_| {}, false).unwrap();
        assert!(result.preserved.is_empty());
        assert!(!root.join(HOST).exists());
        assert!(!root.join(UNINSTALL).exists());
        assert!(!root.join(PENDING).exists());
        assert_eq!(
            fs::read(root.join(".panopticon-setup.lock")).unwrap(),
            b"foreign"
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn unrelated_journal_is_preserved_during_uninstall() {
        let root = temp("foreign-journal");
        let cancel = AtomicBool::new(false);
        install_payload(
            &options(&root),
            &cancel,
            &mut |_| {},
            b"host",
            b"setup",
            false,
        )
        .unwrap();
        fs::write(root.join(PENDING), b"unrelated data").unwrap();
        let result = uninstall_inner(&root, &mut |_| {}, false).unwrap();
        assert_eq!(result.preserved, [root.join(PENDING)]);
        assert_eq!(fs::read(root.join(PENDING)).unwrap(), b"unrelated data");
        assert!(!root.join(HOST).exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn shortcut_created_before_publication_is_preserved() {
        let root = temp("shortcut-conflict");
        fs::create_dir_all(&root).unwrap();
        let mut work = Work::create(&root).unwrap();
        let hash = work.stage_shortcut(0, &root, None).unwrap();
        let shortcut = root.join("Panopticon Hub.lnk");
        fs::write(&shortcut, b"other shortcut").unwrap();
        assert!(work.publish_shortcut(0, &shortcut, &hash, None).is_err());
        assert_eq!(fs::read(&shortcut).unwrap(), b"other shortcut");
        assert!(!platform::delete_matching(&shortcut, &[&hash]).unwrap());
        assert_eq!(fs::read(&shortcut).unwrap(), b"other shortcut");
        drop(work);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn accepts_supported_receipts_across_minor_versions() {
        for supported in ["0.2.0", "0.2.8", "0.2.36", "0.3.0"] {
            assert!(compatible_receipt_version(supported, "0.3.0"), "{supported}");
        }
        for unsupported in ["0.1.9", "0.2.1", "0.2.7", "0.3.1", "0.02.36", "0.2.+36", "0.2.36.0", "0.3.0-alpha"] {
            assert!(!compatible_receipt_version(unsupported, "0.3.0"), "{unsupported}");
        }
        assert!(!compatible_receipt_version("0.2.0", "0.1.9"));
    }

    #[test]
    fn update_relaunch_refuses_incomplete_or_modified_installations() {
        let root = temp("update-relaunch-refuses");
        install_payload(
            &options(&root),
            &AtomicBool::new(false),
            &mut |_| {},
            b"host1",
            b"setup1",
            false,
        ).unwrap();
        let previous = read_update_receipt(&root, Scope::User).unwrap();
        assert!(read_update_receipt(&root, Scope::Machine).is_err());

        fs::write(root.join(HOST), b"modified executable").unwrap();
        assert!(previous.restart_previous_hub().unwrap_err().contains("отличается от манифеста"));
        fs::write(root.join(HOST), b"host1").unwrap();
        write_record(&root, PENDING, &previous, false).unwrap();
        assert!(previous.restart_previous_hub().unwrap_err().contains("подтвердить восстановление"));
        fs::remove_file(root.join(PENDING)).unwrap();
        uninstall_inner(&root, &mut |_| {}, false).unwrap();
    }

    #[test]
    fn dangerous_directories_are_rejected() {
        for path in [
            Path::new("C:\\"),
            Path::new("C:\\Windows"),
            Path::new("C:\\Windows\\Panopticon Hub"),
            Path::new("\\\\server\\share\\Hub"),
            Path::new("C:\\Apps\\..\\Hub"),
            Path::new("C:\\Apps\\NUL.txt"),
            Path::new("C:\\Apps\\Hub:payload"),
            Path::new("C:\\Apps\\Hub."),
            Path::new("C:\\Windows.\\Panopticon Hub"),
        ] {
            assert!(validate_directory(path).is_err(), "{}", path.display());
        }
    }
}
