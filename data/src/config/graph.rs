use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coord {
    pub temp: u8,
    pub percent: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Graph {
    pub name: String,
    #[serde(rename = "coord")]
    pub coords: Vec<Coord>,
    pub input: String,       // Temp or CustomTemp
    pub output: Vec<String>, // Control
}
