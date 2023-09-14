#![feature(return_position_impl_trait_in_trait)]

use conf::{hardware::{HardwareGenerator, FetchHardware}, lm_sensors::LmSensorsGenerator, libre_hardware_monitor::LHMGenerator};


mod conf;

struct App {}

fn main() {

    let windows = false;

    /*
    let hardware_generator = if windows {
        Box::new(LHMGenerator::new())
    } else {
        Box::new(LmSensorsGenerator::new())
    };

  
    let a = hardware_generator.generate_controls();
     */
}
