//! device parameter types
//! multiple devices can use the same parameter specifier. e.g. all devices can use the `Raw` paramter specifier, R L C can use `SingleValue`, etc.
//! a device should be able to choose between all compatible parameter specifier

use iced::{
    widget::{button, text_input},
    Element, Length,
};

#[derive(Debug, Clone)]
pub enum RawPEMsg {
    InputChanged(String),
    InputSubmit,
}

/// this struct to edit device parameters by specifying the spice netlist line (after port connects) directly
#[derive(Debug, Clone)]
pub struct Raw {
    pub raw: String,
    tmp: String,
}
impl Raw {
    pub fn update(&mut self, msg: RawPEMsg) {
        match msg {
            RawPEMsg::InputChanged(s) => {
                self.tmp = s;
            }
            RawPEMsg::InputSubmit => {
                self.raw = self.tmp.clone();
            }
        }
    }
    pub fn view(&self) -> Element<RawPEMsg> {
        iced::widget::column![
            text_input("", &self.tmp)
                .width(50)
                .on_input(RawPEMsg::InputChanged)
                .on_submit(RawPEMsg::InputSubmit),
            button("enter"),
        ]
        .width(Length::Shrink)
        .into()
    }
    pub fn new(raw: String) -> Self {
        Raw {
            raw,
            tmp: Default::default(),
        }
    }
    pub fn set(&mut self, new: String) {
        self.raw = new;
    }
}
