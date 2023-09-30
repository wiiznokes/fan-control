#![feature(return_position_impl_trait_in_trait)]



mod conf;
mod sensors;

use sensors::hardware::Generator;


struct App {}

fn main() {
    let windows = false;

    #[cfg(target_os = "linux")]
    {
        use sensors::lm_sensors::LinuxGenerator;
        let generator = LinuxGenerator::new();
    }

    #[cfg(target_os = "windows")]
    {
        use sensors::libre_hardware_monitor::WindowsGenerator;
        let generator = WindowsGenerator::new();
    }
    

    /*
    
    let hardware_generator = if windows {
        Box::new(LHMGenerator::new())
    } else {
        Box::new(LmSensorsGenerator::new())
    };


    let a = hardware_generator.generate_controls();
     */
}
