pub type Id = u32;

pub struct IdGenerator {
    prec_id: Id,
}

impl Default for IdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl IdGenerator {
    pub fn new() -> Self {
        Self { prec_id: 0 }
    }

    pub fn new_id(&mut self) -> Id {
        self.prec_id += 1;

        self.prec_id
    }
}
