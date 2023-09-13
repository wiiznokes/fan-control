use conf::{hardware::{HardwareGenerator, FetchHardware}, lm_sensors::LmSensorsGenerator, libre_hardware_monitor::LHMGenerator};

mod conf;

struct App {}

fn main() {

    let windows = false;

    let hardware_generator: dyn HardwareGenerator<dyn FetchHardware> = if windows {
        LHMGenerator::new()
    } else {
        LmSensorsGenerator::new()
    };

  
    let a = hardware_generator.generate_controls();
}
