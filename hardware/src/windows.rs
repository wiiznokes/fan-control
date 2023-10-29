use crate::HardwareBridge;

pub struct WindowsGenerator {}

impl HardwareBridge for WindowsBridge {
    fn new() -> impl HardwareBridge
    where
        Self: Sized,
    {
        Self {}
    }

    fn hardware(&self) -> data::config::Hardware {
        todo!()
    }

    fn value(&self, hardware_id: &str) -> Result<Option<i32>, crate::HardwareError> {
        todo!()
    }

    fn set_value(&self, hardware_id: &str, value: i32) -> Result<(), crate::HardwareError> {
        todo!()
    }

    fn info(&self, hardware_id: &str) -> Result<String, crate::HardwareError> {
        todo!()
    }
}
