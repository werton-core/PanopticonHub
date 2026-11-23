fn main() {
    const ICON: &str = "../host/assets/hub-app.ico";
    println!("cargo:rerun-if-changed={ICON}");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        winresource::WindowsResource::new()
            .set_icon(ICON)
            .compile()
            .expect("Не удалось собрать значок окна Panopticon Trial");
    }
}
