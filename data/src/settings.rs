use strum::Display;
use strum::EnumIter;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    #[serde(default)]
    pub theme: AppTheme,

    #[serde(default)]
    pub unit: Unit,

    #[serde(default = "default_update_delay")]
    pub update_delay: u64,

    #[serde(default)]
    pub current_config: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum Unit {
    #[default]
    Celsius,
    Fahrenheit,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, Default, EnumIter, Display)]
pub enum AppTheme {
    #[default]
    System,
    Dark,
    Light,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Default::default(),
            unit: Default::default(),
            update_delay: default_update_delay(),
            current_config: Default::default(),
        }
    }
}

fn default_update_delay() -> u64 {
    2500
}

impl Settings {
    pub fn current_config_text(&self) -> &str {
        match &self.current_config {
            Some(current_config) => current_config,
            None => "None",
        }
    }
}
