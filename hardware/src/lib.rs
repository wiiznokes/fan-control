#![allow(dead_code)]
#![allow(unused_variables)]

use data::{config::Hardware, node::HardwareType};

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

#[derive(Debug, Clone)]
pub enum HardwareError {
    IdNotFound,
    WrongType,
    LmSensors,
}

pub trait HardwareGenerator<'a> {
    fn new() -> impl HardwareGenerator<'a>
    where
        Self: Sized;

    // lifetime workaround on Linux
    fn init<'b: 'a>(&'b mut self);

    fn validate(
        &self,
        hardware_type: &HardwareType,
        hardware_id: &str,
    ) -> Result<(), HardwareError>;

    fn hardware(&self) -> Hardware;

    fn value(&self, hardware_id: &str) -> Result<Option<i32>, HardwareError>;
    fn set_value(&self, hardware_id: &str, value: i32) -> Result<(), HardwareError>;

    fn info(&self, hardware_id: &str) -> Result<String, HardwareError>;
}
