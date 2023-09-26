#![feature(return_position_impl_trait_in_trait)]

use conf::{
    hardware::{FetchHardware, HardwareGenerator},
    libre_hardware_monitor::LHMGenerator,
    lm_sensors::LmSensorsGenerator,
};

mod conf;

struct App {}

fn main() {
    let windows = false;

    
    let hardware_generator = if windows {
        Box::new(LHMGenerator::new())
    } else {
        Box::new(LmSensorsGenerator::new())
    };


    let a = hardware_generator.generate_controls();
    
}
