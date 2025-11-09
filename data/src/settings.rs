use light_enum::Values;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct SettingsState {
    pub show_flatpak_dialog: bool,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            show_flatpak_dialog: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Settings {
    pub theme: AppTheme,
    pub update_delay: u64,
    pub current_config: Option<String>,
    pub start_at_login: bool,
    pub inactive: bool,
}

// todo: find a better solution to expose themes
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, Default, Values)]
pub enum AppTheme {
    System,
    Dark,
    HighContrastDark,
    HighContrastLight,
    // todo: change default to system when dark mode is fixed
    #[default]
    Light,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Default::default(),
            update_delay: 1500,
            current_config: Default::default(),
            start_at_login: false,
            inactive: false,
        }
    }
}

impl Display for AppTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AppTheme::System => fl!("system_theme"),
            AppTheme::Dark => fl!("dark_theme"),
            AppTheme::Light => fl!("light_theme"),
            AppTheme::HighContrastDark => fl!("hight_contrast_dark_theme"),
            AppTheme::HighContrastLight => fl!("hight_contrast_light_theme"),
        };
        write!(f, "{str}")
    }
}
