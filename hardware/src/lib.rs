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

pub type Value = i32;

pub trait HardwareBridge {
    fn new() -> impl HardwareBridge
    where
        Self: Sized;

    fn hardware(&self) -> Hardware;

    fn value(&self, internal_index: &usize) -> Result<Value, HardwareError>;
    fn set_value(&self, internal_index: &usize, value: Value) -> Result<(), HardwareError>;
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

    #[serde(skip)]
    pub internal_index: usize,
}

#[derive(Serialize, Debug, Clone)]
pub struct FanH {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: String,

    #[serde(skip)]
    pub info: String,

    #[serde(skip)]
    pub internal_index: usize,
}

#[derive(Serialize, Debug, Clone)]
pub struct TempH {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: String,
    #[serde(skip)]
    pub info: String,

    #[serde(skip)]
    pub internal_index: usize,
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

impl Hardware {
    pub fn get_internal_index(
        &self,
        hardware_id: &String,
        hardware_type: HardwareType,
    ) -> Option<usize> {
        match hardware_type {
            HardwareType::Control => self
                .controls
                .iter()
                .find(|control| &control.hardware_id == hardware_id)
                .map(|control| control.internal_index),
            HardwareType::Fan => self
                .fans
                .iter()
                .find(|fan| &fan.hardware_id == hardware_id)
                .map(|fan| fan.internal_index),
            HardwareType::Temp => self
                .temps
                .iter()
                .find(|temp| &temp.hardware_id == hardware_id)
                .map(|temp| temp.internal_index),
        }
    }
}
