use serde::{Deserialize, Serialize};

use super::IsValid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Target {
    pub name: String,
    #[serde(rename = "idleTemp", alias = "idle_temp")]
    pub idle_temp: u8,
    #[serde(rename = "idleSpeed", alias = "idle_speed")]
    pub idle_speed: u8,
    #[serde(rename = "loadTemp", alias = "load_temp")]
    pub load_temp: u8,
    #[serde(rename = "loadSpeed", alias = "load_speed")]
    pub load_speed: u8,
    pub input: Option<String>, // Temp or CustomTemp
}

impl IsValid for Target {
    fn is_valid(&self) -> bool {
        self.input.is_some()
    }
}
