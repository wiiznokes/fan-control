#![allow(dead_code)]
#![allow(unused_variables)]

use serde::Serialize;
use std::rc::Rc;

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
    fn new() -> (impl HardwareBridge, Hardware)
    where
        Self: Sized;

    fn value(&self, internal_index: &usize) -> Result<Value, HardwareError>;
    fn set_value(&self, internal_index: &usize, value: Value) -> Result<(), HardwareError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HardwareType {
    Control,
    Fan,
    Temp,
}

#[derive(Debug, Clone)]
pub struct InternalControlIndex {
    pub io: usize,
    pub enable: usize,
}

#[derive(Serialize, Debug, Clone)]
pub struct ControlH {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: String,

    #[serde(skip)]
    pub info: String,

    #[serde(skip)]
    pub internal_index: InternalControlIndex,
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
    pub controls: Vec<Rc<ControlH>>,
    #[serde(default, rename = "Fan")]
    pub fans: Vec<Rc<FanH>>,
    #[serde(default, rename = "Temp")]
    pub temps: Vec<Rc<TempH>>,
}
