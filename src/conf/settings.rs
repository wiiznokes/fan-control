use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default)]
    pub unit: Unit,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum Unit {
    Fahrenheit,
    #[default]
    Celsius,
}
