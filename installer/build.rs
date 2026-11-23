mod languages;
use sha2::{Digest, Sha256};
use std::{env, fs, path::PathBuf};

fn read_exe(variable: &str, path: std::ffi::OsString) -> Vec<u8> {
    let path = PathBuf::from(path);
    println!("cargo:rerun-if-changed={}", path.display());
    let bytes = fs::read(path).unwrap_or_else(|error| panic!("Не удалось прочитать {variable}: {error}"));
    assert!(bytes.len() >= 512 && &bytes[..2] == b"MZ", "{variable} должен быть настоящим Windows EXE");
    let pe = u32::from_le_bytes(bytes[60..64].try_into().unwrap()) as usize;
    assert!(bytes.get(pe..pe + 4) == Some(b"PE\0\0"), "{variable} имеет неверный PE-заголовок");
    assert!(bytes.get(pe + 4..pe + 6) == Some(&[0x64, 0x86]), "{variable} должен быть Windows x86-64");
    bytes
}

fn main() {
    println!("cargo:rerun-if-env-changed=PANOPTICON_HOST_EXE");
    println!("cargo:rerun-if-env-changed=PANOPTICON_SETUP_TEST_PAYLOAD");
    println!("cargo:rerun-if-env-changed=PANOPTICON_SETUP_UNINSTALL_EXE");
    println!("cargo:rerun-if-env-changed=PANOPTICON_SETUP_UNINSTALL_ONLY");
    let output = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let uninstall_only = env::var("PANOPTICON_SETUP_UNINSTALL_ONLY").as_deref() == Ok("1");
    let fixture = env::var("PANOPTICON_SETUP_TEST_PAYLOAD").as_deref() == Ok("1");
    let host = env::var_os("PANOPTICON_HOST_EXE");
    let uninstaller = env::var_os("PANOPTICON_SETUP_UNINSTALL_EXE");
    let (bytes, uninstall_bytes) = match (uninstall_only, fixture, host, uninstaller) {
        (true, false, None, None) | (false, true, None, None) => (Vec::new(), Vec::new()),
        (false, false, Some(host), Some(uninstaller)) => (
            read_exe("PANOPTICON_HOST_EXE", host),
            read_exe("PANOPTICON_SETUP_UNINSTALL_EXE", uninstaller),
        ),
        _ => panic!("Соберите деинсталлятор с PANOPTICON_SETUP_UNINSTALL_ONLY=1, тесты с PANOPTICON_SETUP_TEST_PAYLOAD=1 или задайте PANOPTICON_HOST_EXE и PANOPTICON_SETUP_UNINSTALL_EXE для установщика."),
    };
    let digest = format!("{:x}", Sha256::digest(&bytes));
    fs::write(output.join("payload.exe"), bytes).unwrap();
    fs::write(output.join("uninstaller.exe"), uninstall_bytes).unwrap();
    println!("cargo:rustc-env=PANOPTICON_PAYLOAD_SHA256={digest}");
    println!("cargo:rustc-env=PANOPTICON_PAYLOAD_FIXTURE={fixture}");
    println!("cargo:rustc-env=PANOPTICON_SETUP_UNINSTALL_ONLY={uninstall_only}");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        languages::compile(&output);
        let project = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
        let resources = project.join("resources");
        let icon = project.join("../host/assets/hub.ico");
        println!("cargo:rerun-if-changed={}", icon.display());
        println!("cargo:rerun-if-changed={}", resources.display());
        let mut dialogs = fs::read_to_string(resources.join("wizard.rc")).unwrap()
            .replace("resources/modern-wizard.bmp", &resources.join("modern-wizard.bmp").display().to_string().replace('\\', "/"));
        for id in [103, 105, 106] {
            let path = resources.join(format!("{id}.dlg")).display().to_string().replace('\\', "/");
            dialogs.push_str(&format!("\n{id} RCDATA \"{path}\"\n"));
        }
        winresource::WindowsResource::new()
            .set_icon_with_id(icon.to_str().unwrap(), "103")
            .append_rc_content(&dialogs)
            .set("ProductName", "Panopticon Hub")
            .set("CompanyName", "Panopticon Trial")
            .set("InternalName", if uninstall_only { "PanopticonHubUninstall" } else { "PanopticonHubSetup" })
            .set("FileDescription", if uninstall_only { "Удаление Panopticon Hub" } else { "Установка Panopticon Hub" })
            .set("OriginalFilename", if uninstall_only { "Uninstall Panopticon Hub.exe" } else { "PanopticonHubSetup.exe" })
            .set_manifest(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0"><dependency><dependentAssembly><assemblyIdentity type="win32" name="Microsoft.Windows.Common-Controls" version="6.0.0.0" processorArchitecture="*" publicKeyToken="6595b64144ccf1df" language="*"/></dependentAssembly></dependency><trustInfo xmlns="urn:schemas-microsoft-com:asm.v3"><security><requestedPrivileges><requestedExecutionLevel level="asInvoker" uiAccess="false"/></requestedPrivileges></security></trustInfo><compatibility xmlns="urn:schemas-microsoft-com:compatibility.v1"><application><supportedOS Id="{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}"/></application></compatibility></assembly>"#)
            .compile().expect("Не удалось собрать ресурсы установщика");
    }
}
