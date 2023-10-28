//#![feature(return_position_impl_trait_in_trait)]

use std::collections::HashMap;

pub mod serde;
pub mod node;




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