use conf::{hardware::HardwareGenerator, lm_sensors::LmSensorsGenerator, libre_hardware_monitor::LHMGenerator};

mod conf;

struct App {}

fn main() {

    let windows = false;

    let hardware_generator: HardwareGenerator = if windows {
        LHMGenerator::new()
    } else {
        LmSensorsGenerator::new()
    };

  
    let a = hardware_generator.generate_controls();
}
