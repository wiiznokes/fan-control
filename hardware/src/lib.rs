//#![allow(dead_code)]
//#![allow(unused_variables)]

use derive_more::Display;
use serde::Serialize;
use std::{fmt::Debug, rc::Rc};
use thiserror::Error;

#[macro_use]
extern crate log;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(feature = "fake_hardware")]
pub mod fake_hardware;

#[derive(Error, Debug)]
pub enum HardwareError {
    #[error("Internal index not found")]
    InternalIndexNotFound,
    #[cfg(target_os = "linux")]
    #[error(transparent)]
    Linux(#[from] linux::LinuxError),
    #[cfg(target_os = "windows")]
    #[error(transparent)]
    Windows(#[from] windows::WindowsError),
}

type Result<T> = std::result::Result<T, HardwareError>;

#[derive(Serialize, Debug, Clone, Default)]
pub struct Hardware {
    #[serde(default, rename = "Control")]
    pub controls: Vec<Rc<ControlH>>,
    #[serde(default, rename = "Fan")]
    pub fans: Vec<Rc<FanH>>,
    #[serde(default, rename = "Temp")]
    pub temps: Vec<Rc<TempH>>,
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

pub type Value = i32;

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum Mode {
    Auto,
    Manual,
    Specific(Value),
}

pub trait HardwareBridge {
    fn generate_hardware() -> Result<(Hardware, impl HardwareBridge)>
    where
        Self: Sized;

    fn get_value(&mut self, internal_index: &usize) -> Result<Value>;
    fn set_value(&mut self, internal_index: &usize, value: Value) -> Result<()>;
    fn set_mode(&mut self, internal_index: &usize, mode: &Mode) -> Result<()>;

    /// Used on Windows, because we update all sensors in one function, so
    /// we don't want to update at each call, instead, we call this function
    /// one time in each update iteration.
    fn update(&mut self) -> Result<()> {
        Ok(())
    }

    /// Used on Windows to shutdown the server properly.
    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

pub trait HardwareInfoTrait {
    fn name(&self) -> &String;
    fn id(&self) -> &String;
    fn info(&self) -> &String;
}

impl HardwareInfoTrait for ControlH {
    fn name(&self) -> &String {
        &self.name
    }

    fn id(&self) -> &String {
        &self.hardware_id
    }

    fn info(&self) -> &String {
        &self.info
    }
}

impl HardwareInfoTrait for FanH {
    fn name(&self) -> &String {
        &self.name
    }

    fn id(&self) -> &String {
        &self.hardware_id
    }

    fn info(&self) -> &String {
        &self.info
    }
}

impl HardwareInfoTrait for TempH {
    fn name(&self) -> &String {
        &self.name
    }

    fn id(&self) -> &String {
        &self.hardware_id
    }

    fn info(&self) -> &String {
        &self.info
    }
}
