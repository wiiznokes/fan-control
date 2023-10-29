use crate::HardwareGenerator;




pub struct WindowsGenerator {
    
}



impl HardwareGenerator for WindowsGenerator {

    fn new() -> impl HardwareGenerator
    where
        Self: Sized {
        Self {}
    }

    fn validate(
        &self,
        hardware_type: &data::node::HardwareType,
        hardware_id: &str,
    ) -> Result<(), crate::HardwareError> {
        todo!()
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

