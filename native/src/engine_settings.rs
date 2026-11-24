use super::video::client_command;
use std::{collections::BTreeMap, ffi::CString, fmt::Write};

const VARIABLES: &[&str] = &[
    "sensitivity", "volume", "r_flashlightdepthtexture", "r_waterforceexpensive",
    "mat_hdr_level", "mat_picmip", "r_shadowrendertotexture", "mat_vsync", "m_rawinput",
    "volume_sfx", "mat_forceaniso", "mat_motion_blur_enabled", "mat_antialias", "mat_bumpmap",
    "r_waterforcereflectentities", "m_customaccel", "snd_mute_losefocus", "fov_desired",
    "m_pitch", "r_rootlod", "mat_specular", "m_filter", "fps_max", "mat_trilinear",
    "voice_enable", "voice_scale",
];

fn binding_commands(raw: &[u8]) -> Option<CString> {
    let changes: BTreeMap<String, String> = serde_json::from_slice(raw).ok()?;
    let mut commands = String::new();
    for (key, action) in changes {
        if !(1..=30).contains(&key.len())
            || key.eq_ignore_ascii_case("escape")
            || !key.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || b"_`[]'/.,=+-;\\".contains(&byte)
            })
        {
            return None;
        }
        if action.is_empty() {
            writeln!(commands, "unbind \"{key}\"").ok()?;
        } else if hub_backend::bindings::engine_action(&action) {
            writeln!(commands, "bind \"{key}\" \"{action}\"").ok()?;
        } else {
            return None;
        }
    }
    if !commands.is_empty() {
        commands.push_str("host_writeconfig\n");
    }
    CString::new(commands).ok()
}

pub(super) fn apply_bindings(raw: &[u8]) -> Option<()> {
    let commands = binding_commands(raw)?;
    if commands.as_bytes().is_empty() {
        return Some(());
    }
    client_command(&commands)
}

fn variable_commands(raw: &[u8]) -> Option<CString> {
    let changes: BTreeMap<String, f64> = serde_json::from_slice(raw).ok()?;
    let mut commands = String::new();
    for (name, value) in changes {
        if !VARIABLES.contains(&name.as_str()) || !value.is_finite() || value.abs() >= 100000.0 {
            return None;
        }
        writeln!(commands, "{name} {value}").ok()?;
    }
    CString::new(commands).ok()
}

pub(super) fn apply_variables(raw: &[u8]) -> Option<()> {
    let commands = variable_commands(raw)?;
    if commands.as_bytes().is_empty() {
        return Some(());
    }
    client_command(&commands)
}

pub(super) fn save(video: bool) -> Option<()> {
    client_command(if video {
        c"mat_savechanges\nhost_writeconfig\n"
    } else {
        c"host_writeconfig\n"
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_only_explicit_stock_binding_commands() {
        assert_eq!(
            binding_commands(br#"{"n":"+menu","q":"","mouse1":"+attack"}"#)
                .unwrap()
                .to_str()
                .unwrap(),
            "bind \"mouse1\" \"+attack\"\nbind \"n\" \"+menu\"\nunbind \"q\"\nhost_writeconfig\n"
        );
        for rejected in [
            br#"{"q":"quit"}"#.as_slice(),
            br#"{"escape":""}"#,
            br#"{"q\";quit":"+menu"}"#,
            br#"{"q\nquit":"+menu"}"#,
        ] {
            assert!(binding_commands(rejected).is_none());
        }
    }

    #[test]
    fn accepts_only_numeric_settings_from_the_hub() {
        assert_eq!(
            variable_commands(br#"{"sensitivity":2.5,"voice_enable":0}"#)
                .unwrap()
                .to_str()
                .unwrap(),
            "sensitivity 2.5\nvoice_enable 0\n"
        );
        assert!(variable_commands(br#"{"sensitivity":"2;quit"}"#).is_none());
        assert!(variable_commands(br#"{"mat_setvideomode":2}"#).is_none());
    }
}
