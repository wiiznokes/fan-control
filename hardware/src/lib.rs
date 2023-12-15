#![allow(dead_code)]
#![allow(unused_variables)]

use serde::Serialize;
use std::{fmt::Debug, rc::Rc};

#[macro_use]
extern crate log;

#[cfg(all(not(feature = "fake_hardware"), target_os = "linux"))]
pub mod linux;

#[cfg(all(not(feature = "fake_hardware"), target_os = "windows"))]
pub mod windows;

#[cfg(feature = "fake_hardware")]
pub mod fake_hardware;

#[derive(Debug, Clone, PartialEq)]
pub enum HardwareError {
    InternalIndexNotFound,
    LmSensors(String),
}

pub type Value = i32;
pub type HardwareBridgeT = Box<dyn HardwareBridge>;
pub trait HardwareBridge {
    fn generate_hardware() -> (Hardware, HardwareBridgeT)
    where
        Self: Sized;

    fn get_value(&mut self, internal_index: &usize) -> Result<Value, HardwareError>;
    fn set_value(&mut self, internal_index: &usize, value: Value) -> Result<(), HardwareError>;
    fn set_mode(&mut self, internal_index: &usize, value: Value) -> Result<(), HardwareError>;

    // use on Windows, because we update all sensors in one function, so
    // we don't want to update at each call, instead, we call this function
    // one time in each update iteration
    fn update(&mut self) -> Result<(), HardwareError> {
        Ok(())
    }

    // use on Windows to shutdown the server properly
    fn shutdown(&mut self) -> Result<(), HardwareError> {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HardwareType {
    Control,
    Fan,
    Temp,
}

#[derive(Serialize, Debug)]
pub struct ControlH {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: String,

    #[serde(skip)]
    pub info: String,

    #[serde(skip)]
    pub internal_index: usize,
}

#[derive(Serialize, Debug)]
pub struct FanH {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: String,

    #[serde(skip)]
    pub info: String,

    #[serde(skip)]
    pub internal_index: usize,
}

#[derive(Serialize, Debug)]
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
