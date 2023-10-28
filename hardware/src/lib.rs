#![allow(dead_code)]

use data::{
    node::HardwareType,
    serde::hardware::{Control, Fan, Temp},
};

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

#[derive(Debug, Clone)]
pub enum HardwareError {}

pub trait HardwareGenerator<'a> {
    fn new() -> impl HardwareGenerator<'a>
    where
        Self: Sized;

    fn init<'b: 'a>(&'b mut self);

    fn validate(
        &self,
        hardware_type: &HardwareType,
        hardware_id: &String,
    ) -> Result<(), HardwareError>;

    fn controls(&self) -> Vec<Control>;
    fn temps(&self) -> Vec<Temp>;
    fn fans(&self) -> Vec<Fan>;

    fn value(hardware_id: &String) -> Result<Option<i32>, HardwareError>;

    fn set_value(hardware_id: &String, value: i32) -> Result<(), HardwareError>;
}
