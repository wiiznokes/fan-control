pub type Id = u32;

#[derive(Default, Debug)]
pub struct IdGenerator {
    prec_id: Id,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self { prec_id: 0 }
    }

    pub fn new_id(&mut self) -> Id {
        self.prec_id = self.prec_id.checked_add(1).expect("overflow of id");

        self.prec_id
    }
}
