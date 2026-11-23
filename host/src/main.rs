#![cfg_attr(not(test), windows_subsystem = "windows")]
mod assets;
mod design;
mod game;
mod game_profile;
mod identity;
mod shell;
mod shortcut;
mod updates;
mod workshop;

use hub_backend::{Action, Backend, Event as BackendEvent, WindowCommand};
use serde_json::{json, Value};
use std::{
    ffi::OsStr,
    io::{BufRead, BufReader},
    os::windows::process::CommandExt,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread,
    time::Duration,
};
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy},
    platform::windows::WindowBuilderExtWindows,
    window::{Window, WindowBuilder},
};
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent,
};
use wry::{NewWindowResponse, WebContext, WebView, WebViewBuilder, WebViewBuilderExtWindows};

const ORIGIN: &str = "https://panopticon.localhost";
type Answer = Result<Option<Value>, String>;

enum DesignDialog {
    Asset(String),
    Import,
    Export(Box<design::Profile>),
}

impl DesignDialog {
    fn pick(&self, window: &Window) -> Option<PathBuf> {
        let picker = rfd::FileDialog::new().set_parent(window);
        match self {
            Self::Asset(slot) => {
                let images = ["png", "jpg", "jpeg", "webp", "gif"];
                let formats: &[&str] = match slot.as_str() {
                    "hubIcon" | "gameIcon" => &["ico"],
                    "hubMusic" | "gameMenuMusic" | "gameLoadingMusic" => &["mp3", "ogg", "wav"],
                    "hubLogo" | "gameLogo" => &images,
                    "hubBackground" | "decoration" => &["png", "jpg", "jpeg", "webp", "gif", "webm", "mp4"],
                    _ => &["png", "jpg", "jpeg", "webp", "gif", "webm"],
                };
                picker.set_title("Выберите файл оформления").add_filter("Медиафайлы", formats).pick_file()
            }
            Self::Import => picker.set_title("Импорт оформления")
                .add_filter("Оформление Panopticon Hub", &[design::DESIGN_PRESET_EXTENSION]).pick_file(),
            Self::Export(_) => picker.set_title("Экспорт оформления")
                .add_filter("Оформление Panopticon Hub", &[design::DESIGN_PRESET_EXTENSION])
                .set_file_name("Panopticon.pthubtheme").save_file(),
        }
    }

    fn finish(self, path: Option<PathBuf>) -> Answer {
        let Some(path) = path else {
            return Ok(Some(if matches!(self, Self::Export(_)) { json!(false) } else { Value::Null }));
        };
        match self {
            Self::Asset(slot) => design::pick_asset(&slot, &path).map(|key| Some(json!(key))),
            Self::Import => design::import(&path).map(|profile| Some(json!(profile))),
            Self::Export(profile) => {
                // Диалог Windows подтверждает замену именно выбранного пути.
                design::export(*profile, &path).map(|()| Some(json!(true)))
            }
        }
    }
}

fn design_profile(value: &Value) -> Result<design::Profile, String> {
    serde_json::from_value(value.clone()).map_err(|_| "Не удалось прочитать профиль оформления.".into())
}

enum Ui {
    Ipc(Value),
    Push(Value),
    Act(Option<u64>, Action),
    Pick(u64, Option<PathBuf>),
    DesignPick(u64, DesignDialog),
    DesignSaved(u64, Box<design::Profile>),
    Progress(Value),
    Game(u64, Result<game::Game, String>, bool, bool),
    WorkshopPrepared,
    GameClosed(Result<(), String>),
    Show,
    Quit,
}
enum Work {
    Ipc(Value),
    DesignFile(u64, DesignDialog, Option<PathBuf>),
    Visible(bool),
    Pid(Option<u32>),
    Progress(Value),
    WorkshopPrepared,
    UpdateChecked(u64, Result<Option<updates::Update>, String>),
    UpdateDownloaded(u64, updates::Installation, Result<updates::VerifiedSetup, String>),
}

fn send_ui(proxy: &EventLoopProxy<Ui>, event: Ui) {
    let _ = proxy.send_event(event);
}
fn answer(id: u64, result: Answer) -> Value {
    match result {
        Ok(Some(v)) => json!({"kind":"result","id":id,"ok":true,"value":v}),
        Ok(None) => json!({"kind":"result","id":id,"ok":true,"isUndefined":true}),
        Err(e) => json!({"kind":"result","id":id,"ok":false,"error":{"message":e}}),
    }
}
fn dispatch(web: &WebView, packet: &Value) {
    let _ = web.evaluate_script(&format!("window.__panopticonReceive?.({packet});"));
}
fn addr(url: &str) -> bool {
    url == ORIGIN
        || url
            .strip_prefix(ORIGIN)
            .is_some_and(|tail| tail.starts_with('/'))
        || url.starts_with("panopticon://localhost/")
}
// Closing the hub also releases its Workshop worker, even during a long Steam download.
fn workshop_job(child: &std::process::Child) -> std::io::Result<std::os::windows::io::OwnedHandle> {
    use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
    use windows_sys::Win32::System::JobObjects::*;
    unsafe {
        let raw = CreateJobObjectW(std::ptr::null(), std::ptr::null());
        if raw.is_null() {
            return Err(std::io::Error::last_os_error());
        }
        let job = OwnedHandle::from_raw_handle(raw);
        let mut limits: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = std::mem::zeroed();
        limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        if SetInformationJobObject(
            raw,
            JobObjectExtendedLimitInformation,
            (&limits as *const JOBOBJECT_EXTENDED_LIMIT_INFORMATION).cast(),
            std::mem::size_of_val(&limits) as u32,
        ) == 0
            || AssignProcessToJobObject(raw, child.as_raw_handle()) == 0
        {
            return Err(std::io::Error::last_os_error());
        }
        Ok(job)
    }
}
fn content_command(operation: &str, steam: &Path, game: &Path) -> Result<Command, String> {
    let mut command = Command::new(std::env::current_exe().map_err(|e| e.to_string())?);
    command.arg(operation).arg(steam).arg(game)
        .creation_flags(0x08000000)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    Ok(command)
}

fn content_error(stderr: &str) -> String {
    if stderr.trim().is_empty() {
        "Подготовка Steam завершилась ошибкой".into()
    } else {
        stderr.trim().to_owned()
    }
}

fn preload_content(
    request: &hub_backend::LaunchRequest, panopticon: bool, filter: bool,
    cancel: &AtomicBool, notify: &EventLoopProxy<Ui>,
) -> Result<workshop::MountPlan, String> {
    let mut child = content_command("--content-worker", &request.steam_root, &request.game_root)?
        .args([if panopticon { "both" } else { "shinri" }, if filter { "filtered" } else { "normal" }])
        .spawn()
        .map_err(|e| e.to_string())?;
    let _job = match workshop_job(&child) {
        Ok(job) => job,
        Err(error) => {
            let _ = child.kill();
            let _ = child.wait();
            return Err(error.to_string());
        }
    };
    let out = child.stdout.take().unwrap();
    let proxy = notify.clone();
    let reader = thread::spawn(move || {
        let mut plan = None;
        for line in BufReader::new(out).lines().map_while(Result::ok) {
            if let Some((a, b)) = line
                .split_once('/')
                .and_then(|(a, b)| Some((a.parse::<u64>().ok()?, b.parse::<u64>().ok()?)))
            {
                send_ui(
                    &proxy,
                    Ui::Progress(json!({"phase":"downloadingContent","current":a,"total":b})),
                );
            } else if let Ok(result) = serde_json::from_str::<workshop::MountPlan>(&line) {
                plan = Some(result);
            }
        }
        plan
    });
    loop {
        if cancel.load(Ordering::Acquire) {
            let _ = child.kill();
            let _ = child.wait();
            let _ = reader.join();
            return Err("LAUNCH_CANCELLED".into());
        }
        if let Some(exit) = child.try_wait().map_err(|e| e.to_string())? {
            let plan = reader.join().ok().flatten();
            if exit.success() {
                return plan.ok_or_else(|| "Steam не вернул план загрузки Workshop.".into());
            }
            let err = BufReader::new(child.stderr.take().unwrap())
                .lines()
                .map_while(Result::ok)
                .collect::<Vec<_>>()
                .join(" ");
            return Err(content_error(&err));
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn workshop_request(operation: &str, steam: &Path, game: &Path) -> Answer {
    let output = content_command(operation, steam, game)?.output().map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(content_error(&String::from_utf8_lossy(&output.stderr)));
    }
    serde_json::from_slice(&output.stdout).map(Some).map_err(|error| error.to_string())
}

fn install_receipt() -> Option<Value> {
    use std::io::Read;
    let exe = std::env::current_exe().ok()?;
    let receipt = std::fs::File::open(exe.parent()?.join(".panopticon-hub-install.json")).ok()?;
    let mut bytes = Vec::new();
    receipt.take(128 * 1024).read_to_end(&mut bytes).ok()?;
    let data: Value = serde_json::from_slice(&bytes).ok()?;
    if data["owner"] != "PanopticonHubSetup/1" { return None; }
    Some(data)
}
fn prepare_branch(
    steam: &std::path::Path,
    game: &std::path::Path,
    cancel: &AtomicBool,
    proxy: &EventLoopProxy<Ui>,
) -> Result<hub_backend::LaunchRequest, String> {
    let state = hub_backend::steam::inspect_launch(steam, game);
    let issue = match state {
        Ok(req) => return Ok(req),
        Err(e) => e,
    };
    if !issue.starts_with("STEAM_BETA_REQUIRED:") && !issue.starts_with("STEAM_UPDATE_PENDING:") {
        return Err(issue);
    }
    game_profile::check_running_at(game)?;
    send_ui(proxy, Ui::Progress(json!({"phase":"waitingSteam"})));
    if issue.starts_with("STEAM_BETA_REQUIRED:") {
        let output = Command::new(std::env::current_exe().map_err(|e| e.to_string())?)
            .arg("--select-x86-64")
            .arg(steam)
            .arg(game)
            .creation_flags(0x08000000)
            .stdin(Stdio::null())
            .output()
            .map_err(|e| e.to_string())?;
        if !output.status.success() {
            let _ = shell::open(OsStr::new("steam://gameproperties/4000/betas"));
            return Err(issue);
        }
    }
    // Steam owns the download and manifest. Wait until it reports a fully mounted Win64 build.
    let deadline = std::time::Instant::now() + Duration::from_secs(1800);
    loop {
        match hub_backend::steam::inspect_launch(steam, game) {
            Ok(req) => return Ok(req),
            Err(e)
                if e.starts_with("STEAM_BETA_REQUIRED:")
                    || e.starts_with("STEAM_UPDATE_PENDING:") =>
            {
                if std::time::Instant::now() >= deadline {
                    return Err(e);
                }
            }
            Err(e) => return Err(e),
        }
        for _ in 0..8 {
            if cancel.load(Ordering::Acquire) {
                return Err("LAUNCH_CANCELLED".into());
            }
            thread::sleep(Duration::from_millis(250));
        }
    }
}

fn update_game_active(backend: &mut Backend, launching: bool) -> bool {
    launching
        || backend.invoke("getGmodPid", &json!([])).ok().flatten().is_some()
        || backend.steam_paths().ok().is_some_and(|(_, game)| {
            game_profile::check_running_at(&game).is_err()
        })
}

fn actor(
    rx: mpsc::Receiver<Work>,
    proxy: EventLoopProxy<Ui>,
    quit: Arc<AtomicBool>,
    tx: mpsc::Sender<Work>,
) -> Result<(), String> {
    let p = proxy.clone();
    let mut backend = Backend::new(
        move |BackendEvent {
                  channel,
                  args,
                  undefined_args,
              }| {
            send_ui(
                &p,
                Ui::Push(
                    json!({"kind":"event","channel":channel,"args":args,"undefinedArgs":undefined_args}),
                ),
            )
        },
    )?;
    let receipt = install_receipt().unwrap_or(Value::Null);
    let installation = receipt["installation_id"].as_str();
    if let Some(id) = installation { backend.workshop_installation(id)?; }
    let initial_style = installation.filter(|id| {
        receipt["initial_style_offer"] == Value::Bool(true) && design::valid_installation_id(id)
    });
    let mut busy: Option<Arc<AtomicBool>> = None;
    let mut downloading_update = false;
    let mut update_started = false;
    let mut checked_update: Option<updates::Update> = None;
    let mut checking_update = false;
    for work in rx {
        let packet = match work {
            Work::DesignFile(id, dialog, path) => {
                send_ui(&proxy, Ui::Push(answer(id, dialog.finish(path))));
                continue;
            }
            Work::Visible(v) => {
                backend.set_window_visible(v);
                continue;
            }
            Work::Progress(v) => {
                if v.is_null() {
                    busy = None;
                    backend.set_launch_state(None)
                } else {
                    backend.set_launch_state(Some(v))
                }
                continue;
            }
            Work::Pid(n) => {
                backend.set_game_pid(n);
                continue;
            }
            Work::WorkshopPrepared => {
                backend.invoke("store", &json!(["set", "workshopOnboarding", "done"]))?;
                continue;
            }
            Work::UpdateChecked(id, result) => {
                checking_update = false;
                let result = result.map(|update| {
                    let value = json!(update);
                    checked_update = update;
                    Some(value)
                });
                send_ui(&proxy, Ui::Push(answer(id, result)));
                continue;
            }
            Work::UpdateDownloaded(id, installation, result) => {
                downloading_update = false;
                let result = result.and_then(|setup| {
                    if update_game_active(&mut backend, busy.is_some()) {
                        return Err("UPDATE_GAME_RUNNING".into());
                    }
                    installation.start(setup)?;
                    update_started = true;
                    Ok(Some(json!(true)))
                });
                let close = result.is_ok();
                send_ui(&proxy, Ui::Push(answer(id, result)));
                if close {
                    send_ui(&proxy, Ui::Quit);
                }
                continue;
            }
            Work::Ipc(v) => v,
        };
        let channel = packet["channel"].as_str().unwrap_or("");
        let args = &packet["args"];
        let id = packet["id"].as_u64().unwrap_or(0);
        if packet["kind"] == "send" {
            if let Ok(actions) = backend.send(channel, args) {
                for a in actions {
                    send_ui(&proxy, Ui::Act(None, a));
                }
            }
            continue;
        }
        if packet["kind"] != "invoke" || id == 0 || !args.is_array() {
            continue;
        }
        let result: Answer = match channel {
            "getAppVersion" => Ok(Some(json!(env!("CARGO_PKG_VERSION")))),
            "updateCurrentVersion" if args.as_array().is_some_and(Vec::is_empty) =>
                Ok(Some(json!(updates::installed_version(&receipt)))),
            "updateGetMode" if args.as_array().is_some_and(Vec::is_empty) =>
                updates::mode().map(|mode| Some(json!(mode))),
            "updateSetMode" if args.as_array().is_some_and(|a| a.len() == 1) =>
                serde_json::from_value::<updates::Mode>(args[0].clone())
                    .map_err(|_| "Неизвестный режим обновлений.".to_string())
                    .and_then(updates::set_mode)
                    .map(|()| Some(json!(true))),
            "updateCheck" if args.as_array().is_some_and(Vec::is_empty) => {
                if checking_update {
                    Err("UPDATE_BUSY".into())
                } else {
                    checking_update = true;
                    checked_update = None;
                    let reply = tx.clone();
                    let current = updates::installed_version(&receipt).to_owned();
                    thread::spawn(move || {
                        let _ = reply.send(Work::UpdateChecked(id, updates::check(&current)));
                    });
                    continue;
                }
            }
            "updateInstall" if args.as_array().is_some_and(Vec::is_empty) => {
                if downloading_update || update_started {
                    Err("UPDATE_BUSY".into())
                } else if update_game_active(&mut backend, busy.is_some()) {
                    Err("UPDATE_GAME_RUNNING".into())
                } else {
                    match (updates::Installation::from_receipt(&receipt), checked_update.clone()) {
                        (None, _) => Err("UPDATE_UNMANAGED".into()),
                        (_, None) => Err("UPDATE_NOT_AVAILABLE".into()),
                        (Some(installation), Some(update)) => {
                            downloading_update = true;
                            let reply = tx.clone();
                            thread::spawn(move || {
                                let result = updates::download(&update);
                                let _ = reply.send(Work::UpdateDownloaded(id, installation, result));
                            });
                            continue;
                        }
                    }
                }
            }
            "designLoad" if args.as_array().is_some_and(Vec::is_empty) =>
                design::load().map(|profile| Some(json!(profile))),
            "designStyles" if args.as_array().is_some_and(Vec::is_empty) => {
                initial_style.map_or(Ok(false), design::needs_initial_style)
                    .map(|pending| Some(json!({"pending": pending, "themes": design::builtin_styles()})))
            }
            "designSave" | "designSelectInitialStyle" if args.as_array().is_some_and(|args| args.len() == 1) => {
                let result = if channel == "designSave" {
                    design_profile(&args[0]).and_then(design::save)
                } else if let (Some(installation), Some(choice)) = (initial_style, args[0].as_str()) {
                    design::select_initial_style(installation, choice)
                } else {
                    Err("Выбор начального оформления недоступен.".into())
                };
                match result {
                    Ok(profile) => {
                        send_ui(&proxy, Ui::DesignSaved(id, Box::new(profile)));
                        continue;
                    }
                    Err(error) => Err(error),
                }
            }
            "designPickAsset" | "designImport" | "designExport" => {
                let args = args.as_array().expect("Аргументы IPC проверены.");
                let dialog = match (channel, args.as_slice()) {
                    ("designPickAsset", [slot]) => match slot.as_str() {
                        Some(slot @ ("hubBackground" | "gameMenuBackground" | "gameBrowserBackground"
                            | "gameLoadingBackground" | "hubLogo" | "gameLogo" | "hubIcon" | "gameIcon"
                            | "hubMusic" | "gameMenuMusic" | "gameLoadingMusic"
                            | "decoration")) => Ok(DesignDialog::Asset(slot.into())),
                        _ => Err("Неизвестный элемент оформления.".into()),
                    },
                    ("designImport", []) => Ok(DesignDialog::Import),
                    ("designExport", [profile]) => design_profile(profile)
                        .map(|profile| DesignDialog::Export(Box::new(profile))),
                    _ => Err("Некорректные параметры оформления.".into()),
                };
                match dialog {
                    Ok(dialog) => {
                        send_ui(&proxy, Ui::DesignPick(id, dialog));
                        continue;
                    }
                    Err(error) => Err(error),
                }
            }
            "workshopPreview" | "workshopSubscribe" => {
                let consent = backend.invoke("store", &json!(["get", "legalConsent"]))?.is_some();
                let pending = backend.invoke("store", &json!(["get", "workshopOnboarding"]))?
                    .as_ref().and_then(Value::as_str) == Some("pending");
                let enabled = backend.invoke("store", &json!(["get", "workshopPanopticon"]))? == Some(json!(true));
                if !consent || !pending || !enabled || args.as_array().is_some_and(|args| !args.is_empty()) {
                    Err("Предложение подписки Workshop недоступно.".into())
                } else {
                    match backend.steam_paths() {
                        Err(error) => Err(error),
                        Ok((steam, game)) => {
                            let operation = if channel == "workshopPreview" { "--workshop-preview" } else { "--workshop-subscribe" };
                            let proxy = proxy.clone();
                            thread::spawn(move || {
                                send_ui(&proxy, Ui::Push(answer(id, workshop_request(operation, &steam, &game))));
                            });
                            continue;
                        }
                    }
                }
            }
            "startGmod" if update_started => Err("UPDATE_BUSY".into()),
            "startGmod" if busy.is_some() => Err("Запуск игры уже выполняется.".into()),
            "startGmod" => match backend.steam_paths() {
                Err(e) => Err(e),
                Ok((steam, game)) => {
                    backend.set_launch_state(Some(json!({"phase":"launching"})));
                    let cancel = Arc::new(AtomicBool::new(false));
                    busy = Some(cancel.clone());
                    let p = proxy.clone();
                    let stopping = quit.clone();
                    let minimize = backend
                        .invoke("store", &json!(["get", "minimizeOnJoin"]))?
                        .and_then(|v| v.as_bool())
                        .unwrap_or(true);
                    let restore = backend
                        .invoke("store", &json!(["get", "returnToHub"]))?
                        .and_then(|v| v.as_bool())
                        .unwrap_or(true);
                    let filter = backend.invoke("store", &json!(["get", "workshopFilter"]))? != Some(json!(false));
                    let panopticon_enabled = backend.invoke("store", &json!(["get", "workshopPanopticon"]))? != Some(json!(false));
                    let setup = backend.invoke("store", &json!(["get", "workshopOnboarding"]))?;
                    let workshop_pending = setup.as_ref().and_then(Value::as_str) == Some("pending");
                    let prepare_panopticon = panopticon_enabled && setup.as_ref().and_then(Value::as_str) == Some("prepare");
                    thread::spawn(move || {
                        let result = (|| {
                            if stopping.load(Ordering::Acquire) {
                                return Err("LAUNCH_CANCELLED".into());
                            }
                            let preview_after_update = panopticon_enabled && workshop_pending
                                && hub_backend::steam::inspect_launch(&steam, &game).is_err();
                            let req = prepare_branch(&steam, &game, &cancel, &p)?;
                            if preview_after_update { return Err("WORKSHOP_PREVIEW_READY".into()); }
                            game_profile::check_running_at(&req.game_root)?;
                            let plan = preload_content(&req, prepare_panopticon, filter, &cancel, &p)?;
                            if prepare_panopticon { send_ui(&p, Ui::WorkshopPrepared); }
                            if !plan.unavailable.is_empty() {
                                send_ui(&p, Ui::Push(json!({"kind":"event","channel":"notify","args":["error",{"content":
                                    format!("Steam Workshop не предоставил часть аддонов Panopticon Trial: {:?}.", plan.unavailable)}]})));
                            }
                            send_ui(&p, Ui::Progress(json!({"phase":"launching"})));
                            if stopping.load(Ordering::Acquire) {
                                return Err("LAUNCH_CANCELLED".into());
                            }
                            game::start(&req, &cancel, plan)
                        })();
                        send_ui(&p, Ui::Game(id, result, minimize, restore));
                    });
                    continue;
                }
            },
            "cancelGmodLaunch" => {
                if let Some(c) = &busy {
                    c.store(true, Ordering::Release);
                }
                Ok(None)
            }
            "createShortcut" | "openHostFolder" | "openLogsFolder" | "handleProtocol" => {
                match backend.action_for_invoke(channel, args) {
                    Ok(a) => {
                        send_ui(&proxy, Ui::Act(Some(id), a));
                        continue;
                    }
                    Err(e) => Err(e),
                }
            }
            "openDialog" => {
                send_ui(
                    &proxy,
                    Ui::Pick(
                        id,
                        args.get(0)
                            .and_then(|o| o["defaultPath"].as_str())
                            .map(PathBuf::from),
                    ),
                );
                continue;
            }
            _ => backend.invoke(channel, args),
        };
        send_ui(&proxy, Ui::Push(answer(id, result)));
    }
    if let Some(c) = busy {
        c.store(true, Ordering::Release);
    }
    Ok(())
}

fn window_action(
    action: Action,
    win: &Window,
    tx: &mpsc::Sender<Work>,
    control: &mut ControlFlow,
) -> Answer {
    match action {
        Action::Window {
            command: WindowCommand::Close,
        } => {
            *control = ControlFlow::Exit;
        }
        Action::Window {
            command: WindowCommand::Drag,
        } => {
            win.drag_window().map_err(|err| err.to_string())?;
        }
        Action::Window {
            command: WindowCommand::Hide,
        } => {
            let _ = tx.send(Work::Visible(false));
            win.set_visible(false);
        }
        Action::Window {
            command: WindowCommand::Show,
        } => {
            win.set_visible(true);
            win.set_minimized(false);
            win.set_focus();
            let _ = tx.send(Work::Visible(true));
        }
        Action::OpenExternal { url } => shell::open(OsStr::new(&url))?,
        Action::OpenPath { path } => {
            shell::open(path.as_os_str())?;
            return Ok(Some(json!(path)));
        }
        Action::CreateShortcut { .. } => {
            let exe = std::env::current_exe().map_err(|err| err.to_string())?;
            shortcut::create(&exe, &exe.with_file_name("PanopticonHubCore.exe"))
                .map_err(|err| err.to_string())?;
        }
    }
    Ok(None)
}

fn open_web(win: &Window, proxy: &EventLoopProxy<Ui>) -> Result<WebView, String> {
    let p = proxy.clone();
    let q = proxy.clone();
    let z = proxy.clone();
    let dir = PathBuf::from(std::env::var_os("LOCALAPPDATA").ok_or("Отсутствует LOCALAPPDATA")?)
        .join("PanopticonHub/WebView2");
    let mut context = WebContext::new(Some(dir));
    let web = WebViewBuilder::new_with_web_context(&mut context)
        .with_https_scheme(true)
        .with_background_color((11, 10, 9, 255))
        .with_autoplay(true)
        .with_custom_protocol("panopticon".into(), |_, req| assets::serve(req))
        .with_ipc_handler(move |request| {
            if request.body().len() > 65536 || !addr(&request.uri().to_string()) {
                return;
            }
            if let Ok(v) = serde_json::from_str(request.body()) {
                send_ui(&p, Ui::Ipc(v));
            }
        })
        .with_navigation_handler(move |url| {
            if addr(&url) {
                return true;
            }
            if let Ok(url) = hub_backend::allowed_external(&url) {
                send_ui(&q, Ui::Act(None, Action::OpenExternal { url }));
            }
            false
        })
        .with_new_window_req_handler(move |url, _| {
            if let Ok(url) = hub_backend::allowed_external(&url) {
                send_ui(&z, Ui::Act(None, Action::OpenExternal { url }));
            }
            NewWindowResponse::Deny
        })
        .with_url("panopticon://localhost/index.html")
        .build(win)
        .map_err(|e| format!("Не удалось открыть интерфейс WebView2: {e}"))?;
    Ok(web)
}
fn run_window(
    show: bool,
    win: &Window,
    web: &mut Option<WebView>,
    proxy: &EventLoopProxy<Ui>,
    tx: &mpsc::Sender<Work>,
) {
    if show {
        if web.is_none() {
            match open_web(win, proxy) {
                Ok(view) => *web = Some(view),
                Err(err) => {
                    shell::error(&err);
                    return;
                }
            }
        }
        win.set_visible(true);
        win.set_minimized(false);
        win.set_focus();
    } else {
        win.set_visible(false);
        web.take();
    }
    let _ = tx.send(Work::Visible(show));
}
fn run() -> Result<(), String> {
    let _lock = match shell::Instance::acquire()? {
        Some(lock) => lock,
        None => {
            shell::activate();
            return Ok(());
        }
    };
    assets::start();
    let profile = design::load().unwrap_or_default();
    let appearance = identity::Identity::from_profile(&profile);
    unsafe {
        let _ = windows::Win32::UI::Shell::SetCurrentProcessExplicitAppUserModelID(windows::core::w!("org.panopticon.hub"));
    }
    let ev = EventLoopBuilder::<Ui>::with_user_event().build();
    let proxy = ev.create_proxy();
    let dpi = ev
        .primary_monitor()
        .map(|m| m.size().to_logical::<f64>(m.scale_factor()))
        .unwrap_or(LogicalSize::new(1160.0, 700.0));
    let win = WindowBuilder::new()
        .with_title(&appearance.title)
        .with_window_classname(shell::WINDOW_CLASS)
        .with_inner_size(LogicalSize::new(
            1160.0_f64.min(dpi.width),
            700.0_f64.min(dpi.height),
        ))
        .with_window_icon(appearance.window_icon())
        .with_taskbar_icon(appearance.window_icon())
        .with_decorations(false)
        .with_resizable(false)
        .build(&ev)
        .map_err(|e| e.to_string())?;
    let _ = appearance.update_taskbar(&win);
    let mut web = Some(open_web(&win, &proxy)?);
    let menu = Menu::new();
    let show = MenuItem::with_id("show", appearance.tray_label(), true, None);
    let exit = MenuItem::with_id("exit", "Выход", true, None);
    menu.append_items(&[&show, &exit])
        .map_err(|e| e.to_string())?;
    let tray = TrayIconBuilder::new()
        .with_tooltip(&appearance.title)
        .with_icon(appearance.tray_icon()?)
        .with_menu_on_left_click(false)
        .with_menu(Box::new(menu))
        .build()
        .map_err(|e| e.to_string())?;
    let p = proxy.clone();
    MenuEvent::set_event_handler(Some(move |e: MenuEvent| {
        send_ui(
            &p,
            if e.id.as_ref() == "exit" {
                Ui::Quit
            } else {
                Ui::Show
            },
        )
    }));
    let p = proxy.clone();
    TrayIconEvent::set_event_handler(Some(move |e| {
        if matches!(
            e,
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            }
        ) {
            send_ui(&p, Ui::Show)
        }
    }));
    let (tx, rx) = mpsc::channel();
    let quit = Arc::new(AtomicBool::new(false));
    let stop = quit.clone();
    let worker_proxy = proxy.clone();
    let worker_tx = tx.clone();
    thread::spawn(move || {
        if let Err(err) = actor(rx, worker_proxy.clone(), stop, worker_tx) {
            shell::error(&err);
            send_ui(&worker_proxy, Ui::Quit)
        }
    });
    let mut active_game: Option<bool> = None;
    let _ = tx.send(Work::Visible(true));
    ev.run(move|event,_,control|{
        let _keep=(&tray,&_lock);*control=ControlFlow::Wait;
        match event{
            Event::UserEvent(Ui::Ipc(v))=>{let _=tx.send(Work::Ipc(v));}
            Event::UserEvent(Ui::Push(v))=>{if let Some(view)=&web {dispatch(view,&v)}}
            Event::UserEvent(Ui::Show)=>{run_window(true,&win,&mut web,&proxy,&tx);}
            Event::UserEvent(Ui::Quit)|Event::WindowEvent{event:WindowEvent::CloseRequested,..}=>*control=ControlFlow::Exit,
            Event::UserEvent(Ui::Pick(id,default))=>{
                let mut picker=rfd::FileDialog::new().set_title("Выберите каталог").set_parent(&win);
                if let Some(dir)=default.filter(|p|p.is_dir()){picker=picker.set_directory(dir);}
                let selection=picker.pick_folder();
                if let Some(view)=&web { dispatch(view,&answer(id,Ok(Some(json!({"canceled":selection.is_none(),"filePaths":selection.into_iter().collect::<Vec<_>>()}))))); }
            }
            Event::UserEvent(Ui::DesignPick(id, dialog)) => {
                let path = dialog.pick(&win);
                let _ = tx.send(Work::DesignFile(id, dialog, path));
            }
            Event::UserEvent(Ui::DesignSaved(id, profile)) => {
                identity::Identity::from_profile(&profile).apply(&win, &tray, &show);
                if let Some(view) = &web {
                    dispatch(view, &answer(id, Ok(Some(json!(profile)))));
                }
            }
            Event::UserEvent(Ui::Act(id,a))=>{
                if matches!(a,Action::Window{command:WindowCommand::Hide}) && active_game.is_some(){
                    run_window(false,&win,&mut web,&proxy,&tx);
                } else {
                    let r=window_action(a,&win,&tx,control);
                    if let (Some(id),Some(view))=(id,&web){dispatch(view,&answer(id,r));}
                }
            }
            Event::UserEvent(Ui::Progress(v))=>{let _=tx.send(Work::Progress(v));}
            Event::UserEvent(Ui::WorkshopPrepared)=>{let _=tx.send(Work::WorkshopPrepared);}
            Event::UserEvent(Ui::Game(id,result,minimize,restore))=>{
                if let Ok(child)=&result { let _=tx.send(Work::Pid(Some(child.id()))); }
                let _=tx.send(Work::Progress(Value::Null));
                match result {
                    Ok(mut child)=>{
                        if let Some(view)=&web {dispatch(view,&answer(id,Ok(Some(json!({})))));}
                        if minimize && !restore { *control=ControlFlow::Exit; } else {
                            active_game=Some(restore);
                            let p=proxy.clone();
                            thread::spawn(move||{let result=child.wait();send_ui(&p,Ui::GameClosed(result));});
                            if minimize { run_window(false,&win,&mut web,&proxy,&tx); }
                        }
                    }
                    Err(e)=>if let Some(view)=&web {dispatch(view,&answer(id,Err(e)));}
                }
            }
            Event::UserEvent(Ui::GameClosed(result))=>{
                let _=tx.send(Work::Pid(None));
                if active_game.take()==Some(true) {run_window(true,&win,&mut web,&proxy,&tx);}
                if let Err(error)=result {shell::error(&error);}
            }
            Event::WindowEvent{event:WindowEvent::Focused(true),..} if web.is_none() && win.is_visible()=>{
                run_window(true,&win,&mut web,&proxy,&tx);
            }
            Event::WindowEvent{event:WindowEvent::Resized(_),..}=>{let _=tx.send(Work::Visible(!win.is_minimized() && win.is_visible()));}
            Event::LoopDestroyed => {
                identity::Identity::clear_taskbar(&win);
                quit.store(true, Ordering::Release);
                let _ = tx.send(Work::Ipc(json!({"kind":"invoke","channel":"cancelGmodLaunch","args":[],"id":1})));
            }
            _=>{}
        }
    });
}
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let result = match args.as_slice() {
        [] => run(),
        [arg] if arg == "--check-cache" => workshop::run(workshop::Operation::Verify),
        [arg, steam, root, packs, mode] if arg == "--content-worker"
            && matches!(packs.as_str(), "both" | "shinri") && matches!(mode.as_str(), "filtered" | "normal") =>
            workshop::run_at(workshop::Operation::Prepare { panopticon: packs == "both", filter: mode == "filtered" }, Path::new(steam), Path::new(root)),
        [arg, steam, root] if arg == "--select-x86-64" =>
            workshop::run_at(workshop::Operation::SelectBranch, Path::new(steam), Path::new(root)),
        [arg, steam, root] if matches!(arg.as_str(), "--workshop-preview" | "--workshop-subscribe") => {
            let operation = if arg == "--workshop-preview" { workshop::Operation::Preview } else { workshop::Operation::Subscribe };
            workshop::run_at(operation, Path::new(steam), Path::new(root))
        }
        [arg, steam, root] if arg == "--stock-game-worker" => {
            if let Err(error) = game::run(Path::new(steam), Path::new(root)) {
                use std::io::Write as _;
                let _ = writeln!(std::io::stdout().lock(), "ERR {}", error.replace(['\r', '\n'], " "));
                std::process::exit(1);
            }
            Ok(())
        }
        _ => Err("Неизвестные параметры Panopticon Hub".into()),
    };
    if let Err(e) = result {
        if args.is_empty() {
            shell::error(&e)
        } else {
            eprintln!("{e}")
        }
        std::process::exit(1)
    }
}
