#![cfg_attr(not(test), windows_subsystem = "windows")]
mod wizard;
use panopticon_setup::{platform, InstallOptions, Receipt, Scope, HOST, UNINSTALL};
use std::{
    num::NonZeroU32,
    path::{Path, PathBuf},
    sync::atomic::AtomicBool,
};

const HUB_EXIT_TIMEOUT_MS: u32 = 30_000;

#[derive(Clone)]
pub struct Args {
    pub directory: Option<PathBuf>,
    pub scope: Scope,
    pub desktop: bool,
    pub start_menu: bool,
    pub silent: bool,
    pub uninstall: bool,
    pub helper: bool,
    pub check: bool,
    from_hub_pid: Option<NonZeroU32>,
    relaunch: bool,
}

fn parse() -> Result<Args, String> {
    let current = std::env::current_exe().map_err(|e| e.to_string())?;
    parse_options(std::env::args().skip(1), &current)
}

fn parse_options(mut args: impl Iterator<Item = String>, current: &Path) -> Result<Args, String> {
    let mut result = Args {
        directory: None,
        scope: Scope::Machine,
        desktop: true,
        start_menu: true,
        silent: false,
        uninstall: env!("PANOPTICON_SETUP_UNINSTALL_ONLY") == "true",
        helper: false,
        check: false,
        from_hub_pid: None,
        relaunch: false,
    };
    if current
        .file_name()
        .is_some_and(|name| name.to_string_lossy().eq_ignore_ascii_case(UNINSTALL))
    {
        result.uninstall = true;
        result.directory = current.parent().map(Path::to_owned);
    }
    let mut explicit_scope = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--current-user" | "/currentuser" => {
                result.scope = Scope::User;
                explicit_scope = true;
            }
            "--all-users" | "/allusers" => {
                result.scope = Scope::Machine;
                explicit_scope = true;
            }
            "--silent" | "/S" => result.silent = true,
            "--no-desktop-shortcut" | "--no-desktop" => result.desktop = false,
            "--no-start-menu-shortcut" => result.start_menu = false,
            "--dir" => {
                result.directory = Some(PathBuf::from(
                    args.next().ok_or("После --dir требуется каталог.")?,
                ))
            }
            "--uninstall-dir" => {
                result.uninstall = true;
                result.directory = Some(PathBuf::from(
                    args.next().ok_or("Не указан каталог удаления.")?,
                ));
            }
            "--temporary-helper" => result.helper = true,
            "--check-payload" => result.check = true,
            "--from-hub-pid" => {
                if result.from_hub_pid.is_some() {
                    return Err("Укажите --from-hub-pid один раз.".into());
                }
                result.from_hub_pid = Some(
                    args.next()
                        .and_then(|value| value.parse::<NonZeroU32>().ok())
                        .ok_or("После --from-hub-pid требуется положительный PID.")?,
                );
            }
            "--relaunch" => result.relaunch = true,
            _ if arg.starts_with("/D=") => result.directory = Some(PathBuf::from(&arg[3..])),
            _ => return Err(format!("Неизвестный параметр установщика: {arg}")),
        }
    }
    if result.from_hub_pid.is_some() {
        if !result.silent
            || !explicit_scope
            || result.directory.is_none()
            || result.uninstall
            || result.helper
            || result.check
        {
            return Err(
                "Обновление из Hub требует --silent, --dir и --current-user или --all-users."
                    .into(),
            );
        }
        result.desktop = false;
        result.start_menu = false;
    } else if result.relaunch {
        return Err("Для --relaunch требуется --from-hub-pid.".into());
    }
    Ok(result)
}

fn elevation_args(args: &Args, directory: &Path) -> Vec<String> {
    let mut result = vec![
        if args.uninstall {
            "--uninstall-dir"
        } else {
            "--dir"
        }
        .into(),
        directory.to_string_lossy().into_owned(),
        "--all-users".into(),
    ];
    if args.silent {
        result.push("--silent".into())
    }
    if !args.desktop {
        result.push("--no-desktop-shortcut".into())
    }
    if !args.start_menu {
        result.push("--no-start-menu-shortcut".into())
    }
    if let Some(pid) = args.from_hub_pid {
        result.extend(["--from-hub-pid".into(), pid.to_string()]);
    }
    result
}

fn relaunch_updated_hub(executable: &Path, directory: &Path) -> Result<(), String> {
    platform::launch_hub(executable, directory)
        .map_err(|error| format!("Обновление установлено. {error}"))
}

fn temporary_uninstaller(args: &Args, directory: &Path) -> Result<bool, String> {
    let current = std::env::current_exe().map_err(|e| e.to_string())?;
    if args.helper
        || current.parent().and_then(|path| path.canonicalize().ok())
            != directory.canonicalize().ok()
    {
        return Ok(false);
    }
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    let temp = std::env::temp_dir().join(format!(
        "PanopticonHub-uninstall-{}-{nonce}.exe",
        std::process::id()
    ));
    platform::reject_links(&temp)?;
    use std::io::Write;
    let mut output = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temp)
        .map_err(|e| e.to_string())?;
    output
        .write_all(&std::fs::read(&current).map_err(|e| e.to_string())?)
        .and_then(|_| output.sync_all())
        .map_err(|e| e.to_string())?;
    drop(output);
    let mut command = std::process::Command::new(&temp);
    command
        .arg("--uninstall-dir")
        .arg(directory)
        .arg("--temporary-helper")
        .arg(if args.scope == Scope::Machine {
            "--all-users"
        } else {
            "--current-user"
        });
    if args.silent {
        command.arg("--silent");
    }
    // Helper ждёт освобождения исходного EXE без остановки чужих процессов.
    command
        .env(
            "PANOPTICON_UNINSTALL_PARENT",
            std::process::id().to_string(),
        )
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(true)
}

fn wait_parent() {
    if let Ok(parent) = std::env::var("PANOPTICON_UNINSTALL_PARENT") {
        if let Ok(pid) = parent.parse() {
            unsafe {
                use windows_sys::Win32::{
                    Foundation::CloseHandle,
                    System::Threading::{OpenProcess, WaitForSingleObject, PROCESS_SYNCHRONIZE},
                };
                let handle = OpenProcess(PROCESS_SYNCHRONIZE, 0, pid);
                if !handle.is_null() {
                    WaitForSingleObject(handle, 30000);
                    CloseHandle(handle);
                }
            }
        }
    }
}

fn app(mut args: Args, previous: &mut Option<Receipt>) -> Result<(), String> {
    if args.check {
        println!(
            "Panopticon Hub payload: {} bytes, SHA256 {}, fixture={}",
            panopticon_setup::PAYLOAD.len(),
            panopticon_setup::PAYLOAD_SHA256,
            env!("PANOPTICON_PAYLOAD_FIXTURE")
        );
        return Ok(());
    }
    let directory = match &args.directory {
        Some(directory) => directory.clone(),
        None => platform::default_directory(args.scope)?,
    };
    if let Some(pid) = args.from_hub_pid {
        let receipt = panopticon_setup::read_update_receipt(&directory, args.scope)?;
        platform::wait_for_hub(pid, &directory.join(HOST), HUB_EXIT_TIMEOUT_MS)?;
        receipt.check_hub_closed()?;
        if args.relaunch {
            *previous = Some(receipt);
        }
    }
    if args.uninstall {
        let receipt = panopticon_setup::read_receipt(&directory)?;
        args.scope = receipt.scope;
        if args.silent && !args.helper {
            // Тихий запуск должен вернуть ошибку до передачи управления временному EXE.
            receipt.check_hub_closed()?;
        }
    }
    if args.scope == Scope::Machine && !platform::is_admin() {
        // После UAC запускаем Hub из исходного процесса с правами вызвавшего пользователя.
        platform::elevate(&elevation_args(&args, &directory), args.relaunch)?;
        if args.relaunch {
            *previous = None;
            relaunch_updated_hub(&directory.join(HOST), &directory)?;
        }
        return Ok(());
    }
    if args.uninstall && temporary_uninstaller(&args, &directory)? {
        return Ok(());
    }
    if args.helper {
        wait_parent();
    }
    if args.silent {
        if args.uninstall {
            panopticon_setup::uninstall(&directory, |s| println!("{s}"))?;
        } else {
            let install = panopticon_setup::install(
                &InstallOptions {
                    directory: directory.clone(),
                    scope: args.scope,
                    desktop_shortcut: args.desktop,
                    start_menu_shortcut: args.start_menu,
                },
                &AtomicBool::new(false),
                |s| println!("{s}"),
            )?;
            if args.relaunch {
                *previous = None;
                relaunch_updated_hub(&install.executable, &directory)?;
            }
        }
        return Ok(());
    }
    wizard::run(args, directory)
}

fn main() {
    if std::env::args_os().len() > 1 {
        unsafe {
            windows_sys::Win32::System::Console::AttachConsole(
                windows_sys::Win32::System::Console::ATTACH_PARENT_PROCESS,
            );
        }
    }
    let args = parse();
    let show_dialog = match &args {
        Ok(args) => !args.silent || args.relaunch,
        Err(_) => {
            !std::env::args().any(|arg| arg == "--silent" || arg == "/S")
                || std::env::args().any(|arg| arg == "--relaunch")
        }
    };
    let mut previous = None;
    if let Err(mut error) = args.and_then(|args| app(args, &mut previous)) {
        if let Some(previous) = previous {
            match previous.restart_previous_hub() {
                Ok(()) => error.push_str("\n\nПрежняя версия Hub снова доступна."),
                Err(recovery) => error.push_str(&format!("\n\nПрежний Hub не запущен. {recovery}")),
            }
        }
        if show_dialog {
            unsafe {
                windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxW(
                    std::ptr::null_mut(),
                    platform::wide(&error).as_ptr(),
                    platform::wide("Установка Panopticon Hub").as_ptr(),
                    windows_sys::Win32::UI::WindowsAndMessaging::MB_OK
                        | windows_sys::Win32::UI::WindowsAndMessaging::MB_ICONERROR,
                );
            }
        } else {
            eprintln!("{error}");
        }
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_test(args: &[&str]) -> Result<Args, String> {
        parse_options(
            args.iter().map(|arg| (*arg).to_owned()),
            Path::new("PanopticonHubSetup.exe"),
        )
    }

    #[test]
    fn update_requires_silent_explicit_destination_scope_and_pid() {
        assert!(parse_test(&["--relaunch"]).is_err());
        assert!(parse_test(&[
            "--from-hub-pid", "12", "--dir", r"C:\Apps\Hub", "--all-users",
        ]).is_err());
        assert!(parse_test(&[
            "--from-hub-pid", "12", "--silent", "--dir", r"C:\Apps\Hub",
        ]).is_err());
        assert!(parse_test(&[
            "--from-hub-pid", "12", "--silent", "--all-users",
        ]).is_err());
        for bad_pid in ["0", "-1", "4294967296", "invalid"] {
            assert!(parse_test(&["--from-hub-pid", bad_pid]).is_err());
        }
    }

    #[test]
    fn elevated_update_keeps_pid_without_requesting_relaunch() {
        let directory = r"C:\Apps\Panopticon Hub";
        let args = parse_test(&[
            "--silent", "--dir", directory, "--all-users", "--from-hub-pid", "12", "--relaunch",
        ]).unwrap();
        assert!(args.relaunch);
        assert!(!args.desktop && !args.start_menu);
        let elevated = elevation_args(&args, Path::new(directory));
        let elevated = parse_options(elevated.into_iter(), Path::new("PanopticonHubSetup.exe")).unwrap();
        assert_eq!(elevated.from_hub_pid, NonZeroU32::new(12));
        assert_eq!(elevated.directory.as_deref(), Some(Path::new(directory)));
        assert_eq!(elevated.scope, Scope::Machine);
        assert!(!elevated.relaunch);
    }
}
