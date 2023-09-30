#![feature(return_position_impl_trait_in_trait)]



mod conf;
mod sensors;

use sensors::hardware::Generator;


struct App {}

fn main() {

    let generator: Box<dyn Generator> = if cfg!(target_os = "linux") {
        use sensors::lm_sensors::LinuxGenerator;
        Box::new(LinuxGenerator::new())
    } else if cfg!(target_os = "windows") {
        use sensors::libre_hardware_monitor::WindowsGenerator;
        Box::new(WindowsGenerator::new())
    } else {
        // Handle other platforms or provide a default generator
        // For example, you can return an error or a default generator here.
        // LinuxGenerator::new() or WindowsGenerator::new() could also be used as defaults.
        panic!("Unsupported operating system");
    };
    
    
    let temps = generator.temps();
    
    
}
