use serde_json::{Map, Value};
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    os::windows::fs::MetadataExt,
    path::{Path, PathBuf},
    sync::Mutex,
};

const MAX_FILE_BYTES: usize = 1024 * 1024;
pub const MAX_PATCH_BYTES: usize = 8192;
const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
const SERVER_LOADING_SCREENS_FILE: &str = "server-loading.txt";
const LOADING_INDICATOR_FILE: &str = "loading-indicator.txt";
static SETTINGS_WRITE: Mutex<()> = Mutex::new(());

const SWITCHES: &[&str] = &[
    "corpseDiscoverSound",
    "monokumaNotifySound",
    "closelookSound",
    "musicInDances",
    "musicInOwnDances",
    "showTooltips",
    "showBindTooltips",
    "showNamesAboveHead",
    "showHUD",
    "showInteractionMarkers",
    "closelookPostprocessing",
    "displayQuests",
    "corpseDetectNoise",
    "streamerMode",
    "droview_bob",
    "dro_thirdperson",
    "dro_thirdperson_crosshair",
];
const RANGES: &[(&str, f64, f64)] = &[
    ("volume_dialogs", 0.0, 1.0),
    ("volume_dances", 0.0, 1.0),
    ("volume.effects", 0.0, 1.0),
    ("dro_thirdperson_pos_updown", -15.0, 15.0),
    ("dro_thirdperson_pos_rightleft", -20.0, 20.0),
    ("dro_thirdperson_pos_forwardback", -120.0, -50.0),
];

pub(super) fn keys() -> impl Iterator<Item = &'static str> {
    SWITCHES
        .iter()
        .copied()
        .chain(RANGES.iter().map(|rule| rule.0))
}

fn check_path(path: &Path) -> io::Result<()> {
    for entry in path.ancestors() {
        match fs::symlink_metadata(entry) {
            Ok(metadata) if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 => {
                return Err(io::Error::other("В пути настроек обнаружена ссылка."));
            }
            Ok(_) => {}
            Err(err) if err.kind() == io::ErrorKind::NotFound => {}
            Err(err) => return Err(err),
        }
    }
    Ok(())
}

fn read_bytes(path: &Path, limit: usize) -> io::Result<Option<Vec<u8>>> {
    check_path(path)?;
    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(err),
    };
    let mut data = Vec::new();
    file.take(limit as u64 + 1).read_to_end(&mut data)?;
    if data.len() > limit {
        return Err(io::Error::other("Файл настроек слишком велик."));
    }
    Ok(Some(data))
}

fn stock_config(executable: &Path, launched: bool) -> io::Result<PathBuf> {
    fn named(path: &Path, name: &str) -> bool {
        path.file_name()
            .and_then(|component| component.to_str())
            .is_some_and(|component| component.eq_ignore_ascii_case(name))
    }
    let mut path = executable.ancestors();
    if !launched
        || !executable.is_absolute()
        || !path.next().is_some_and(|path| named(path, "gmod.exe"))
        || !path.next().is_some_and(|path| named(path, "win64"))
        || !path.next().is_some_and(|path| named(path, "bin"))
    {
        return Err(io::Error::other(
            "Garry’s Mod запущен вне сеанса Panopticon Hub.",
        ));
    }
    let root = path
        .next()
        .ok_or_else(|| io::Error::other("Не найден каталог Garry’s Mod."))?;
    let game = root.join("garrysmod");
    let session = game.join(".panopticon-stock-session.json");
    check_path(&session)?;
    if !session.is_file() {
        return Err(io::Error::other("Не найден активный сеанс Panopticon Hub."));
    }
    Ok(game.join("data/drp.settings.txt"))
}

pub(crate) fn config() -> io::Result<PathBuf> {
    let launched = std::env::args_os().skip(1).any(|arg| arg == "-panopticon-hub-session");
    stock_config(&std::env::current_exe()?, launched)
}

fn loading_preference_path(data: &Path, filename: &str) -> PathBuf {
    data.join("PanopticonHub/game-data").join(filename)
}

fn loading_preference_config(filename: &str) -> io::Result<PathBuf> {
    config()?;
    let data = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .ok_or_else(|| io::Error::other("Не найдена папка LOCALAPPDATA."))?;
    Ok(loading_preference_path(&data, filename))
}

fn read_loading_preference(path: &Path) -> bool {
    read_bytes(path, MAX_FILE_BYTES).ok().flatten().as_deref() == Some(b"1\n")
}

fn save_loading_preference(path: &Path, enabled: bool) -> io::Result<()> {
    let _lock = SETTINGS_WRITE.lock().map_err(|_| io::Error::other("Запись настроек прервана."))?;
    let original = read_bytes(path, MAX_FILE_BYTES)?;
    let bytes = if enabled { b"1\n" } else { b"0\n" };
    if original.as_deref() == Some(bytes) {
        return Ok(());
    }
    write_atomic(path, original.as_deref(), bytes)
}

pub(super) fn server_loading_screens() -> Option<bool> {
    loading_preference_config(SERVER_LOADING_SCREENS_FILE)
        .ok()
        .map(|path| read_loading_preference(&path))
}

pub(super) fn set_server_loading_screens(enabled: bool) -> io::Result<()> {
    save_loading_preference(&loading_preference_config(SERVER_LOADING_SCREENS_FILE)?, enabled)
}

pub(super) fn loading_indicator() -> Option<bool> {
    loading_preference_config(LOADING_INDICATOR_FILE)
        .ok()
        .map(|path| read_loading_preference(&path))
}

pub(super) fn set_loading_indicator(enabled: bool) -> io::Result<()> {
    save_loading_preference(&loading_preference_config(LOADING_INDICATOR_FILE)?, enabled)
}

pub(super) fn accepts(key: &str, value: &Value, allow_reset: bool) -> bool {
    if SWITCHES.contains(&key) {
        return value.is_boolean() || allow_reset && value.is_null();
    }
    RANGES
        .iter()
        .find(|&&(name, _, _)| key == name)
        .is_some_and(|&(_, min, max)| {
            value
                .as_f64()
                .is_some_and(|number| (min..=max).contains(&number))
                || allow_reset && value.is_null()
        })
}

fn strip_bom(data: &[u8]) -> &[u8] {
    data.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(data)
}

fn blank(data: &[u8]) -> bool {
    strip_bom(data).iter().all(u8::is_ascii_whitespace)
}

fn parse_file(data: &[u8]) -> io::Result<Map<String, Value>> {
    // Сборка 0.2.6 могла обнулить файл ошибочным вызовом MENU file.Write.
    if blank(data) {
        return Ok(Map::new());
    }
    serde_json::from_slice(strip_bom(data)).map_err(io::Error::other)
}

fn read_from(path: &Path) -> io::Result<Value> {
    let raw = read_bytes(path, MAX_FILE_BYTES)?.unwrap_or_default();
    let mut settings = parse_file(&raw)?;
    settings.retain(|key, value| accepts(key, value, false));
    Ok(Value::Object(settings))
}

fn merge_at(path: &Path, patch: Map<String, Value>) -> io::Result<()> {
    if !patch.iter().all(|(key, value)| accepts(key, value, true)) {
        return Err(io::Error::other("Недопустимый параметр настроек."));
    }
    let _lock = SETTINGS_WRITE
        .lock()
        .map_err(|_| io::Error::other("Запись настроек прервана."))?;
    let original = read_bytes(path, MAX_FILE_BYTES)?;
    let mut settings = parse_file(original.as_deref().unwrap_or_default())?;
    let mut changed = original.as_deref().is_some_and(blank);
    for (key, value) in patch {
        if value.is_null() {
            changed |= settings.remove(&key).is_some();
        } else if settings.get(&key) != Some(&value) {
            settings.insert(key, value);
            changed = true;
        }
    }
    if !changed {
        return Ok(());
    }
    let bytes = serde_json::to_vec(&settings).map_err(io::Error::other)?;
    if bytes.len() > MAX_FILE_BYTES {
        return Err(io::Error::other(
            "Файл настроек превысил допустимый размер.",
        ));
    }
    write_atomic(path, original.as_deref(), &bytes)
}

fn write_atomic(path: &Path, original: Option<&[u8]>, bytes: &[u8]) -> io::Result<()> {
    let directory = path
        .parent()
        .ok_or_else(|| io::Error::other("Не найдена папка настроек."))?;
    fs::create_dir_all(directory)?;
    check_path(directory)?;
    let mut temp = tempfile::NamedTempFile::new_in(directory)?;
    temp.write_all(bytes)?;
    temp.as_file().sync_all()?;
    if read_bytes(path, MAX_FILE_BYTES)?.as_deref() != original {
        return Err(io::Error::other("Настройки изменились во время записи."));
    }
    if original.is_some() {
        temp.persist(path)?;
    } else {
        temp.persist_noclobber(path)?;
    }
    Ok(())
}

pub fn read() -> io::Result<Value> {
    read_from(&config()?)
}

pub fn apply(raw: &[u8]) -> io::Result<()> {
    if raw.len() > MAX_PATCH_BYTES {
        return Err(io::Error::other("Изменение настроек слишком велико."));
    }
    let patch = serde_json::from_slice(raw).map_err(io::Error::other)?;
    merge_at(&config()?, patch)
}

pub fn reset() -> io::Result<()> {
    let patch = keys().map(|key| (key.to_owned(), Value::Null)).collect();
    merge_at(&config()?, patch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn merge(path: &Path, patch: Value) -> io::Result<()> {
        merge_at(path, patch.as_object().unwrap().clone())
    }

    #[test]
    fn loading_screens_persist_separately_and_default_off() {
        let temp = tempfile::tempdir().unwrap();
        let preference = loading_preference_path(temp.path(), SERVER_LOADING_SCREENS_FILE);
        let stock = temp.path().join("config.cfg");
        fs::write(&stock, "cl_enable_loadingurl \"0\"\n").unwrap();
        assert!(!read_loading_preference(&preference));
        save_loading_preference(&preference, true).unwrap();
        assert!(read_loading_preference(&preference));
        assert_eq!(fs::read_to_string(&stock).unwrap(), "cl_enable_loadingurl \"0\"\n");
        fs::write(&stock, "cl_enable_loadingurl \"1\"\n").unwrap();
        save_loading_preference(&preference, false).unwrap();
        assert!(!read_loading_preference(&preference));
        assert_eq!(fs::read_to_string(&stock).unwrap(), "cl_enable_loadingurl \"1\"\n");
        fs::write(&preference, "invalid").unwrap();
        assert!(!read_loading_preference(&preference));
        save_loading_preference(&preference, true).unwrap();
        assert!(read_loading_preference(&preference));
    }

    #[test]
    fn loading_indicator_preserves_existing_server_loading_preference() {
        let temp = tempfile::tempdir().unwrap();
        let server_screens = temp.path().join("PanopticonHub/game-data/server-loading.txt");
        let indicator = loading_preference_path(temp.path(), LOADING_INDICATOR_FILE);
        fs::create_dir_all(server_screens.parent().unwrap()).unwrap();
        fs::write(&server_screens, b"1\n").unwrap();
        assert!(!read_loading_preference(&indicator));
        assert_eq!(loading_preference_path(temp.path(), SERVER_LOADING_SCREENS_FILE), server_screens);
        save_loading_preference(&indicator, true).unwrap();
        assert!(read_loading_preference(&indicator));
        save_loading_preference(&server_screens, false).unwrap();
        assert!(read_loading_preference(&indicator));
        save_loading_preference(&server_screens, true).unwrap();
        save_loading_preference(&indicator, false).unwrap();
        assert!(!read_loading_preference(&indicator));
        assert!(read_loading_preference(&server_screens));
    }

    #[test]
    fn repairs_empty_file_and_preserves_unrelated_settings() {
        let temp = tempfile::tempdir().unwrap();
        let file = temp.path().join("drp.settings.txt");
        fs::write(&file, "").unwrap();
        assert_eq!(read_from(&file).unwrap(), json!({}));
        merge(&file, json!({"dro_thirdperson": null})).unwrap();
        assert_eq!(fs::read_to_string(&file).unwrap(), "{}");
        merge(&file, json!({"dro_thirdperson": true})).unwrap();
        assert_eq!(read_from(&file).unwrap(), json!({"dro_thirdperson": true}));
        fs::write(
            &file,
            r#"{"thirdParty":{"a":5},"dro_thirdperson":false,"showHUD":42}"#,
        )
        .unwrap();
        merge(
            &file,
            json!({"dro_thirdperson_crosshair": false,"volume_dialogs": 0.4}),
        )
        .unwrap();
        let all = parse_file(&fs::read(&file).unwrap()).unwrap();
        assert_eq!(all["thirdParty"], json!({"a":5}));
        assert_eq!(all["showHUD"], json!(42));
        assert!(!read_from(&file)
            .unwrap()
            .as_object()
            .unwrap()
            .contains_key("showHUD"));
        merge(&file, json!({"dro_thirdperson_crosshair": null})).unwrap();
        assert_eq!(
            read_from(&file).unwrap(),
            json!({"dro_thirdperson": false,"volume_dialogs": 0.4})
        );
    }

    #[test]
    fn preserves_corrupt_file_and_rejects_invalid_patch_as_a_unit() {
        let temp = tempfile::tempdir().unwrap();
        let file = temp.path().join("drp.settings.txt");
        fs::write(&file, "{bad").unwrap();
        assert!(merge(&file, json!({"showHUD": true})).is_err());
        assert_eq!(fs::read_to_string(&file).unwrap(), "{bad");
        fs::write(&file, "{}").unwrap();
        for patch in [
            json!({"showHUD": 1}),
            json!({"showHUD": true,"unknown": null}),
            json!({"dro_thirdperson_pos_forwardback": -6000}),
            json!({"volume.effects": 2.0}),
        ] {
            assert!(merge(&file, patch).is_err());
        }
        assert_eq!(fs::read_to_string(&file).unwrap(), "{}");
    }

    #[test]
    fn resolves_only_stock_executable_with_active_session() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let executable = root.join("bin/win64/gmod.exe");
        fs::create_dir(root.join("garrysmod")).unwrap();
        assert!(stock_config(&executable, true).is_err());
        fs::write(root.join("garrysmod/.panopticon-stock-session.json"), "{}").unwrap();
        assert!(stock_config(&executable, false).is_err());
        assert_eq!(
            stock_config(&executable, true).unwrap(),
            root.join("garrysmod/data/drp.settings.txt")
        );
        assert!(stock_config(&root.join("bin/win64/PanopticonGame.exe"), true).is_err());
    }
}
