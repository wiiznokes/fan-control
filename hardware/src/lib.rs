//#![allow(dead_code)]
//#![allow(unused_variables)]

use serde::Serialize;
use std::{fmt::Debug, rc::Rc};
use thiserror::Error;

#[macro_use]
extern crate log;

#[cfg(all(not(feature = "fake_hardware"), target_os = "linux"))]
pub mod linux;

#[cfg(all(not(feature = "fake_hardware"), target_os = "windows"))]
pub mod windows;

#[cfg(feature = "fake_hardware")]
pub mod fake_hardware;

#[derive(Error, Debug)]
pub enum HardwareError {
    #[error("Internal index not found")]
    InternalIndexNotFound,
    #[cfg(all(not(feature = "fake_hardware"), target_os = "linux"))]
    #[error(transparent)]
    LmSensors(String),
    #[cfg(all(not(feature = "fake_hardware"), target_os = "windows"))]
    #[error(transparent)]
    Windows(#[from] windows::WindowsError),
}

type Result<T> = std::result::Result<T, HardwareError>;

pub type Value = i32;
pub type HardwareBridgeT = Box<dyn HardwareBridge>;
pub trait HardwareBridge {
    fn generate_hardware() -> Result<(Hardware, HardwareBridgeT)>
    where
        Self: Sized;

    fn get_value(&mut self, internal_index: &usize) -> Result<Value>;
    fn set_value(&mut self, internal_index: &usize, value: Value) -> Result<()>;
    fn set_mode(&mut self, internal_index: &usize, value: Value) -> Result<()>;

    // use on Windows, because we update all sensors in one function, so
    // we don't want to update at each call, instead, we call this function
    // one time in each update iteration
    fn update(&mut self) -> Result<()> {
        Ok(())
    }

    // use on Windows to shutdown the server properly
    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
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
