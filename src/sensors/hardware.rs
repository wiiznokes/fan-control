

use crate::conf::hardware::Temp;







pub trait Generator<'a> {


    fn new() -> impl Generator<'a>;


    fn temps(&self) -> Vec<Box<Temp<'a>>>;
}