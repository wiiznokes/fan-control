use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hardware<S> {
    #[serde(default, rename = "Control")]
    pub controls: Vec<Control>,
    #[serde(default, rename = "Temp")]
    pub temps: Vec<Temp<S>>,
    #[serde(default, rename = "Fan")]
    pub fans: Vec<Fan<S>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Control {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Temp<S> {
    pub name: String,

    #[serde(skip)]
    pub sensor: S

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fan<S> {
    pub name: String,

    #[serde(skip)]
    pub sensor: S
}


pub trait FetchHardware {

    fn get_value() -> i32;
}


pub trait SetHardware {
    
    fn set_value(value: i32);
}



pub trait HardwareGenerator {
    
    fn generate_controls(&self) -> Vec<Control>;
    fn generate_temps(&self) -> Vec<Temp>;
    fn generate_fans(&self) -> Vec<Fan>;
}