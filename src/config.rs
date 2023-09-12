use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hardware {
    pub controls: Vec<Control>,
    pub temps: Vec<Temp>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(rename(serialize = "CustomTemp"))]
    pub custom_temps: Vec<CustomTemp>,
    #[serde(rename(serialize = "Graph"))]
    pub graphs: Vec<Graph>,
    #[serde(rename(serialize = "Flat"))]
    pub flats: Vec<Flat>,
    #[serde(rename(serialize = "Linear"))]
    pub linears: Vec<Linear>,
    #[serde(rename(serialize = "Target"))]
    pub targets: Vec<Target>,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Control {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Temp {
    pub name: String,
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
    #[serde(rename(serialize = "Coord"))]
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
    pub min_temp: u8,
    pub min_speed: u8,
    pub max_temp: u8,
    pub max_speed: u8,
    pub input: String,       // Temp or CustomTemp
    pub output: Vec<String>, // Control
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Target {
    pub name: String,
    pub idle_temp: u8,
    pub idle_speed: u8,
    pub load_temp: u8,
    pub load_speed: u8,
    pub input: String,       // Temp or CustomTemp
    pub output: Vec<String>, // Control
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Unit {
    Fahrenheit,
    Celsius,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Behavior {
    CustomTemp(CustomTemp),
    Graph(Graph),
    Flat(Flat),
    Linear(Linear),
    Target(Target),
}
