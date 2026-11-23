//! Общая настройка лаунчера и игрового меню; отдельная от серверного профиля.
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

pub const KEY: &str = "shinriIdentity";

fn path() -> Result<PathBuf, String> {
    let root = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .filter(|root| root.is_absolute())
        .ok_or("Не найдена папка LOCALAPPDATA.")?;
    Ok(root.join("PanopticonHub/game-data/shinri-identity.txt"))
}

pub fn ordinary(path: &Path) -> Result<(), String> {
    for parent in path.ancestors() {
        match fs::symlink_metadata(parent) {
            Ok(meta) => {
                #[cfg(windows)]
                let link = {
                    use std::os::windows::fs::MetadataExt;
                    meta.file_attributes() & 0x400 != 0
                };
                #[cfg(not(windows))]
                let link = meta.file_type().is_symlink();
                if link {
                    return Err("Путь идентификатора содержит ссылку.".into());
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.to_string()),
        }
    }
    Ok(())
}

fn read_at(path: &Path) -> Result<bool, String> {
    ordinary(path)?;
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(true),
        Err(error) => return Err(error.to_string()),
    };
    let mut bytes = Vec::new();
    file.take(4)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    match bytes.as_slice() {
        b"1\n" => Ok(true),
        b"0\n" => Ok(false),
        _ => Err("Повреждена настройка идентификатора Shinri Hub.".into()),
    }
}

pub fn enabled() -> Result<bool, String> {
    read_at(&path()?)
}

pub fn set_enabled(enabled: bool) -> Result<(), String> {
    let path = path()?;
    ordinary(&path)?;
    let parent = path.parent().ok_or("Не найдена папка настройки.")?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    ordinary(&path)?;
    let mut temporary =
        tempfile::NamedTempFile::new_in(parent).map_err(|error| error.to_string())?;
    let file = temporary.as_file_mut();
    file.write_all(if enabled { b"1\n" } else { b"0\n" })
        .map_err(|error| error.to_string())?;
    file.sync_all().map_err(|error| error.to_string())?;
    ordinary(&path)?;
    temporary
        .persist(&path)
        .map_err(|error| error.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_preference_defaults_on_but_explicit_off_and_invalid_data_are_preserved() {
        let root = std::env::temp_dir().join(format!("hub-identity-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let path = root.join("mode.txt");
        assert!(read_at(&path).unwrap());
        for bytes in [b"true".as_slice(), b"1\nxx", b"", b"1"] {
            fs::write(&path, bytes).unwrap();
            assert!(read_at(&path).is_err());
        }
        fs::write(&path, b"1\n").unwrap();
        assert!(read_at(&path).unwrap());
        fs::write(&path, b"0\n").unwrap();
        assert!(!read_at(&path).unwrap());
        fs::remove_file(path).unwrap();
        fs::remove_dir(root).unwrap();
    }
}
