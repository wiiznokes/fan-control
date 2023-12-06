use cosmic::iced_core::alignment::{Horizontal, Vertical};

pub struct Anchor {
    pub vertical: Vertical,
    pub horizontal: Horizontal,
}

impl Anchor {
    pub fn new(vertical: Vertical, horizontal: Horizontal) -> Self {
        Self {
            vertical,
            horizontal,
        }
    }
}
