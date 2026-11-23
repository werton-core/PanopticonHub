use std::{
    env, fs,
    path::{Path, PathBuf},
};
fn entries(root: &Path, dir: &Path, rows: &mut Vec<(String, PathBuf)>) {
    for item in fs::read_dir(dir).expect("Не удалось прочитать ресурсы интерфейса")
    {
        let path = item.unwrap().path();
        if path.is_dir() {
            entries(root, &path, rows)
        } else {
            let key = path
                .strip_prefix(root)
                .unwrap()
                .to_str()
                .unwrap()
                .replace('\\', "/");
            rows.push((key, path.canonicalize().unwrap()));
        }
    }
}
fn main() {
    let release = env::var("PROFILE").as_deref() == Ok("release");
    let root = Path::new("../ui/dist");
    let mut all = Vec::new();
    if root.join("index.html").is_file() {
        entries(root, root, &mut all);
    } else if release {
        panic!("Для сборки нужны ресурсы прежнего лаунчера Panopticon в ui/dist");
    }
    for key in [
        "bridge.js",
        "online.js",
        "settings.js",
        "workshop.js",
        "workshop.css",
    ] {
        let path = Path::new("../ui").join(key);
        println!("cargo:rerun-if-changed={}", path.display());
        all.push((key.into(), path.canonicalize().unwrap()));
    }
    all.sort();
    let mut source = String::from("fn embedded(key:&str)->Option<&'static [u8]>{match key {\n");
    for (key, path) in all {
        source.push_str(&format!("{key:?}=>Some(include_bytes!({path:?})),\n"));
    }
    source.push_str("_=>None}}");
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("ui.rs"),
        source,
    )
    .unwrap();
    let game = Path::new("../game-ui/dist");
    let mut game_files = Vec::new();
    if game.join("html/menu.html").is_file() && game.join("html/loading.html").is_file() {
        entries(game, &game.join("html"), &mut game_files);
        let menu = fs::read_to_string(game.join("html/menu.html")).expect("Нет HTML игрового меню");
        assert!(
            menu.contains("panopticon/entry.js"),
            "Меню не подключает мост Panopticon"
        );
    } else if release {
        panic!("Для сборки нужны ресурсы прежнего игрового меню в game-ui/dist");
    }
    for relative in [
        "lua/menu/panopticon_public.lua",
        "lua/menu/panopticon_directory.lua",
        "lua/menu/panopticon_favorites.lua",
        "lua/menu/panopticon_reconnect.lua",
        "lua/menu/panopticon_loading_policy.lua",
        "html/panopticon/entry.js",
        "html/panopticon/language.js",
        "html/panopticon/language.css",
        "html/panopticon/intro.js",
        "html/panopticon/intro.css",
        "html/panopticon/intro.webm",
        "html/panopticon/intro-panopticon.png",
        "html/panopticon/backdrops/academy-01.jpg",
        "html/panopticon/backdrops/academy-02.jpg",
        "html/panopticon/backdrops/academy-03.jpg",
        "html/panopticon/directory.css",
        "html/panopticon/design.js",
        "html/panopticon/design.css",
        "html/panopticon/loading-bridge.js",
        "html/panopticon/voice-test.js",
        "html/panopticon/panopticon-menu-logo.png",
        "html/panopticon/settings-background.webp",
    ] {
        let path = Path::new("../game-ui/source")
            .join(relative)
            .canonicalize()
            .expect("Не найден файл моста Panopticon");
        println!("cargo:rerun-if-changed={}", path.display());
        game_files.retain(|(name, _)| name != relative);
        game_files.push((relative.to_owned(), path));
    }
    if let Some(path) = env::var_os("PANOPTICON_GAME_MODULE") {
        let path = PathBuf::from(path)
            .canonicalize()
            .expect("Native game module must be built first");
        println!("cargo:rerun-if-changed={}", path.display());
        game_files.push(("lua/bin/gmsv_panopticon_native_win64.dll".to_owned(), path));
    } else if release {
        panic!("Для сборки нужен модуль Panopticon Native. Используйте tools/build-host.ps1");
    }
    game_files.sort();
    let mut code = String::from("static GAME_FILES:&[(&str,&[u8])]=&[\n");
    for (name, path) in game_files {
        code.push_str(&format!("({name:?},include_bytes!({path:?})),\n"));
    }
    code.push_str("];");
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("game_assets.rs"),
        code,
    )
    .unwrap();
    println!("cargo:rerun-if-env-changed=PANOPTICON_GAME_MODULE");
    println!("cargo:rerun-if-changed=../game-ui/dist");
    println!("cargo:rerun-if-changed=../ui/dist");
    println!("cargo:rerun-if-changed=assets/hub-app.ico");
    winresource::WindowsResource::new()
        .set_icon("assets/hub-app.ico")
        .set_icon_with_id("assets/hub-app.ico", "204")
        .set("ProductName", "Panopticon Hub")
        .set("CompanyName", "Panopticon Trial")
        .set("FileDescription", "Panopticon Hub")
        .set("OriginalFilename", "PanopticonHub.exe")
        .compile()
        .expect("Windows version/icon");
}
