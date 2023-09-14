use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hardware<Output: ?Sized + FetchHardware> {
    #[serde(default, rename = "Control")]
    pub controls: Vec<Control>,
    #[serde(default, rename = "Temp")]
    pub temps: Vec<Box<Temp<Output>>>,
    #[serde(default, rename = "Fan")]
    pub fans: Vec<Box<Fan<Output>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Control {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Temp<S: ?Sized + FetchHardware> {
    pub name: String,

    #[serde(skip)]
    pub sensor: S,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fan<S: ?Sized + FetchHardware> {
    pub name: String,

    #[serde(skip)]
    pub sensor: S,
}

pub trait FetchHardware {
    fn get_value(&self) -> i32;

    fn new(name: String) -> impl FetchHardware;
}

pub trait SetHardware {
    fn set_value(value: i32);
}

pub trait HardwareGenerator {

    type Output: ?Sized + FetchHardware;

    fn new() -> impl HardwareGenerator;

    fn generate_controls(&self) -> Vec<Control>;
    fn generate_temps(&self) -> Vec<Box<Temp<Self::Output>>>;
    fn generate_fans(&self) -> Vec<Box<Fan<Self::Output>>>;
}