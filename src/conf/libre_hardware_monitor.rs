
use super::hardware::{HardwareGenerator, Temp, FetchHardware, Control, Fan};

pub struct LHMGenerator {}



impl HardwareGenerator for LHMGenerator {

    type Output = LHMSensor;
    
    fn new() -> impl HardwareGenerator {
        return Self{ };
    }

    fn generate_controls(&self) -> Vec<Control> {
        todo!()
    }

    fn generate_temps(&self) -> Vec<Box<Temp<LHMSensor>>> {
        todo!()
    }

    fn generate_fans(&self) -> Vec<Box<Fan<LHMSensor>>> {
        todo!()
    }

   
}

struct LHMSensor {

}

impl FetchHardware for LHMSensor {
    fn get_value(&self) -> i32 {
        todo!()
    }

    fn new(name: String) -> impl FetchHardware {
        
        Self {}
    }
}