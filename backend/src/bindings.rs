use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::Path,
};

const QUEUE: &str = "panopticon-hub-bindings.json";
const MAX_QUEUE: usize = 4096;
const MAX_CONFIG: usize = 1024 * 1024;
const COMMANDS: &[&str] = &[
    "+attack",
    "+attack2",
    "+back",
    "+duck",
    "+forward",
    "+jump",
    "+menu",
    "+menu_context",
    "+moveleft",
    "+moveright",
    "+reload",
    "+speed",
    "+use",
    "+voicerecord",
    "+walk",
    "gmod_undo",
    "invnext",
    "invprev",
    "lastinv",
    "messagemode",
    "noclip",
    "toggleconsole",
];
type Changes = BTreeMap<String, String>;

fn allowed_key(key: &str) -> bool {
    (1..=30).contains(&key.len())
        && !key.eq_ignore_ascii_case("escape")
        && key
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"_`[]'/.,=+-".contains(&byte))
}

fn validate(changes: &Changes) -> Result<(), String> {
    if changes.iter().all(|(command, key)| {
        COMMANDS.contains(&command.as_str()) && (key.is_empty() || allowed_key(key))
    }) {
        Ok(())
    } else {
        Err("Недопустимая привязка клавиши.".into())
    }
}

fn read(path: &Path, limit: usize) -> Result<Option<Vec<u8>>, String> {
    super::identity::ordinary(path)?;
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.to_string()),
    };
    let mut bytes = Vec::new();
    file.take(limit as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() > limit {
        return Err("Файл привязок слишком велик.".into());
    }
    Ok(Some(bytes))
}

fn write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    super::identity::ordinary(path)?;
    let parent = path
        .parent()
        .ok_or("Не найден каталог настроек управления.")?;
    let mut file = tempfile::NamedTempFile::new_in(parent).map_err(|error| error.to_string())?;
    file.write_all(bytes).map_err(|error| error.to_string())?;
    file.as_file()
        .sync_all()
        .map_err(|error| error.to_string())?;
    file.persist(path)
        .map_err(|error| error.error.to_string())?;
    Ok(())
}

fn read_queue(cfg: &Path) -> Result<Option<(bool, Changes)>, String> {
    let Some(bytes) = read(&cfg.join(QUEUE), MAX_QUEUE)? else {
        return Ok(None);
    };
    let (reset, changes) = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    validate(&changes)?;
    Ok(Some((reset, changes)))
}

fn defaults(cfg: &Path) -> Result<Vec<u8>, String> {
    read(&cfg.join("config_default.cfg"), MAX_CONFIG)?
        .ok_or("Не найден штатный config_default.cfg.".into())
}

pub fn engine_action(command: &str) -> bool {
    COMMANDS.contains(&command)
}

pub fn reset(cfg: &Path) -> Result<(), String> {
    defaults(cfg)?;
    write(&cfg.join(QUEUE), b"[true,{}]")
}

pub fn queue(cfg: &Path, raw: &[u8]) -> Result<(), String> {
    if raw.len() > MAX_QUEUE {
        return Err("Слишком много изменений управления.".into());
    }
    let changes: Changes = serde_json::from_slice(raw).map_err(|error| error.to_string())?;
    validate(&changes)?;
    if changes.is_empty() {
        return Ok(());
    }
    for (command, key) in &changes {
        if !key.is_empty()
            && changes
                .iter()
                .any(|(other, value)| other != command && key.eq_ignore_ascii_case(value))
        {
            return Err("Одна клавиша назначена нескольким действиям.".into());
        }
    }
    let (reset, mut pending) = read_queue(cfg)?.unwrap_or_default();
    for (command, key) in changes {
        for (other, value) in &mut pending {
            if *other != command && key.eq_ignore_ascii_case(value) {
                value.clear();
            }
        }
        pending.insert(command, key.to_ascii_uppercase());
    }
    let bytes = serde_json::to_vec(&(reset, pending)).map_err(|error| error.to_string())?;
    write(&cfg.join(QUEUE), &bytes)
}

pub fn pending(cfg: &Path) -> Result<Changes, String> {
    let (reset, pending) = read_queue(cfg)?.unwrap_or_default();
    if !reset {
        return Ok(pending);
    }
    let defaults = defaults(cfg)?;
    let config = read(&cfg.join("config.cfg"), MAX_CONFIG)?;
    let mut changes: Changes = COMMANDS
        .iter()
        .map(|command| (command.to_string(), String::new()))
        .collect();
    for (key, command) in default_bindings(config.as_deref().unwrap_or(&defaults), &defaults) {
        if pending
            .values()
            .any(|value| value.eq_ignore_ascii_case(&key))
        {
            continue;
        }
        if let Some(value) = changes.get_mut(&command) {
            if value.is_empty() {
                *value = key.to_ascii_uppercase();
            }
        }
    }
    changes.extend(pending);
    Ok(changes)
}

fn default_bindings(config: &[u8], defaults: &[u8]) -> Vec<(String, String)> {
    defaults
        .split(|&b| b == b'\n')
        .filter_map(binding)
        .filter(|(key, command)| allowed_key(key) && COMMANDS.contains(command))
        .filter(|(key, _)| {
            !config
                .split(|&b| b == b'\n')
                .filter_map(binding)
                .any(|(current, command)| {
                    current.eq_ignore_ascii_case(key) && !COMMANDS.contains(&command)
                })
        })
        .map(|(key, command)| (key.to_owned(), command.to_owned()))
        .collect()
}

fn binding(line: &[u8]) -> Option<(&str, &str)> {
    let line = std::str::from_utf8(line)
        .ok()?
        .trim()
        .trim_start_matches('\u{feff}');
    let (verb, args) = line.split_once(char::is_whitespace)?;
    if !verb.eq_ignore_ascii_case("bind") {
        return None;
    }
    let (key, args) = args.trim_start().strip_prefix('"')?.split_once('"')?;
    let (command, suffix) = args.trim_start().strip_prefix('"')?.split_once('"')?;
    let suffix = suffix.trim();
    (suffix.is_empty() || suffix.starts_with("//")).then_some((key, command))
}

fn merge(config: &[u8], replacements: &[(String, String)], managed: &[&str]) -> Vec<u8> {
    if replacements.is_empty() && managed.is_empty() {
        return config.to_vec();
    }
    let mut output = Vec::with_capacity(config.len() + 512);
    let (bom, config) = match config.strip_prefix(b"\xef\xbb\xbf") {
        Some(config) => (b"\xef\xbb\xbf".as_slice(), config),
        None => (b"".as_slice(), config),
    };
    output.extend_from_slice(bom);
    for line in config.split_inclusive(|&b| b == b'\n') {
        if !binding(line).is_some_and(|(key, command)| {
            managed.contains(&command)
                || replacements
                    .iter()
                    .any(|(replacement, _)| replacement.eq_ignore_ascii_case(key))
        }) {
            output.extend_from_slice(line);
        }
    }
    if output.len() > bom.len() && !output.ends_with(b"\n") {
        output.extend_from_slice(b"\r\n");
    }
    for (key, command) in replacements {
        output.extend_from_slice(format!("bind \"{key}\" \"{command}\"\r\n").as_bytes());
    }
    output
}

// Вызывается процессом Hub под блокировкой сеанса, когда игра закрыта.
pub fn flush(cfg: &Path) -> Result<(), String> {
    let Some((reset, pending)) = read_queue(cfg)? else {
        return Ok(());
    };
    let path = cfg.join("config.cfg");
    let config = match read(&path, MAX_CONFIG)? {
        Some(config) => config,
        None => defaults(cfg)?,
    };
    let reset_config;
    let base = if reset {
        reset_config = merge(
            &config,
            &default_bindings(&config, &defaults(cfg)?),
            COMMANDS,
        );
        &reset_config
    } else {
        &config
    };
    let managed: Vec<_> = pending.keys().map(String::as_str).collect();
    let replacements: Vec<_> = pending
        .iter()
        .filter(|(_, key)| !key.is_empty())
        .map(|(command, key)| (key.clone(), command.clone()))
        .collect();
    let updated = merge(base, &replacements, &managed);
    if updated != config {
        write(&path, &updated)?;
    }
    fs::remove_file(cfg.join(QUEUE)).map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queues_a_swap_and_clear_until_the_game_exits() {
        let directory = tempfile::tempdir().unwrap();
        let cfg = directory.path();
        let original = "unbindall\r\nbind \"A\" \"+moveleft\"\r\nbind \"D\" \"+moveright\"\r\n\
                        bind \"ENTER\" \"+attack\"\r\nbind \"MOUSE1\" \"+attack\"\r\n\
                        bind \"F11\" \"my_custom_action\"\r\nsensitivity \"2\"\r\n";
        let stock = cfg.join("config.cfg");
        fs::write(&stock, original).unwrap();
        queue(cfg, br#"{"+moveleft":"d","+moveright":"a","+attack":""}"#).unwrap();
        assert_eq!(pending(cfg).unwrap()["+moveleft"], "D");
        assert_eq!(fs::read_to_string(&stock).unwrap(), original);
        assert!(queue(cfg, br#"{"+jump":"SPACE\";exec\""}"#).is_err());
        assert!(queue(cfg, br#"{"connect":"A"}"#).is_err());
        flush(cfg).unwrap();
        assert!(pending(cfg).unwrap().is_empty());
        let actual = fs::read_to_string(&stock).unwrap();
        assert!(actual.contains("bind \"D\" \"+moveleft\"\r\n"));
        assert!(actual.contains("bind \"A\" \"+moveright\"\r\n"));
        assert!(actual.contains("bind \"F11\" \"my_custom_action\"\r\nsensitivity \"2\"\r\n"));
        assert!(!actual.contains("+attack"));
        flush(cfg).unwrap();
        assert_eq!(fs::read_to_string(&stock).unwrap(), actual);
    }

    #[test]
    fn reset_keeps_secondary_defaults_and_preserves_unmanaged_keys() {
        let directory = tempfile::tempdir().unwrap();
        let cfg = directory.path();
        let stock = cfg.join("config.cfg");
        fs::write(
            &stock,
            "unbindall\nbind \"W\" \"custom_forward\"\nbind \"L\" \"+forward\"\n",
        )
        .unwrap();
        fs::write(cfg.join("config_default.cfg"),
            "unbindall\nbind \"W\" \"+forward\"\nbind \"ENTER\" \"+attack\"\nbind \"MOUSE1\" \"+attack\"\n").unwrap();
        reset(cfg).unwrap();
        queue(cfg, br#"{"+jump":"SPACE"}"#).unwrap();
        assert_eq!(pending(cfg).unwrap()["+forward"], "");
        flush(cfg).unwrap();
        let actual = fs::read_to_string(&stock).unwrap();
        assert!(actual.contains("bind \"W\" \"custom_forward\"\n"));
        assert!(actual.contains("bind \"ENTER\" \"+attack\"\r\n"));
        assert!(actual.contains("bind \"MOUSE1\" \"+attack\"\r\n"));
        assert!(actual.contains("bind \"SPACE\" \"+jump\"\r\n"));
        assert!(!actual.contains("+forward"));
    }
}
