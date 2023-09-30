use super::hardware::Generator;


pub struct WindowsGenerator {}



impl Generator for WindowsGenerator {


    fn new() -> impl Generator {
        
        Self {}
    }

    fn temps<'a>(&'a self) -> Vec<Box<crate::conf::hardware::Temp<'a>>> {
        todo!()
    }
}

