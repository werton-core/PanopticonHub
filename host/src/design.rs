use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, Metadata, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::Mutex,
    time::SystemTime,
};
use tempfile::NamedTempFile;

pub const DESIGN_PRESET_EXTENSION: &str = "pthubtheme";
const MANIFEST_LIMIT: usize = 16 * 1024;
const MIB: u64 = 1024 * 1024;
const PRESET_LIMIT: u64 = 256 * MIB;
const STORE_LIMIT: u64 = 768 * MIB;
const STORE_FILE_LIMIT: usize = 256;
const PRESET_MAGIC: &[u8] = b"PTHUBTHEME\x01";
const BACKGROUND_SLOTS: &[&str] = &[
    "hubBackground",
    "gameMenuBackground",
    "gameBrowserBackground",
    "gameLoadingBackground",
];
const STYLE_BACKGROUNDS: &[&str] = &["hubBackground", "gameMenuBackground"];
const PANOPTICON_ACCENT: &str = "#f0504f";

const LAYOUT_IDS: &[&str] = &[
    "play",
    "settings",
    "logo",
    "social",
    "onlinePanopticon",
    "onlineShinri",
    "promo",
    "minimize",
    "close",
];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Profile {
    pub version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builtin_style: Option<BuiltinStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_name: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub assets: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_loading_volume: Option<u8>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub background_filters: BTreeMap<String, BackgroundFilter>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub layout: BTreeMap<String, Layout>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub decorations: Vec<Decoration>,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            version: 1,
            accent: None,
            builtin_style: None,
            hub_name: None,
            game_name: None,
            assets: BTreeMap::new(),
            game_loading_volume: None,
            background_filters: BTreeMap::new(),
            layout: BTreeMap::new(),
            decorations: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum BuiltinStyle {
    Panopticon,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BackgroundFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    hue: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saturation: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    brightness: Option<i16>,
}

impl BackgroundFilter {
    fn normalize(&mut self) -> Result<(), String> {
        for (field, default, range) in [
            (&mut self.hue, 0, -180..=180),
            (&mut self.saturation, 100, 0..=200),
            (&mut self.brightness, 100, 0..=200),
        ] {
            if field.is_some_and(|value| !range.contains(&value)) {
                return Err("Значение цветокоррекции фона вне допустимого диапазона.".into());
            }
            if *field == Some(default) {
                *field = None;
            }
        }
        Ok(())
    }

    fn is_default(&self) -> bool {
        self.hue.is_none() && self.saturation.is_none() && self.brightness.is_none()
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Layout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Decoration {
    pub asset: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
}

#[derive(Clone, Copy)]
enum Slot {
    HubBackground,
    GameBackground,
    Logo,
    Icon,
    Decoration,
    Music,
}

impl Slot {
    fn parse(name: &str) -> Option<Self> {
        match name {
            "hubBackground" => Some(Self::HubBackground),
            "gameMenuBackground" | "gameBrowserBackground" | "gameLoadingBackground" => {
                Some(Self::GameBackground)
            }
            "hubLogo" | "gameLogo" => Some(Self::Logo),
            "hubIcon" | "gameIcon" => Some(Self::Icon),
            "hubMusic" | "gameMenuMusic" | "gameLoadingMusic" => Some(Self::Music),
            "decoration" => Some(Self::Decoration),
            _ => None,
        }
    }

    fn accepts(self, kind: Kind) -> bool {
        match self {
            Self::HubBackground | Self::Decoration => {
                kind.image() || matches!(kind, Kind::Webm | Kind::Mp4)
            }
            Self::GameBackground => kind.image() || kind == Kind::Webm,
            Self::Logo => kind.image(),
            Self::Icon => kind == Kind::Ico,
            Self::Music => matches!(kind, Kind::Mp3 | Kind::Wav | Kind::Ogg),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Png,
    Jpg,
    Webp,
    Gif,
    Webm,
    Mp4,
    Ico,
    Mp3,
    Wav,
    Ogg,
}

impl Kind {
    fn parse(extension: &str) -> Option<Self> {
        match extension {
            "png" => Some(Self::Png),
            "jpg" => Some(Self::Jpg),
            "webp" => Some(Self::Webp),
            "gif" => Some(Self::Gif),
            "webm" => Some(Self::Webm),
            "mp4" => Some(Self::Mp4),
            "ico" => Some(Self::Ico),
            "mp3" => Some(Self::Mp3),
            "wav" => Some(Self::Wav),
            "ogg" => Some(Self::Ogg),
            _ => None,
        }
    }

    fn extension(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpg => "jpg",
            Self::Webp => "webp",
            Self::Gif => "gif",
            Self::Webm => "webm",
            Self::Mp4 => "mp4",
            Self::Ico => "ico",
            Self::Mp3 => "mp3",
            Self::Wav => "wav",
            Self::Ogg => "ogg",
        }
    }

    fn image(self) -> bool {
        matches!(self, Self::Png | Self::Jpg | Self::Webp | Self::Gif)
    }

    fn limit(self) -> u64 {
        match self {
            Self::Ico => MIB,
            Self::Webm | Self::Mp4 => 64 * MIB,
            Self::Mp3 | Self::Wav | Self::Ogg => 32 * MIB,
            _ => 20 * MIB,
        }
    }
}

fn key_kind(key: &str) -> Option<Kind> {
    let (hash, extension) = key.split_once('.')?;
    if hash.len() != 64
        || !hash
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return None;
    }
    Kind::parse(extension)
}

pub fn valid_media_key(key: &str) -> bool {
    key_kind(key).is_some()
}

impl Profile {
    fn apply_builtin_style(&mut self, style: Profile) {
        self.accent = style.accent;
        self.builtin_style = style.builtin_style;
        for slot in STYLE_BACKGROUNDS {
            self.background_filters.remove(*slot);
        }
        self.background_filters.extend(style.background_filters);
    }

    fn refresh_legacy_panopticon(&mut self) {
        let is_original_filter = |slot: &str, saturation, brightness| {
            self.background_filters.get(slot) == Some(&BackgroundFilter {
                hue: Some(-32),
                saturation: Some(saturation),
                brightness: Some(brightness),
            })
        };
        if self.builtin_style.is_none()
            && self.accent.as_deref() == Some("#dc5964")
            && STYLE_BACKGROUNDS.iter().all(|slot| !self.assets.contains_key(*slot))
            && is_original_filter("hubBackground", 108, 98)
            && is_original_filter("gameMenuBackground", 104, 95)
        {
            self.apply_builtin_style(panopticon_style());
        }
    }

    fn normalize(mut self) -> Result<Self, String> {
        if self.version != 1 {
            return Err("Версия оформления не поддерживается.".into());
        }
        if let Some(accent) = &mut self.accent {
            if accent.len() != 7
                || !accent.starts_with('#')
                || !accent.as_bytes()[1..].iter().all(u8::is_ascii_hexdigit)
            {
                return Err("Цвет оформления должен быть задан в формате #rrggbb.".into());
            }
            accent.make_ascii_lowercase();
        }
        for name in [&mut self.hub_name, &mut self.game_name] {
            if let Some(value) = name.take() {
                let value = value.trim();
                if value.encode_utf16().count() > 64 || value.chars().any(char::is_control) {
                    return Err(
                        "Имя оформления должно содержать до 64 символов без управляющих знаков."
                            .into(),
                    );
                }
                if !value.is_empty() {
                    *name = Some(value.to_owned());
                }
            }
        }
        for (key, item) in &mut self.layout {
            if !LAYOUT_IDS.contains(&key.as_str())
                || [item.x, item.y]
                    .into_iter()
                    .flatten()
                    .any(|v| !(0.0..=100.0).contains(&v))
            {
                return Err("Недопустимое положение элемента оформления.".into());
            }
            if matches!(key.as_str(), "play" | "settings") {
                item.hidden = None;
            }
        }
        self.layout
            .retain(|_, item| item.x.is_some() || item.y.is_some() || item.hidden.is_some());
        for (slot, key) in &self.assets {
            let slot = Slot::parse(slot)
                .filter(|slot| !matches!(slot, Slot::Decoration))
                .ok_or("Неизвестное назначение файла оформления.")?;
            Self::validate_key(slot, key)?;
        }
        if self.game_loading_volume.is_some_and(|volume| volume > 100) {
            return Err("Громкость музыки загрузки должна быть от 0 до 100%.".into());
        }
        if self.game_loading_volume == Some(30) || !self.assets.contains_key("gameLoadingMusic") {
            self.game_loading_volume = None;
        }
        for (slot, filter) in &mut self.background_filters {
            if !BACKGROUND_SLOTS.contains(&slot.as_str()) {
                return Err("Неизвестный фон для цветокоррекции.".into());
            }
            filter.normalize()?;
        }
        self.background_filters
            .retain(|_, filter| !filter.is_default());
        if self.decorations.len() > 8 {
            return Err("В оформлении может быть до восьми украшений.".into());
        }
        for item in &self.decorations {
            Self::validate_key(Slot::Decoration, &item.asset)?;
            if !(0.0..=100.0).contains(&item.x)
                || !(0.0..=100.0).contains(&item.y)
                || !(5.0..=90.0).contains(&item.width)
            {
                return Err("Недопустимые размеры или положение украшения.".into());
            }
        }
        Ok(self)
    }

    fn validate_key(slot: Slot, key: &str) -> Result<(), String> {
        if key_kind(key).is_some_and(|kind| slot.accepts(kind)) {
            Ok(())
        } else {
            Err("Формат файла не подходит для выбранного элемента оформления.".into())
        }
    }

    fn keys(&self) -> BTreeSet<&str> {
        self.assets
            .values()
            .map(String::as_str)
            .chain(self.decorations.iter().map(|item| item.asset.as_str()))
            .collect()
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    profile: Profile,
    media: Vec<MediaEntry>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct MediaEntry {
    key: String,
    size: u64,
}

#[derive(PartialEq, Eq)]
struct Stamp {
    length: u64,
    created: SystemTime,
    modified: SystemTime,
}

impl Stamp {
    fn from_file(meta: &Metadata) -> Option<Self> {
        Some(Self {
            length: meta.len(),
            created: meta.created().ok()?,
            modified: meta.modified().ok()?,
        })
    }
}

struct Store {
    root: PathBuf,
    assets: PathBuf,
}

struct Cache {
    verified: BTreeMap<String, Stamp>,
    staged: BTreeSet<String>,
}

static CACHE: Mutex<Cache> = Mutex::new(Cache {
    verified: BTreeMap::new(),
    staged: BTreeSet::new(),
});

fn with_store<T>(op: impl FnOnce(&Store, &mut Cache) -> Result<T, String>) -> Result<T, String> {
    let local = std::env::var_os("LOCALAPPDATA").ok_or("Папка LOCALAPPDATA недоступна.")?;
    let store = Store::open(Path::new(&local)).map_err(io_error)?;
    let mut cache = CACHE
        .lock()
        .map_err(|_| "Хранилище оформления недоступно.")?;
    // Игровой процесс должен открыть весь снимок до смены профиля и очистки медиа.
    let _lock = store.lock().map_err(io_error)?;
    op(&store, &mut cache)
}

pub fn load() -> Result<Profile, String> {
    with_store(|store, cache| store.load(cache))
}

#[derive(Default)]
pub struct GameMedia(BTreeMap<String, File>);

impl GameMedia {
    pub fn take(&mut self, key: &str) -> Result<File, String> {
        self.0.remove(key).ok_or_else(|| "Не найден файл оформления игры.".into())
    }
}

pub fn load_for_game() -> Result<(Profile, GameMedia), String> {
    with_store(Store::load_with_media)
}

pub fn save(profile: Profile) -> Result<Profile, String> {
    with_store(|store, cache| store.save(profile, cache))
}

pub fn update_draft(keys: Vec<String>) -> Result<(), String> {
    if !keys.iter().all(|key| valid_media_key(key)) {
        return Err("Некорректные файлы черновика оформления.".into());
    }
    with_store(|store, cache| store.update_draft(keys.into_iter().collect(), cache))
}

pub fn builtin_styles() -> serde_json::Value {
    serde_json::json!({
        "panopticon": panopticon_style(),
        "shinri": Profile::default(),
    })
}

fn builtin_style(id: &str) -> Option<Profile> {
    match id {
        "shinri" => Some(Profile::default()),
        "panopticon" => Some(panopticon_style()),
        _ => None,
    }
}

fn panopticon_style() -> Profile {
    Profile {
        accent: Some(PANOPTICON_ACCENT.into()),
        builtin_style: Some(BuiltinStyle::Panopticon),
        ..Profile::default()
    }
}

pub fn valid_installation_id(id: &str) -> bool {
    id.len() == 32 && id.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

pub fn needs_initial_style(id: &str) -> Result<bool, String> {
    with_store(|store, _| store.style_marker(id).map(|(_, pending)| pending))
}

pub fn select_initial_style(installation: &str, id: &str) -> Result<Profile, String> {
    let style = builtin_style(id).ok_or("Неизвестная тема оформления.")?;
    with_store(|store, cache| store.select_initial_style(installation, style, cache))
}

pub fn pick_asset(slot: &str, path: &Path) -> Result<String, String> {
    with_store(|store, cache| store.pick(slot, path, cache))
}

pub fn import(path: &Path) -> Result<Profile, String> {
    with_store(|store, cache| store.import(path, cache))
}

pub fn export(profile: Profile, path: &Path) -> Result<(), String> {
    with_store(|store, cache| store.export(profile, path, cache))
}

pub fn media(key: &str) -> Result<PathBuf, String> {
    with_store(|store, cache| store.verified(key, cache).map(|(path, _)| path))
}

pub fn open_media(key: &str) -> Result<File, String> {
    with_store(|store, cache| store.verified(key, cache).map(|(_, file)| file))
}

impl Store {
    fn open(local: &Path) -> io::Result<Self> {
        let local = local.canonicalize()?;
        let app = local.join("PanopticonHub");
        let root = app.join("design");
        let assets = root.join("assets");
        for directory in [&app, &root, &assets] {
            ensure_directory(directory)?;
        }
        Ok(Self { root, assets })
    }

    fn lock(&self) -> io::Result<File> {
        let path = self.root.join(".store-lock");
        check_replace_target(&path)?;
        let mut options = OpenOptions::new();
        options.read(true).write(true).create(true);
        #[cfg(windows)]
        {
            use std::os::windows::fs::OpenOptionsExt;
            use windows_sys::Win32::Storage::FileSystem::{
                FILE_FLAG_OPEN_REPARSE_POINT, FILE_SHARE_READ, FILE_SHARE_WRITE,
            };
            options.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
                .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE);
        }
        let lock = options.open(path)?;
        let meta = lock.metadata()?;
        if !meta.is_file() || is_link(&meta) {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Нужен обычный файл блокировки оформления."));
        }
        lock.lock()?;
        Ok(lock)
    }

    fn style_marker(&self, id: &str) -> Result<(PathBuf, bool), String> {
        if !valid_installation_id(id) {
            return Err("Некорректный идентификатор установки.".into());
        }
        let directory = self.root.join("initial-style");
        ensure_directory(&directory).map_err(io_error)?;
        let path = directory.join(id);
        match regular_metadata(&path) {
            Ok(_) => Ok((path, false)),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok((path, true)),
            Err(error) => Err(io_error(error)),
        }
    }

    fn select_initial_style(&self, id: &str, style: Profile, cache: &mut Cache) -> Result<Profile, String> {
        let (marker, pending) = self.style_marker(id)?;
        let mut active = self.load(cache)?;
        if !pending {
            return Ok(active);
        }
        let mut completed = NamedTempFile::new_in(marker.parent().unwrap()).map_err(io_error)?;
        completed.write_all(b"1").map_err(io_error)?;
        active.apply_builtin_style(style);
        let saved = self.write_profile(active, cache)?;
        persist(completed, &marker)?;
        Ok(saved)
    }

    fn active(&self) -> Result<Profile, String> {
        let path = self.root.join("profile.json");
        let file = match open_regular(&path, false) {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Profile::default()),
            Err(error) => return Err(io_error(error)),
        };
        let mut bytes = Vec::new();
        file.take(MANIFEST_LIMIT as u64 + 1)
            .read_to_end(&mut bytes)
            .map_err(io_error)?;
        if bytes.len() > MANIFEST_LIMIT {
            return Err("Файл активного оформления слишком велик.".into());
        }
        serde_json::from_slice::<Profile>(&bytes)
            .map_err(|error| format!("Повреждён файл активного оформления. {error}"))?
            .normalize()
    }

    fn load(&self, cache: &mut Cache) -> Result<Profile, String> {
        self.load_with_media(cache).map(|(profile, _)| profile)
    }

    fn load_with_media(&self, cache: &mut Cache) -> Result<(Profile, GameMedia), String> {
        let mut profile = self.active()?;
        profile.refresh_legacy_panopticon();
        let mut files = BTreeMap::new();
        for key in profile.keys() {
            if let Ok((_, file)) = self.verified(key, cache) {
                files.insert(key.to_owned(), file);
            }
        }
        profile.assets.retain(|_, key| files.contains_key(key));
        if !profile.assets.contains_key("gameLoadingMusic") {
            profile.game_loading_volume = None;
        }
        profile.decorations.retain(|item| files.contains_key(&item.asset));
        Ok((profile, GameMedia(files)))
    }

    fn save(&self, profile: Profile, cache: &mut Cache) -> Result<Profile, String> {
        let profile = self.write_profile(profile, cache)?;
        let _ = self.update_draft(BTreeSet::new(), cache);
        Ok(profile)
    }

    fn write_profile(&self, profile: Profile, cache: &mut Cache) -> Result<Profile, String> {
        let profile = profile.normalize()?;
        self.collect_media(&profile, cache)?;
        let path = self.root.join("profile.json");
        check_replace_target(&path).map_err(io_error)?;
        let mut output = NamedTempFile::new_in(&self.root).map_err(io_error)?;
        serde_json::to_writer_pretty(&mut output, &profile).map_err(|error| error.to_string())?;
        persist(output, &path)?;
        Ok(profile)
    }

    fn update_draft(&self, keys: BTreeSet<String>, cache: &mut Cache) -> Result<(), String> {
        cache.staged = keys;
        self.remove_unused(cache)
    }

    fn remove_unused(&self, cache: &mut Cache) -> Result<(), String> {
        let active = self.active()?;
        let live = active.keys();
        for key in self.entries()?.keys() {
            if !live.contains(key.as_str()) && !cache.staged.contains(key) {
                self.remove_asset(key, cache);
            }
        }
        Ok(())
    }

    fn remove_asset(&self, key: &str, cache: &mut Cache) -> bool {
        if fs::remove_file(self.assets.join(key)).is_err() {
            return false;
        }
        cache.verified.remove(key);
        true
    }

    fn pick(&self, slot: &str, path: &Path, cache: &mut Cache) -> Result<String, String> {
        let slot = Slot::parse(slot).ok_or("Неизвестное назначение файла оформления.")?;
        let mut extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        if extension == "jpeg" {
            extension = "jpg".into();
        }
        let kind = Kind::parse(&extension)
            .filter(|kind| slot.accepts(*kind))
            .ok_or("Формат файла не подходит для выбранного элемента оформления.")?;
        let file = open_regular(path, false).map_err(io_error)?;
        let size = file.metadata().map_err(io_error)?.len();
        let staged = Staged::read(file, kind, size, &self.assets)?;
        let key = staged.entry.key.clone();
        self.install(vec![staged], cache)?;
        Ok(key)
    }

    fn import(&self, path: &Path, cache: &mut Cache) -> Result<Profile, String> {
        let mut file = open_regular(path, false).map_err(io_error)?;
        let file_length = file.metadata().map_err(io_error)?.len();
        let mut header = [0; PRESET_MAGIC.len() + 4];
        file.read_exact(&mut header).map_err(invalid_preset)?;
        if header[..PRESET_MAGIC.len()] != *PRESET_MAGIC {
            return Err("Формат файла оформления не поддерживается.".into());
        }
        let length = u32::from_le_bytes(header[PRESET_MAGIC.len()..].try_into().unwrap()) as usize;
        if length == 0 || length > MANIFEST_LIMIT {
            return Err("Некорректный размер описания оформления.".into());
        }
        let mut bytes = vec![0; length];
        file.read_exact(&mut bytes).map_err(invalid_preset)?;
        let manifest: Manifest = serde_json::from_slice(&bytes).map_err(invalid_preset)?;
        let profile = manifest.profile.normalize()?;
        let mut expected = profile.keys();
        if manifest.media.len() != expected.len() {
            return Err("Список вложений не соответствует оформлению.".into());
        }
        let mut total = 0;
        for entry in &manifest.media {
            if !expected.remove(entry.key.as_str()) {
                return Err("Недопустимое вложение в оформлении.".into());
            }
            let kind = key_kind(&entry.key).unwrap();
            check_size(kind, entry.size)?;
            total += entry.size;
        }
        if total > PRESET_LIMIT || total + (header.len() + length) as u64 != file_length {
            return Err("Некорректный размер файла оформления.".into());
        }
        let mut staged = Vec::with_capacity(manifest.media.len());
        for entry in manifest.media {
            let kind = key_kind(&entry.key).unwrap();
            let item = Staged::read((&mut file).take(entry.size), kind, entry.size, &self.assets)?;
            if item.entry.key != entry.key {
                return Err("Контрольная сумма вложения не совпадает.".into());
            }
            staged.push(item);
        }
        self.install(staged, cache)?;
        Ok(profile)
    }

    fn export(&self, profile: Profile, path: &Path, cache: &mut Cache) -> Result<(), String> {
        if !path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case(DESIGN_PRESET_EXTENSION))
        {
            return Err("Для файла оформления нужно расширение .pthubtheme.".into());
        }
        let profile = profile.normalize()?;
        let (media, files) = self.collect_media(&profile, cache)?;
        let manifest = Manifest { profile, media };
        let bytes = serde_json::to_vec(&manifest).map_err(invalid_preset)?;
        if bytes.len() > MANIFEST_LIMIT {
            return Err("Описание оформления слишком велико.".into());
        }
        let parent = path
            .parent()
            .ok_or("Не указана папка для сохранения оформления.")?;
        let parent = parent.canonicalize().map_err(io_error)?;
        let path = parent.join(path.file_name().ok_or("Не указано имя файла оформления.")?);
        check_replace_target(&path).map_err(io_error)?;
        let mut output = NamedTempFile::new_in(parent).map_err(io_error)?;
        output.write_all(PRESET_MAGIC).map_err(io_error)?;
        output
            .write_all(&(bytes.len() as u32).to_le_bytes())
            .map_err(io_error)?;
        output.write_all(&bytes).map_err(io_error)?;
        for mut file in files {
            io::copy(&mut file, &mut output).map_err(io_error)?;
        }
        persist(output, &path)
    }

    fn collect_media(
        &self,
        profile: &Profile,
        cache: &mut Cache,
    ) -> Result<(Vec<MediaEntry>, Vec<File>), String> {
        let mut entries = Vec::new();
        let mut files = Vec::new();
        let mut total = 0;
        for key in profile.keys() {
            let (_, file) = self.verified(key, cache)?;
            let size = file.metadata().map_err(io_error)?.len();
            entries.push(MediaEntry {
                key: key.into(),
                size,
            });
            files.push(file);
            total += size;
        }
        if total > PRESET_LIMIT {
            return Err("Медиа в оформлении занимают больше 256 МиБ.".into());
        }
        Ok((entries, files))
    }

    fn verified(&self, key: &str, cache: &mut Cache) -> Result<(PathBuf, File), String> {
        let kind = key_kind(key).ok_or("Некорректный идентификатор файла оформления.")?;
        let path = self.assets.join(key);
        let mut file = open_regular(&path, true).map_err(io_error)?;
        let meta = file.metadata().map_err(io_error)?;
        check_size(kind, meta.len())?;
        let stamp = Stamp::from_file(&meta);
        if stamp.as_ref().is_none() || stamp.as_ref() != cache.verified.get(key) {
            validate_media(&mut file, kind, meta.len())?;
            let mut hasher = Sha256::new();
            io::copy(&mut file, &mut hasher).map_err(io_error)?;
            let actual = format!("{:x}.{}", hasher.finalize(), kind.extension());
            if key != actual {
                return Err("Контрольная сумма файла оформления не совпадает.".into());
            }
            file.rewind().map_err(io_error)?;
            if let Some(stamp) = stamp {
                if cache.verified.len() >= STORE_FILE_LIMIT {
                    cache.verified.clear();
                }
                cache.verified.insert(key.into(), stamp);
            }
        }
        Ok((path, file))
    }

    fn install(&self, staged: Vec<Staged>, cache: &mut Cache) -> Result<(), String> {
        if staged.is_empty() {
            return Ok(());
        }
        let incoming: BTreeSet<String> = staged.iter().map(|item| item.entry.key.clone()).collect();
        let entries = self.entries()?;
        self.make_space(&staged, &incoming, &entries, cache)?;
        let mut installed = Vec::new();
        let result = (|| {
            for item in staged {
                let path = self.assets.join(&item.entry.key);
                if self.verified(&item.entry.key, cache).is_err() {
                    check_replace_target(&path).map_err(io_error)?;
                    cache.verified.remove(&item.entry.key);
                    persist(item.temporary, &path)?;
                    if !entries.contains_key(&item.entry.key) {
                        installed.push(path);
                    }
                }
            }
            Ok(())
        })();
        if result.is_err() {
            for path in installed {
                let _ = fs::remove_file(path);
            }
        } else {
            cache.staged.extend(incoming);
        }
        result
    }

    fn entries(&self) -> Result<BTreeMap<String, Metadata>, String> {
        let mut entries = BTreeMap::new();
        for item in fs::read_dir(&self.assets).map_err(io_error)? {
            let item = item.map_err(io_error)?;
            if let Some(name) = item
                .file_name()
                .to_str()
                .filter(|name| valid_media_key(name))
            {
                if let Ok(meta) = regular_metadata(&item.path()) {
                    entries.insert(name.to_owned(), meta);
                }
            }
        }
        Ok(entries)
    }

    fn make_space(
        &self,
        staged: &[Staged],
        incoming: &BTreeSet<String>,
        entries: &BTreeMap<String, Metadata>,
        cache: &mut Cache,
    ) -> Result<(), String> {
        let retained = entries
            .iter()
            .filter(|(key, _)| !incoming.contains(key.as_str()));
        let mut count = retained.clone().count() + staged.len();
        let mut total = retained.map(|(_, meta)| meta.len()).sum::<u64>()
            + staged.iter().map(|item| item.entry.size).sum::<u64>();
        if count <= STORE_FILE_LIMIT && total <= STORE_LIMIT {
            return Ok(());
        }
        let active = self.active()?;
        let live = active.keys();
        let mut removable: Vec<_> = entries
            .iter()
            .filter(|(key, _)| {
                !live.contains(key.as_str())
                    && !incoming.contains(key.as_str())
                    && !cache.staged.contains(*key)
            })
            .collect();
        removable.sort_by_key(|(_, meta)| meta.modified().ok());
        for (key, meta) in removable {
            if self.remove_asset(key, cache) {
                total -= meta.len();
                count -= 1;
            }
            if count <= STORE_FILE_LIMIT && total <= STORE_LIMIT {
                return Ok(());
            }
        }
        Err("Хранилище оформления заполнено. Уменьшите размер медиа в теме или черновике.".into())
    }
}

struct Staged {
    entry: MediaEntry,
    temporary: NamedTempFile,
}

impl Staged {
    fn read(mut input: impl Read, kind: Kind, size: u64, folder: &Path) -> Result<Self, String> {
        check_size(kind, size)?;
        let mut temporary = tempfile::Builder::new()
            .prefix(".design-")
            .suffix(&format!(".{}", kind.extension()))
            .tempfile_in(folder)
            .map_err(io_error)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0; 64 * 1024];
        let mut remaining = size;
        while remaining != 0 {
            let chunk = remaining.min(buffer.len() as u64) as usize;
            input.read_exact(&mut buffer[..chunk]).map_err(io_error)?;
            hasher.update(&buffer[..chunk]);
            temporary.write_all(&buffer[..chunk]).map_err(io_error)?;
            remaining -= chunk as u64;
        }

        validate_media(temporary.as_file_mut(), kind, size)?;
        let key = format!("{:x}.{}", hasher.finalize(), kind.extension());
        Ok(Self {
            entry: MediaEntry { key, size },
            temporary,
        })
    }
}

fn check_size(kind: Kind, size: u64) -> Result<(), String> {
    if size == 0 || size > kind.limit() {
        return Err(format!(
            "Размер файла .{} должен быть в пределах {} МиБ.",
            kind.extension(),
            kind.limit() / MIB
        ));
    }
    Ok(())
}

fn persist(file: NamedTempFile, path: &Path) -> Result<(), String> {
    file.as_file().sync_all().map_err(io_error)?;
    file.persist(path).map_err(|error| io_error(error.error))?;
    Ok(())
}

fn io_error(error: io::Error) -> String {
    format!("Ошибка чтения или записи оформления. {error}")
}

fn invalid_preset(error: impl std::fmt::Display) -> String {
    format!("Повреждён файл оформления. {error}")
}

fn ensure_directory(path: &Path) -> io::Result<()> {
    match fs::create_dir(path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
        Err(error) => return Err(error),
    }
    let meta = fs::symlink_metadata(path)?;
    if meta.is_dir() && !is_link(&meta) {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Папка оформления является ссылкой.",
        ))
    }
}

fn is_link(meta: &Metadata) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT;
        meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 || meta.file_type().is_symlink()
    }
    #[cfg(not(windows))]
    {
        meta.file_type().is_symlink()
    }
}

fn regular_metadata(path: &Path) -> io::Result<Metadata> {
    let meta = fs::symlink_metadata(path)?;
    if meta.is_file() && !is_link(&meta) {
        Ok(meta)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Нужен обычный файл оформления.",
        ))
    }
}

fn check_replace_target(path: &Path) -> io::Result<()> {
    match regular_metadata(path) {
        Ok(_) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

fn open_regular(path: &Path, _allow_delete: bool) -> io::Result<File> {
    regular_metadata(path)?;
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        use windows_sys::Win32::Storage::FileSystem::{
            FILE_FLAG_OPEN_REPARSE_POINT, FILE_SHARE_DELETE, FILE_SHARE_READ,
        };
        let sharing = if _allow_delete { FILE_SHARE_READ | FILE_SHARE_DELETE } else { FILE_SHARE_READ };
        options
            .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
            .share_mode(sharing);
    }
    let file = options.open(path)?;
    let meta = file.metadata()?;
    if meta.is_file() && !is_link(&meta) {
        Ok(file)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Нужен обычный файл оформления.",
        ))
    }
}

fn validate_media(file: &mut File, kind: Kind, size: u64) -> Result<(), String> {
    let result = match kind {
        Kind::Png => png(file, size),
        Kind::Jpg => jpeg(file, size),
        Kind::Webp => webp(file, size),
        Kind::Gif => gif(file, size),
        Kind::Ico => ico(file, size),
        Kind::Webm => webm(file, size),
        Kind::Mp4 => mp4(file, size),
        Kind::Mp3 => mp3(file, size),
        Kind::Wav => wav(file, size),
        Kind::Ogg => ogg(file, size),
    };
    result.map_err(|_| {
        format!(
            "Файл повреждён или не соответствует формату .{}.",
            kind.extension()
        )
    })?;
    file.rewind().map_err(io_error)
}

fn require(condition: bool) -> io::Result<()> {
    if condition {
        Ok(())
    } else {
        Err(io::ErrorKind::InvalidData.into())
    }
}

fn read_at<const N: usize>(file: &mut (impl Read + Seek), offset: u64) -> io::Result<[u8; N]> {
    let mut bytes = [0; N];
    file.seek(SeekFrom::Start(offset))?;
    file.read_exact(&mut bytes)?;
    Ok(bytes)
}

fn wav(file: &mut File, size: u64) -> io::Result<()> {
    let header = read_at::<12>(file, 0)?;
    let end = 8 + u64::from(u32::from_le_bytes(header[4..8].try_into().unwrap()));
    require(&header[..4] == b"RIFF" && &header[8..] == b"WAVE" && end <= size)?;
    let mut offset = 12;
    let mut alignment = None;
    let mut data = None;
    // Most files have two to six chunks. Bound work on a malformed file full of empty chunks.
    for _ in 0..1024 {
        require(offset + 8 <= end)?;
        let chunk = read_at::<8>(file, offset)?;
        let length = u64::from(u32::from_le_bytes(chunk[4..].try_into().unwrap()));
        let start = offset + 8;
        offset = start + length + (length % 2);
        require(offset <= end)?;
        match &chunk[..4] {
            b"fmt " => {
                require(length >= 16)?;
                let format = read_at::<16>(file, start)?;
                let mut codec = u16::from_le_bytes(format[..2].try_into().unwrap());
                if codec == 0xfffe {
                    require(length >= 40)?;
                    let extra = read_at::<24>(file, start + 16)?;
                    require(u16::from_le_bytes(extra[..2].try_into().unwrap()) >= 22
                        && extra[10..] == [0, 0, 0, 0, 0x10, 0, 0x80, 0, 0, 0xaa, 0, 0x38, 0x9b, 0x71])?;
                    codec = u16::from_le_bytes(extra[8..10].try_into().unwrap());
                }
                let channels = u16::from_le_bytes(format[2..4].try_into().unwrap());
                let rate = u32::from_le_bytes(format[4..8].try_into().unwrap());
                let block = u16::from_le_bytes(format[12..14].try_into().unwrap());
                let bits = u16::from_le_bytes(format[14..16].try_into().unwrap());
                require(matches!((codec, bits), (1, 8 | 16 | 24 | 32) | (3, 32))
                    && (1..=8).contains(&channels) && (8000..=192000).contains(&rate)
                    && block == channels * (bits / 8))?;
                alignment = Some(u64::from(block));
            }
            b"data" => data = Some(length),
            _ => {}
        }
        if let (Some(block), Some(length)) = (alignment, data) {
            return require(length >= block && length % block == 0);
        }
    }
    require(false)
}

fn ogg(file: &mut File, size: u64) -> io::Result<()> {
    let header = read_at::<27>(file, 0)?;
    let segments = u64::from(header[26]);
    require(&header[..5] == b"OggS\x00" && header[5] == 2 && segments != 0
        && size > 27 + segments)?;
    let first = u64::from(read_at::<1>(file, 27)?[0]);
    let packet = 27 + segments;
    require(first >= 19 && packet + first <= size)?;
    let prefix = read_at::<19>(file, packet)?;
    if prefix.starts_with(b"OpusHead") {
        return require(prefix[8] == 1 && prefix[9] > 0);
    }
    require(first >= 30 && prefix.starts_with(b"\x01vorbis\x00\x00\x00\x00")
        && prefix[11] > 0 && prefix[12..16] != [0; 4])
}

fn mp3_frame(file: &mut File, offset: u64) -> io::Result<(u64, u8, u8)> {
    let header = read_at::<4>(file, offset)?;
    let version = (header[1] >> 3) & 3;
    let rate_index = (header[2] >> 2) & 3;
    let bitrate_index = (header[2] >> 4) as usize;
    require(header[0] == 0xff && header[1] & 0xe6 == 0xe2 && version != 1
        && rate_index != 3 && (1..15).contains(&bitrate_index))?;
    let divisor = match version { 3 => 1, 2 => 2, _ => 4 };
    let sample_rate = [44100, 48000, 32000][usize::from(rate_index)] / divisor;
    let bitrates = if version == 3 {
        [0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320]
    } else {
        [0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160]
    };
    let coefficient = if version == 3 { 144_000 } else { 72_000 };
    let length = coefficient * bitrates[bitrate_index] / sample_rate + u64::from((header[2] >> 1) & 1);
    Ok((length, version, rate_index))
}

fn mp3(file: &mut File, size: u64) -> io::Result<()> {
    let header = read_at::<10>(file, 0)?;
    let start = if header.starts_with(b"ID3") {
        require((2..=4).contains(&header[3]) && header[6..].iter().all(|byte| *byte < 128))?;
        let tag_size = header[6..].iter().fold(0_u64, |length, byte| (length << 7) | u64::from(*byte));
        10 + tag_size + if header[3] == 4 && header[5] & 0x10 != 0 { 10 } else { 0 }
    } else {
        0
    };
    require(start + 4 <= size)?;
    let (length, version, rate) = mp3_frame(file, start)?;
    require(start + length + 4 <= size)?;
    let (next, next_version, next_rate) = mp3_frame(file, start + length)?;
    require(version == next_version && rate == next_rate && start + length + next <= size)
}

fn image_dimensions(width: u32, height: u32) -> io::Result<()> {
    require(
        (1..=8192).contains(&width)
            && (1..=8192).contains(&height)
            && u64::from(width) * u64::from(height) <= 32 * MIB,
    )
}

fn png_dimensions(data: &[u8; 24]) -> io::Result<(u32, u32)> {
    require(data[..16] == *b"\x89PNG\r\n\x1a\n\0\0\0\x0dIHDR")?;
    let width = u32::from_be_bytes(data[16..20].try_into().unwrap());
    let height = u32::from_be_bytes(data[20..24].try_into().unwrap());
    image_dimensions(width, height)?;
    Ok((width, height))
}

fn png(file: &mut File, size: u64) -> io::Result<()> {
    require(size >= 57)?;
    png_dimensions(&read_at(file, 0)?)?;
    require(read_at::<12>(file, size - 12)? == *b"\0\0\0\0IEND\xaeB\x60\x82")?;
    let mut offset = 33;
    for _ in 0..4096 {
        require(offset + 12 < size)?;
        let header = read_at::<8>(file, offset)?;
        let length = u64::from(u32::from_be_bytes(header[..4].try_into().unwrap()));
        offset += length + 12;
        require(offset <= size - 12)?;
        if header[4..] == *b"IDAT" {
            return Ok(());
        }
    }
    require(false)
}

fn jpeg(file: &mut File, size: u64) -> io::Result<()> {
    require(size >= 16)?;
    require(read_at::<2>(file, 0)? == [0xff, 0xd8])?;
    require(read_at::<2>(file, size - 2)? == [0xff, 0xd9])?;
    file.seek(SeekFrom::Start(2))?;
    let mut marker_padding = 0;
    for _ in 0..1024 {
        let mut marker = [0; 2];
        file.read_exact(&mut marker)?;
        require(marker[0] == 0xff)?;
        while marker[1] == 0xff {
            marker_padding += 1;
            require(marker_padding <= 4096)?;
            file.read_exact(&mut marker[1..])?;
        }
        require(!matches!(marker[1], 0 | 0xd8..=0xda))?;
        let mut bytes = [0; 2];
        file.read_exact(&mut bytes)?;
        let length = u64::from(u16::from_be_bytes(bytes));
        let offset = file.stream_position()?;
        require(length >= 2 && offset + length - 2 <= size - 2)?;
        if matches!(marker[1], 0xc0..=0xc3 | 0xc5..=0xc7 | 0xc9..=0xcb | 0xcd..=0xcf) {
            require(length >= 8)?;
            let data = read_at::<5>(file, offset)?;
            let height = u16::from_be_bytes([data[1], data[2]]);
            let width = u16::from_be_bytes([data[3], data[4]]);
            return image_dimensions(width.into(), height.into());
        }
        file.seek(SeekFrom::Start(offset + length - 2))?;
    }
    require(false)
}

fn webp(file: &mut File, size: u64) -> io::Result<()> {
    let header = read_at::<30>(file, 0)?;
    let riff_size = u64::from(u32::from_le_bytes(header[4..8].try_into().unwrap()));
    let chunk_size = u64::from(u32::from_le_bytes(header[16..20].try_into().unwrap()));
    require(
        header[..4] == *b"RIFF"
            && header[8..12] == *b"WEBP"
            && riff_size + 8 == size
            && chunk_size + chunk_size % 2 + 20 <= size,
    )?;
    let data = &header[20..];
    match &header[12..16] {
        b"VP8 " => {
            require(chunk_size >= 10 && data[0] & 1 == 0 && data[3..6] == [0x9d, 0x01, 0x2a])?;
            let width = u16::from_le_bytes([data[6], data[7]]) & 0x3fff;
            let height = u16::from_le_bytes([data[8], data[9]]) & 0x3fff;
            image_dimensions(width.into(), height.into())
        }
        b"VP8L" => {
            require(chunk_size >= 5 && data[0] == 0x2f)?;
            let bits = u32::from_le_bytes(data[1..5].try_into().unwrap());
            image_dimensions(1 + (bits & 0x3fff), 1 + ((bits >> 14) & 0x3fff))
        }
        b"VP8X" => {
            require(chunk_size == 10 && size > 38)?;
            let width = u32::from_le_bytes([data[4], data[5], data[6], 0]) + 1;
            let height = u32::from_le_bytes([data[7], data[8], data[9], 0]) + 1;
            image_dimensions(width, height)
        }
        _ => require(false),
    }
}

fn gif(file: &mut File, size: u64) -> io::Result<()> {
    require(size >= 26)?;
    let bytes = read_at::<13>(file, 0)?;
    require(matches!(&bytes[..6], b"GIF87a" | b"GIF89a"))?;
    require(read_at::<1>(file, size - 1)? == [0x3b])?;
    let width = u16::from_le_bytes([bytes[6], bytes[7]]);
    let height = u16::from_le_bytes([bytes[8], bytes[9]]);
    image_dimensions(width.into(), height.into())
}

fn ico(file: &mut File, size: u64) -> io::Result<()> {
    let mut bytes = vec![0; size as usize];
    file.rewind()?;
    file.read_exact(&mut bytes)?;
    require(bytes.len() >= 22 && bytes[..4] == [0, 0, 1, 0])?;
    let count = usize::from(u16::from_le_bytes(bytes[4..6].try_into().unwrap()));
    require((1..=64).contains(&count) && 6 + count * 16 <= bytes.len())?;
    for entry in bytes[6..6 + count * 16].chunks_exact(16) {
        let length = u32::from_le_bytes(entry[8..12].try_into().unwrap()) as usize;
        let offset = u32::from_le_bytes(entry[12..16].try_into().unwrap()) as usize;
        require(
            entry[3] == 0
                && offset >= 6 + count * 16
                && length >= 24
                && offset
                    .checked_add(length)
                    .is_some_and(|end| end <= bytes.len()),
        )?;
        let data = &bytes[offset..offset + length];
        let dimension = |n| if n == 0 { 256 } else { u32::from(n) };
        let (width, height) = (dimension(entry[0]), dimension(entry[1]));
        if data.starts_with(b"\x89PNG") {
            require(png_dimensions(data[..24].try_into().unwrap())? == (width, height))?;
        } else {
            let header = u32::from_le_bytes(data[..4].try_into().unwrap()) as usize;
            let w = u32::from_le_bytes(data[4..8].try_into().unwrap());
            let h = u32::from_le_bytes(data[8..12].try_into().unwrap());
            require(header >= 40 && header < data.len() && w == width && h == 2 * height)?;
        }
        require(native_icon(data, width, height))?;
    }
    Ok(())
}

#[cfg(windows)]
fn native_icon(data: &[u8], width: u32, height: u32) -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::{CreateIconFromResourceEx, DestroyIcon};
    // ICO entries can begin at offsets that are not DWORD-aligned.
    let mut aligned = vec![0u32; data.len().div_ceil(4)];
    unsafe {
        std::ptr::copy_nonoverlapping(data.as_ptr(), aligned.as_mut_ptr().cast(), data.len());
        let icon = CreateIconFromResourceEx(
            aligned.as_ptr().cast(),
            data.len() as u32,
            1,
            0x0003_0000,
            width as i32,
            height as i32,
            0,
        );
        if icon.is_null() {
            return false;
        }
        DestroyIcon(icon);
    }
    true
}

#[cfg(not(windows))]
fn native_icon(_: &[u8], _: u32, _: u32) -> bool {
    true
}

struct Ebml {
    id: u32,
    start: u64,
    end: u64,
    unknown_size: bool,
}

fn ebml_vint(file: &mut File, max_width: u32, remove_marker: bool) -> io::Result<(u64, u32)> {
    let mut byte = [0];
    file.read_exact(&mut byte)?;
    let width = byte[0].leading_zeros() + 1;
    require(width <= max_width)?;
    let first = if remove_marker {
        byte[0] & (0xffu16 >> width) as u8
    } else {
        byte[0]
    };
    let mut value = u64::from(first);
    for _ in 1..width {
        file.read_exact(&mut byte)?;
        value = (value << 8) | u64::from(byte[0]);
    }
    Ok((value, width))
}

fn ebml(file: &mut File, offset: u64, bound: u64) -> io::Result<Ebml> {
    require(offset + 2 <= bound)?;
    file.seek(SeekFrom::Start(offset))?;
    let (id, _) = ebml_vint(file, 4, false)?;
    let (length, width) = ebml_vint(file, 8, true)?;
    let start = file.stream_position()?;
    let unknown_size = length == ((1 << (7 * width)) - 1);
    let end = if unknown_size { bound } else { start + length };
    require(start <= end && end <= bound)?;
    Ok(Ebml {
        id: id as u32,
        start,
        end,
        unknown_size,
    })
}

fn webm(file: &mut File, size: u64) -> io::Result<()> {
    let header = ebml(file, 0, size)?;
    require(header.id == 0x1a45dfa3 && !header.unknown_size && header.end <= 4096)?;
    let mut offset = header.start;
    let mut doctype = false;
    while offset < header.end {
        let element = ebml(file, offset, header.end)?;
        require(!element.unknown_size)?;
        if element.id == 0x4282 {
            require(
                !doctype
                    && element.end - element.start == 4
                    && read_at::<4>(file, element.start)? == *b"webm",
            )?;
            doctype = true;
        }
        offset = element.end;
    }
    require(doctype)?;
    let mut offset = header.end;
    let segment = loop {
        let element = ebml(file, offset, size)?;
        if element.id == 0x18538067 {
            break element;
        }
        require(element.id == 0xec && !element.unknown_size && element.end <= 4096)?;
        offset = element.end;
    };
    require(segment.end == size)?;
    let mut offset = segment.start;
    let mut info = false;
    let mut video = false;
    for _ in 0..4096 {
        let element = ebml(file, offset, segment.end)?;
        if element.id == 0x1f43b675 {
            require(info && video)?;
            return webm_cluster(file, &element);
        }
        require(!element.unknown_size)?;
        match element.id {
            0x1549a966 => info = element.start < element.end,
            0x1654ae6b => video = webm_video(file, &element)?,
            _ => {}
        }
        offset = element.end;
    }
    require(false)
}

fn webm_video(file: &mut File, tracks: &Ebml) -> io::Result<bool> {
    let mut offset = tracks.start;
    for _ in 0..256 {
        if offset == tracks.end {
            break;
        }
        let entry = ebml(file, offset, tracks.end)?;
        require(!entry.unknown_size)?;
        if entry.id == 0xae {
            let mut field = entry.start;
            let mut video = false;
            let mut codec = false;
            let mut dimensions = false;
            for _ in 0..256 {
                if field == entry.end {
                    break;
                }
                let element = ebml(file, field, entry.end)?;
                require(!element.unknown_size)?;
                let length = element.end - element.start;
                match element.id {
                    0x83 => video = ebml_uint(file, &element)? == 1,
                    0xe0 => dimensions = webm_dimensions(file, &element)?,
                    0x86 if length == 5 => {
                        codec = matches!(
                            &read_at::<5>(file, element.start)?,
                            b"V_VP8" | b"V_VP9" | b"V_AV1"
                        );
                    }
                    _ => {}
                }
                if video && codec && dimensions {
                    return Ok(true);
                }
                field = element.end;
            }
        }
        offset = entry.end;
    }
    Ok(false)
}

fn ebml_uint(file: &mut File, element: &Ebml) -> io::Result<u64> {
    let size = (element.end - element.start) as usize;
    require((1..=8).contains(&size))?;
    let mut bytes = [0; 8];
    file.seek(SeekFrom::Start(element.start))?;
    file.read_exact(&mut bytes[8 - size..])?;
    Ok(u64::from_be_bytes(bytes))
}

fn webm_dimensions(file: &mut File, element: &Ebml) -> io::Result<bool> {
    let mut width = 0;
    let mut height = 0;
    let mut offset = element.start;
    for _ in 0..128 {
        if offset == element.end {
            break;
        }
        let child = ebml(file, offset, element.end)?;
        require(!child.unknown_size)?;
        match child.id {
            0xb0 => width = ebml_uint(file, &child)?,
            0xba => height = ebml_uint(file, &child)?,
            _ => {}
        }
        offset = child.end;
    }
    Ok(u32::try_from(width)
        .ok()
        .zip(u32::try_from(height).ok())
        .is_some_and(|(width, height)| image_dimensions(width, height).is_ok()))
}

fn webm_cluster(file: &mut File, cluster: &Ebml) -> io::Result<()> {
    let mut offset = cluster.start;
    let mut timestamp = false;
    let mut media = false;
    for _ in 0..4096 {
        if offset == cluster.end {
            break;
        }
        let element = ebml(file, offset, cluster.end)?;
        require(!element.unknown_size)?;
        let length = element.end - element.start;
        match element.id {
            0xe7 => timestamp = (1..=8).contains(&length),
            0xa0 => media |= length >= 7, // BlockGroup
            0xa3 => media |= length >= 5, // SimpleBlock
            _ => {}
        }
        if timestamp && media {
            return Ok(());
        }
        offset = element.end;
    }
    require(false)
}

struct Mp4Box {
    kind: [u8; 4],
    start: u64,
    end: u64,
}

fn mp4_box(file: &mut File, offset: u64, bound: u64) -> io::Result<Mp4Box> {
    require(offset + 8 <= bound)?;
    let header = read_at::<8>(file, offset)?;
    let short = u32::from_be_bytes(header[..4].try_into().unwrap());
    let (length, header_length) = match short {
        0 => (bound - offset, 8),
        1 => {
            require(offset + 16 <= bound)?;
            (u64::from_be_bytes(read_at(file, offset + 8)?), 16)
        }
        n => (u64::from(n), 8),
    };
    require(length >= header_length && length <= bound - offset)?;
    Ok(Mp4Box {
        kind: header[4..].try_into().unwrap(),
        start: offset + header_length,
        end: offset + length,
    })
}

fn mp4(file: &mut File, size: u64) -> io::Result<()> {
    let mut offset = 0;
    let mut brand = false;
    let mut video = false;
    let mut data = false;
    for _ in 0..4096 {
        if offset == size {
            return require(brand && video && data);
        }
        let item = mp4_box(file, offset, size)?;
        match &item.kind {
            b"ftyp" => {
                require(item.end - item.start >= 8 && item.end - item.start <= 4096)?;
                for at in (item.start..item.end - 3).step_by(4) {
                    if at == item.start + 4 {
                        continue;
                    }
                    brand |= matches!(
                        &read_at::<4>(file, at)?,
                        b"isom"
                            | b"iso2"
                            | b"iso3"
                            | b"iso4"
                            | b"iso5"
                            | b"iso6"
                            | b"iso7"
                            | b"iso8"
                            | b"iso9"
                            | b"mp41"
                            | b"mp42"
                            | b"avc1"
                            | b"dash"
                            | b"M4V "
                            | b"MSNV"
                            | b"cmfc"
                            | b"cmfs"
                            | b"F4V "
                            | b"hev1"
                            | b"hvc1"
                    );
                }
            }
            b"moov" => video |= mp4_video(file, &item, &[*b"trak", *b"mdia", *b"hdlr"])?,
            b"mdat" => data |= item.start < item.end,
            _ => {}
        }
        offset = item.end;
    }
    require(false)
}

fn mp4_video(file: &mut File, parent: &Mp4Box, path: &[[u8; 4]]) -> io::Result<bool> {
    let Some((wanted, path)) = path.split_first() else {
        return Ok(
            parent.start + 12 <= parent.end && read_at::<4>(file, parent.start + 8)? == *b"vide"
        );
    };
    let mut offset = parent.start;
    for _ in 0..1024 {
        if offset == parent.end {
            break;
        }
        let item = mp4_box(file, offset, parent.end)?;
        if &item.kind == wanted && mp4_video(file, &item, path)? {
            return Ok(true);
        }
        offset = item.end;
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(name)
    }

    fn isolated() -> (tempfile::TempDir, Store, Cache) {
        let target = fixture("../target");
        fs::create_dir_all(&target).unwrap();
        let directory = tempfile::tempdir_in(target).unwrap();
        let store = Store::open(directory.path()).unwrap();
        let cache = Cache {
            verified: BTreeMap::new(),
            staged: BTreeSet::new(),
        };
        (directory, store, cache)
    }

    #[test]
    fn starting_style_is_scoped_to_the_installation_and_preserves_custom_media() {
        let (_dir, store, mut cache) = isolated();
        let first = "0123456789abcdef0123456789abcdef";
        let reinstall = "fedcba9876543210fedcba9876543210";
        assert!(store.style_marker("../profile.json").is_err());
        assert!(store.style_marker(first).unwrap().1);
        let mut custom = Profile { hub_name: Some("Custom Hub".into()), ..Profile::default() };
        custom.assets.insert("hubLogo".into(), store.pick(
            "hubLogo", &fixture("assets/panopticon-online.png"), &mut cache,
        ).unwrap());
        custom.background_filters.insert("gameBrowserBackground".into(), BackgroundFilter {
            hue: Some(12), ..BackgroundFilter::default()
        });
        store.save(custom.clone(), &mut cache).unwrap();

        let red = store.select_initial_style(first, builtin_style("panopticon").unwrap(), &mut cache).unwrap();
        assert_eq!(red.accent.as_deref(), Some(PANOPTICON_ACCENT));
        assert_eq!(red.builtin_style, Some(BuiltinStyle::Panopticon));
        assert_eq!(red.background_filters.len(), 1);
        assert_eq!(red.hub_name, custom.hub_name);
        assert_eq!(red.assets, custom.assets);
        assert!(!store.style_marker(first).unwrap().1);
        assert!(store.style_marker(reinstall).unwrap().1);
        let gold = builtin_style("shinri").unwrap();
        assert_eq!(store.select_initial_style(first, gold.clone(), &mut cache).unwrap(), red);
        let renewed = store.select_initial_style(reinstall, gold, &mut cache).unwrap();
        assert_eq!(renewed, custom);
        assert!(!store.style_marker(reinstall).unwrap().1);
    }

    #[test]
    fn refreshes_the_exact_old_red_preset_and_keeps_user_variations() {
        let mut old: Profile = serde_json::from_value(serde_json::json!({
            "version": 1,
            "accent": "#dc5964",
            "hubName": "My Hub",
            "backgroundFilters": {
                "hubBackground": {"hue": -32, "saturation": 108, "brightness": 98},
                "gameMenuBackground": {"hue": -32, "saturation": 104, "brightness": 95},
                "gameBrowserBackground": {"hue": 12}
            }
        })).unwrap();
        let mut custom = old.clone();
        custom.background_filters.get_mut("hubBackground").unwrap().brightness = Some(91);
        let expected = custom.clone();
        custom.refresh_legacy_panopticon();
        assert_eq!(custom, expected);

        let mut with_media = old.clone();
        with_media.assets.insert("hubBackground".into(), format!("{}.jpg", "a".repeat(64)));
        let expected = with_media.clone();
        with_media.refresh_legacy_panopticon();
        assert_eq!(with_media, expected);

        old.refresh_legacy_panopticon();
        assert_eq!(old.accent.as_deref(), Some(PANOPTICON_ACCENT));
        assert_eq!(old.builtin_style, Some(BuiltinStyle::Panopticon));
        assert_eq!(old.hub_name.as_deref(), Some("My Hub"));
        assert_eq!(old.background_filters.keys().collect::<Vec<_>>(), vec!["gameBrowserBackground"]);
    }

    #[test]
    fn music_round_trips_and_resets_independently() {
        let (source, store, mut cache) = isolated();
        let sound = "../game-ui/dist/html/assets";
        let wav = fixture("../game-ui/source/html/panopticon/intro-audio.wav");
        let ogg = fixture(&format!("{sound}/sfx_ui_hover.ogg"));
        let mp3 = fixture(&format!("{sound}/shinri_trial_main_theme.mp3"));
        let hub_key = store.pick("hubMusic", &wav, &mut cache).unwrap();
        let game_key = store.pick("gameMenuMusic", &mp3, &mut cache).unwrap();
        let loading_key = store.pick("gameLoadingMusic", &ogg, &mut cache).unwrap();
        assert!(store.pick("hubBackground", &wav, &mut cache).is_err());
        assert!(store.pick("gameMenuMusic", &fixture("assets/panopticon-online.png"), &mut cache).is_err());
        let spoof = source.path().join("spoof.mp3");
        fs::copy(&wav, &spoof).unwrap();
        assert!(store.pick("hubMusic", &spoof, &mut cache).is_err());
        fs::write(&spoof, &fs::read(mp3).unwrap()[..50]).unwrap();
        assert!(store.pick("hubMusic", &spoof, &mut cache).is_err());

        let mut profile = Profile::default();
        profile.assets.insert("hubMusic".into(), hub_key);
        profile.assets.insert("gameMenuMusic".into(), game_key.clone());
        profile.assets.insert("gameLoadingMusic".into(), loading_key.clone());
        profile.game_loading_volume = Some(0);
        let mut saved = store.save(profile, &mut cache).unwrap();
        assert_eq!(saved.game_loading_volume, Some(0));
        let preset = source.path().join("audio.pthubtheme");
        store.export(saved.clone(), &preset, &mut cache).unwrap();
        let (_target, destination, mut imported) = isolated();
        assert_eq!(destination.import(&preset, &mut imported).unwrap(), saved);
        assert_eq!(destination.load(&mut imported).unwrap(), Profile::default());
        saved.assets.remove("hubMusic");
        let mut saved = store.save(saved, &mut cache).unwrap();
        assert_eq!(store.load(&mut cache).unwrap().assets, BTreeMap::from([
            ("gameMenuMusic".into(), game_key),
            ("gameLoadingMusic".into(), loading_key),
        ]));
        saved.game_loading_volume = Some(101);
        assert!(store.save(saved.clone(), &mut cache).is_err());
        saved.game_loading_volume = Some(30);
        let mut saved = store.save(saved, &mut cache).unwrap();
        assert_eq!(saved.game_loading_volume, None);
        saved.assets.remove("gameLoadingMusic");
        saved.game_loading_volume = Some(60);
        assert_eq!(store.save(saved, &mut cache).unwrap().game_loading_volume, None);
    }

    #[test]
    fn background_filters_round_trip_without_replacing_media() {
        use serde_json::json;

        let (_source, store, mut cache) = isolated();
        let profile = serde_json::from_value(json!({
            "version": 1,
            "backgroundFilters": {
                "hubBackground": {"hue": -35, "brightness": 100},
                "gameMenuBackground": {"hue": 180, "saturation": 0, "brightness": 125},
                "gameBrowserBackground": {"hue": 0, "brightness": 100},
                "gameLoadingBackground": {"brightness": 0}
            }
        }))
        .unwrap();
        let saved = store.save(profile, &mut cache).unwrap();
        assert!(saved.assets.is_empty());
        assert_eq!(
            serde_json::to_value(&saved).unwrap()["backgroundFilters"],
            json!({
                "hubBackground": {"hue": -35},
                "gameMenuBackground": {"hue": 180, "saturation": 0, "brightness": 125},
                "gameLoadingBackground": {"brightness": 0}
            })
        );
        assert_eq!(store.load(&mut cache).unwrap(), saved);
        let path = store.root.join("colors.pthubtheme");
        store.export(saved.clone(), &path, &mut cache).unwrap();
        let (_target, destination, mut cache) = isolated();
        assert_eq!(destination.import(&path, &mut cache).unwrap(), saved);
        assert_eq!(destination.load(&mut cache).unwrap(), Profile::default());

        for (slot, value) in [
            ("hubLogo", json!({"hue": 10})),
            ("hubBackground", json!({"hue": -181})),
            ("gameMenuBackground", json!({"saturation": 201})),
            ("gameBrowserBackground", json!({"brightness": -1})),
        ] {
            let invalid = serde_json::from_value(json!({
                "version": 1, "backgroundFilters": { (slot): value }
            }))
            .unwrap();
            assert!(destination.save(invalid, &mut cache).is_err());
        }
        assert_eq!(
            serde_json::from_str::<Profile>(r#"{"version":1}"#).unwrap(),
            Profile::default()
        );
    }

    #[test]
    fn theme_round_trip_stages_without_applying() {
        let (source, store, mut cache) = isolated();
        let key = store
            .pick(
                "hubLogo",
                &fixture("assets/panopticon-online.png"),
                &mut cache,
            )
            .unwrap();
        assert!(valid_media_key(&key));
        let mut profile = Profile {
            accent: Some("#A0B1C2".into()),
            game_name: Some(" Игра ".into()),
            ..Profile::default()
        };
        profile.assets.insert("hubLogo".into(), key.clone());
        profile.layout.insert(
            "play".into(),
            Layout {
                x: Some(42.0),
                y: None,
                hidden: Some(true),
            },
        );
        profile.decorations.push(Decoration {
            asset: key.clone(),
            x: 10.0,
            y: 20.0,
            width: 15.0,
        });
        let profile = store.save(profile, &mut cache).unwrap();
        assert_eq!(profile.accent.as_deref(), Some("#a0b1c2"));
        assert_eq!(profile.game_name.as_deref(), Some("Игра"));
        assert_eq!(profile.layout["play"].hidden, None);
        assert_eq!(profile.layout["play"].x, Some(42.0));
        assert_eq!(store.load(&mut cache).unwrap(), profile);

        let path = source.path().join("round-trip.pthubtheme");
        store.export(profile.clone(), &path, &mut cache).unwrap();

        let (_destination, store, mut cache) = isolated();
        let staged = store.import(&path, &mut cache).unwrap();
        assert_eq!(staged, profile);
        assert_eq!(store.load(&mut cache).unwrap(), Profile::default());
        assert_eq!(store.entries().unwrap().len(), 1);
        store.save(staged, &mut cache).unwrap();
        assert_eq!(store.load(&mut cache).unwrap(), profile);

        fs::remove_file(store.assets.join(key)).unwrap();
        let loaded = store.load(&mut cache).unwrap();
        assert_eq!(loaded.accent, profile.accent);
        assert_eq!(loaded.layout, profile.layout);
        assert!(loaded.assets.is_empty());
        assert!(loaded.decorations.is_empty());
    }

    #[test]
    fn replacing_and_discarding_drafts_removes_only_unreferenced_media() {
        let (_directory, store, mut cache) = isolated();
        let active_key = store
            .pick("hubLogo", &fixture("assets/panopticon-online.png"), &mut cache)
            .unwrap();
        let active = Profile {
            assets: BTreeMap::from([("hubLogo".into(), active_key.clone())]),
            ..Profile::default()
        };
        store.save(active.clone(), &mut cache).unwrap();
        let first_key = store
            .pick("gameLogo", &fixture("assets/shinri-online.png"), &mut cache)
            .unwrap();
        let second_key = store
            .pick("hubIcon", &fixture("assets/hub-app.ico"), &mut cache)
            .unwrap();
        let first_preset = store.assets.join("first.pthubtheme");
        let second_preset = store.root.join("second.pthubtheme");
        let first = Profile {
            decorations: vec![Decoration {
                asset: first_key.clone(),
                x: 10.0,
                y: 20.0,
                width: 15.0,
            }],
            ..Profile::default()
        };
        let second = Profile {
            assets: BTreeMap::from([("hubIcon".into(), second_key.clone())]),
            ..Profile::default()
        };
        store.export(first.clone(), &first_preset, &mut cache).unwrap();
        store.export(second.clone(), &second_preset, &mut cache).unwrap();
        store.update_draft(BTreeSet::new(), &mut cache).unwrap();
        assert_eq!(store.entries().unwrap().len(), 1);

        assert_eq!(store.import(&first_preset, &mut cache).unwrap(), first);
        store.update_draft(BTreeSet::from([first_key.clone()]), &mut cache).unwrap();
        assert_eq!(store.import(&second_preset, &mut cache).unwrap(), second);
        let (_, mut previous_preview) = store.verified(&first_key, &mut cache).unwrap();
        store.update_draft(BTreeSet::from([second_key.clone()]), &mut cache).unwrap();
        assert_eq!(read_at::<8>(&mut previous_preview, 0).unwrap(), *b"\x89PNG\r\n\x1a\n");
        drop(previous_preview);
        assert!(!store.assets.join(first_key).exists());
        assert!(store.verified(&second_key, &mut cache).is_ok());
        assert_eq!(store.load(&mut cache).unwrap(), active);

        store.update_draft(BTreeSet::new(), &mut cache).unwrap();
        assert!(!store.assets.join(second_key).exists());
        assert!(store.verified(&active_key, &mut cache).is_ok());
        assert_eq!(store.import(&first_preset, &mut cache).unwrap(), first);
    }

    #[test]
    fn game_snapshot_survives_cleanup_and_keeps_the_desktop_draft() {
        let (directory, store, mut cache) = isolated();
        let previous_key = store
            .pick("gameLogo", &fixture("assets/shinri-online.png"), &mut cache)
            .unwrap();
        let previous = Profile {
            assets: BTreeMap::from([("gameLogo".into(), previous_key.clone())]),
            ..Profile::default()
        };
        store.save(previous.clone(), &mut cache).unwrap();
        let replacement_key = store
            .pick("gameLogo", &fixture("assets/panopticon-online.png"), &mut cache)
            .unwrap();
        let replacement = Profile {
            assets: BTreeMap::from([("gameLogo".into(), replacement_key.clone())]),
            ..Profile::default()
        };
        let mut invalid = replacement.clone();
        invalid.assets.insert("hubLogo".into(), format!("{}.png", "a".repeat(64)));
        assert!(store.save(invalid, &mut cache).is_err());
        assert!(cache.staged.contains(&replacement_key));

        let worker = Store::open(directory.path()).unwrap();
        let mut worker_cache = Cache { verified: BTreeMap::new(), staged: BTreeSet::new() };
        let (snapshot, mut media) = {
            let _lock = worker.lock().unwrap();
            worker.load_with_media(&mut worker_cache).unwrap()
        };
        assert_eq!(snapshot, previous);
        assert!(store.assets.join(&replacement_key).exists());

        let _lock = store.lock().unwrap();
        store.save(replacement.clone(), &mut cache).unwrap();
        assert!(cache.staged.is_empty());
        let mut original = Vec::new();
        media.take(&previous_key).unwrap().read_to_end(&mut original).unwrap();
        assert_eq!(original, fs::read(fixture("assets/shinri-online.png")).unwrap());
        assert!(!store.assets.join(&previous_key).exists());
        assert_eq!(store.load(&mut cache).unwrap(), replacement);
    }

    #[test]
    fn startup_cleanup_preserves_files_if_the_active_profile_cannot_be_read() {
        let (_directory, store, mut cache) = isolated();
        let leftover = store.assets.join(format!("{}.png", "a".repeat(64)));
        let unrelated = store.assets.join("notes.txt");
        fs::write(&leftover, b"old media").unwrap();
        fs::write(&unrelated, b"user file").unwrap();
        let profile = store.root.join("profile.json");
        fs::write(&profile, br#"{"version":1,"assets": "#).unwrap();
        assert!(store.update_draft(BTreeSet::new(), &mut cache).is_err());
        assert!(leftover.is_file());

        fs::write(&profile, br#"{"version":1}"#).unwrap();
        store.update_draft(BTreeSet::new(), &mut cache).unwrap();
        assert!(!leftover.exists());
        assert!(unrelated.is_file());
    }

    #[test]
    fn bad_preset_never_publishes_valid_earlier_assets() {
        let (source, store, mut cache) = isolated();
        let mut profile = Profile::default();
        profile.assets.insert(
            "hubLogo".into(),
            store
                .pick(
                    "hubLogo",
                    &fixture("assets/panopticon-online.png"),
                    &mut cache,
                )
                .unwrap(),
        );
        profile.assets.insert(
            "gameLogo".into(),
            store
                .pick("gameLogo", &fixture("assets/shinri-online.png"), &mut cache)
                .unwrap(),
        );
        let path = source.path().join("bad-theme.pthubtheme");
        store.export(profile, &path, &mut cache).unwrap();
        let mut bytes = fs::read(&path).unwrap();
        *bytes.last_mut().unwrap() ^= 1;
        fs::write(&path, bytes).unwrap();

        let (_destination, store, mut cache) = isolated();
        assert!(store.import(&path, &mut cache).is_err());
        assert_eq!(store.load(&mut cache).unwrap(), Profile::default());
        assert_eq!(fs::read_dir(&store.assets).unwrap().count(), 0);
    }

    #[test]
    fn disallows_false_webm_mismatched_images_and_path_keys() {
        let (directory, store, mut cache) = isolated();
        let webm = directory.path().join("fake.webm");
        fs::write(&webm, b"\x1a\x45\xdf\xa3\x87\x42\x82\x84webm<html>").unwrap();
        assert!(store.pick("hubBackground", &webm, &mut cache).is_err());
        let png = directory.path().join("fake.png");
        fs::write(&png, b"<svg onload=alert()>").unwrap();
        assert!(store.pick("hubBackground", &png, &mut cache).is_err());
        assert!(!valid_media_key("../test.png"));
        assert!(!valid_media_key(&format!("{}.svg", "a".repeat(64))));
        assert!(store.verified("../profile.json", &mut cache).is_err());
        let mut profile = Profile::default();
        profile.layout.insert("brush".into(), Layout::default());
        assert!(store.save(profile, &mut cache).is_err());
    }

    #[test]
    fn accepts_real_media_in_its_slots() {
        let (_directory, store, mut cache) = isolated();
        let video = fixture("../game-ui/source/html/panopticon/intro.webm");
        assert!(store
            .pick("gameLoadingBackground", &video, &mut cache)
            .is_ok());
        assert!(store.pick("gameLogo", &video, &mut cache).is_err());
        for name in [
            "../game-ui/source/html/panopticon/backdrops/academy-01.jpg",
            "../game-ui/source/html/panopticon/settings-background.webp",
        ] {
            assert!(
                store
                    .pick("gameBrowserBackground", &fixture(name), &mut cache)
                    .is_ok(),
                "{name}"
            );
        }
        for name in ["assets/hub.ico", "assets/hub-app.ico"] {
            let icon = fixture(name);
            assert!(store.pick("gameIcon", &icon, &mut cache).is_ok(), "{name}");
            assert!(store.pick("hubBackground", &icon, &mut cache).is_err());
        }
    }

    #[test]
    fn export_requires_exact_selected_extension() {
        let (directory, store, mut cache) = isolated();
        let selected = directory.path().join("example");
        let existing = directory.path().join("example.pthubtheme");
        fs::write(&existing, b"existing").unwrap();
        assert!(store
            .export(Profile::default(), &selected, &mut cache)
            .is_err());
        assert!(!selected.exists());
        assert_eq!(fs::read(existing).unwrap(), b"existing");
    }
}
