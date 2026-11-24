//! Порт публичных IPC-контрактов. Игровой запуск и системные окна исполняет GUI-хост.

pub mod bindings;
mod http;
pub mod steam;
mod store;
pub mod identity;

use serde_json::{json, Value};

pub mod online;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub use steam::LaunchRequest;

#[derive(Debug, Clone)]
pub struct Event {
    pub channel: String,
    pub args: Vec<Value>,
    pub undefined_args: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowCommand {
    Minimize,
    Drag,
    Close,
    Show,
}

#[derive(Debug, Clone)]
pub enum Action {
    Window { command: WindowCommand },
    OpenExternal { url: String },
    OpenPath { path: PathBuf },
    CreateShortcut { location: String },
}

pub struct Backend {
    store: store::Store,
    sink: Arc<dyn Fn(Event) + Send + Sync>,
    pid: Option<u32>,
    window_visible: bool,
    protocol: Option<String>,
    passed_checks: bool,
    api_base: Option<String>,
}

impl Backend {
    pub fn new(sink: impl Fn(Event) + Send + Sync + 'static) -> Result<Self, String> {
        Self::at(store::default_path()?, sink)
    }

    pub fn at(path: PathBuf, sink: impl Fn(Event) + Send + Sync + 'static) -> Result<Self, String> {
        Ok(Self {
            store: store::Store::open(path)?,
            sink: Arc::new(sink),
            pid: None,
            window_visible: false,
            protocol: None,
            passed_checks: false,
            api_base: None,
        })
    }

    pub fn invoke(&mut self, channel: &str, args: &Value) -> Result<Option<Value>, String> {
        let args = arguments(args)?;
        match channel {
            "store" => match string(args, 0)? {
                "get" if string(args, 1)? == identity::KEY => Ok(Some(json!(identity::enabled().unwrap_or(false)))),
                "get" => Ok(self.store.get(string(args, 1)?).cloned()),
                "set" => {
                    let key = string(args, 1)?;
                    let value = args.get(2).ok_or("Отсутствует значение store.set")?;
                    if key == identity::KEY {
                        if self.pid.is_some() { return Err("Измените идентификатор после выхода из игры.".into()); }
                        identity::set_enabled(value.as_bool().ok_or("Идентификатор должен быть переключателем.")?)?;
                        return Ok(None);
                    }
                    let changed = self.store.get(key) != Some(value);
                    self.store.set(key, value.clone())?;
                    if changed && key == "animatedBackground" {
                        self.emit("animatedBackgroundChanged", vec![value.clone()], vec![]);
                    }
                    if changed && key == "region" {
                        self.api_base = None;
                        let base = self.health();
                        self.emit_optional("stApiBaseUrlChanged", base.map(Value::String));
                    }
                    Ok(None)
                }
                _ => Ok(Some(Value::Null)),
            },
            "localeChanged" => Ok(None),
            "isHubWindowVisible" => Ok(Some(json!(self.window_visible))),
            "getAppVersion" => Ok(Some(json!(env!("CARGO_PKG_VERSION")))),
            "getProtocol" => Ok(self.protocol.clone().map(Value::String)),
            "passedChecks" => {
                self.passed_checks = true;
                Ok(None)
            }
            "checkRequirements" => Ok(steam::requirements_failure()),
            "findSteam" => Ok(steam::find_steam().map(|p| json!(p.to_string_lossy()))),
            "isSteamDirectory" => Ok(Some(json!(steam::is_steam_directory(Path::new(string(
                args, 0
            )?))))),
            "findGmod" => {
                let steam = self.selected_steam();
                match steam {
                    Some(root) => Ok(steam::find_game(&root)?.map(|p| json!(p.to_string_lossy()))),
                    None => Ok(None),
                }
            }
            "isGmodDirectory" => Ok(Some(json!(steam::is_game_directory(Path::new(string(
                args, 0
            )?))))),
            "getGmodPid" => Ok(self.pid.map(|pid| json!(pid))),
            "checkStApiHealth" => Ok(self.health().map(Value::String)),
            "stApiGet" => {
                let path = string(args, 0)?;
                match path {
                    "/media" => return Ok(Some(json!({"success":true,"result":social_links()}))),
                    "/ghost/latest" => return Ok(Some(json!({"success":true,"result":[]}))),
                    _ => {}
                }
                if !public_path(path) {
                    return Err("API_PATH_NOT_ALLOWED: разрешены только публичные данные, используемые интерфейсом".into());
                }
                let Some(base) = self.api_base.clone().or_else(|| self.health()) else {
                    return Ok(None);
                };
                match http::get_json(&base, path) {
                    Ok(value) => Ok(Some(value)),
                    Err(_) => Ok(None),
                }
            }
            "links" => Ok(Some(links())),
            "startGmod" | "cancelGmodLaunch" | "openDialog" | "createShortcut"
            | "openLogsFolder" | "openHostFolder" | "handleProtocol" => {
                Err(format!("HOST_ACTION_REQUIRED:{channel}"))
            }
            _ => Err(format!("UNKNOWN_IPC_CHANNEL:{channel}")),
        }
    }

    pub fn workshop_installation(&mut self, id: &str) -> Result<(), String> {
        self.store.workshop_installation(id)
    }

    pub fn send(&mut self, channel: &str, args: &Value) -> Result<Vec<Action>, String> {
        let args = arguments(args)?;
        match channel {
            "window-control" => {
                let command = match string(args, 0)? {
                    "minimize" => WindowCommand::Minimize,
                    "drag" => WindowCommand::Drag,
                    "close" => WindowCommand::Close,
                    _ => return Err("Неизвестная команда окна".into()),
                };
                Ok(vec![Action::Window { command }])
            }
            "openExternal" => Ok(vec![Action::OpenExternal {
                url: allowed_external(string(args, 0)?)?,
            }]),
            _ => Err(format!("UNKNOWN_IPC_CHANNEL:{channel}")),
        }
    }

    /// Хост использует это для invoke-операций с реальными побочными действиями.
    /// Значение Promise возвращается только после успешного выполнения Action хостом.
    pub fn action_for_invoke(&self, channel: &str, args: &Value) -> Result<Action, String> {
        let args = arguments(args)?;
        match channel {
            "createShortcut" => {
                let location = string(args, 0)?;
                if location != "Desktop" {
                    return Err("Разрешён ярлык только на рабочем столе".into());
                }
                Ok(Action::CreateShortcut {
                    location: location.into(),
                })
            }
            "openLogsFolder" => {
                let path = self
                    .store
                    .path()
                    .parent()
                    .ok_or("Не найден каталог настроек")?
                    .join("logs");
                std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
                Ok(Action::OpenPath { path })
            }
            "openHostFolder" => {
                let root = self
                    .store
                    .get("gmodPath")
                    .and_then(Value::as_str)
                    .map(PathBuf::from)
                    .or_else(|| {
                        self.selected_steam()
                            .and_then(|root| steam::find_game(&root).ok().flatten())
                    })
                    .ok_or("HOST_FOLDER_MISSING")?;
                if !steam::is_game_directory(&root) {
                    return Err("HOST_FOLDER_MISSING".into());
                }
                Ok(Action::OpenPath { path: root })
            }
            "handleProtocol" => {
                let protocol = string(args, 0)?;
                let address = protocol
                    .strip_prefix("st://connect/")
                    .or_else(|| protocol.strip_prefix("panopticon://connect/"))
                    .ok_or("Неподдерживаемая команда протокола")?;
                if !steam::public_address(address) {
                    return Err("Некорректный публичный адрес сервера".into());
                }
                Ok(Action::OpenExternal {
                    url: format!("steam://connect/{address}"),
                })
            }
            _ => Err(format!("HOST_ACTION_REQUIRED:{channel}")),
        }
    }

    pub fn steam_paths(&self) -> Result<(PathBuf, PathBuf), String> {
        let steam = self
            .selected_steam()
            .ok_or("STEAM_NOT_FOUND: не найдена установка Steam")?;
        let game = self
            .store
            .get("gmodPath")
            .and_then(Value::as_str)
            .map(PathBuf::from)
            .filter(|p| steam::is_game_directory(p))
            .or(steam::find_game(&steam)?)
            .ok_or("Лицензированная Garry’s Mod app4000 не установлена в Steam")?;
        Ok((steam, game))
    }

    pub fn launch_request(&self) -> Result<LaunchRequest, String> {
        let (steam, game) = self.steam_paths()?;
        steam::inspect_launch(&steam, &game)
    }

    pub fn set_game_pid(&mut self, pid: Option<u32>) {
        self.pid = pid.filter(|p| *p != 0);
        self.emit_optional("gmodUpdatePid", self.pid.map(|p| json!(p)));
    }
    pub fn set_launch_state(&self, state: Option<Value>) {
        self.emit(
            "gmodLaunchState",
            vec![state.unwrap_or(Value::Null)],
            vec![],
        );
    }
    pub fn set_window_visible(&mut self, visible: bool) {
        self.window_visible = visible;
        self.emit("hubWindowVisible", vec![json!(visible)], vec![]);
    }
    pub fn set_protocol(&mut self, protocol: Option<String>) {
        self.protocol = protocol;
    }
    pub fn emit_notification(&self, level: &str, options: Value) {
        self.emit("notify", vec![json!(level), options], vec![]);
    }
    pub fn settings_path(&self) -> &Path {
        self.store.path()
    }
    fn selected_steam(&self) -> Option<PathBuf> {
        self.store
            .get("steamPath")
            .and_then(Value::as_str)
            .map(PathBuf::from)
            .filter(|p| steam::is_steam_directory(p))
            .or_else(steam::find_steam)
    }
    fn health(&mut self) -> Option<String> {
        self.api_base = None;
        for base in regional_bases(
            self.store
                .get("region")
                .and_then(Value::as_str)
                .unwrap_or("cis"),
        ) {
            if http::get_json(base, "/health")
                .ok()
                .is_some_and(|v| v.get("success") == Some(&Value::Bool(true)))
            {
                self.api_base = Some((*base).into());
                break;
            }
        }
        self.api_base.clone()
    }
    fn emit_optional(&self, channel: &str, value: Option<Value>) {
        match value {
            Some(value) => self.emit(channel, vec![value], vec![]),
            None => self.emit(channel, vec![Value::Null], vec![0]),
        }
    }
    fn emit(&self, channel: &str, args: Vec<Value>, undefined_args: Vec<usize>) {
        (self.sink)(Event {
            channel: channel.into(),
            args,
            undefined_args,
        });
    }
}

fn social_links() -> Value {
    serde_json::from_str(include_str!("../../ui/social.json")).expect("embedded social links")
}

pub fn links() -> Value {
    let mut result = json!({
    "website":"https://panopticon-trial.neocities.org/",
    "gmodSetupGuide":"https://panopticon-trial.neocities.org/wiki/start/soft",
    "rules":"https://panopticon-trial.neocities.org/wiki/rules/rules-game.html",
    "agreement":"https://panopticon-trial.neocities.org/wiki/documents/user-agreement.html",
    "consent":"https://panopticon-trial.neocities.org/wiki/documents/personal-data-consent.html",
    "privacy":"https://panopticon-trial.neocities.org/wiki/documents/privacy-policy.html",
    "wiki":"https://panopticon-trial.neocities.org/wiki/",
    "gmodStore":"https://store.steampowered.com/app/4000/Garrys_Mod/" });
    for link in social_links().as_array().unwrap() {
        result[link["name"].as_str().unwrap().to_ascii_lowercase()] = link["url"].clone();
    }
    result
}
pub fn allowed_external(url: &str) -> Result<String, String> {
    if links()
        .as_object()
        .unwrap()
        .values()
        .any(|value| value.as_str() == Some(url))
        || [
            "steam://store/4000",
            "steam://open/games/details/4000",
            "steam://gameproperties/4000/betas",
            "steam://install/4000",
        ]
        .contains(&url)
    {
        Ok(url.into())
    } else {
        Err("EXTERNAL_URL_NOT_ALLOWED".into())
    }
}
fn regional_bases(region: &str) -> &'static [&'static str] {
    if region == "eu" {
        &["https://api.shinri-trial.com"]
    } else {
        &["https://api.shinri.red", "https://api-fb1.shinri.red"]
    }
}
fn public_path(path: &str) -> bool {
    path == "/games/servers/total-online"
}
fn arguments(value: &Value) -> Result<&[Value], String> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| "IPC args должен быть массивом".into())
}
fn string(args: &[Value], index: usize) -> Result<&str, String> {
    args.get(index)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("IPC argument {index} должен быть строкой"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    fn backend() -> (Backend, Arc<Mutex<Vec<Event>>>, PathBuf) {
        let path = std::env::temp_dir().join(format!(
            "hub-backend-{}-{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let events = Arc::new(Mutex::new(Vec::new()));
        let out = events.clone();
        let backend =
            Backend::at(path.clone(), move |event| out.lock().unwrap().push(event)).unwrap();
        (backend, events, path)
    }
    #[test]
    fn undefined_pid_is_distinct_from_null() {
        let (mut backend, events, path) = backend();
        assert_eq!(backend.invoke("getGmodPid", &json!([])).unwrap(), None);
        backend.set_game_pid(None);
        let event = events.lock().unwrap().pop().unwrap();
        assert_eq!(event.undefined_args, vec![0]);
        assert_eq!(event.args, vec![Value::Null]);
        let _ = std::fs::remove_file(path);
    }
    #[test]
    fn settings_persist_and_validate() {
        let (mut backend, _, path) = backend();
        assert_eq!(
            backend
                .invoke("store", &json!(["set", "language", "fr"]))
                .unwrap(),
            None
        );
        assert!(backend
            .invoke("store", &json!(["set", "uiVolume", 2]))
            .is_err());
        let reopened = Backend::at(path.clone(), |_| {}).unwrap();
        assert_eq!(reopened.store.get("language"), Some(&json!("fr")));
        let _ = std::fs::remove_file(path);
    }
    #[test]
    fn drops_retired_experiments_when_loading_an_old_profile() {
        let (_, _, path) = backend();
        let retired = [
            "experimentalFeatures",
            "experimentalRustGeometry",
            "experimentalTextureFilter",
            "experimentalLuaRust",
            "experimentalVulkan",
            "vulkanMediumTextures",
        ];
        let mut old = json!({"region": "eu"});
        for key in retired {
            old[key] = json!(true);
        }
        std::fs::write(&path, serde_json::to_vec(&old).unwrap()).unwrap();
        let mut reopened = Backend::at(path.clone(), |_| {}).unwrap();
        assert_eq!(reopened.store.get("region"), Some(&json!("eu")));
        let persisted: Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        for key in retired {
            assert!(persisted.get(key).is_none());
            assert!(reopened.store.set(key, json!(true)).is_err());
        }
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn launch_is_never_faked() {
        let (mut backend, _, path) = backend();
        assert!(backend
            .invoke("startGmod", &json!([]))
            .unwrap_err()
            .contains("HOST_ACTION_REQUIRED"));
        assert!(backend.invoke("notAChannel", &json!([])).is_err());
        let _ = std::fs::remove_file(path);
    }
    #[test]
    fn opens_installed_game_without_a_profile() {
        let (mut backend, _, path) = backend();
        let game = path.with_extension("game");
        std::fs::create_dir_all(game.join("garrysmod")).unwrap();
        let executable = game.join("gmod.exe");
        std::fs::write(&executable, []).unwrap();
        backend.store.set("gmodPath", json!(&game)).unwrap();
        for args in [json!([]), json!([false]), json!([true])] {
            assert!(matches!(
                backend.action_for_invoke("openHostFolder", &args),
                Ok(Action::OpenPath { path }) if path == game
            ));
        }
        std::fs::remove_file(executable).unwrap();
        std::fs::remove_dir(game.join("garrysmod")).unwrap();
        std::fs::remove_dir(game).unwrap();
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn external_urls_are_allowlisted() {
        assert!(allowed_external("https://panopticon-trial.neocities.org/").is_ok());
        assert!(allowed_external("https://t.me/HET_HOMEP").is_ok());
        for url in [
            "file:///C:/Windows/System32/cmd.exe",
            "https://panopticon-trial.neocities.org.evil/",
            "https://t.me/HET_HEMEP",
            "javascript:alert(1)",
        ] {
            assert!(allowed_external(url).is_err());
        }
    }
    #[test]
    fn social_links_are_available_without_an_api_base() {
        let (mut backend, _, path) = backend();
        let response = backend.invoke("stApiGet", &json!(["/media"])).unwrap().unwrap();
        let links = response["result"].as_array().unwrap();
        assert_eq!(links.len(), 3);
        assert_eq!(links[1]["url"], "https://t.me/HET_HOMEP");
        assert!(links.iter().all(|link| link["iconUrl"].as_str().unwrap().starts_with("./social/")));
        let _ = std::fs::remove_file(path);
    }
    #[test]
    fn event_notify_preserves_two_arguments() {
        let (backend, events, path) = backend();
        backend.emit_notification("warning", json!({"content":"Тест"}));
        assert_eq!(
            events.lock().unwrap()[0].args,
            vec![json!("warning"), json!({"content":"Тест"})]
        );
        let _ = std::fs::remove_file(path);
    }
}
