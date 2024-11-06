use cosmic::{
    widget::{button, dialog, text},
    Element,
};

use crate::message::AppMsg;

pub enum Dialog {
    Flatpak,
}

impl Dialog {
    pub fn view(&self) -> Element<AppMsg> {
        match self {
            Dialog::Flatpak => view_flatpak_dialog(),
        }
    }
}

fn view_flatpak_dialog() -> Element<'static, AppMsg> {
    dialog("Udev rules")
        .body("body")
        .control(text("control"))
        .primary_action(button::text("Got it"))
        .secondary_action(button::text("Got it2"))
        .tertiary_action(button::text("Got it3"))
        .into()
}
