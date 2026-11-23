use crate::{design, game};
use hub_backend::{
    online::{self, Count},
    steam,
};
use serde_json::{json, Value};
use std::{
    borrow::Cow,
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
    sync::{Mutex, OnceLock},
    thread,
    time::Duration,
};
use wry::http::{Method, Request, Response};

include!(concat!(env!("OUT_DIR"), "/ui.rs"));

type Asset = Response<Cow<'static, [u8]>>;
const REGION_FILE: &str = "garrysmod/data/panopticon_hub_region.txt";
static GAME_ROOT: OnceLock<Option<PathBuf>> = OnceLock::new();
static LIVE: Mutex<Count> = Mutex::new(Count { available: 0, total: None });

pub fn start() {
    GAME_ROOT.get_or_init(|| {
        let steam_root = steam::find_steam()?;
        steam::find_game(&steam_root).ok().flatten()
    });
    thread::spawn(|| loop {
        let count = online::panopticon();
        if let Ok(mut current) = LIVE.lock() {
            *current = count;
        }
        thread::sleep(Duration::from_secs(35));
    });
}

fn online_count() -> Asset {
    let counts = LIVE.lock().map(|count| *count).unwrap_or_default();
    api(
        200,
        json!({"panopticon": {
            "total": counts.total, "available": counts.available, "expected": online::EXPECTED
        }}),
    )
}

fn region_path() -> Result<PathBuf, String> {
    let root = GAME_ROOT
        .get()
        .and_then(Option::as_deref)
        .ok_or("Не найден установленный Garry’s Mod.")?;
    game::ordinary(root, REGION_FILE)
}

fn game_region() -> &'static str {
    let mut bytes = Vec::with_capacity(4);
    if region_path()
        .ok()
        .and_then(|path| File::open(path).ok())
        .is_some_and(|file| file.take(4).read_to_end(&mut bytes).is_ok() && bytes == b"eu")
    {
        "eu"
    } else {
        "cis"
    }
}

fn save_game_region(region: &str) -> Result<(), String> {
    let path = region_path()?;
    let parent = path.parent().ok_or("Не найдена папка настроек.")?;
    std::fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|err| err.to_string())?;
    temp.write_all(region.as_bytes()).map_err(|err| err.to_string())?;
    temp.as_file().sync_all().map_err(|err| err.to_string())?;
    temp.persist(path).map_err(|err| err.error.to_string())?;
    Ok(())
}

fn api(status: u16, value: Value) -> Asset {
    Response::builder()
        .status(status)
        .header("content-type", "application/json; charset=utf-8")
        .header("cache-control", "no-store")
        .header("X-Content-Type-Options", "nosniff")
        .body(Cow::Owned(value.to_string().into_bytes()))
        .unwrap()
}

fn range(value: &str, len: usize) -> Option<(usize, usize)> {
    let (first, last) = value.strip_prefix("bytes=")?.split_once('-')?;
    if len == 0 {
        return None;
    }
    if first.is_empty() {
        let size = last.parse::<usize>().ok()?;
        return (size > 0).then(|| (len.saturating_sub(size), len - 1));
    }
    let start = first.parse().ok()?;
    let end = if last.is_empty() { len - 1 } else { last.parse::<usize>().ok()?.min(len - 1) };
    (start <= end && end < len).then_some((start, end))
}

fn media_response(req: &Request<Vec<u8>>, key: &str) -> Asset {
    let mut file = match design::open_media(key) {
        Ok(file) => file,
        Err(_) => return api(404, Value::Null),
    };
    let Ok(length) = file.metadata().map(|meta| meta.len() as usize) else {
        return api(404, Value::Null);
    };
    if length == 0 {
        return api(404, Value::Null);
    }
    let mut response = Response::builder()
        .header("x-content-type-options", "nosniff")
        .header("content-type", mime(key))
        .header("cache-control", "private, max-age=31536000, immutable")
        .header("accept-ranges", "bytes");
    let (first, last) = if let Some(header) = req.headers().get("range") {
        let Some((first, last)) = header.to_str().ok().and_then(|value| range(value, length)) else {
            return response.status(416)
                .header("content-range", format!("bytes */{length}"))
                .body(Cow::Borrowed(&[][..]))
                .unwrap();
        };
        response = response.status(206)
            .header("content-range", format!("bytes {first}-{last}/{length}"));
        (first, last)
    } else {
        (0, length - 1)
    };
    let size = last - first + 1;
    let mut bytes = Vec::new();
    if req.method() != Method::HEAD {
        let result = file.seek(SeekFrom::Start(first as u64))
            .and_then(|_| file.take(size as u64).read_to_end(&mut bytes));
        if !matches!(result, Ok(read) if read == size) {
            return api(500, Value::Null);
        }
    }
    response.header("content-length", size).body(Cow::Owned(bytes)).unwrap()
}

fn mime(key: &str) -> &'static str {
    match key.rsplit('.').next().unwrap_or("") {
        "js" => "text/javascript",
        "html" => "text/html; charset=utf-8",
        "css" => "text/css",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "ico" => "image/x-icon",
        "webm" => "video/webm",
        "mp4" => "video/mp4",
        "woff2" => "font/woff2",
        "svg" => "image/svg+xml",
        "ogg" => "audio/ogg",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "json" => "application/json",
        _ => "application/octet-stream",
    }
}

fn design_default(key: &str) -> Option<(&'static str, &'static [u8])> {
    let preview = match key {
        "design-defaults/gameMenuBackground" => ("background.webm",
            include_bytes!("../../game-ui/dist/html/assets/background.webm").as_slice()),
        "design-defaults/panopticon-background.webm" => ("panopticon-background.webm",
            include_bytes!("../../game-ui/dist/html/assets/panopticon-background.webm").as_slice()),
        "design-defaults/panopticon-background.jpg" => ("panopticon-background.jpg",
            include_bytes!("../../game-ui/dist/html/assets/panopticon-background.jpg").as_slice()),
        "design-defaults/gameMenuMusic" => ("menu.mp3",
            include_bytes!("../../game-ui/dist/html/assets/shinri_trial_main_theme.mp3").as_slice()),
        "design-defaults/gameBrowserBackground" | "design-defaults/gameLoadingBackground" =>
            ("academy.jpg",
                include_bytes!("../../game-ui/source/html/panopticon/backdrops/academy-01.jpg").as_slice()),
        "design-defaults/gameLogo" => ("logo.png",
            include_bytes!("../../game-ui/source/html/panopticon/panopticon-menu-logo.png").as_slice()),
        "design-defaults/gameLoadingLogo" => ("logo.png",
            include_bytes!("../../game-ui/dist/html/assets/logo.png").as_slice()),
        "design-defaults/hubIcon" | "design-defaults/gameIcon" =>
            ("hub.ico", include_bytes!("../assets/hub-app.ico").as_slice()),
        _ => return None,
    };
    Some(preview)
}

pub fn serve(req: Request<Vec<u8>>) -> Asset {
    let key = req.uri().path().trim_start_matches('/');
    let key = if key.is_empty() { "index.html" } else { key };
    if let Some(media) = key.strip_prefix("design-media/") {
        return match *req.method() {
            Method::GET | Method::HEAD => media_response(&req, media),
            _ => api(405, Value::Null),
        };
    }
    match (req.method(), key) {
        (&Method::GET, "online") => return online_count(),
        (&Method::GET, "game-region") => return api(200, json!({"region": game_region()})),
        (&Method::POST, "game-region/cis" | "game-region/eu") => {
            if req.headers().get("origin").is_some_and(|value| {
                value != "https://panopticon.localhost" && value != "http://panopticon.localhost"
            }) {
                return api(403, Value::Null);
            }
            let region = key.trim_start_matches("game-region/");
            return match save_game_region(region) {
                Ok(()) => api(200, json!({"region": region})),
                Err(error) => api(500, json!({"error": error})),
            };
        }
        (&Method::GET, _) => {}
        _ => return api(405, Value::Null),
    }
    let response = Response::builder().header("X-Content-Type-Options", "nosniff");
    let preview = design_default(key);
    let bytes = match key {
        "panopticon-online.png" => Some(include_bytes!("../assets/panopticon-online.png").as_slice()),
        "shinri-online.png" => Some(include_bytes!("../assets/shinri-online.png").as_slice()),
        _ => preview.map(|(_, bytes)| bytes).or_else(|| embedded(key)),
    };
    let Some(bytes) = bytes else {
        return response.status(404).body(Cow::Borrowed(&[][..])).unwrap();
    };
    let mime = mime(preview.map_or(key, |(name, _)| name));
    let response = response.header("content-type", mime).header("accept-ranges", "bytes");
    if let Some(header) = req.headers().get("range") {
        if let Some((first, last)) = header.to_str().ok().and_then(|s| range(s, bytes.len())) {
            return response
                .status(206)
                .header("content-range", format!("bytes {first}-{last}/{}", bytes.len()))
                .header("content-length", last - first + 1)
                .body(Cow::Borrowed(&bytes[first..=last]))
                .unwrap();
        }
        return response
            .status(416)
            .header("content-range", format!("bytes */{}", bytes.len()))
            .body(Cow::Borrowed(&[][..]))
            .unwrap();
    }
    response.header("content-length", bytes.len()).body(Cow::Borrowed(bytes)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn webm_ranges() {
        assert_eq!(range("bytes=-6", 10), Some((4, 9)));
        assert_eq!(range("bytes=3-", 10), Some((3, 9)));
        assert_eq!(range("bytes=3-2", 10), None);
        assert_eq!(range("bytes=10-99", 10), None);
    }

    #[test]
    fn default_menu_music_can_be_previewed_with_range_requests() {
        let response = serve(
            Request::get("/design-defaults/gameMenuMusic")
                .header("range", "bytes=0-15")
                .body(Vec::new())
                .unwrap(),
        );
        assert_eq!(response.status(), 206);
        assert_eq!(response.headers()["content-type"], "audio/mpeg");
        assert_eq!(response.body().len(), 16);
    }

    #[test]
    fn embedded_ui_support_routes_use_the_same_host() {
        let get = |path| Request::get(path).body(Vec::new()).unwrap();
        for path in ["/online", "/game-region", "/settings.js", "/panopticon-online.png"] {
            assert_eq!(serve(get(path)).status(), 200);
        }
        let foreign = Request::post("/game-region/eu")
            .header("origin", "https://other.invalid")
            .body(Vec::new())
            .unwrap();
        assert_eq!(serve(foreign).status(), 403);
    }
}

