use crate::design;
use std::path::PathBuf;
use tao::{
    platform::windows::{IconExtWindows, WindowExtWindows},
    window::{Icon, Window},
};
use tray_icon::{menu::MenuItem, TrayIcon};
use windows::{
    core::GUID,
    Win32::{
        Foundation::{HWND, PROPERTYKEY},
        System::Com::StructuredStorage::PROPVARIANT,
        UI::Shell::PropertiesSystem::{IPropertyStore, SHGetPropertyStoreForWindow},
    },
};

pub const HUB_NAME: &str = "Panopticon Hub";
const APP_MODEL_GUID: GUID = GUID::from_u128(0x9f4c2855_9f79_4b39_a8d0_e1d42de1d5f3);
const ICON: &[u8] = include_bytes!("../assets/hub-app.rgba");

pub struct Identity {
    pub title: String,
    pub icon: Option<PathBuf>,
}

impl Identity {
    pub fn from_profile(profile: &design::Profile) -> Self {
        Self {
            title: profile.hub_name.as_deref().unwrap_or(HUB_NAME).to_owned(),
            icon: profile
                .assets
                .get("hubIcon")
                .and_then(|key| design::media(key).ok()),
        }
    }

    pub fn window_icon(&self) -> Option<Icon> {
        self.icon
            .as_ref()
            .and_then(|path| Icon::from_path(path, None).ok())
            .or_else(|| Icon::from_rgba(ICON.to_vec(), 32, 32).ok())
    }

    pub fn tray_icon(&self) -> Result<tray_icon::Icon, String> {
        self.icon
            .as_ref()
            .and_then(|path| tray_icon::Icon::from_path(path, Some((32, 32))).ok())
            .map_or_else(
                || {
                    tray_icon::Icon::from_rgba(ICON.to_vec(), 32, 32)
                        .map_err(|error| error.to_string())
                },
                Ok,
            )
    }

    pub fn update_taskbar(&self, window: &Window) -> windows::core::Result<()> {
        let Ok(exe) = std::env::current_exe() else {
            return Ok(());
        };
        let command = format!(r#""{}""#, exe.display());
        let name = if self.title.starts_with('@') {
            format!("\u{200b}{}", self.title)
        } else {
            self.title.clone()
        };
        let icon = self.icon.as_ref().map_or_else(
            || format!("{},-204", exe.display()),
            |path| format!("{},0", path.display()),
        );
        let store: IPropertyStore =
            unsafe { SHGetPropertyStoreForWindow(HWND(window.hwnd() as _))? };
        for (pid, value) in [
            (2, command.as_str()),
            (3, icon.as_str()),
            (4, name.as_str()),
            (5, "org.panopticon.hub"),
        ] {
            unsafe {
                store.SetValue(
                    &PROPERTYKEY {
                        fmtid: APP_MODEL_GUID,
                        pid,
                    },
                    &PROPVARIANT::from(value),
                )?
            };
        }
        Ok(())
    }

    pub fn clear_taskbar(window: &Window) {
        let store: windows::core::Result<IPropertyStore> =
            unsafe { SHGetPropertyStoreForWindow(HWND(window.hwnd() as _)) };
        if let Ok(store) = store {
            for pid in [5, 4, 3, 2] {
                let _ = unsafe {
                    store.SetValue(
                        &PROPERTYKEY {
                            fmtid: APP_MODEL_GUID,
                            pid,
                        },
                        &PROPVARIANT::default(),
                    )
                };
            }
        }
    }

    pub fn tray_label(&self) -> String {
        format!("Открыть {}", self.title.replace('&', "&&"))
    }

    pub fn apply(&self, window: &Window, tray: &TrayIcon, show: &MenuItem) {
        window.set_title(&self.title);
        let _ = self.update_taskbar(window);
        let icon = self.window_icon();
        window.set_window_icon(icon.clone());
        window.set_taskbar_icon(icon);
        let _ = tray.set_tooltip(Some(&self.title));
        if let Ok(icon) = self.tray_icon() {
            let _ = tray.set_icon(Some(icon));
        }
        show.set_text(self.tray_label());
    }
}
