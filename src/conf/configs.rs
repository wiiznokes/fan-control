use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default, rename = "CustomTemp")]
    pub custom_temps: Vec<CustomTemp>,
    #[serde(default, rename = "Graph")]
    pub graphs: Vec<Graph>,
    #[serde(default, rename = "Flat")]
    pub flats: Vec<Flat>,
    #[serde(default, rename = "Linear")]
    pub linears: Vec<Linear>,
    #[serde(default, rename = "Target")]
    pub targets: Vec<Target>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CustomTempType {
    Min,
    Max,
    Average,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomTemp {
    pub name: String,
    pub kind: CustomTempType,
    pub input: Vec<String>, // Temp or CustomTemp
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flat {
    pub name: String,
    pub value: i32,
    pub output: Vec<String>, // Control
}

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
    pub input: String,       // Temp or CustomTemp
    pub output: Vec<String>, // Control
}

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
    pub input: String,       // Temp or CustomTemp
    pub output: Vec<String>, // Control
}
