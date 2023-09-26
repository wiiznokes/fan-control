use super::hardware::{FetchHardware, HardwareGenerator, TempH};

pub struct LHMGenerator {}

impl<'a> HardwareGenerator<'a> for LHMGenerator {

    type Output = LHMSensor
    ;
    fn new() -> impl HardwareGenerator<'a> {
        Self {}
    }

    fn temps(&self) -> Vec<Box<TempH<'a, Self::Output>>> {
        todo!()
    }


  
}

#[derive(Debug, Clone)]
pub struct LHMSensor {}

impl FetchHardware for LHMSensor {
    fn get_value(&self) -> i32 {
        todo!()
    }

    fn new(_name: String) -> Self {
        Self {}
    }
}
