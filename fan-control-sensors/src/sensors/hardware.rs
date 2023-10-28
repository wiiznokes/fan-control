

use crate::conf::hardware::Temp;







pub trait Generator {


    fn new() -> impl Generator where Self: Sized;


    fn temps<'a>(&'a self) -> Vec<Temp<'a>>;
}