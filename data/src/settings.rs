use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub unit: Unit,

    pub update_delay: u64,

    pub disable_pwm_value: u8,

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
            update_delay: 1000,
            disable_pwm_value: 2,
            current_config: Default::default(),
        }
    }
}

