use serde::{Deserialize, Serialize};

use crate::node::IsValid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Linear {
    pub name: String,
    #[serde(rename = "minTemp", alias = "min_temp")]
    pub min_temp: u8,
    #[serde(rename = "minSpeed", alias = "min_speed")]
    pub min_speed: u8,
    #[serde(rename = "maxTemp", alias = "max_temp")]
    pub max_temp: u8,
    #[serde(rename = "maxSpeed", alias = "max_speed")]
    pub max_speed: u8,
    pub input: Option<String>, // Temp or CustomTemp
}

impl IsValid for Linear {
    fn is_valid(&self) -> bool {
        self.input.is_some()
    }
}
