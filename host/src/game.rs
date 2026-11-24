use crate::{design, game_profile, workshop::MountPlan};
use hub_backend::{steam, LaunchRequest};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, ErrorKind, Read, Write},
    os::windows::{
        fs::{MetadataExt, OpenOptionsExt},
        process::CommandExt,
    },
    path::{Component, Path, PathBuf},
    process::{Child, ChildStdout, Command, Stdio},
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::{Duration, Instant},
};
use windows_sys::Win32::{
    Foundation::{ERROR_FILE_NOT_FOUND, ERROR_SUCCESS},
    System::Registry::{
        RegCloseKey, RegDeleteValueW, RegGetValueW, RegOpenKeyExW, RegSetValueExW,
        HKEY, HKEY_CURRENT_USER, KEY_QUERY_VALUE, KEY_SET_VALUE, REG_DWORD, RRF_RT_REG_DWORD,
    },
};
use windows_sys::w;

include!(concat!(env!("OUT_DIR"), "/game_assets.rs"));

const MODULE: &str = "lua/bin/gmsv_panopticon_native_win64.dll";
const SETTINGS: &str = "data/drp.settings.txt";
const BACKGROUND: &str = "lua/menu/background.lua";
const WORKSHOP_FILTER: &str = "cfg/addonnomount.txt";
const LAB_CONFIG: &str = "cfg/config.cfg";
const LAB_SCRIPT: &str = "lua/autorun/client/panopticon_stock_probe.lua";
const LAB_MODULE: &str = "lua/bin/gmcl_panopticon_perf_win64.dll";
const LEGACY_GEOMETRY: &str = "lua/bin/gmcl_panopticon_geometry_win64.dll";
const BACKGROUND_ADAPTER: &[u8] =
    include_bytes!("../../game-ui/source/lua/menu/panopticon_background.lua");
const LEGACY_SKIPPED_LOADING_PAGE: &str = "html/panopticon/loading-skipped.html";
const DESIGN_PROFILE: &str = "html/panopticon/design-profile.js";
const GAME_IDENTITY: &str = "html/panopticon/design-identity.json";
const DESIGN_MEDIA: &str = "html/panopticon/design-media/";
const DESIGN_LIMIT: u64 = 64 * 1024 * 1024;

const STATE: &str = ".panopticon-stock-session.json";
const LAB_STATE: &str = ".panopticon-experimental-session.json";
const LEGACY_LAB_STATE: &str = ".panopticon-renderlab-session.json";
const LIMIT: u64 = 8 * 1024 * 1024;
const STEAM_START_TIMEOUT: Duration = Duration::from_secs(60);

// Восстановление прерванного сеанса после обновления со старых версий.
const VIDEO_SETTINGS: *const u16 = w!("SOFTWARE\\Valve\\Source\\garrysmod\\Settings");
const FILTERING: *const u16 = w!("mat_forceaniso");

#[derive(Clone, Copy, Deserialize, Serialize)]
struct LegacyFiltering {
    before: Option<u32>,
    applied: u32,
}

impl LegacyFiltering {
    fn read() -> Result<Option<u32>, String> {
        let mut number = 0u32;
        let mut size = 4;
        unsafe {
            match RegGetValueW(HKEY_CURRENT_USER, VIDEO_SETTINGS, FILTERING, RRF_RT_REG_DWORD,
                std::ptr::null_mut(), (&mut number as *mut u32).cast(), &mut size)
            {
                ERROR_SUCCESS => Ok(Some(number)),
                ERROR_FILE_NOT_FOUND => Ok(None),
                err => Err(io::Error::from_raw_os_error(err as i32).to_string()),
            }
        }
    }

    fn restore(self) -> Result<(), String> {
        let current = Self::read()?;
        if current == self.before || current != Some(self.applied) {
            return Ok(());
        }
        let mut key: HKEY = std::ptr::null_mut();
        let open = unsafe { RegOpenKeyExW(HKEY_CURRENT_USER, VIDEO_SETTINGS, 0, KEY_QUERY_VALUE | KEY_SET_VALUE, &mut key) };
        if open == ERROR_FILE_NOT_FOUND {
            return Ok(());
        }
        if open != ERROR_SUCCESS {
            return Err(io::Error::from_raw_os_error(open as i32).to_string());
        }
        let status = unsafe {
            let status = if let Some(before) = self.before {
                let bytes = before.to_le_bytes();
                RegSetValueExW(key, FILTERING, 0, REG_DWORD, bytes.as_ptr(), bytes.len() as u32)
            } else {
                RegDeleteValueW(key, FILTERING)
            };
            RegCloseKey(key);
            status
        };
        if matches!(status, ERROR_SUCCESS | ERROR_FILE_NOT_FOUND) {
            return Ok(());
        }
        Err(io::Error::from_raw_os_error(status as i32).to_string())
    }
}

#[derive(Serialize, Deserialize)]
struct Entry {
    name: String,
    before: Option<Vec<u8>>,
    installed: String,
}

#[derive(Serialize, Deserialize)]
struct Session {
    settings: Option<PathBuf>,
    started: bool,
    entries: Vec<Entry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    filtering: Option<LegacyFiltering>,
}

pub(crate) fn ordinary(root: &Path, relative: &str) -> Result<PathBuf, String> {
    let rel = Path::new(relative);
    if rel
        .components()
        .any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err("Недопустимый путь игровых файлов".into());
    }
    let path = root.join(rel);
    for ancestor in path.ancestors() {
        match ancestor.symlink_metadata() {
            Ok(meta) if meta.file_attributes() & 0x400 != 0 => {
                return Err(format!("Каталог перенаправлен: {}", ancestor.display()));
            }
            Err(err) if err.kind() != ErrorKind::NotFound => return Err(err.to_string()),
            _ => {}
        }
    }
    Ok(path)
}

fn read(path: &Path, limit: u64) -> Result<Option<Vec<u8>>, String> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(err.to_string()),
    };
    let mut bytes = Vec::new();
    file.take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(|err| err.to_string())?;
    if bytes.len() as u64 > limit {
        return Err(format!(
            "Слишком большой файл для подготовки меню: {}",
            path.display()
        ));
    }
    Ok(Some(bytes))
}

fn put(path: &Path, bytes: Option<&[u8]>) -> Result<(), String> {
    match bytes {
        Some(bytes) => {
            let parent = path.parent().ok_or("Недопустимый путь игровых файлов")?;
            fs::create_dir_all(parent).map_err(|err| err.to_string())?;
            let mut temp =
                tempfile::NamedTempFile::new_in(parent).map_err(|err| err.to_string())?;
            temp.write_all(bytes).map_err(|err| err.to_string())?;
            temp.as_file().sync_all().map_err(|err| err.to_string())?;
            temp.persist(path).map_err(|err| err.error.to_string())?;
        }
        None => match fs::remove_file(path) {
            Ok(()) => {}
            Err(err) if err.kind() == ErrorKind::NotFound => {}
            Err(err) => return Err(err.to_string()),
        },
    }
    Ok(())
}

fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn media_hash(input: impl Read) -> Result<String, String> {
    let mut input = input.take(DESIGN_LIMIT + 1);
    let mut digest = Sha256::new();
    let size = io::copy(&mut input, &mut digest).map_err(|err| err.to_string())?;
    if size > DESIGN_LIMIT {
        return Err("Медиа оформления превышает допустимый размер.".into());
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn installed_media(path: &Path) -> Result<Option<String>, String> {
    match File::open(path) {
        Ok(file) => media_hash(file).map(Some),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err.to_string()),
    }
}

fn stage_media(root: &Path, session: &mut Session, key: &str) -> Result<(), String> {
    if !design::valid_media_key(key) {
        return Err("Некорректный файл оформления игры.".into());
    }
    let name = format!("{DESIGN_MEDIA}{key}");
    let installed = key[..64].to_owned();
    match installed_media(&ordinary(root, &name)?)? {
        None => session.entries.push(Entry { name, before: None, installed }),
        Some(hash) if hash == installed => {}
        Some(_) => return Err(format!("Файл игрового оформления уже занят: {name}")),
    }
    Ok(())
}

fn apply_media(root: &Path, entries: &[Entry], mut media: design::GameMedia) -> Result<(), String> {
    for entry in entries {
        let key = entry.name.strip_prefix(DESIGN_MEDIA).ok_or("Не найден путь игрового оформления.")?;
        let path = ordinary(root, &entry.name)?;
        let parent = path.parent().ok_or("Не найден каталог игрового оформления.")?;
        fs::create_dir_all(parent).map_err(|err| err.to_string())?;
        let mut source = media.take(key)?;
        let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|err| err.to_string())?;
        io::copy(&mut source, &mut temp).map_err(|err| err.to_string())?;
        temp.as_file().sync_all().map_err(|err| err.to_string())?;
        temp.persist_noclobber(&path).map_err(|err| err.error.to_string())?;
    }
    Ok(())
}

fn save_session(journal: &Path, session: &Session) -> Result<(), String> {
    let data = serde_json::to_vec(session).map_err(|err| err.to_string())?;
    put(journal, Some(&data))
}

fn recover(root: &Path, journal: &Path, settings: &Path) -> Result<(), String> {
    if let Some(bytes) = read(journal, LIMIT)? {
        let previous: Session = serde_json::from_slice(&bytes).map_err(|err| err.to_string())?;
        let private = match previous.settings.as_deref() {
            None => settings.to_owned(),
            Some(path) if path == settings => settings.to_owned(),
            _ => stable_settings()?,
        };
        restore(root, journal, &private, &previous, previous.started)?;
    }
    Ok(())
}

fn config_record(line: &[u8]) -> Option<(&str, &str)> {
    let line = std::str::from_utf8(line).ok()?.trim_start_matches('\u{feff}');
    let (key, value) = line.trim().split_once(char::is_whitespace)?;
    Some((key, value.trim().trim_matches('"')))
}

fn restore_config(
    path: &Path,
    original: Option<&[u8]>,
    current: Option<&[u8]>,
    overrides: &str,
) -> Result<(), String> {
    let Some(current) = current else {
        return Ok(());
    };
    let original = original.unwrap_or_default();
    let mut overrides = overrides.split_whitespace().filter_map(|pair| pair.split_once('=')).peekable();
    let legacy = [("fps_max", "0")];
    let keys: Vec<_> = if overrides.peek().is_none() { legacy.to_vec() } else { overrides.collect() };
    let mut restored = Vec::with_capacity(current.len());
    for line in current.split_inclusive(|&byte| byte == b'\n') {
        let Some((key, current_value)) = config_record(line) else {
            restored.extend_from_slice(line);
            continue;
        };
        if !matches!(
            key,
            "fps_max" | "r_waterforcereflectentities" | "r_shadowmaxrendered"
                | "r_waterforceexpensive" | "mat_forceaniso"
        )
            || !keys.iter().any(|(name, forced)| *name == key && *forced == current_value)
        {
            restored.extend_from_slice(line);
            continue;
        }
        if let Some(before) = original
            .split_inclusive(|&byte| byte == b'\n')
            .rfind(|before| config_record(before).is_some_and(|(name, _)| name == key))
        {
            let before = before.strip_suffix(b"\n").unwrap_or(before);
            let before = before.strip_suffix(b"\r").unwrap_or(before);
            restored.extend_from_slice(before);
            if line.ends_with(b"\r\n") {
                restored.extend_from_slice(b"\r\n");
            } else if line.ends_with(b"\n") {
                restored.push(b'\n');
            }
        }
    }
    if restored != current {
        put(path, Some(&restored))?;
    }
    Ok(())
}

fn restore(
    root: &Path,
    journal: &Path,
    settings: &Path,
    session: &Session,
    played: bool,
) -> Result<(), String> {
    let mut media_error = None;
    for entry in session.entries.iter().rev() {
        if let Some(key) = entry.name.strip_prefix(DESIGN_MEDIA) {
            if !design::valid_media_key(key) || entry.before.is_some() || entry.installed != key[..64] {
                return Err("Некорректный файл оформления в журнале восстановления.".into());
            }
            let result = (|| {
                let path = ordinary(root, &entry.name)?;
                match fs::symlink_metadata(&path) {
                    Err(err) if err.kind() == ErrorKind::NotFound => return Ok(()),
                    Err(err) => return Err(err.to_string()),
                    Ok(meta) if !meta.is_file() || meta.len() > DESIGN_LIMIT => return Ok(()),
                    Ok(_) => {}
                }
                if installed_media(&path)?.as_deref() == Some(entry.installed.as_str()) {
                    put(&path, None)?;
                }
                Ok::<(), String>(())
            })();
            if let Err(error) = result {
                media_error.get_or_insert(error);
            }
            continue;
        }
        if entry.name != SETTINGS
            && entry.name != BACKGROUND
            && entry.name != MODULE
            && entry.name != WORKSHOP_FILTER
            && entry.name != LAB_CONFIG
            && entry.name != LAB_SCRIPT
            && entry.name != LAB_MODULE
            && entry.name != LEGACY_GEOMETRY
            && entry.name != DESIGN_PROFILE
            && entry.name != GAME_IDENTITY
            && entry.name != LEGACY_SKIPPED_LOADING_PAGE
            && !GAME_FILES
                .iter()
                .any(|(name, _)| *name == entry.name)
        {
            return Err(format!(
                "Неизвестный файл в журнале восстановления: {}",
                entry.name
            ));
        }
        let path = ordinary(root, &entry.name)?;
        let current = read(&path, LIMIT)?;
        if current == entry.before {
            continue;
        } else if entry.name == LAB_CONFIG {
            if played {
                restore_config(&path, entry.before.as_deref(), current.as_deref(), &entry.installed)?;
            }
            continue;
        } else if entry.name == SETTINGS && played {
            if session.settings.as_deref() == Some(settings) {
                put(settings, current.as_deref())?;
            }
        } else if current.as_deref().map(hash).as_deref() != Some(entry.installed.as_str()) {
            // Если Steam заменил файл, сохраняем его новую версию.
            continue;
        }
        if current != entry.before {
            put(&path, entry.before.as_deref())?;
        }
    }
    if played {
        if let Some(filtering) = session.filtering {
            filtering.restore()?;
        }
    }
    if let Some(error) = media_error {
        return Err(error);
    }
    put(journal, None)
}

fn apply(root: &Path, session: &Session, assets: &[(&str, &[u8])]) -> Result<(), String> {
    for (entry, (_, bytes)) in session.entries.iter().zip(assets) {
        let path = ordinary(root, &entry.name)?;
        if read(&path, LIMIT)? != entry.before {
            return Err(format!("Файл изменился при подготовке: {}", path.display()));
        }
        if entry.before.as_deref() != Some(*bytes) {
            put(&path, Some(bytes))?;
        }
    }
    Ok(())
}

fn private_settings() -> Result<PathBuf, String> {
    let data = PathBuf::from(std::env::var_os("LOCALAPPDATA").ok_or("Отсутствует LOCALAPPDATA")?);
    ordinary(&data, "PanopticonHub/game-data/drp.settings.txt")
}

fn stable_settings() -> Result<PathBuf, String> {
    let data = PathBuf::from(std::env::var_os("LOCALAPPDATA").ok_or("Отсутствует LOCALAPPDATA")?);
    ordinary(&data, "PanopticonHubRenderLab/game-data/drp.settings.txt")
}

fn wait_for_game_exit(game_root: &Path) -> Result<(), String> {
    let mut last_error = None;
    for _ in 0..40 {
        match game_profile::game_running(game_root) {
            Ok(false) => return Ok(()),
            Ok(true) => last_error = None,
            // The renderer processes can disappear between enumeration and inspection.
            Err(error) => last_error = Some(error),
        }
        thread::sleep(Duration::from_millis(250));
    }
    Err(last_error.unwrap_or_else(|| "Ещё работают процессы Garry’s Mod. Меню будет восстановлено при следующем запуске Hub.".into()))
}

fn workshop_filter(original: Option<&[u8]>, plan: &MountPlan) -> Result<Option<Vec<u8>>, String> {
    if plan.allowed.is_empty() { return Ok(None); }
    let mut disabled = BTreeSet::new();
    if let Some(original) = original.filter(|data| !data.is_empty()) {
        let text = std::str::from_utf8(original).map_err(|error| error.to_string())?;
        let config = steam::parse_vdf(text.trim_start_matches('\u{feff}'))?;
        let entries = config["addonnomount"].as_object().ok_or("Не удалось прочитать список отключённых аддонов Garry’s Mod.")?;
        for value in entries.values() {
            let id = value.as_str().and_then(|id| id.parse::<u64>().ok())
                .filter(|id| *id != 0).ok_or("Некорректный Workshop ID в cfg/addonnomount.txt.")?;
            disabled.insert(id);
        }
    }
    let before = disabled.clone();
    // Reproduce the stock indexed KeyValues format without changing Steam subscriptions.
    disabled.extend(plan.blocked.iter().copied());
    for id in &plan.allowed { disabled.remove(id); }
    if disabled == before { return Ok(None); }
    let mut contents = String::from("\"addonnomount\"\n{\n");
    use std::fmt::Write as _;
    for (index, id) in disabled.into_iter().enumerate() {
        writeln!(contents, "\t\"{}\"\t\t\"{id}\"", index + 1).expect("writing String");
    }
    contents.push_str("}\n");
    Ok(Some(contents.into_bytes()))
}

#[derive(Deserialize, Serialize)]
struct LaunchOptions {
    #[serde(flatten)]
    workshop: MountPlan,
    #[serde(default)]
    skip_loading_screen: bool,
}

pub fn run(steam_root: &Path, game_root: &Path) -> Result<(), String> {
    let LaunchOptions { workshop: plan, skip_loading_screen } = serde_json::from_reader(std::io::stdin().lock().take(LIMIT))
        .map_err(|err| format!("Не удалось прочитать параметры запуска: {err}"))?;
    let request = steam::inspect_launch(steam_root, game_root)?;
    game_profile::check_running_at(game_root)?;
    let root = ordinary(game_root, "garrysmod")?;
    let journal = ordinary(&root, STATE)?;
    let lock = ordinary(&root, ".panopticon-stock-session.lock")?;
    let _guard = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .share_mode(0)
        .open(lock)
        .map_err(|err| format!("Подготовка игры уже выполняется: {err}"))?;
    game_profile::check_running_at(game_root)?;
    let settings = private_settings()?;
    recover(&root, &journal, &settings)?;
    recover(&root, &ordinary(&root, LEGACY_LAB_STATE)?, &settings)?;
    recover(&root, &ordinary(&root, LAB_STATE)?, &settings)?;
    let cfg = ordinary(&root, "cfg")?;
    hub_backend::bindings::flush(&cfg)?;

    if !GAME_FILES.iter().any(|(name, _)| *name == "html/menu.html") {
        return Err("В сборке отсутствует игровое меню Panopticon.".into());
    }
    if !GAME_FILES.iter().any(|(name, _)| *name == MODULE) {
        return Err("В сборке отсутствует игровой модуль Panopticon.".into());
    }
    migrate_settings_if_missing(&ordinary(&root, SETTINGS)?, &settings)?;
    let (design, game_media) = design::load_for_game().unwrap_or_default();
    let game_assets: std::collections::BTreeMap<_, _> = design.assets.iter()
        .filter(|(slot, _)| matches!(slot.as_str(),
            "gameMenuBackground" | "gameBrowserBackground" | "gameLoadingBackground" | "gameLogo" | "gameIcon"
                | "gameMenuMusic" | "gameLoadingMusic"))
        .map(|(slot, key)| (slot.as_str(), key.as_str()))
        .collect();
    let game_filters: std::collections::BTreeMap<_, _> = design.background_filters.iter()
        .filter(|(slot, _)| slot.as_str() != "hubBackground")
        .collect();
    let profile = serde_json::json!({
        "version": design.version,
        "accent": design.accent,
        "builtinStyle": design.builtin_style,
        "gameName": design.game_name,
        "assets": game_assets,
        "gameLoadingVolume": design.game_loading_volume,
        "backgroundFilters": game_filters,
        "skipLoadingScreen": skip_loading_screen,
    });
    let profile = format!("window.PanopticonDesign = {profile};");
    let identity = serde_json::json!({
        "name": design.game_name,
        "icon": game_assets.get("gameIcon"),
    }).to_string();
    let mut assets = GAME_FILES.to_vec();
    assets.push((DESIGN_PROFILE, profile.as_bytes()));
    assets.push((GAME_IDENTITY, identity.as_bytes()));
    let original_background = read(&ordinary(&root, BACKGROUND)?, LIMIT)?;
    let patched_background = original_background.as_deref()
        .map(|original| [original, b"\n", BACKGROUND_ADAPTER].concat());
    if let Some(bytes) = patched_background.as_deref() {
        if bytes.len() as u64 > LIMIT {
            return Err("Сценарий фона слишком большой для подготовки меню.".into());
        }
        assets.push((BACKGROUND, bytes));
    }
    let original_filter = if plan.allowed.is_empty() { None } else { read(&ordinary(&root, WORKSHOP_FILTER)?, LIMIT)? };
    let filtered = workshop_filter(original_filter.as_deref(), &plan)?;
    if let Some(bytes) = filtered.as_deref() {
        assets.push((WORKSHOP_FILTER, bytes));
    }
    let mut session = Session {
        settings: None,
        started: false,
        entries: Vec::new(),
        filtering: None,
    };
    for &(name, bytes) in &assets {
        let path = ordinary(&root, name)?;
        session.entries.push(Entry {
            name: name.into(),
            before: if name == BACKGROUND {
                original_background.clone()
            } else if name == WORKSHOP_FILTER {
                original_filter.clone()
            } else {
                read(&path, LIMIT)?
            },
            installed: hash(bytes),
        });
    }
    for key in game_assets.values().copied().collect::<BTreeSet<_>>() {
        stage_media(&root, &mut session, key)?;
    }
    save_session(&journal, &session)?;
    let mut played = false;
    let result = (|| {
        apply(&root, &session, &assets)?;
        apply_media(&root, &session.entries[assets.len()..], game_media)?;
        game_profile::check_running_at(game_root)?;
        let refreshed = steam::inspect_launch(steam_root, game_root)?;
        if refreshed.build_id != request.build_id || refreshed.executable != request.executable {
            return Err("Steam обновил Garry’s Mod при подготовке. Запустите игру ещё раз.".into());
        }
        session.started = true;
        save_session(&journal, &session)?;
        let mut launcher = Command::new(request.steam_root.join("steam.exe"))
            .current_dir(&request.steam_root)
            .args(["-applaunch", "4000", "-novid", "-panopticon-hub-session"])
            .creation_flags(0x08000000)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|err| format!("Не удалось передать запуск в Steam: {err}"))?;
        let deadline = Instant::now() + STEAM_START_TIMEOUT;
        let client = loop {
            if let Some(client) = game_profile::find_client(&request.executable)
                .map_err(|err| format!("Не удалось найти игровой процесс Steam: {err}"))?
            {
                break client;
            }
            if let Some(status) = launcher.try_wait().map_err(|err| err.to_string())? {
                if !status.success() {
                    return Err(format!("Steam завершил запрос запуска с ошибкой ({status})."));
                }
            }
            if Instant::now() >= deadline {
                return Err("Steam не открыл окно Garry’s Mod (64-bit). Если Steam предложил вариант запуска, выберите 64-bit.".into());
            }
            thread::sleep(Duration::from_millis(200));
        };
        played = true;
        let _ = writeln!(std::io::stdout().lock(), "PID {}", client.pid);
        let status = client.wait().map_err(|err| err.to_string())?;
        if !status.success() {
            return Err(format!("Garry’s Mod завершился с ошибкой ({status})"));
        }
        Ok(())
    })();
    if let Err(cleanup) = wait_for_game_exit(game_root) {
        return Err(match result {
            Ok(()) => cleanup,
            Err(error) => format!("{error}\n{cleanup}"),
        });
    }
    restore(&root, &journal, &settings, &session, played)?;
    result
}

fn migrate_settings_if_missing(live: &Path, legacy: &Path) -> Result<(), String> {
    if read(live, LIMIT)?.is_none() {
        if let Some(previous) = read(legacy, LIMIT)? {
            put(live, Some(&previous))?;
        }
    }
    Ok(())
}

pub struct Game {
    pid: u32,
    worker: Child,
    output: BufReader<ChildStdout>,
}

impl Game {
    pub fn id(&self) -> u32 {
        self.pid
    }

    pub fn wait(&mut self) -> Result<(), String> {
        if self.worker.wait().map_err(|err| err.to_string())?.success() {
            return Ok(());
        }
        let mut line = String::new();
        self.output
            .read_line(&mut line)
            .map_err(|err| err.to_string())?;
        Err(worker_error(
            &line,
            "Не удалось восстановить файлы меню Garry’s Mod.",
        ))
    }
}

fn worker_error(line: &str, default: &str) -> String {
    line.trim_end()
        .strip_prefix("ERR ")
        .unwrap_or(default)
        .to_owned()
}

pub fn start(req: &LaunchRequest, cancel: &AtomicBool, plan: MountPlan, skip_loading_screen: bool) -> Result<Game, String> {
    if cancel.load(Ordering::Acquire) {
        return Err("LAUNCH_CANCELLED".into());
    }
    let mut worker = Command::new(std::env::current_exe().map_err(|err| err.to_string())?)
        .arg("--stock-game-worker")
        .arg(&req.steam_root)
        .arg(&req.game_root)
        .creation_flags(0x08000000)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|err| err.to_string())?;
    let input = worker.stdin.take().ok_or("Не удалось передать список Workshop.")?;
    let options = LaunchOptions { workshop: plan, skip_loading_screen };
    if let Err(error) = serde_json::to_writer(input, &options) {
        let _ = worker.wait();
        return Err(error.to_string());
    }
    let mut output = BufReader::new(worker.stdout.take().ok_or("Нет ответа процесса запуска")?);
    let mut line = String::new();
    output.read_line(&mut line).map_err(|err| err.to_string())?;
    if let Some(Ok(pid)) = line.trim_end().strip_prefix("PID ").map(str::parse) {
        return Ok(Game {
            pid,
            worker,
            output,
        });
    }
    let _ = worker.wait();
    Err(worker_error(&line, "Не удалось запустить Garry’s Mod."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loading_screen_opt_out_reaches_the_game_worker() {
        let legacy = serde_json::to_vec(&MountPlan::default()).unwrap();
        assert!(!serde_json::from_slice::<LaunchOptions>(&legacy).unwrap().skip_loading_screen);
        let options = LaunchOptions {
            workshop: MountPlan { allowed: vec![42], ..MountPlan::default() },
            skip_loading_screen: true,
        };
        let json = serde_json::to_vec(&options).unwrap();
        let received: LaunchOptions = serde_json::from_slice(&json).unwrap();
        assert!(received.skip_loading_screen);
        assert_eq!(received.workshop.allowed, vec![42]);
    }

    #[test]
    fn design_media_recovery_preserves_independent_files() {
        let tree = tempfile::tempdir().unwrap();
        let root = tree.path();
        let owned_media = b"assigned to this session";
        let installed = hash(owned_media);
        let key = format!("{installed}.webm");
        let name = format!("{DESIGN_MEDIA}{key}");
        let path = ordinary(root, &name).unwrap();
        let mut session = Session {
            settings: None,
            filtering: None,
            started: false,
            entries: Vec::new(),
        };
        stage_media(root, &mut session, &key).unwrap();
        assert_eq!(session.entries.len(), 1);
        let journal = root.join(STATE);
        put(&path, Some(b"changed outside Hub")).unwrap();
        restore(root, &journal, &root.join("settings"), &session, false).unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"changed outside Hub");
        File::create(&path).unwrap().set_len(DESIGN_LIMIT + 1).unwrap();
        restore(root, &journal, &root.join("settings"), &session, false).unwrap();
        assert_eq!(fs::metadata(&path).unwrap().len(), DESIGN_LIMIT + 1);

        put(&path, Some(owned_media)).unwrap();
        let mut previous = Session { entries: Vec::new(), ..session };
        stage_media(root, &mut previous, &key).unwrap();
        assert!(previous.entries.is_empty());
        assert!(stage_media(root, &mut previous, "../theme.webm").is_err());
        previous.entries.push(Entry { name, before: None, installed });
        save_session(&journal, &previous).unwrap();
        recover(root, &journal, &root.join("settings")).unwrap();
        assert!(!path.exists());
        assert!(!journal.exists());
    }

    #[test]
    fn workshop_filter_applies_and_restores_the_original_stock_list() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let path = root.join(WORKSHOP_FILTER);
        let journal = root.join(STATE);
        let original = b"\"addonnomount\"\n{\n\t\"1\" \"200\"\n\t\"2\" \"400\"\n}\n";
        let plan = MountPlan { allowed: vec![200], blocked: vec![300], ..MountPlan::default() };
        let filtered = workshop_filter(Some(original), &plan).unwrap().unwrap();
        let value = steam::parse_vdf(std::str::from_utf8(&filtered).unwrap()).unwrap();
        assert_eq!(value["addonnomount"]["1"], "300");
        assert_eq!(value["addonnomount"]["2"], "400");
        assert!(workshop_filter(Some(original), &MountPlan::default()).unwrap().is_none());
        put(&path, Some(original)).unwrap();
        let session = Session { settings: None, filtering: None, started: true, entries: vec![Entry {
            name: WORKSHOP_FILTER.into(), before: Some(original.to_vec()), installed: hash(&filtered),
        }] };
        save_session(&journal, &session).unwrap();
        apply(root, &session, &[(WORKSHOP_FILTER, &filtered)]).unwrap();
        restore(root, &journal, &root.join("settings"), &session, true).unwrap();
        assert_eq!(fs::read(&path).unwrap(), original);
        assert!(!journal.exists());
    }

    #[test]
    fn restores_stock_files_and_preserves_changes_to_saved_settings() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let journal = root.join(STATE);
        let settings = root.join("private-settings.txt");
        let (name, payload) = GAME_FILES
            .iter()
            .copied()
            .find(|(name, _)| *name == "html/panopticon/entry.js")
            .unwrap();
        let path = root.join(name);
        let live_settings = root.join(SETTINGS);
        put(&path, Some(b"stock")).unwrap();
        put(&live_settings, Some(b"stock settings")).unwrap();
        let session = Session {
            settings: Some(settings.clone()),
            started: true,
            filtering: None,
            entries: vec![
                Entry {
                    name: name.into(),
                    before: Some(b"stock".to_vec()),
                    installed: hash(payload),
                },
                Entry {
                    name: SETTINGS.into(),
                    before: Some(b"stock settings".to_vec()),
                    installed: hash(b"saved"),
                },
            ],
        };
        save_session(&journal, &session).unwrap();
        apply(root, &session, &[(name, payload), (SETTINGS, b"saved")]).unwrap();
        put(&live_settings, Some(b"edited")).unwrap();
        restore(root, &journal, &settings, &session, true).unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"stock");
        assert_eq!(fs::read(&live_settings).unwrap(), b"stock settings");
        assert_eq!(fs::read(&settings).unwrap(), b"edited");
        assert!(!journal.exists());
        restore(root, &journal, &settings, &session, true).unwrap();
        assert_eq!(fs::read(&settings).unwrap(), b"edited");
    }

    #[test]
    fn restores_owned_files_from_an_older_session() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let journal = root.join(LAB_STATE);
        let config = root.join(LAB_CONFIG);
        let module = root.join(MODULE);
        let old_script = root.join(LAB_SCRIPT);
        let old_dll = root.join(LAB_MODULE);
        let session = Session {
            settings: None,
            started: true,
            filtering: None,
            entries: vec![
                Entry {
                    name: MODULE.into(),
                    before: None,
                    installed: hash(b"temporary"),
                },
                Entry {
                    name: LAB_CONFIG.into(),
                    before: Some(b"fps_max 50\nfps_max 60".to_vec()),
                    installed: String::new(),
                },
                Entry {
                    name: LAB_SCRIPT.into(),
                    before: None,
                    installed: hash(b"owned script"),
                },
                Entry {
                    name: LAB_MODULE.into(),
                    before: None,
                    installed: hash(b"owned DLL"),
                },
            ],
        };
        put(&module, Some(b"temporary")).unwrap();
        put(&config, Some(b"fps_max 0\nvolume 0.5\n")).unwrap();
        put(&old_script, Some(b"owned script")).unwrap();
        put(&old_dll, Some(b"user replacement")).unwrap();
        save_session(&journal, &session).unwrap();
        recover(root, &journal, &root.join("private-settings.txt")).unwrap();

        assert_eq!(fs::read(config).unwrap(), b"fps_max 60\nvolume 0.5\n");
        assert!(!module.exists());
        assert!(!old_script.exists());
        assert_eq!(fs::read(old_dll).unwrap(), b"user replacement");
        assert!(!journal.exists());
        assert!(!root.join(STATE).exists());
    }

    #[test]
    fn recovers_an_interrupted_session_from_before_geometry_removal() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let journal = root.join(STATE);
        let dll = root.join(LEGACY_GEOMETRY);
        let payload = b"old hub module";
        put(&dll, Some(payload)).unwrap();
        save_session(&journal, &Session {
            settings: None,
            started: true,
            entries: vec![Entry {
                name: LEGACY_GEOMETRY.into(),
                before: None,
                installed: hash(payload),
            }],
            filtering: None,
        }).unwrap();
        recover(root, &journal, &root.join("private-settings.txt")).unwrap();
        assert!(!dll.exists());
        assert!(!journal.exists());
    }

    #[test]
    fn restores_only_owned_values_in_a_new_game_config() {
        let dir = tempfile::tempdir().unwrap();
        let config = dir.path().join("config.cfg");
        let current = b"// \xff\r\nfps_max \"0\"\r\nvolume \"0.5\"\r\nmat_forceaniso \"4\"\r\n";
        put(&config, Some(current)).unwrap();
        restore_config(&config, None, Some(current), "fps_max=0 mat_forceaniso=8").unwrap();
        assert_eq!(
            fs::read(config).unwrap(),
            b"// \xff\r\nvolume \"0.5\"\r\nmat_forceaniso \"4\"\r\n"
        );
    }
}
    #[test]
    fn imports_legacy_settings_only_when_the_game_has_no_profile() {
        let directory = tempfile::tempdir().unwrap();
        let live = directory.path().join("drp.settings.txt");
        let legacy = directory.path().join("hub-settings.txt");
        fs::write(&legacy, br#"{"showHUD":false}"#).unwrap();
        migrate_settings_if_missing(&live, &legacy).unwrap();
        assert_eq!(fs::read(&live).unwrap(), fs::read(&legacy).unwrap());

        fs::write(&live, br#"{"showHUD":true}"#).unwrap();
        migrate_settings_if_missing(&live, &legacy).unwrap();
        assert_eq!(fs::read(&live).unwrap(), br#"{"showHUD":true}"#);
    }
    #[test]
    fn recovers_the_legacy_loading_page_after_a_game_session() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        let journal = root.join(STATE);
        let name = LEGACY_SKIPPED_LOADING_PAGE;
        let content = b"<!doctype html>\n<meta charset=\"utf-8\">\n<style>html { background: #000; }</style>\n";
        let target = root.join(name);
        let session = Session {
            settings: None,
            started: true,
            entries: vec![Entry { name: name.into(), before: None, installed: hash(content) }],
            filtering: None,
        };
        put(&target, Some(content)).unwrap();
        save_session(&journal, &session).unwrap();
        assert_eq!(fs::read(&target).unwrap(), content);

        recover(root, &journal, &root.join("settings")).unwrap();
        assert!(!target.exists());
        assert!(!journal.exists());
    }
