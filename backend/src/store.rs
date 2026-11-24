use serde_json::{json, Map, Value};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct Store {
    path: PathBuf,
    values: Map<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loading_screen_opt_out_defaults_off_and_survives_reopening() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("settings.json");
        fs::write(&path, br#"{"language":"ru"}"#).unwrap();
        let mut store = Store::open(path.clone()).unwrap();
        assert_eq!(store.get("skipLoadingScreen"), Some(&json!(false)));
        store.set("skipLoadingScreen", json!(true)).unwrap();
        assert!(store.set("skipLoadingScreen", json!("false")).is_err());
        let mut store = Store::open(path.clone()).unwrap();
        assert_eq!(store.get("skipLoadingScreen"), Some(&json!(true)));
        store.set("skipLoadingScreen", json!(false)).unwrap();
        let store = Store::open(path).unwrap();
        assert_eq!(store.get("skipLoadingScreen"), Some(&json!(false)));
    }
}
impl Store {
    pub fn open(path: PathBuf) -> Result<Self, String> {
        let mut values = if path.exists() {
            let bytes = fs::read(&path).map_err(|e| e.to_string())?;
            if bytes.len() > 1024 * 1024 {
                return Err("Файл настроек превышает 1 МиБ".into());
            }
            serde_json::from_slice::<Value>(&bytes)
                .map_err(|e| format!("Повреждены настройки: {e}"))?
                .as_object()
                .cloned()
                .ok_or("Настройки должны быть объектом")?
        } else {
            Map::new()
        };
        if values.get("region") == Some(&json!("fr")) {
            values.insert("region".into(), json!("eu"));
        }
        for key in [
            "experimentalFeatures",
            "experimentalRustGeometry",
            "experimentalTextureFilter",
            "experimentalLuaRust",
            "experimentalVulkan",
            "vulkanMediumTextures",
        ] {
            values.remove(key);
        }
        let workshop_setup = if values.contains_key("legalConsent") { "done" } else { "pending" };
        let language = system_language();
        for (key, value) in [
            (
                "language",
                json!(if ["ru", "en", "fr"].contains(&language.as_str()) {
                    language
                } else {
                    "en".into()
                }),
            ),
            ("region", json!("cis")),
            ("returnToHub", json!(true)),
            ("minimizeOnJoin", json!(true)),
            ("skipLoadingScreen", json!(false)),
            ("uiVolume", json!(0.3)),
            ("animatedBackground", json!(true)),
            ("workshopFilter", json!(true)),
            ("workshopPanopticon", json!(true)),
            ("workshopOnboarding", json!(workshop_setup)),
        ] {
            values.entry(key.to_owned()).or_insert(value);
        }
        for (key, value) in &values {
            validate(key, value)?;
        }
        let store = Self { path, values };
        store.persist()?;
        Ok(store)
    }
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn set(&mut self, key: &str, value: Value) -> Result<(), String> {
        validate(key, &value)?;
        let old = self.values.insert(key.into(), value);
        if let Err(error) = self.persist() {
            match old {
                Some(value) => {
                    self.values.insert(key.into(), value);
                }
                None => {
                    self.values.remove(key);
                }
            }
            return Err(error);
        }
        Ok(())
    }
    pub fn workshop_installation(&mut self, id: &str) -> Result<(), String> {
        let value = json!(id);
        validate("workshopInstallId", &value)?;
        if self.get("workshopInstallId") == Some(&value) { return Ok(()); }
        let old = self.values.clone();
        self.values.insert("workshopInstallId".into(), value);
        self.values.insert("workshopOnboarding".into(), json!("pending"));
        self.values.insert("workshopPanopticon".into(), json!(true));
        if let Err(error) = self.persist() {
            self.values = old;
            return Err(error);
        }
        Ok(())
    }
    fn persist(&self) -> Result<(), String> {
        let parent = self.path.parent().ok_or("Некорректный путь настроек")?;
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        if let Ok(meta) = fs::symlink_metadata(&self.path) {
            if meta.file_type().is_symlink() {
                return Err("Настройки не записываются через ссылку".into());
            }
        }
        let temporary = self
            .path
            .with_extension(format!("json.{}.tmp", std::process::id()));
        let bytes = serde_json::to_vec_pretty(&self.values).map_err(|e| e.to_string())?;
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|e| e.to_string())?;
        let result = (|| {
            file.write_all(&bytes).map_err(|e| e.to_string())?;
            file.sync_all().map_err(|e| e.to_string())?;
            drop(file);
            replace(&temporary, &self.path)
        })();
        if result.is_err() {
            let _ = fs::remove_file(temporary);
        }
        result
    }
}
pub fn default_path() -> Result<PathBuf, String> {
    let root = std::env::var_os("APPDATA").ok_or("APPDATA не определён")?;
    Ok(PathBuf::from(root).join("PanopticonHub/port-store.json"))
}
fn validate(key: &str, value: &Value) -> Result<(), String> {
    let valid = match key {
        "language" => value
            .as_str()
            .is_some_and(|v| ["ru", "en", "fr"].contains(&v)),
        "region" => value.as_str().is_some_and(|v| ["cis", "eu"].contains(&v)),
        "returnToHub" | "minimizeOnJoin" | "skipLoadingScreen" | "animatedBackground" | "workshopFilter" |
        "workshopPanopticon" => value.is_boolean(),
        "workshopOnboarding" => matches!(value.as_str(), Some("pending" | "prepare" | "done")),
        "workshopInstallId" => value.as_str().is_some_and(|id| id.len() == 32 && id.bytes().all(|byte| byte.is_ascii_hexdigit())),
        "uiVolume" => value
            .as_f64()
            .is_some_and(|v| v.is_finite() && (0.0..=1.0).contains(&v)),
        "steamPath" | "gmodPath" => value
            .as_str()
            .is_some_and(|v| !v.contains(['\0', '\n', '\r']) && v.len() <= 32768),
        "legalConsent" => value.as_object().is_some_and(|o| {
            o.len() == 2
                && o.get("version").is_some_and(Value::is_string)
                && o.get("acceptedAt").is_some_and(Value::is_string)
        }),
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(format!("Некорректное значение настройки {key}"))
    }
}
#[cfg(windows)]
fn system_language() -> String {
    #[link(name = "kernel32")]
    extern "system" {
        fn GetUserDefaultLocaleName(name: *mut u16, length: i32) -> i32;
    }
    let mut name = [0u16; 85];
    let length = unsafe { GetUserDefaultLocaleName(name.as_mut_ptr(), 85) };
    if length > 1 {
        String::from_utf16_lossy(&name[..length as usize - 1])
            .split('-')
            .next()
            .unwrap_or("en")
            .to_lowercase()
    } else {
        "en".into()
    }
}
#[cfg(not(windows))]
fn system_language() -> String {
    std::env::var("LANG")
        .unwrap_or_else(|_| "en".into())
        .split(['_', '-', '.'])
        .next()
        .unwrap_or("en")
        .to_lowercase()
}
#[cfg(windows)]
fn replace(from: &Path, to: &Path) -> Result<(), String> {
    use std::os::windows::ffi::OsStrExt;
    #[link(name = "kernel32")]
    extern "system" {
        fn MoveFileExW(from: *const u16, to: *const u16, flags: u32) -> i32;
    }
    let from: Vec<u16> = from.as_os_str().encode_wide().chain(Some(0)).collect();
    let to: Vec<u16> = to.as_os_str().encode_wide().chain(Some(0)).collect();
    if unsafe { MoveFileExW(from.as_ptr(), to.as_ptr(), 1 | 8) } == 0 {
        Err(format!(
            "Не записаны настройки: {}",
            std::io::Error::last_os_error()
        ))
    } else {
        Ok(())
    }
}
#[cfg(not(windows))]
fn replace(from: &Path, to: &Path) -> Result<(), String> {
    fs::rename(from, to).map_err(|e| e.to_string())
}
