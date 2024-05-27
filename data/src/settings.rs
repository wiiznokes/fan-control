use light_enum::Values;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    #[serde(default)]
    pub theme: AppTheme,

    #[serde(default = "default_update_delay")]
    pub update_delay: u64,

    #[serde(default)]
    pub current_config: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, Default, Values)]
pub enum AppTheme {
    System,
    Dark,
    // todo: change default to system when dark mode is fixed
    #[default]
    Light,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Default::default(),
            update_delay: default_update_delay(),
            current_config: Default::default(),
        }
    }
}

fn default_update_delay() -> u64 {
    1500
}

impl Settings {
    pub fn current_config_text(&self) -> &str {
        match &self.current_config {
            Some(current_config) => current_config,
            None => "",
        }
    }
}

impl Display for AppTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AppTheme::System => fl!("system_theme"),
            AppTheme::Dark => fl!("dark_theme"),
            AppTheme::Light => fl!("light_theme"),
        };
        write!(f, "{}", str)
    }
}
