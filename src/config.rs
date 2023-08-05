use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub unit: Unit,
    pub fans: Vec<Fan>,
    pub temps: Vec<Temp>,
    pub controls: Vec<Control>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Unit {
    F, // Fahrenheit
    C, // Celsius
}

// fan = control
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fan {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Temp {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Control {
    TempMath(TempMath),
    Graph(Graph),
    Flat(Flat),
    Linear(Linear),
    Target(Target),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TempMathType {
    Min,
    Max,
    Average,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TempMath {
    pub name: String,
    pub kind: TempMathType,
    pub input: Vec<String>, // Temp or TempMath
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coord {
    pub temp: i32,
    pub percent: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Graph {
    pub name: String,
    pub coord: Vec<Coord>,
    pub input: String,       // Temp or TempMath
    pub output: Vec<String>, // Fan
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flat {
    pub name: String,
    pub value: i32,
    pub output: Vec<String>, // Fan
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Linear {
    pub name: String,
    pub min: Coord,
    pub max: Coord,
    pub input: String,       // Temp or TempMath
    pub output: Vec<String>, // Fan
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Target {
    pub name: String,
    pub ideal: Coord,
    pub load: Coord,
    pub input: String,       // Temp or TempMath
    pub output: Vec<String>, // Fan
}
