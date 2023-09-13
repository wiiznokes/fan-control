
use super::hardware::{HardwareGenerator, Temp, FetchHardware, Sensor};

pub struct LHMGenerator {}

impl LHMGenerator {}

impl HardwareGenerator for LHMGenerator {

    
    fn new() -> Self {
        Self {}
    }

    fn generate_controls(&self) -> Vec<super::hardware::Control> {
        todo!()
    }

    fn generate_temps(&self) -> Vec<Box<dyn Sensor>> {
        todo!()
    }

    fn generate_fans(&self) -> Vec<Box<dyn Sensor>> {
        todo!()
    }

   
}

struct LHMSensor {

}

impl FetchHardware for LHMSensor {
    fn get_value(&self) -> i32 {
        todo!()
    }
}

impl Sensor for LHMSensor {

    fn name(&self) -> String {
        todo!()
    }
}