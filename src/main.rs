#![feature(return_position_impl_trait_in_trait)]



mod conf;
mod sensors;

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
