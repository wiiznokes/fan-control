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
        panic!("Unsupported operating system");
    };
    
    
    let temps = generator.temps();

    for temp in temps {

        if let Some(value) = temp.value() {
            println!("{}: {}", temp.name, value);
        }
    }
    
}
