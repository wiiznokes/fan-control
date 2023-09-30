use super::hardware::Generator;


pub struct WindowsGenerator {}



impl<'a> Generator<'a> for WindowsGenerator {


    fn new() -> impl Generator<'a> {
        
        Self {}
    }

    fn temps(&self) -> Vec<Box<crate::conf::hardware::Temp<'a>>> {
        todo!()
    }
}

