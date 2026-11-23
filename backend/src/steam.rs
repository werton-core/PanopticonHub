use serde_json::Value;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct LaunchRequest {
    pub steam_root: PathBuf,
    pub game_root: PathBuf,
    pub executable: PathBuf,
    pub branch: String,
    pub build_id: String,
}

pub fn profile_root(game_root: &Path) -> Result<PathBuf, String> {
    let library = game_root
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .ok_or("Некорректный путь библиотеки установленной игры")?;
    Ok(library.join("PanopticonHub/profile"))
}

/// Только два базовых depot app4000 из установленного appmanifest, не Workshop.
pub fn installed_depots(req: &LaunchRequest) -> Result<Vec<(u32, u64, PathBuf)>, String> {
    let steamapps = req
        .game_root
        .parent()
        .and_then(Path::parent)
        .ok_or("Не найден steamapps")?;
    let library = steamapps.parent().ok_or("Не найдена библиотека Steam")?;
    let document = read_vdf(&steamapps.join("appmanifest_4000.acf"))?;
    let installed = object(object(&document, "AppState")?, "InstalledDepots")?;
    let mut result = Vec::new();
    for depot in [4001u32, 4002] {
        let manifest = text(object(installed, &depot.to_string())?, "manifest")?
            .parse::<u64>()
            .map_err(|_| "Некорректный номер depot manifest")?;
        let name = format!("{depot}_{manifest}.manifest");
        let path=[req.steam_root.join("depotcache").join(&name),library.join("depotcache").join(&name)].into_iter().find(|p|p.is_file())
            .ok_or_else(||format!("DEPOT_MANIFEST_MISSING: Steam не сохранил локальный список depot {depot}; выполните проверку файлов Garry’s Mod"))?;
        result.push((depot, manifest, path));
    }
    Ok(result)
}

/// Идентификаторы фактически установленных версий Workshop из локального индекса Steam.
pub fn workshop_manifests(path: &Path) -> Result<BTreeMap<u64, u64>, String> {
    let document = read_vdf(path)?;
    let installed = object(object(&document, "AppWorkshop")?, "WorkshopItemsInstalled")?;
    Ok(installed
        .as_object()
        .unwrap()
        .iter()
        .filter_map(|(id, item)| {
            let manifest = number(item, "manifest").ok()?;
            (manifest != 0).then_some((id.parse().ok()?, manifest))
        })
        .collect())
}

pub fn is_steam_directory(root: &Path) -> bool {
    root.join("steam.exe").is_file() && root.join("steamapps").is_dir()
}
pub fn is_game_directory(root: &Path) -> bool {
    root.join("garrysmod").is_dir()
        && (root.join("gmod.exe").is_file() || root.join("hl2.exe").is_file())
}

pub fn find_steam() -> Option<PathBuf> {
    for variable in ["ProgramFiles(x86)", "ProgramFiles"] {
        if let Some(path) = std::env::var_os(variable) {
            let root = PathBuf::from(path).join("Steam");
            if is_steam_directory(&root) {
                return Some(root);
            }
        }
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        let registry = std::env::var_os("WINDIR")
            .map(PathBuf::from)
            .map(|p| p.join("System32/reg.exe"))
            .unwrap_or_else(|| PathBuf::from("reg.exe"));
        let output = std::process::Command::new(registry)
            .args(["QUERY", "HKCU\\Software\\Valve\\Steam", "/v", "SteamPath"])
            .creation_flags(0x08000000)
            .output()
            .ok()?;
        if !output.status.success() {
            return None;
        }
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            let line = line.trim();
            if !line.starts_with("SteamPath") {
                continue;
            }
            let value = line.split_once("REG_SZ")?.1.trim();
            let path = PathBuf::from(value);
            if is_steam_directory(&path) {
                return Some(path);
            }
        }
    }
    None
}

pub fn find_game(steam: &Path) -> Result<Option<PathBuf>, String> {
    for library in libraries(steam)? {
        let manifest = library.join("steamapps/appmanifest_4000.acf");
        if !manifest.is_file() {
            continue;
        }
        let document = read_vdf(&manifest)?;
        let app = object(&document, "AppState")?;
        let directory = text(app, "installdir")?;
        safe_install_name(directory)?;
        let root = library.join("steamapps/common").join(directory);
        if is_game_directory(&root) {
            return Ok(Some(root));
        }
    }
    Ok(None)
}

pub fn inspect_launch(steam: &Path, game: &Path) -> Result<LaunchRequest, String> {
    if !is_steam_directory(steam) {
        return Err("STEAM_NOT_FOUND: каталог Steam не подтверждён".into());
    }
    if let Some(failure) = requirements_failure() {
        return Err(format!("OS_ARCH: {failure}"));
    }
    let manifest = game
        .parent()
        .and_then(Path::parent)
        .ok_or("Некорректный каталог Garry’s Mod")?
        .join("appmanifest_4000.acf");
    let document = read_vdf(&manifest)?;
    let app = object(&document, "AppState")?;
    if text(app, "appid")? != "4000" {
        return Err("Steam manifest не принадлежит app4000".into());
    }
    let registered = find_game(steam)?.ok_or("Garry’s Mod не найдена в библиотеках Steam")?;
    if normalized(game)? != normalized(&registered)? {
        return Err("Каталог игры не соответствует установленной Steam app4000".into());
    }
    let branch = app
        .get("UserConfig")
        .and_then(|o| o.get("BetaKey"))
        .and_then(Value::as_str)
        .unwrap_or("");
    let mounted = app
        .get("MountedConfig")
        .and_then(|o| o.get("BetaKey"))
        .and_then(Value::as_str)
        .unwrap_or("");
    if branch != "x86-64" {
        return Err("STEAM_BETA_REQUIRED: выберите ветку x86-64 в Steam и дождитесь обновления. Для Panopticon Hub в Windows GModPatchTool не нужен.".into());
    }
    let flags = number(app, "StateFlags")?;
    // A suspended Workshop download can leave partial byte counts and an
    // update result even after Steam reports the base game fully installed.
    // Gate on the mounted build and pending changes to the game depot.
    if mounted != "x86-64"
        || !(flags == 4 || flags == 68)
        || number(app, "StagingSize")? != 0
        || number(app, "TargetBuildID")? != 0
    {
        return Err("STEAM_UPDATE_PENDING: дождитесь окончания загрузки ветки x86-64 в разделе «Загрузки» Steam.".into());
    }
    for filename in [
        "gmod.exe",
        "engine.dll",
        "client.dll",
        "launcher.dll",
        "lua_shared.dll",
        "steam_api64.dll",
    ] {
        check_amd64(&game.join("bin/win64").join(filename))
            .map_err(|error| format!("STEAM_X64_INSTALL_INVALID: {error}"))?;
    }
    if !game.join("garrysmod/gameinfo.txt").is_file() {
        return Err(
            "STEAM_X64_INSTALL_INVALID: отсутствует garrysmod/gameinfo.txt: проверьте файлы через Steam".into(),
        );
    }
    Ok(LaunchRequest {
        steam_root: steam.into(),
        game_root: game.into(),
        executable: game.join("bin/win64/gmod.exe"),
        branch: branch.into(),
        build_id: text(app, "buildid")?.into(),
    })
}

pub fn check_amd64(path: &Path) -> Result<(), String> {
    let mut input = File::open(path).map_err(|e| format!("Не открыт {}: {e}", path.display()))?;
    let mut dos = [0u8; 64];
    input
        .read_exact(&mut dos)
        .map_err(|_| format!("Обрезанный PE-файл {}", path.display()))?;
    let offset = u32::from_le_bytes(dos[60..64].try_into().unwrap()) as u64;
    let length = input.metadata().map_err(|e| e.to_string())?.len();
    if dos[..2] != *b"MZ" || offset < 64 || offset > length.saturating_sub(26) {
        return Err(format!("Некорректный PE-файл {}", path.display()));
    }
    input
        .seek(SeekFrom::Start(offset))
        .map_err(|e| e.to_string())?;
    let mut header = [0; 26];
    input.read_exact(&mut header).map_err(|e| e.to_string())?;
    if header[..4] != *b"PE\0\0"
        || header[4..6] != [0x64, 0x86]
        || header[24..26] != [0x0b, 0x02]
        || u16::from_le_bytes([header[22], header[23]]) & 2 == 0
    {
        return Err(format!(
            "Файл {} не является штатным исполняемым Windows AMD64 PE32+",
            path.display()
        ));
    }
    let sections = u16::from_le_bytes([header[6], header[7]]) as u64;
    let optional = u16::from_le_bytes([header[20], header[21]]) as u64;
    if sections == 0
        || sections > 96
        || optional < 112
        || offset + 24 + optional + sections * 40 > length
    {
        return Err(format!("Обрезанная таблица PE-секций {}", path.display()));
    }
    input
        .seek(SeekFrom::Start(offset + 24 + optional))
        .map_err(|e| e.to_string())?;
    for _ in 0..sections {
        let mut section = [0; 40];
        input.read_exact(&mut section).map_err(|e| e.to_string())?;
        let size = u32::from_le_bytes(section[16..20].try_into().unwrap()) as u64;
        let start = u32::from_le_bytes(section[20..24].try_into().unwrap()) as u64;
        if size != 0 && (start == 0 || start + size > length) {
            return Err(format!("Обрезанная PE-секция {}", path.display()));
        }
    }
    Ok(())
}

pub fn requirements_failure() -> Option<Value> {
    #[cfg(all(windows, target_arch = "x86_64"))]
    {
        None
    }
    #[cfg(not(all(windows, target_arch = "x86_64")))]
    {
        Some(
            serde_json::json!({"code":"OS_ARCH","os":if cfg!(target_pointer_width="64"){"x64"}else{"x86"},"cpu":if cfg!(target_pointer_width="64"){"x64"}else{"x86"}}),
        )
    }
}

pub fn public_address(address: &str) -> bool {
    let Ok(address) = address.parse::<std::net::SocketAddrV4>() else {
        return false;
    };
    let ip = address.ip();
    let [a, b, c, _] = ip.octets();
    address.port() != 0
        && !ip.is_private()
        && !ip.is_loopback()
        && !ip.is_link_local()
        && !ip.is_unspecified()
        && !ip.is_multicast()
        && !ip.is_broadcast()
        && a != 0
        && a < 224
        && !(a == 100 && (64..=127).contains(&b))
        && !(a == 192 && b == 0 && (c == 0 || c == 2))
        && !(a == 198 && (b == 18 || b == 19 || b == 51 && c == 100))
        && !(a == 203 && b == 0 && c == 113)
}

fn libraries(steam: &Path) -> Result<Vec<PathBuf>, String> {
    let mut roots = vec![steam.to_owned()];
    let path = steam.join("steamapps/libraryfolders.vdf");
    if !path.exists() {
        return Ok(roots);
    }
    let document = read_vdf(&path)?;
    let folders = document
        .get("libraryfolders")
        .unwrap_or(&document)
        .as_object()
        .ok_or("Некорректный libraryfolders.vdf")?;
    for (key, value) in folders {
        if key.parse::<u32>().is_err() {
            continue;
        }
        let path = value
            .as_str()
            .or_else(|| value.get("path").and_then(Value::as_str));
        if let Some(path) = path {
            let path = PathBuf::from(path);
            if !path.is_absolute() {
                return Err("Относительная библиотека Steam запрещена".into());
            }
            if !roots.iter().any(|p| {
                p.to_string_lossy()
                    .eq_ignore_ascii_case(&path.to_string_lossy())
            }) {
                roots.push(path);
            }
        }
    }
    Ok(roots)
}
fn normalized(path: &Path) -> Result<String, String> {
    Ok(std::fs::canonicalize(path)
        .map_err(|e| e.to_string())?
        .to_string_lossy()
        .trim_start_matches("\\\\?\\")
        .replace('/', "\\")
        .to_lowercase())
}
fn number(object: &Value, key: &str) -> Result<u64, String> {
    match object.get(key) {
        None => Ok(0),
        Some(value) => value
            .as_str()
            .ok_or_else(|| format!("Поле Steam {key} должно быть строкой"))?
            .parse()
            .map_err(|_| format!("Поле Steam {key} не является числом")),
    }
}
fn text<'a>(value: &'a Value, key: &str) -> Result<&'a str, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("В Steam manifest отсутствует {key}"))
}
fn object<'a>(value: &'a Value, key: &str) -> Result<&'a Value, String> {
    value
        .get(key)
        .filter(|v| v.is_object())
        .ok_or_else(|| format!("В VDF отсутствует объект {key}"))
}
fn safe_install_name(name: &str) -> Result<(), String> {
    if name.is_empty() || name == "." || name == ".." || name.contains(['\\', '/', ':', '\0']) {
        Err("Некорректный Steam installdir".into())
    } else {
        Ok(())
    }
}

fn read_vdf(path: &Path) -> Result<Value, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("Не прочитан {}: {e}", path.display()))?;
    if bytes.len() > 4 * 1024 * 1024 {
        return Err("Steam manifest слишком велик".into());
    }
    let source = std::str::from_utf8(&bytes).map_err(|e| e.to_string())?;
    parse_vdf(source)
}
pub fn parse_vdf(source: &str) -> Result<Value, String> {
    #[derive(Debug)]
    enum Token {
        String(String),
        Open,
        Close,
    }
    let mut chars = source.trim_start_matches('\u{feff}').chars().peekable();
    let mut tokens = Vec::new();
    while let Some(c) = chars.next() {
        match c {
            c if c.is_whitespace() => {}
            '/' if chars.peek() == Some(&'/') => {
                for c in chars.by_ref() {
                    if c == '\n' {
                        break;
                    }
                }
            }
            '{' => tokens.push(Token::Open),
            '}' => tokens.push(Token::Close),
            '"' => {
                let mut out = String::new();
                let mut closed = false;
                while let Some(c) = chars.next() {
                    if c == '"' {
                        closed = true;
                        break;
                    }
                    if c == '\\' && matches!(chars.peek(), Some('\\' | '"')) {
                        out.push(chars.next().unwrap());
                    } else {
                        out.push(c);
                    }
                }
                if !closed {
                    return Err("Незакрытая VDF строка".into());
                }
                tokens.push(Token::String(out));
            }
            c => {
                let mut out = String::from(c);
                while let Some(c) = chars.peek() {
                    if c.is_whitespace() || *c == '{' || *c == '}' {
                        break;
                    }
                    out.push(chars.next().unwrap());
                }
                tokens.push(Token::String(out));
            }
        }
    }
    fn level(
        tokens: &mut std::iter::Peekable<std::vec::IntoIter<Token>>,
        depth: u32,
    ) -> Result<Value, String> {
        if depth > 32 {
            return Err("Слишком глубокий VDF".into());
        }
        let mut out = serde_json::Map::new();
        loop {
            match tokens.peek() {
                None if depth == 0 => return Ok(Value::Object(out)),
                Some(Token::Close) if depth > 0 => {
                    tokens.next();
                    return Ok(Value::Object(out));
                }
                _ => {}
            }
            let key = match tokens.next() {
                Some(Token::String(key)) => key,
                _ => return Err("Ожидался VDF ключ".into()),
            };
            if out.keys().any(|k| k.eq_ignore_ascii_case(&key)) {
                return Err("Повторный VDF ключ".into());
            }
            let value = match tokens.next() {
                Some(Token::String(value)) => Value::String(value),
                Some(Token::Open) => level(tokens, depth + 1)?,
                _ => return Err("Ожидалось VDF значение".into()),
            };
            out.insert(key, value);
        }
    }
    level(&mut tokens.into_iter().peekable(), 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vdf_preserves_escaped_windows_paths() {
        let value = parse_vdf(r#""libraryfolders" { "1" { "path" "D:\\SteamLibrary" } }"#).unwrap();
        assert_eq!(value["libraryfolders"]["1"]["path"], "D:\\SteamLibrary");
    }
    #[test]
    fn malformed_vdf_fails_closed() {
        for source in [r#""a" {"b" "c""#, r#""a" "1" "A" "2""#] {
            assert!(parse_vdf(source).is_err());
        }
    }
    #[test]
    fn external_server_address_has_real_public_ipv4() {
        assert!(public_address("185.71.66.87:28022"));
        assert!(public_address("203.23.179.220:27016"));
        for address in [
            "127.0.0.1:27015",
            "192.168.1.1:27015",
            "100.64.0.1:27015",
            "203.0.113.1:27015",
            "185.71.66.87:0",
            "evil.example:27015",
            "185.71.66.87:27015/+quit",
        ] {
            assert!(!public_address(address), "{address}");
        }
    }
    #[test]
    fn steam_install_cannot_escape_common() {
        for name in ["..", "../game", "D:\\game", "a/b"] {
            assert!(safe_install_name(name).is_err());
        }
    }

    #[test]
    fn launch_requires_installed_beta_complete_update_and_real_amd64() {
        let root =
            std::env::temp_dir().join(format!("panopticon-steam-fixture-{}", std::process::id()));
        let game = root.join("steamapps/common/GarrysMod");
        assert!(inspect_launch(&root.join("no-steam"), &game)
            .unwrap_err()
            .starts_with("STEAM_NOT_FOUND"));
        std::fs::create_dir_all(game.join("garrysmod")).unwrap();
        std::fs::create_dir_all(game.join("bin/win64")).unwrap();
        std::fs::write(root.join("steam.exe"), []).unwrap();
        std::fs::write(game.join("gmod.exe"), []).unwrap();
        std::fs::write(game.join("garrysmod/gameinfo.txt"), "GameInfo {}").unwrap();
        let mut bytes = vec![0u8; 512];
        bytes[..2].copy_from_slice(b"MZ");
        bytes[60..64].copy_from_slice(&64u32.to_le_bytes());
        bytes[64..68].copy_from_slice(b"PE\0\0");
        bytes[68..70].copy_from_slice(&0x8664u16.to_le_bytes());
        bytes[70..72].copy_from_slice(&1u16.to_le_bytes());
        bytes[84..86].copy_from_slice(&112u16.to_le_bytes());
        bytes[86..88].copy_from_slice(&2u16.to_le_bytes());
        bytes[88..90].copy_from_slice(&0x20bu16.to_le_bytes());
        bytes[216..220].copy_from_slice(&32u32.to_le_bytes());
        bytes[220..224].copy_from_slice(&256u32.to_le_bytes());
        for name in [
            "gmod.exe",
            "engine.dll",
            "client.dll",
            "launcher.dll",
            "lua_shared.dll",
            "steam_api64.dll",
        ] {
            std::fs::write(game.join("bin/win64").join(name), &bytes).unwrap();
        }
        let manifest = root.join("steamapps/appmanifest_4000.acf");
        let write = |branch: &str, mounted: &str, flags: u32, extra: &str| {
            std::fs::write(
                &manifest,
                format!(
                    r#""AppState" {{ "appid" "4000" "installdir" "GarrysMod" "StateFlags" "{flags}" "buildid" "123" "UserConfig" {{"BetaKey" "{branch}"}} "MountedConfig" {{"BetaKey" "{mounted}"}} {extra} }}"#
                ),
            )
            .unwrap()
        };
        write("public", "public", 4, "");
        assert!(inspect_launch(&root, &game)
            .unwrap_err()
            .starts_with("STEAM_BETA_REQUIRED:"));
        write("x86-64", "public", 4, "");
        assert!(inspect_launch(&root, &game)
            .unwrap_err()
            .starts_with("STEAM_UPDATE_PENDING:"));
        for (flags, extra) in [(6, ""), (4, r#""TargetBuildID" "456""#), (4, r#""StagingSize" "50""#)] {
            write("x86-64", "x86-64", flags, extra);
            assert!(inspect_launch(&root, &game)
                .unwrap_err()
                .starts_with("STEAM_UPDATE_PENDING:"));
        }
        let suspended_workshop = r#"
            "TargetBuildID" "0" "StagingSize" "0" "UpdateResult" "4"
            "BytesToDownload" "3169280032" "BytesDownloaded" "494153392"
            "BytesToStage" "7118141218" "BytesStaged" "1102037802"
        "#;
        for flags in [4, 68] {
            write("x86-64", "x86-64", flags, suspended_workshop);
            let request = inspect_launch(&root, &game).unwrap();
            assert_eq!(request.build_id, "123");
            assert_eq!(request.executable, game.join("bin/win64/gmod.exe"));
        }
        bytes[68..70].copy_from_slice(&0x14cu16.to_le_bytes());
        std::fs::write(game.join("bin/win64/gmod.exe"), &bytes).unwrap();
        assert!(inspect_launch(&root, &game)
            .unwrap_err()
            .starts_with("STEAM_X64_INSTALL_INVALID:"));
        std::fs::remove_dir_all(root).unwrap();
    }
}
