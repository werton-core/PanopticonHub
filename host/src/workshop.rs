//! Публичный контент Panopticon через кэш авторизованного Steam app 4000.
use hub_backend::{steam, Backend};
use libloading::os::windows::Library;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ffi::{c_char, c_void, CStr},
    fs::{self, File, Metadata},
    io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write},
    os::windows::fs::MetadataExt,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

const SUBSCRIBED: u32 = 1;
const INSTALLED: u32 = 4;
const UPDATING: u32 = 8 | 16 | 32;
const MAX_GMA_INDEX: u64 = 16 * 1024 * 1024;
const MAX_GMA_STRING: u64 = 64 * 1024;
type Ptr = *mut c_void;

unsafe fn sym<T: Copy>(lib: &Library, name: &[u8]) -> Result<T, String> {
    lib.get::<T>(name).map(|value| *value).map_err(|_| {
        format!(
            "В установленной Steam API отсутствует {}.",
            String::from_utf8_lossy(name)
        )
    })
}

struct Session(unsafe extern "C" fn());

impl Drop for Session {
    fn drop(&mut self) {
        unsafe { (self.0)() }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Prepare { panopticon: bool, filter: bool },
    Verify,
    SelectBranch,
    Preview,
    Subscribe,
}

#[derive(Deserialize, Serialize)]
struct Addon {
    id: u64,
    title: String,
    author: String,
    bytes: Option<u64>,
    preview: Option<String>,
}

#[derive(Deserialize)]
struct Collections {
    shinri: Vec<u64>,
    panopticon: Vec<Addon>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct MountPlan {
    pub allowed: Vec<u64>,
    pub blocked: Vec<u64>,
    pub unavailable: Vec<u64>,
}

type ItemState = unsafe extern "C" fn(Ptr, u64) -> u32;

fn output(value: impl Serialize) -> Result<(), String> {
    let line = serde_json::to_string(&value).map_err(|error| error.to_string())?;
    writeln!(io::stdout().lock(), "{line}").map_err(|error| error.to_string())
}

fn server_download(root: &Path, id: u64) -> bool {
    let Ok(path) = crate::game::ordinary(root, &format!("garrysmod/cache/workshop/{id}.gma")) else { return false; };
    fs::symlink_metadata(&path).is_ok_and(|metadata| {
        metadata.is_file() && metadata.file_attributes() & 0x400 == 0
            && valid_archive(&path, &metadata, false)
    })
}

unsafe fn subscribed_items(lib: &Library, ugc: Ptr) -> Result<Vec<u64>, String> {
    let count = sym::<unsafe extern "C" fn(Ptr, bool) -> u32>(lib, b"SteamAPI_ISteamUGC_GetNumSubscribedItems")?;
    let read = sym::<unsafe extern "C" fn(Ptr, *mut u64, u32, bool) -> u32>(lib, b"SteamAPI_ISteamUGC_GetSubscribedItems")?;
    for _ in 0..3 {
        let total = count(ugc, true);
        if total > 100_000 { return Err("Слишком много подписок Steam для проверки.".into()); }
        let mut items = vec![0; total as usize];
        let written = read(ugc, items.as_mut_ptr(), total, true);
        if written == total && count(ugc, true) == total && !items.contains(&0) {
            return Ok(items);
        }
    }
    Err("Список подписок Steam изменился. Повторите запуск.".into())
}

#[repr(C)]
struct SubscriptionResult {
    result: i32,
    id: u64,
}

unsafe fn subscribe_missing(
    lib: &Library, ugc: Ptr, pump: unsafe extern "C" fn(), missing: Vec<&Addon>,
) -> Result<Vec<u64>, String> {
    let utils = sym::<unsafe extern "C" fn() -> Ptr>(lib, b"SteamAPI_SteamUtils_v010")?();
    if utils.is_null() { return Err("Steam не готов оформить подписку.".into()); }
    let subscribe = sym::<unsafe extern "C" fn(Ptr, u64) -> u64>(lib, b"SteamAPI_ISteamUGC_SubscribeItem")?;
    let completed = sym::<unsafe extern "C" fn(Ptr, u64, *mut bool) -> bool>(lib, b"SteamAPI_ISteamUtils_IsAPICallCompleted")?;
    let result = sym::<unsafe extern "C" fn(Ptr, u64, *mut SubscriptionResult, i32, i32, *mut bool) -> bool>(lib, b"SteamAPI_ISteamUtils_GetAPICallResult")?;
    let mut calls: Vec<_> = missing.iter().map(|item| (item.id, subscribe(ugc, item.id))).collect();
    let mut unavailable = Vec::new();
    let deadline = Instant::now() + Duration::from_secs(20);
    while !calls.is_empty() {
        pump();
        let mut index = 0;
        while index < calls.len() {
            let (id, handle) = calls[index];
            let mut failed = false;
            if handle == 0 { return Err("Steam не принял запрос подписки. Повторите попытку.".into()); }
            if !completed(utils, handle, &mut failed) { index += 1; continue; }
            let mut reply = SubscriptionResult { result: 0, id: 0 };
            // RemoteStorageSubscribePublishedFileResult_t is callback 1313 on Win64.
            if failed || !result(utils, handle, &mut reply, std::mem::size_of::<SubscriptionResult>() as i32, 1313, &mut failed)
                || failed || reply.result == 1 && reply.id != id {
                return Err("Steam не подтвердил подписку. Повторите попытку.".into());
            }
            match reply.result {
                1 => {},
                9 | 15 | 24 => unavailable.push(id),
                _ => return Err(format!("Steam не подтвердил подписку на Workshop {id}. Код {}.", reply.result)),
            }
            calls.swap_remove(index);
        }
        if !calls.is_empty() {
            if Instant::now() >= deadline { return Err("Steam не ответил на запрос подписки.".into()); }
            std::thread::sleep(Duration::from_millis(100));
        }
    }
    Ok(unavailable)
}

pub fn run(op: Operation) -> Result<(), String> {
    let backend = Backend::new(|_| {})?;
    let (steam, game) = backend.steam_paths()?;
    run_at(op, &steam, &game)
}

pub fn run_at(op: Operation, steam_root: &Path, game: &Path) -> Result<(), String> {
    let root = if op == Operation::SelectBranch {
        // Only the Steam-registered app may supply the Steamworks library.
        match steam::inspect_launch(steam_root, game) {
            Ok(_) => return Ok(()),
            Err(e) if e.starts_with("STEAM_UPDATE_PENDING:") => return Ok(()),
            Err(e) if e.starts_with("STEAM_BETA_REQUIRED:") => {}
            Err(e) => return Err(e),
        }
        crate::game_profile::check_running_at(game)?;
        steam::check_amd64(&game.join("bin/win64/steam_api64.dll"))?;
        game.to_owned()
    } else {
        steam::inspect_launch(steam_root, game)?.game_root
    };
    let check = op == Operation::Verify;
    let collections: Collections = serde_json::from_str(include_str!("../assets/workshop.json"))
        .map_err(|error| error.to_string())?;
    // Изменение окружения ограничено отдельным процессом подготовки.
    std::env::set_var("SteamAppId", "4000");
    std::env::set_var("SteamGameId", "4000");
    unsafe {
        let lib = Library::load_with_flags(crate::game::ordinary(&root, "bin/win64/steam_api64.dll")?, 0x100 | 0x1000)
            .map_err(|error| error.to_string())?;
        let init = sym::<unsafe extern "C" fn(*mut c_char) -> i32>(&lib, b"SteamAPI_InitFlat")?;
        let shutdown = sym::<unsafe extern "C" fn()>(&lib, b"SteamAPI_Shutdown")?;
        let pump = sym::<unsafe extern "C" fn()>(&lib, b"SteamAPI_RunCallbacks")?;
        let mut error = [0u8; 1024];
        if init(error.as_mut_ptr().cast()) != 0 {
            let details = CStr::from_bytes_until_nul(&error)
                .map(|message| message.to_string_lossy())
                .unwrap_or_default();
            return Err(format!(
                "STEAM_NOT_READY: войдите в Steam и повторите запуск. {details}"
            ));
        }
        let _session = Session(shutdown);
        let apps = sym::<unsafe extern "C" fn() -> Ptr>(&lib, b"SteamAPI_SteamApps_v009")?();
        let user = sym::<unsafe extern "C" fn() -> Ptr>(&lib, b"SteamAPI_SteamUser_v023")?();
        if apps.is_null() || user.is_null() {
            return Err("Службы Steam недоступны".into());
        }
        let license = sym::<unsafe extern "C" fn(Ptr, u32) -> bool>(
            &lib,
            b"SteamAPI_ISteamApps_BIsSubscribedApp",
        )?;
        let logged =
            sym::<unsafe extern "C" fn(Ptr) -> bool>(&lib, b"SteamAPI_ISteamUser_BLoggedOn")?;
        if !logged(user) || !license(apps, 4000) {
            return Err("Требуется вход в Steam с лицензией Garry’s Mod.".into());
        }
        if op == Operation::SelectBranch {
            let select = sym::<unsafe extern "C" fn(Ptr, *const c_char) -> bool>(
                &lib,
                b"SteamAPI_ISteamApps_SetActiveBeta",
            )?;
            return if select(apps, c"x86-64".as_ptr()) {
                Ok(())
            } else {
                Err("Steam отклонил переключение на x86-64.".into())
            };
        }
        let ugc = sym::<unsafe extern "C" fn() -> Ptr>(&lib, b"SteamAPI_SteamUGC_v021")?();
        if ugc.is_null() { return Err("Steam Workshop недоступен.".into()); }
        let state = sym::<ItemState>(&lib, b"SteamAPI_ISteamUGC_GetItemState")?;
        if matches!(op, Operation::Preview | Operation::Subscribe) {
            let mut has_any = false;
            let mut missing = Vec::new();
            let mut unsubscribed = Vec::new();
            for item in &collections.panopticon {
                let flags = state(ugc, item.id);
                let installed = flags & INSTALLED != 0 || server_download(&root, item.id);
                let subscribed = flags & SUBSCRIBED != 0;
                has_any |= installed || subscribed;
                if !installed {
                    missing.push(item);
                    if !subscribed { unsubscribed.push(item); }
                }
            }
            return if op == Operation::Preview {
                output(json!({"missing": missing, "hasAny": has_any}))
            } else {
                output(json!({"unavailable": subscribe_missing(&lib, ugc, pump, unsubscribed)?}))
            };
        }
        let include_panopticon = matches!(op, Operation::Verify | Operation::Prepare { panopticon: true, .. });
        let filter = matches!(op, Operation::Prepare { filter: true, .. });
        let mut ids = collections.shinri;
        let panopticon_ids = collections.panopticon.iter().map(|item| item.id);
        let mut plan = MountPlan::default();
        if filter {
            plan.allowed.extend(ids.iter().copied().chain(panopticon_ids.clone()));
        }
        if include_panopticon { ids.extend(panopticon_ids); }
        let optional_unavailable: HashSet<_> = collections.panopticon.iter()
            .filter(|item| item.bytes.is_none()).map(|item| item.id).collect();
        let download = sym::<unsafe extern "C" fn(Ptr, u64, bool) -> bool>(
            &lib,
            b"SteamAPI_ISteamUGC_DownloadItem",
        )?;
        let install = sym::<
            unsafe extern "C" fn(Ptr, u64, *mut u64, *mut c_char, u32, *mut u32) -> bool,
        >(&lib, b"SteamAPI_ISteamUGC_GetItemInstallInfo")?;
        let info = sym::<unsafe extern "C" fn(Ptr, u64, *mut u64, *mut u64) -> bool>(
            &lib,
            b"SteamAPI_ISteamUGC_GetItemDownloadInfo",
        )?;
        let total = ids.len();
        let cached: HashSet<_> = ids.iter().copied().filter(|&id| server_download(&root, id)).collect();
        let mut remaining = ids;
        let mut active = HashSet::new();
        let cache_file = std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .map(|root| root.join("PanopticonHub/workshop-verified-v1.json"));
        let mut verified = Verified::load(cache_file);
        let mut last = (usize::MAX, 0u64);
        let mut last_change = Instant::now();
        loop {
            pump();
            let mut invalid = Vec::new();
            let mut manifests = HashMap::new();
            remaining.retain(|id| {
                let flags = state(ugc, *id);
                if flags & INSTALLED == 0 || flags & UPDATING != 0 {
                    return !cached.contains(id);
                }
                let (mut size, mut at, mut buffer) = (0u64, 0u32, [0u8; 4096]);
                let installed = install(
                    ugc,
                    *id,
                    &mut size,
                    buffer.as_mut_ptr().cast(),
                    buffer.len() as u32,
                    &mut at,
                );
                if installed
                    && CStr::from_bytes_until_nul(&buffer)
                        .ok()
                        .and_then(|path| path.to_str().ok())
                        .is_some_and(|path| {
                            verified.verify(*id, Path::new(path), size, at, check, &mut manifests)
                        })
                {
                    active.remove(id);
                    false
                } else {
                    invalid.push(*id);
                    true
                }
            });
            let done = total - remaining.len() - plan.unavailable.len();
            if remaining.is_empty() {
                println!("{done}/{total}");
                if filter {
                    plan.blocked = subscribed_items(&lib, ugc)?.into_iter()
                        .filter(|id| !plan.allowed.contains(id)).collect();
                }
                return if check { Ok(()) } else { output(plan) };
            }
            if check {
                return Err(format!("В кэше Steam готово {done}/{total}. Требуется обновить Workshop {remaining:?}."));
            }
            for &id in &remaining {
                if active.len() >= 2 {
                    break;
                }
                if !active.contains(&id) {
                    if !download(ugc, id, false) {
                        if optional_unavailable.contains(&id) {
                            plan.unavailable.push(id);
                            continue;
                        }
                        return Err(format!("Steam отклонил загрузку Workshop {id}."));
                    }
                    active.insert(id);
                }
            }
            remaining.retain(|id| !plan.unavailable.contains(id));
            let bytes = active
                .iter()
                .map(|&id| {
                    let (mut downloaded, mut size) = (0u64, 0u64);
                    if info(ugc, id, &mut downloaded, &mut size) {
                        downloaded
                    } else {
                        0
                    }
                })
                .sum();
            if (done, bytes) != last {
                if done != last.0 {
                    println!("{done}/{total}");
                }
                last = (done, bytes);
                last_change = Instant::now();
            }
            if last_change.elapsed() > Duration::from_secs(25)
                && remaining.iter().all(|id| optional_unavailable.contains(id)) {
                plan.unavailable.append(&mut remaining);
                continue;
            }
            if last_change.elapsed() > Duration::from_secs(180) {
                if !invalid.is_empty() {
                    return Err(format!("Steam не восстановил содержимое Workshop {invalid:?}. Проверьте файлы через Steam."));
                }
                return Err(format!(
                    "Steam не продвинул загрузку за три минуты. Незавершённые элементы: {active:?}"
                ));
            }
            std::thread::sleep(Duration::from_millis(350));
        }
    }
}

type WorkshopManifests = HashMap<PathBuf, BTreeMap<u64, u64>>;

struct Verified {
    path: Option<PathBuf>,
    items: BTreeMap<u64, Value>,
}

impl Verified {
    fn load(path: Option<PathBuf>) -> Self {
        let items = path
            .as_deref()
            .and_then(|path| {
                let file = File::open(path).ok()?;
                if file.metadata().ok()?.len() > 128 * 1024 {
                    return None;
                }
                serde_json::from_reader(BufReader::new(file)).ok()
            })
            .unwrap_or_default();
        Self { path, items }
    }

    fn verify(
        &mut self,
        id: u64,
        item_path: &Path,
        size: u64,
        installed_at: u32,
        force: bool,
        manifests: &mut WorkshopManifests,
    ) -> bool {
        let Some((archive, metadata)) = item_archive(item_path, size) else {
            return false;
        };
        let stamp = manifest_for(&archive, id, manifests)
            .map(|manifest| archive_stamp(&archive, &metadata, manifest, installed_at));
        let deep = force || stamp.is_none() || self.items.get(&id) != stamp.as_ref();
        if !valid_archive(&archive, &metadata, deep) {
            if self.items.remove(&id).is_some() {
                self.save();
            }
            return false;
        }
        if let Some(stamp) = stamp {
            if self.items.get(&id) != Some(&stamp) {
                self.items.insert(id, stamp);
                self.save();
            }
        }
        true
    }

    fn save(&self) {
        let _ = (|| -> io::Result<()> {
            let Some(path) = &self.path else {
                return Ok(());
            };
            let directory = path.parent().ok_or(io::ErrorKind::InvalidInput)?;
            fs::create_dir_all(directory)?;
            let mut temporary = tempfile::NamedTempFile::new_in(directory)?;
            serde_json::to_writer(&mut temporary, &self.items)?;
            temporary.flush()?;
            temporary.persist(path)?;
            Ok(())
        })();
    }
}

fn manifest_for(path: &Path, id: u64, manifests: &mut WorkshopManifests) -> Option<u64> {
    let item = path.parent()?;
    if item.file_name()?.to_str()?.parse::<u64>().ok()? != id {
        return None;
    }
    let app = item.parent()?;
    let content = app.parent()?;
    let workshop = content.parent()?;
    if app.file_name()? != "4000"
        || !content.file_name()?.eq_ignore_ascii_case("content")
        || !workshop.file_name()?.eq_ignore_ascii_case("workshop")
    {
        return None;
    }
    manifests
        .entry(workshop.join("appworkshop_4000.acf"))
        .or_insert_with_key(|path| steam::workshop_manifests(path).unwrap_or_default())
        .get(&id)
        .copied()
}

fn archive_stamp(path: &Path, metadata: &Metadata, manifest: u64, installed_at: u32) -> Value {
    json!({
        "path": path,
        "manifest": manifest,
        "size": metadata.len(),
        "mtime": metadata.last_write_time(),
        "created": metadata.creation_time(),
        "installed_at": installed_at,
    })
}

fn item_archive(path: &Path, expected_size: u64) -> Option<(PathBuf, Metadata)> {
    if !path.is_absolute() || expected_size == 0 {
        return None;
    }
    let metadata = fs::symlink_metadata(path).ok()?;
    if metadata.file_attributes() & 0x400 != 0 {
        return None;
    }
    let archive = if metadata.is_file() {
        path.to_path_buf()
    } else {
        let mut archive = None;
        for entry in fs::read_dir(path).ok()? {
            let path = entry.ok()?.path();
            if is_gma(&path) && archive.replace(path).is_some() {
                return None;
            }
        }
        archive?
    };
    let metadata = fs::symlink_metadata(&archive).ok()?;
    (is_gma(&archive)
        && metadata.is_file()
        && metadata.file_attributes() & 0x400 == 0
        && metadata.len() == expected_size)
        .then_some((archive, metadata))
}

fn is_gma(path: &Path) -> bool {
    path.extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("gma"))
}

fn valid_archive(path: &Path, expected: &Metadata, check_crc: bool) -> bool {
    let unchanged = |metadata: Metadata| {
        metadata.is_file()
            && metadata.file_attributes() & 0x400 == 0
            && metadata.len() == expected.len()
            && metadata.last_write_time() == expected.last_write_time()
            && metadata.creation_time() == expected.creation_time()
    };
    File::open(path).is_ok_and(|mut file| {
        file.metadata().is_ok_and(unchanged)
            && parse_gma(&mut file, expected.len(), check_crc).unwrap_or(false)
            && fs::symlink_metadata(path).is_ok_and(unchanged)
    })
}

// Формат Facepunch GMAD. Размер каждого блока берётся из индекса, а не из имени файла.
fn parse_gma(file: &mut File, size: u64, check_crc: bool) -> io::Result<bool> {
    let mut index = BufReader::new(Read::by_ref(file).take(MAX_GMA_INDEX));
    let mut header = [0u8; 21];
    index.read_exact(&mut header)?;
    if &header[..4] != b"GMAD" || !(1..=3).contains(&header[4]) {
        return Ok(false);
    }
    let mut string = Vec::new();
    if header[4] > 1 {
        while read_string(&mut index, &mut string)? {}
    }
    for _ in 0..3 {
        read_string(&mut index, &mut string)?;
    }
    let mut word = [0u8; 4];
    index.read_exact(&mut word)?;
    let mut payload_size = 0u64;
    let mut entries = Vec::new();
    for number in 1u32.. {
        index.read_exact(&mut word)?;
        match u32::from_le_bytes(word) {
            0 => break,
            actual if actual == number => {}
            _ => return Ok(false),
        }
        if !read_string(&mut index, &mut string)? {
            return Ok(false);
        }
        let mut bytes = [0u8; 8];
        index.read_exact(&mut bytes)?;
        index.read_exact(&mut word)?;
        let length = i64::from_le_bytes(bytes);
        if length <= 0 {
            return Ok(false);
        }
        let length = length as u64;
        let Some(total) = payload_size
            .checked_add(length)
            .filter(|&total| total < size)
        else {
            return Ok(false);
        };
        payload_size = total;
        if check_crc {
            entries.push((length, u32::from_le_bytes(word)));
        }
    }
    let index_size = MAX_GMA_INDEX - index.get_ref().limit() - index.buffer().len() as u64;
    if payload_size == 0 || index_size.checked_add(payload_size + 4) != Some(size) {
        return Ok(false);
    }
    if !check_crc {
        return Ok(true);
    }
    file.seek(SeekFrom::Start(index_size))?;
    let mut input = BufReader::with_capacity(1024 * 1024, file);
    let mut buffer = vec![0u8; 1024 * 1024];
    for (length, crc) in entries {
        // GMAD умеет создавать пакеты без контрольных сумм.
        let contents = hash_block(&mut input, &mut buffer, length)?;
        if crc != 0 && crc != contents {
            return Ok(false);
        }
    }
    // У нескольких штатных GMA итоговый CRC трейлера отличается при корректных CRC всех файлов.
    input.read_exact(&mut word)?;
    Ok(true)
}

fn read_string(input: &mut impl BufRead, buffer: &mut Vec<u8>) -> io::Result<bool> {
    buffer.clear();
    input.take(MAX_GMA_STRING).read_until(0, buffer)?;
    if buffer.last() != Some(&0) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Повреждён индекс GMA",
        ));
    }
    Ok(buffer.len() > 1)
}

fn hash_block(file: &mut impl Read, buffer: &mut [u8], mut length: u64) -> io::Result<u32> {
    let mut hash = crc32fast::Hasher::new();
    while length != 0 {
        let count = length.min(buffer.len() as u64) as usize;
        file.read_exact(&mut buffer[..count])?;
        hash.update(&buffer[..count]);
        length -= count as u64;
    }
    Ok(hash.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Vec<u8> {
        let mut gma = b"GMAD\x03".to_vec();
        gma.extend([0; 16]);
        gma.extend(b"\0Example\0Description\0Author\0");
        gma.extend(1u32.to_le_bytes());
        gma.extend(1u32.to_le_bytes());
        gma.extend(b"materials/test.vmt\0");
        let payload = b"example";
        gma.extend((payload.len() as i64).to_le_bytes());
        gma.extend(crc32fast::hash(payload).to_le_bytes());
        gma.extend(0u32.to_le_bytes());
        gma.extend(payload);
        gma.extend(crc32fast::hash(&gma).to_le_bytes());
        gma
    }

    fn valid_cache(path: &Path, size: u64, deep: bool) -> bool {
        item_archive(path, size)
            .is_some_and(|(path, metadata)| valid_archive(&path, &metadata, deep))
    }

    #[test]
    fn install_info_accepts_directory_or_legacy_gma_only() {
        let item = tempfile::tempdir().unwrap();
        let file = item.path().join("addon.GMA");
        let data = fixture();
        let size = data.len() as u64;
        fs::write(item.path().join("notes.txt"), &data).unwrap();
        assert!(!valid_cache(item.path(), size, false));
        fs::write(&file, &data).unwrap();
        assert!(valid_cache(item.path(), size, false));
        assert!(valid_cache(&file, size, true));
        assert!(!valid_cache(&file, size + 1, false));
        fs::write(item.path().join("second.gma"), &data).unwrap();
        assert!(!valid_cache(item.path(), size, false));
    }

    #[test]
    fn rejects_empty_truncated_foreign_and_bad_crc() {
        let item = tempfile::tempdir().unwrap();
        let file = item.path().join("addon.gma");
        let mut data = fixture();
        for invalid in [&[][..], b"unrelated", &data[..data.len() - 1]] {
            fs::write(&file, invalid).unwrap();
            assert!(!valid_cache(&file, invalid.len() as u64, false));
        }
        let payload = data.len() - 5;
        data[payload] ^= 1;
        fs::write(&file, &data).unwrap();
        assert!(!valid_cache(&file, data.len() as u64, true));
    }

    #[test]
    fn cache_is_persisted_and_a_manifest_change_rechecks_crc() {
        let library = tempfile::tempdir().unwrap();
        let workshop = library.path().join("workshop");
        let item = workshop.join("content/4000/7");
        fs::create_dir_all(&item).unwrap();
        let index = workshop.join("appworkshop_4000.acf");
        let write_index = |manifest| {
            fs::write(&index, format!(r#""AppWorkshop" {{"WorkshopItemsInstalled" {{"7" {{"manifest" "{manifest}"}}}}}}"#)).unwrap();
        };
        write_index(111);
        let file = item.join("test.gma");
        let mut bytes = fixture();
        let size = bytes.len() as u64;
        // Трейлер может оставаться от другой редакции корректного архива.
        bytes[size as usize - 1] ^= 1;
        fs::write(&file, &bytes).unwrap();
        let cache_file = Some(library.path().join("cache/project.json"));
        let mut verified = Verified::load(cache_file.clone());
        assert!(verified.verify(7, &item, size, 1, false, &mut HashMap::new()));
        let mut loaded = Verified::load(cache_file);
        assert_eq!(loaded.items, verified.items);
        assert_eq!(loaded.items[&7]["manifest"], 111);
        let source_times = fs::metadata(&file).unwrap().modified().unwrap();
        bytes[size as usize - 5] ^= 1;
        fs::write(&file, &bytes).unwrap();
        File::options()
            .write(true)
            .open(&file)
            .unwrap()
            .set_modified(source_times)
            .unwrap();
        assert!(loaded.verify(7, &item, size, 1, false, &mut HashMap::new()));
        write_index(222);
        assert!(!loaded.verify(7, &item, size, 1, false, &mut HashMap::new()));
        assert!(!loaded.items.contains_key(&7));
    }
}
