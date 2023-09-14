
use super::hardware::{HardwareGenerator, Temp, FetchHardware, Control, Fan};

pub struct LHMGenerator {}



impl <'a>HardwareGenerator<'a> for LHMGenerator {
    type Output = LHMSensor;

    fn new() -> impl HardwareGenerator<'a> {
        Self {}
    }

    fn generate_controls(&self) -> Vec<Control> {
        todo!()
    }

    fn generate_temps(&self) -> Vec<Box<Temp<'a, Self::Output>>> {
        todo!()
    }

    fn generate_fans(&self) -> Vec<Box<Fan<'a, Self::Output>>> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct LHMSensor {

}

impl FetchHardware for LHMSensor {
    fn get_value(&self) -> i32 {
        todo!()
    }

    fn new(name: String) -> impl FetchHardware {
        
        Self {}
    }
}