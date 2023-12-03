use crate::{utils::icon_button, AppMsg, CreateConfigMsg, SettingsMsg};
use cosmic::{
    iced_core::{Alignment, Length},
    iced_widget::{PickList, Rule},
    widget::{Container, Row, TextInput},
    Element,
};
use data::{directories::DirManager, settings::Settings};

pub fn top_bar_view<'a>(
    settings: &'a Settings,
    dir_manager: &'a DirManager,
    current_config_name: &'a str,
) -> Element<'a, AppMsg> {
    let mut elems = vec![];

    if settings.current_config.is_some() {
        let save_button = icon_button("topBar/save40", AppMsg::SaveConfig);

        elems.push(save_button);
    }

    if !dir_manager.config_names.is_empty() {
        let choose_config = if settings.current_config.is_some() {
            TextInput::new("name", current_config_name)
                .on_input(AppMsg::RenameConfig)
                .width(Length::Fixed(200.0))
                .into()
        } else {
            PickList::new(
                &dir_manager.config_names,
                Some(current_config_name.to_owned()),
                |name| AppMsg::ChangeConfig(Some(name)),
            )
            .into()
        };
        elems.push(choose_config);
    }

    let new_button = icon_button(
        "sign/plus/add40",
        AppMsg::CreateConfig(CreateConfigMsg::Init),
    );
    elems.push(new_button);

    let separator = Rule::vertical(2).into();
    elems.push(separator);

    let settings_button = icon_button("topbar/settings40", AppMsg::Settings(SettingsMsg::Open));
    elems.push(settings_button);

    let content = Row::with_children(elems)
        .align_items(Alignment::Center)
        .width(Length::Fill);

    Container::new(content).height(Length::Fixed(40.0)).into()
}
