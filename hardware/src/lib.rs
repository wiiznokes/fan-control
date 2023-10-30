#![allow(dead_code)]
#![allow(unused_variables)]

use serde::Serialize;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

#[derive(Debug, Clone)]
pub enum HardwareError {
    IdNotFound,
    LmSensors,
}

pub trait HardwareBridge {
    fn new() -> impl HardwareBridge
    where
        Self: Sized;

    fn hardware(&self) -> Hardware;

    fn value(&self, hardware_id: &str) -> Result<i32, HardwareError>;
    fn set_value(&self, hardware_id: &str, value: i32) -> Result<(), HardwareError>;

    fn info(&self, hardware_id: &str) -> Result<String, HardwareError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HardwareType {
    Control,
    Fan,
    Temp,
}

#[derive(Serialize, Debug, Clone)]
pub struct ControlH {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: String,

    #[serde(skip)]
    pub info: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct FanH {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: String,

    #[serde(skip)]
    pub info: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct TempH {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: String,
    #[serde(skip)]
    pub info: String,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Hardware {
    #[serde(default, rename = "Control")]
    pub controls: Vec<ControlH>,
    #[serde(default, rename = "Fan")]
    pub fans: Vec<FanH>,
    #[serde(default, rename = "Temp")]
    pub temps: Vec<TempH>,
}
