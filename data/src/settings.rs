use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    #[serde(default)]
    pub unit: Unit,

    #[serde(default = "default_update_delay")]
    pub update_delay: u64,

    #[serde(default)]
    pub current_config: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum Unit {
    Fahrenheit,
    #[default]
    Celsius,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            unit: Default::default(),
            update_delay: default_update_delay(),
            current_config: Default::default(),
        }
    }
}

fn default_update_delay() -> u64 {
    2500
}