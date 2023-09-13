
use super::hardware::{HardwareGenerator, Temp, FetchHardware};

pub struct LHMGenerator {}

impl LHMGenerator {}

impl HardwareGenerator<LHMSensor> for LHMGenerator {
    fn new() -> Self {
        Self {}
    }

    fn generate_controls(&self) -> Vec<super::hardware::Control> {
        todo!()
    }

    fn generate_temps(&self) -> Vec<super::hardware::Temp<LHMSensor>> {
        todo!()
    }

    fn generate_fans(&self) -> Vec<super::hardware::Fan<LHMSensor>> {
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