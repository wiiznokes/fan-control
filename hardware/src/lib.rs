use derive_more::Display;
use serde::Serialize;
use std::{fmt::Debug, rc::Rc, time::Duration};
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

#[derive(Serialize, Debug, Clone, Eq)]
pub struct HItem {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: String,

    #[serde(skip)]
    pub info: String,

    #[serde(skip)]
    pub internal_index: usize,
}

impl ToString for HItem {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl PartialEq for HItem {
    fn eq(&self, other: &Self) -> bool {
        self.internal_index == other.internal_index
    }
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Hardware {
    #[serde(default, rename = "Control")]
    pub controls: Vec<Rc<HItem>>,
    #[serde(default, rename = "Fan")]
    pub fans: Vec<Rc<HItem>>,
    #[serde(default, rename = "Temp")]
    pub temps: Vec<Rc<HItem>>,
}

pub type Value = i32;

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum Mode {
    Auto,
    Manual,
    Specific(Value),
}

/// Try to construct a new hardware bridge
pub fn new() -> Result<impl HardwareBridge> {
    #[cfg(feature = "fake_hardware")]
    return fake_hardware::FakeHardwareBridge::new();

    #[cfg(all(not(feature = "fake_hardware"), target_os = "windows"))]
    return windows::WindowsBridge::new();

    #[cfg(all(not(feature = "fake_hardware"), target_os = "linux"))]
    return linux::LinuxBridge::new();
}

pub trait HardwareBridge {
    /// Approximative time to update sensors on my pc
    const TIME_TO_UPDATE: Duration = Duration::from_millis(0);

    fn new() -> Result<Self>
    where
        Self: Sized;

    fn hardware(&self) -> &Hardware;

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
