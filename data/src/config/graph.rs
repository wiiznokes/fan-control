use serde::{Deserialize, Serialize};

use crate::node::IsValid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coord {
    pub temp: u8,
    pub percent: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Graph {
    pub name: String,
    #[serde(rename = "coord")]
    pub coords: Vec<Coord>,
    pub input: Option<String>, // Temp or CustomTemp
}

impl IsValid for Graph {
    fn is_valid(&self) -> bool {
        self.input.is_some() //TODO: add conditions on coords
    }
}
