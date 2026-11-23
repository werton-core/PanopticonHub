//! Открытые контракты Shinri: реестр узлов, A2S_INFO и подготовка клиентского GMA.
use hub_backend::identity;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
    path::{Path, PathBuf},
    ptr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};
use windows_sys::Win32::Networking::WinHttp::*;

const INFO: &[u8] = b"\xff\xff\xff\xffTSource Engine Query\0";
const MAX_PACK: usize = 64 * 1024 * 1024;
const NODES_HOSTS: &[&str] = &[
    "api.shinri.red",
    "api-fb1.shinri.red",
    "api.shinri-trial.com",
];
const CDN_HOSTS: &[&str] = &[
    "cdn.shinri.red",
    "cdn-fb1.shinri.red",
    "luapack.shinri-trial.com",
];

fn check(cancel: &AtomicBool) -> Result<(), String> {
    if cancel.load(Ordering::Acquire) {
        Err("Подготовка подключения отменена.".into())
    } else {
        Ok(())
    }
}
struct Handle(*mut std::ffi::c_void);
impl Handle {
    fn new(value: *mut std::ffi::c_void) -> Result<Self, String> {
        if value.is_null() {
            Err(network_error())
        } else {
            Ok(Self(value))
        }
    }
}
impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            WinHttpCloseHandle(self.0);
        }
    }
}
fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(Some(0)).collect()
}
fn network_error() -> String {
    format!(
        "Сеть Shinri недоступна: {}",
        std::io::Error::last_os_error()
    )
}

fn get(host: &str, path: &str, limit: usize, cancel: &AtomicBool) -> Result<Vec<u8>, String> {
    check(cancel)?;
    // Хосты выбираются из констант, перенаправления и понижение HTTPS запрещены.
    unsafe {
        let session = Handle::new(WinHttpOpen(
            wide("ShinriTrial/client").as_ptr(),
            WINHTTP_ACCESS_TYPE_AUTOMATIC_PROXY,
            ptr::null(),
            ptr::null(),
            0,
        ))?;
        if WinHttpSetTimeouts(session.0, 3000, 3000, 5000, 7000) == 0 {
            return Err(network_error());
        }
        let connection = Handle::new(WinHttpConnect(session.0, wide(host).as_ptr(), 443, 0))?;
        let request = Handle::new(WinHttpOpenRequest(
            connection.0,
            wide("GET").as_ptr(),
            wide(path).as_ptr(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
            WINHTTP_FLAG_SECURE,
        ))?;
        let policy = WINHTTP_OPTION_REDIRECT_POLICY_NEVER;
        if WinHttpSetOption(
            request.0,
            WINHTTP_OPTION_REDIRECT_POLICY,
            (&policy as *const u32).cast(),
            4,
        ) == 0
            || WinHttpSendRequest(request.0, ptr::null(), 0, ptr::null_mut(), 0, 0, 0) == 0
            || WinHttpReceiveResponse(request.0, ptr::null_mut()) == 0
        {
            return Err(network_error());
        }
        let mut status = 0u32;
        let mut size = 4;
        if WinHttpQueryHeaders(
            request.0,
            WINHTTP_QUERY_STATUS_CODE | WINHTTP_QUERY_FLAG_NUMBER,
            ptr::null(),
            (&mut status as *mut u32).cast(),
            &mut size,
            ptr::null_mut(),
        ) == 0
        {
            return Err(network_error());
        }
        if status != 200 {
            return Err(format!("Сервер Shinri вернул HTTP {status}."));
        }
        let deadline = Instant::now() + Duration::from_secs(40);
        let mut body = Vec::new();
        let mut buffer = [0u8; 32768];
        loop {
            check(cancel)?;
            if Instant::now() > deadline {
                return Err("Истекло время загрузки Shinri.".into());
            }
            let mut read = 0;
            if WinHttpReadData(
                request.0,
                buffer.as_mut_ptr().cast(),
                buffer.len() as u32,
                &mut read,
            ) == 0
            {
                return Err(network_error());
            }
            if read == 0 {
                break;
            }
            if body.len() + read as usize > limit {
                return Err("Ответ Shinri превышает допустимый размер.".into());
            }
            body.extend_from_slice(&buffer[..read as usize]);
        }
        Ok(body)
    }
}

fn subnet(ip: Ipv4Addr, cidr: &str) -> Option<bool> {
    let (network, prefix) = cidr.split_once('/')?;
    let network: Ipv4Addr = network.parse().ok()?;
    let prefix: u8 = prefix.parse().ok()?;
    if prefix > 32 {
        return None;
    }
    let mask = if prefix == 0 {
        0
    } else {
        u32::MAX << (32 - prefix)
    };
    Some((u32::from(ip) & mask) == (u32::from(network) & mask))
}
fn node_allowed(body: &[u8], ip: Ipv4Addr) -> Result<bool, String> {
    let root: Value =
        serde_json::from_slice(body).map_err(|_| "Некорректный реестр узлов Shinri.")?;
    if root["success"] != true {
        return Err("Реестр узлов Shinri сообщил об ошибке.".into());
    }
    let nodes = root["result"]["nodes"]
        .as_array()
        .ok_or("В реестре Shinri нет узлов.")?;
    let networks = root["result"]["subnets"]
        .as_array()
        .ok_or("В реестре Shinri нет подсетей.")?;
    if nodes.len() + networks.len() > 4096 {
        return Err("Реестр Shinri слишком велик.".into());
    }
    if nodes.iter().any(|node| {
        node["ipAddress"]
            .as_str()
            .and_then(|v| v.parse::<Ipv4Addr>().ok())
            == Some(ip)
    }) {
        return Ok(true);
    }
    Ok(networks.iter().any(|network| {
        let Some(cidr) = network["cidr"].as_str() else {
            return false;
        };
        if cidr.contains('/') {
            return subnet(ip, cidr) == Some(true);
        }
        // Совместимость со схемой {ipAddress, cidr: "24"} из старого клиента.
        network["ipAddress"]
            .as_str()
            .is_some_and(|address| subnet(ip, &format!("{address}/{cidr}")) == Some(true))
    }))
}

struct NodeCache {
    fetched: Instant,
    body: Vec<u8>,
}
static NODES: Mutex<Option<NodeCache>> = Mutex::new(None);
fn guard(address: SocketAddrV4, cancel: &AtomicBool) -> Result<(), String> {
    if let Some(cache) = NODES
        .lock()
        .map_err(|_| "Реестр Shinri недоступен.")?
        .as_ref()
    {
        if cache.fetched.elapsed() < Duration::from_secs(300) {
            return if node_allowed(&cache.body, *address.ip())? {
                Ok(())
            } else {
                Err("Сервер не входит в реестр узлов Shinri. Выключите идентификатор для обычного подключения.".into())
            };
        }
    }
    let mut last = "Реестр Shinri недоступен.".to_string();
    for host in NODES_HOSTS {
        check(cancel)?;
        match get(host, "/nodes", 1024 * 1024, cancel) {
            Ok(body) => {
                let allowed = match node_allowed(&body, *address.ip()) {
                    Ok(value) => value,
                    Err(error) => {
                        last = error;
                        continue;
                    }
                };
                *NODES.lock().map_err(|_| "Реестр Shinri недоступен.")? = Some(NodeCache {
                    fetched: Instant::now(),
                    body,
                });
                return if allowed {
                    Ok(())
                } else {
                    Err("Сервер не входит в реестр узлов Shinri. Выключите идентификатор для обычного подключения.".into())
                };
            }
            Err(error) => last = error,
        }
    }
    Err(last)
}

fn info_hash(bytes: &[u8]) -> Result<Option<String>, String> {
    let mut data = bytes
        .strip_prefix(b"\xff\xff\xff\xffI")
        .and_then(|v| v.get(1..))
        .ok_or("Неизвестный ответ A2S_INFO.")?;
    let mut game = &b""[..];
    for i in 0..4 {
        let end = data
            .iter()
            .position(|b| *b == 0)
            .filter(|end| *end <= 1024)
            .ok_or("Неполная строка A2S_INFO.")?;
        if i == 3 {
            game = &data[..end];
        }
        data = &data[end + 1..];
    }
    let app = data
        .get(..2)
        .and_then(|v| v.try_into().ok())
        .map(u16::from_le_bytes);
    if app != Some(4000) {
        return Err("A2S_INFO не относится к Garry’s Mod.".into());
    }
    if game.len() == 64 && game.iter().all(u8::is_ascii_hexdigit) {
        Ok(Some(
            std::str::from_utf8(game).unwrap().to_ascii_lowercase(),
        ))
    } else {
        Ok(None)
    }
}
fn query(address: SocketAddrV4, cancel: &AtomicBool) -> Result<Option<String>, String> {
    let mut last = String::new();
    for attempt in 0..3 {
        check(cancel)?;
        if attempt > 0 {
            // The reference retries the whole query after 500/1000 ms.
            for _ in 0..attempt * 10 {
                check(cancel)?;
                thread::sleep(Duration::from_millis(50));
            }
        }
        check(cancel)?;
        match query_once(address, cancel) {
            Ok(hash) => {
                check(cancel)?;
                return Ok(hash);
            }
            Err(error) => last = error,
        }
    }
    check(cancel)?;
    Err(last)
}

fn query_once(address: SocketAddrV4, cancel: &AtomicBool) -> Result<Option<String>, String> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).map_err(|e| e.to_string())?;
    socket.connect(address).map_err(|e| e.to_string())?;
    socket
        .set_read_timeout(Some(Duration::from_millis(1500)))
        .map_err(|e| e.to_string())?;
    check(cancel)?;
    socket.send(INFO).map_err(|e| e.to_string())?;
    let mut buffer = [0u8; 8192];
    let mut length = socket
        .recv(&mut buffer)
        .map_err(|e| format!("Нет ответа A2S_INFO: {e}"))?;
    if let Some(challenge) = buffer[..length].strip_prefix(b"\xff\xff\xff\xffA") {
        if challenge.len() != 4 {
            return Err("Некорректный challenge A2S_INFO.".into());
        }
        let mut packet = INFO.to_vec();
        packet.extend_from_slice(challenge);
        check(cancel)?;
        socket.send(&packet).map_err(|e| e.to_string())?;
        length = socket
            .recv(&mut buffer)
            .map_err(|e| format!("Нет ответа на challenge A2S_INFO: {e}"))?;
    }
    info_hash(&buffer[..length])
}

fn nul<'a>(bytes: &mut &'a [u8]) -> Result<&'a [u8], String> {
    let end = bytes
        .iter()
        .position(|v| *v == 0)
        .filter(|i| *i <= 4096)
        .ok_or("Повреждена строка GMA.")?;
    let value = &bytes[..end];
    *bytes = &bytes[end + 1..];
    Ok(value)
}
fn take<'a>(bytes: &mut &'a [u8], length: usize) -> Result<&'a [u8], String> {
    let value = bytes.get(..length).ok_or("Пакет GMA обрезан.")?;
    *bytes = &bytes[length..];
    Ok(value)
}
fn validate_gma(bytes: &[u8]) -> Result<(), String> {
    if bytes.len() > MAX_PACK {
        return Err("Lua-пак слишком велик.".into());
    }
    let mut data = bytes
        .strip_prefix(b"GMAD\x03")
        .ok_or("Lua-пак не является GMA v3.")?;
    take(&mut data, 16)?;
    // Внешние зависимости архива не загружаются автоматически.
    if !nul(&mut data)?.is_empty() {
        return Err("Не поддерживаются зависимости Lua-пака.".into());
    }
    for _ in 0..3 {
        nul(&mut data)?;
    }
    take(&mut data, 4)?;
    let mut entries = Vec::new();
    let mut seen = std::collections::HashSet::new();
    loop {
        let index = u32::from_le_bytes(take(&mut data, 4)?.try_into().unwrap());
        if index == 0 {
            break;
        }
        if index as usize != entries.len() + 1 || entries.len() >= 8192 {
            return Err("Некорректный каталог GMA.".into());
        }
        let name =
            std::str::from_utf8(nul(&mut data)?).map_err(|_| "Некорректное имя файла GMA.")?;
        if name.len() > 512
            || name.contains(['\\', ':'])
            || name
                .split('/')
                .any(|v| v.is_empty() || v == "." || v == "..")
            || !(name.starts_with("data_static/") || name == "lua/includes/cl_precache.lua")
            || !seen.insert(name)
        {
            return Err("Lua-пак содержит недопустимый путь.".into());
        }
        let size = u64::from_le_bytes(take(&mut data, 8)?.try_into().unwrap());
        let size = usize::try_from(size)
            .ok()
            .filter(|v| *v <= MAX_PACK)
            .ok_or("Некорректный размер GMA.")?;
        let crc = u32::from_le_bytes(take(&mut data, 4)?.try_into().unwrap());
        entries.push((size, crc));
    }
    if !seen.contains("lua/includes/cl_precache.lua") {
        return Err("В Lua-паке отсутствует загрузчик.".into());
    }
    for (size, crc) in entries {
        let payload = take(&mut data, size)?;
        if crc != 0 && crc32fast::hash(payload) != crc {
            return Err("Контрольная сумма файла GMA не совпала.".into());
        }
    }
    let crc = u32::from_le_bytes(take(&mut data, 4)?.try_into().unwrap());
    if !data.is_empty() || crc != 0 && crc32fast::hash(&bytes[..bytes.len() - 4]) != crc {
        return Err("Контрольная сумма архива GMA не совпала.".into());
    }
    Ok(())
}

fn pack(root: &Path, hash: &str, cancel: &AtomicBool) -> Result<(), String> {
    let destination = root
        .join("download/data/st_content")
        .join(format!("{hash}.bsp"));
    identity::ordinary(&destination)?;
    let digest_path = destination.with_extension("bsp.panopticon-sha256");
    identity::ordinary(&digest_path)?;
    match fs::File::open(&destination) {
        Ok(file) => {
            let mut bytes = Vec::new();
            file.take(MAX_PACK as u64 + 1)
                .read_to_end(&mut bytes)
                .map_err(|e| e.to_string())?;
            if validate_gma(&bytes).is_ok() {
                let mut saved = Vec::new();
                if let Ok(file) = fs::File::open(&digest_path) {
                    if file.take(66).read_to_end(&mut saved).is_ok()
                        && saved == format!("{:x}\n", Sha256::digest(&bytes)).as_bytes()
                    {
                        return Ok(());
                    }
                }
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.to_string()),
    }
    let path = format!("/file/data/st_content/{hash}.bsp");
    let mut last = "Не удалось загрузить Lua-пак Shinri.".to_string();
    for host in CDN_HOSTS {
        check(cancel)?;
        let result = (|| {
            let bytes = get(host, &path, MAX_PACK, cancel)?;
            validate_gma(&bytes)?;
            check(cancel)?;
            identity::ordinary(&destination)?;
            let parent = destination.parent().unwrap();
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            identity::ordinary(&destination)?;
            let mut temporary =
                tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
            temporary.write_all(&bytes).map_err(|e| e.to_string())?;
            temporary.as_file().sync_all().map_err(|e| e.to_string())?;
            // Имя пакета — идентификатор сервера, а не SHA его GMA-байтов.
            // Локальный digest относится к байтам, полученным по проверенному HTTPS.
            identity::ordinary(&digest_path)?;
            let mut digest = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
            writeln!(digest, "{:x}", Sha256::digest(&bytes)).map_err(|e| e.to_string())?;
            digest.as_file().sync_all().map_err(|e| e.to_string())?;
            check(cancel)?;
            identity::ordinary(&destination)?;
            identity::ordinary(&digest_path)?;
            digest.persist(&digest_path).map_err(|e| e.to_string())?;
            temporary.persist(&destination).map_err(|e| e.to_string())?;
            Ok(())
        })();
        match result {
            Ok(()) => return Ok(()),
            Err(error) => last = error,
        }
    }
    Err(last)
}

struct Job {
    id: u64,
    cancel: Arc<AtomicBool>,
    task: JoinHandle<Result<Option<String>, String>>,
}
static JOB: Mutex<Option<Job>> = Mutex::new(None);
static SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
pub fn busy() -> bool {
    let Ok(mut job) = JOB.lock() else { return true };
    if job
        .as_ref()
        .is_some_and(|job| job.cancel.load(Ordering::Acquire) && job.task.is_finished())
    {
        if let Some(old) = job.take() {
            let _ = old.task.join();
        }
    }
    job.is_some()
}
pub fn cancel() {
    if let Ok(job) = JOB.lock() {
        if let Some(job) = job.as_ref() {
            job.cancel.store(true, Ordering::Release);
        }
    }
}
pub fn shutdown() {
    let old = JOB.lock().ok().and_then(|mut job| job.take());
    if let Some(old) = old {
        old.cancel.store(true, Ordering::Release);
        let _ = old.task.join();
    }
}
pub fn start(address: &str, root: PathBuf, shinri: bool) -> Result<Value, String> {
    if !shinri || !identity::enabled().unwrap_or(false) {
        super::shinri_protocol::set(false)?;
        return Ok(json!({"status":"ready"}));
    }
    super::shinri_protocol::verify_supported()?;
    let address: SocketAddrV4 = address
        .parse()
        .map_err(|_| "Идентификатор Shinri поддерживает прямые IPv4-подключения из каталога.")?;
    if address.port() == 0 {
        return Err("Некорректный порт сервера.".into());
    }
    let mut job = JOB
        .lock()
        .map_err(|_| "Подготовка подключения недоступна.")?;
    if job.as_ref().is_some_and(|job| !job.task.is_finished()) {
        return Err("Подготовка другого подключения ещё выполняется.".into());
    }
    if let Some(old) = job.take() {
        let _ = old.task.join();
    }
    let id = SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let cancel = Arc::new(AtomicBool::new(false));
    let worker_cancel = cancel.clone();
    let task = thread::Builder::new()
        .name("Shinri preparation".into())
        .spawn(move || {
            guard(address, &worker_cancel)?;
            check(&worker_cancel)?;
            let hash = query(address, &worker_cancel)?;
            if let Some(hash) = &hash {
                pack(&root, hash, &worker_cancel)?;
            }
            check(&worker_cancel)?;
            Ok(hash)
        })
        .map_err(|error| format!("Не удалось начать подготовку подключения: {error}"))?;
    *job = Some(Job { id, cancel, task });
    Ok(json!({"status":"pending","id":id}))
}
pub fn poll(id: u64) -> Result<Value, String> {
    let mut job = JOB
        .lock()
        .map_err(|_| "Подготовка подключения недоступна.")?;
    let active = job
        .as_ref()
        .filter(|job| job.id == id)
        .ok_or("Попытка подключения устарела.")?;
    if !active.task.is_finished() {
        return Ok(json!({"status":"pending","id":id}));
    }
    let old = job.take().unwrap();
    let cancelled = old.cancel.load(Ordering::Acquire);
    let result = old
        .task
        .join()
        .map_err(|_| "Подготовка подключения прервана.")??;
    if cancelled {
        return Err("Подготовка подключения отменена.".into());
    }
    super::shinri_protocol::set(true)?;
    Ok(json!({"status":"ready","id":id,"packHash":result}))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn registry_uses_actual_node_and_cidr_schema() {
        let body=br#"{"success":true,"result":{"nodes":[{"ipAddress":"185.71.66.223"}],"subnets":[{"cidr":"186.246.32.0/24"}]}}"#;
        assert!(node_allowed(body, "185.71.66.223".parse().unwrap()).unwrap());
        assert!(node_allowed(body, "186.246.32.34".parse().unwrap()).unwrap());
        assert!(!node_allowed(body, "186.246.33.34".parse().unwrap()).unwrap());
        assert!(node_allowed(br#"{"success":false}"#, "127.0.0.1".parse().unwrap()).is_err());
        assert_eq!(subnet(Ipv4Addr::LOCALHOST, "127.0.0.1/33"), None);
    }
    #[test]
    fn a2s_retries_a_lost_query_then_parses_challenge_and_game_field() {
        let server = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        server
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let address = match server.local_addr().unwrap() {
            std::net::SocketAddr::V4(v) => v,
            _ => unreachable!(),
        };
        let thread = thread::spawn(move || {
            let mut bytes = [0; 512];
            // Simulate one lost UDP request before a complete challenge exchange.
            let (size, _) = server.recv_from(&mut bytes).unwrap();
            assert_eq!(&bytes[..size], INFO);
            let (size, peer) = server.recv_from(&mut bytes).unwrap();
            assert_eq!(&bytes[..size], INFO);
            assert_eq!(size, 25);
            server
                .send_to(b"\xff\xff\xff\xffA\x01\x02\x03\x04", peer)
                .unwrap();
            let (size, again) = server.recv_from(&mut bytes).unwrap();
            assert_eq!(peer, again);
            assert_eq!(size, 29);
            assert_eq!(&bytes[25..29], b"\x01\x02\x03\x04");
            let mut response = b"\xff\xff\xff\xffI\x11Lobby\0map\0garrysmod\0".to_vec();
            response.extend_from_slice(&[b'A'; 64]);
            response.extend_from_slice(b"\0\xa0\x0f\0\x40");
            server.send_to(&response, peer).unwrap();
        });
        assert_eq!(
            query(address, &AtomicBool::new(false)).unwrap(),
            Some("a".repeat(64))
        );
        thread.join().unwrap();
        assert!(info_hash(b"\xff\xff\xff\xffI\x11short").is_err());
        assert_eq!(
            info_hash(b"\xff\xff\xff\xffI\x11Lobby\0map\0garrysmod\0Game\0\xa0\x0f").unwrap(),
            None
        );
    }
    #[test]
    fn a2s_cancelled_during_receive_cannot_start_pack_preparation() {
        let server = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        server
            .set_read_timeout(Some(Duration::from_secs(3)))
            .unwrap();
        let address = match server.local_addr().unwrap() {
            std::net::SocketAddr::V4(v) => v,
            _ => unreachable!(),
        };
        let cancel = Arc::new(AtomicBool::new(false));
        let server_cancel = Arc::clone(&cancel);
        let task = thread::spawn(move || {
            let mut bytes = [0; 512];
            let (size, peer) = server.recv_from(&mut bytes).unwrap();
            assert_eq!(&bytes[..size], INFO);
            server_cancel.store(true, Ordering::Release);
            let mut response = b"\xff\xff\xff\xffI\x11Lobby\0map\0garrysmod\0".to_vec();
            response.extend_from_slice(&[b'a'; 64]);
            response.extend_from_slice(b"\0\xa0\x0f");
            server.send_to(&response, peer).unwrap();
        });
        assert!(query(address, &cancel).unwrap_err().contains("отменена"));
        task.join().unwrap();
    }

    #[test]
    #[ignore = "Проверка локальных паков официального клиента"]
    fn existing_official_gma_is_valid_but_tampered_catalog_is_not() {
        let root = Path::new(
            r"D:\SteamLibrary\steamapps\common\Shinri Trial Host\garrysmod\download\data\st_content",
        );
        let paths = fs::read_dir(root)
            .unwrap()
            .filter_map(Result::ok)
            .map(|v| v.path())
            .filter(|p| p.extension().is_some_and(|e| e == "bsp"))
            .collect::<Vec<_>>();
        assert!(!paths.is_empty());
        for path in paths {
            let bytes = fs::read(path).unwrap();
            validate_gma(&bytes).unwrap();
            assert!(validate_gma(&bytes[..bytes.len() - 1]).is_err());
            let mut changed = bytes;
            changed[0] = 0;
            assert!(validate_gma(&changed).is_err());
        }
    }
    fn fixture(name: &str, crc: u32) -> Vec<u8> {
        let mut bytes = b"GMAD\x03".to_vec();
        bytes.extend_from_slice(&[0; 16]);
        bytes.extend_from_slice(b"\0Fixture\0{}\0Panopticon\0\0\0\0\0");
        bytes.extend_from_slice(&1u32.to_le_bytes());
        bytes.extend_from_slice(name.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(&3u64.to_le_bytes());
        bytes.extend_from_slice(&crc.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(b"--\n");
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes
    }
    #[test]
    fn gma_rejects_paths_corruption_and_trailing_payload() {
        validate_gma(&fixture(
            "lua/includes/cl_precache.lua",
            crc32fast::hash(b"--\n"),
        ))
        .unwrap();
        for name in [
            "../lua/includes/cl_precache.lua",
            "lua/bin/evil.dll",
            "/data_static/pack.dat",
            "data_static/../pack.dat",
            "C:/pack",
        ] {
            assert!(validate_gma(&fixture(name, 0)).is_err());
        }
        assert!(validate_gma(&fixture("lua/includes/cl_precache.lua", 123)).is_err());
        let mut extra = fixture("lua/includes/cl_precache.lua", 0);
        extra.push(0);
        assert!(validate_gma(&extra).is_err());
    }
    #[test]
    fn cancelled_completed_job_cannot_return_ready() {
        let task = thread::spawn(|| Ok(None));
        while !task.is_finished() {
            thread::yield_now();
        }
        let mut lock = JOB.lock().unwrap();
        assert!(lock.is_none());
        *lock = Some(Job {
            id: 999,
            cancel: Arc::new(AtomicBool::new(true)),
            task,
        });
        drop(lock);
        assert!(poll(999).unwrap_err().contains("отменена"));
        assert!(!busy());
    }
}
