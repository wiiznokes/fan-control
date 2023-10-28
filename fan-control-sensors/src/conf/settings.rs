use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Settings {
    #[serde(default)]
    pub unit: Unit,

    #[serde(default = "one")]
    pub update_delay: u8,

    #[serde(default = "two")]
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

fn one() -> u8 {
    1
}

fn two() -> u8 {
    2
}
