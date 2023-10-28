//#![feature(return_position_impl_trait_in_trait)]

use std::collections::HashMap;

pub mod configs;
pub mod settings;
pub mod hardware;


#[cfg(test)]
mod test;



pub mod id {
    
    pub struct Id {
        prec_id: u32
    }
    
    impl Id {
        
        pub fn new_id(&mut self) -> u32 {
            
            self.prec_id += 1;
    
            return self.prec_id;
        }
    }
    
}









//pub mod hardware;
//mod sensors;

//use sensors::hardware::Generator;

/*
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
 */