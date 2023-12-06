use cosmic::{iced_core::Length, iced_widget::PickList, widget::TextInput, Element};
use data::dir_manager::{self, DirManager};

use crate::{utils::icon_button, AppMsg};

pub fn header_center<'a>(
    dir_manager: &'a DirManager,
    current_config: &'a String,
) -> Vec<Element<'a, AppMsg>> {
    let settings = dir_manager.settings();

    let mut elems = vec![];

    if settings.current_config.is_some() {
        let save_button = icon_button("topBar/save40")
            .on_press(AppMsg::SaveConfig)
            .into();

        elems.push(save_button);
    }

    let mut name = TextInput::new(&fl!("config_name"), current_config)
        .on_input(AppMsg::RenameConfig)
        .width(Length::Fixed(150.0));

    if dir_manager.config_names.is_valid_name(current_config) {
        name = name.error("this name is already beeing use");
    }

    elems.push(name.into());

    if !dir_manager.config_names.is_empty() {
        let selected = match &settings.current_config {
            Some(name) => name.clone(),
            None => fl!("none"),
        };
        let selection = PickList::new(
            dir_manager.config_names.names(&settings.current_config),
            Some(selected),
            |name| AppMsg::ChangeConfig(dir_manager::filter_none(name)),
        )
        .width(Length::Fixed(100.0))
        .into();

        elems.push(selection);
    }

    let mut new_button = icon_button("sign/plus/add40");

    if dir_manager.config_names.is_valid_create(current_config) {
        new_button = new_button.on_press(AppMsg::CreateConfig(current_config.to_owned()));
    }

    elems.push(new_button.into());

    elems
}
