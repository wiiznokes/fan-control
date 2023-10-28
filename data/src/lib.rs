//#![feature(return_position_impl_trait_in_trait)]

pub mod node;
pub mod serde;

pub mod id {

    pub struct Id {
        prec_id: u32,
    }

    impl Id {
        pub fn new_id(&mut self) -> u32 {
            self.prec_id += 1;

            self.prec_id
        }
    }
}
