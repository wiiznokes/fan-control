use cosmic::{Element, iced_core::Length, iced_widget::text, widget::tooltip};
use data::AppState;
use hardware::HardwareBridge;

use crate::{AppMsg, ToogleMsg, icon_button};

static ICON_LENGTH: Length = Length::Fixed(33.0);

pub fn header_start<'a>() -> Vec<Element<'a, AppMsg>> {
    let elems = vec![];

    // let app_icon = icon!("toys_fan/48").into();
    // elems.push(app_icon);

    // elems.push(Space::new(Length::Fixed(10.0), 0.0).into());
    // let app_name = Text::new("fan-control").into();
    // elems.push(app_name);

    elems
}

pub fn header_end<'a, H: HardwareBridge>(app_state: &'a AppState<H>) -> Vec<Element<'a, AppMsg>> {
    let dir_manager = &app_state.dir_manager;
    let settings = dir_manager.settings();

    let mut elems = vec![];

    // save button
    if let Some(name) = &settings.current_config {
        elems.push(
            tooltip(
                icon_button!("save/40")
                    .height(ICON_LENGTH)
                    .width(ICON_LENGTH)
                    .on_press(AppMsg::SaveConfig(name.to_string())),
                text(fl!("save_config")),
                tooltip::Position::Bottom,
            )
            .into(),
        );
    }

    let settings_button = icon_button!("settings/40")
        .on_press(AppMsg::Toggle(ToogleMsg::Settings))
        .height(ICON_LENGTH)
        .width(ICON_LENGTH)
        .into();
    elems.push(settings_button);

    elems
}
