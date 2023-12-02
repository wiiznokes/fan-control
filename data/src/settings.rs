use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {

    #[serde(default)]
    pub unit: Unit,

    #[serde(default = "default_update_delay")]
    pub update_delay: u64,

    #[serde(default = "default_disable_pwm_value")]
    pub disable_pwm_value: u8,

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
            disable_pwm_value: default_disable_pwm_value(),
            current_config: Default::default(),
        }
    }
}

fn default_update_delay() -> u64 {
    1000
}

fn default_disable_pwm_value() -> u8 {
    2
}