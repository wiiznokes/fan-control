use crate::conf::hardware::Temp;

use super::hardware::Generator;


pub struct WindowsGenerator {}



impl Generator for WindowsGenerator {


    fn new() -> impl Generator {
        
        Self {}
    }

    fn temps<'a>(&'a self) -> Vec<Temp<'a>> {
        todo!()
    }
}

