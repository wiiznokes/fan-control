use std::fmt::Debug;

use crate::conf::hardware::Temp;



#[derive(Debug, Clone)]
pub struct TempH<'a, S: FetchHardware> {
    pub temp: Temp,

    pub sensor: &'a S,
}

impl<'a, S: FetchHardware> TempH<'a, S> {
    pub fn new(name: String) -> Box<TempH<'a, S>> {
        let sensor = &<S as FetchHardware>::new(name);
        Box::new(
            Self { 
            temp: Temp { name: name.clone() }, 
            sensor
            }
        )
    }
}



pub trait FetchHardware
where
    Self: Debug + Clone,
{
    fn get_value(&self) -> i32;

    fn new(name: String) -> Self
    where
        Self: Sized;
}


pub trait HardwareGenerator<'a> {

    type Output: FetchHardware;

    fn new() -> impl HardwareGenerator<'a>;


    fn temps(&self) -> Vec<Box<TempH<'a, Self::Output>>>;
}
