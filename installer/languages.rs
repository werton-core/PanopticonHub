use std::collections::BTreeMap;
use std::{fs, path::Path};

pub fn compile(output: &Path) {
    let mut tables = BTreeMap::<(bool, u16), BTreeMap<u8, String>>::new();
    for (uninstall, path) in [
        (false, "reference/[NSIS].nsi"),
        (true, "reference/uninstall/[NSIS].nsi"),
    ] {
        println!("cargo:rerun-if-changed={path}");
        let source = fs::read_to_string(path).expect("Missing recovered NSIS language tables");
        for row in source
            .lines()
            .filter_map(|line| line.strip_prefix("LangString LSTR_"))
        {
            let mut parts = row.splitn(3, ' ');
            let id: u8 = parts.next().unwrap().parse().unwrap();
            let lang: u16 = parts.next().unwrap().parse().unwrap();
            if id > 4 && !(32..=35).contains(&id) && id < 37 {
                continue;
            }
            let raw = parts.next().unwrap().trim();
            let value = raw
                .strip_prefix('"')
                .and_then(|v| v.strip_suffix('"'))
                .unwrap_or(raw)
                .replace(r#"$\""#, "\"")
                .replace(r"$\r", "\r")
                .replace(r"$\n", "\n")
                .replace(
                    "Shinri Hub 26.11.23196",
                    concat!("Panopticon Hub ", env!("CARGO_PKG_VERSION")),
                )
                .replace("Shinri Hub", "Panopticon Hub");
            tables
                .entry((uninstall, lang))
                .or_default()
                .insert(id, value);
        }
    }
    let mut generated = String::from("pub static TABLES: &[(bool, u16, &[&str])] = &[\n");
    for (&(uninstall, lang), values) in &tables {
        generated.push_str(&format!("({uninstall}, {lang}, &["));
        for id in 0..=76 {
            let mut text = values.get(&id).cloned().unwrap_or_default();
            for nested in [2, 70, 76] {
                if let Some(name) = values.get(&nested) {
                    text = text.replace(&format!("$(LSTR_{nested})"), name);
                }
            }
            if text.contains("$_CLICK") {
                let click = if uninstall { 63 } else { 69 };
                text = text.replace("$_CLICK", &values[&click]);
            }
            generated.push_str(&format!("{text:?},"));
        }
        generated.push_str("]),\n");
    }
    generated.push_str("];\n");
    fs::write(output.join("wizard_languages.rs"), generated).unwrap();
}
