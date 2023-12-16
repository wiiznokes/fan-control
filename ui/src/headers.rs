use cosmic::{
    iced_core::{Alignment, Length},
    iced_widget::{Button, Column},
    theme,
    widget::{Container, Row, Text, TextInput},
    Element,
};
use data::dir_manager::DirManager;

use crate::{
    message::ConfigMsg,
    my_widgets::drop_down,
    utils::{expand_icon, icon_button, my_icon},
    AppMsg, ToogleMsg,
};

static ICON_LENGHT: Length = Length::Fixed(35.0);

pub fn header_start<'a>() -> Vec<Element<'a, AppMsg>> {
    let mut elems = vec![];

    let app_icon = my_icon("toys_fan/48").into();
    elems.push(app_icon);

    // elems.push(Space::new(Length::Fixed(10.0), 0.0).into());
    // let app_name = Text::new("fan-control").into();
    // elems.push(app_name);

    elems
}

pub fn header_center<'a>(
    dir_manager: &'a DirManager,
    current_config: &'a String,
    expanded: bool,
) -> Vec<Element<'a, AppMsg>> {
    let settings = dir_manager.settings();
    let mut elems = Vec::new();

    let mut save_button = icon_button("save/40")
        .tooltip(fl!("save_config"))
        .height(ICON_LENGHT)
        .width(ICON_LENGHT);

    if settings.current_config.is_some()
        && dir_manager
            .config_names
            .is_valid_name(&settings.current_config, current_config)
    {
        save_button = save_button.on_press(ConfigMsg::Save.into());
    }

    elems.push(save_button.into());

    let mut name = TextInput::new(fl!("config_name"), current_config)
        .on_input(|name| ConfigMsg::Rename(name).into())
        .width(Length::Fixed(180.0));

    if !dir_manager
        .config_names
        .is_valid_name(&settings.current_config, current_config)
    {
        name = name.error("this name is already beeing use");
    }

    let mut configs = Vec::new();

    if !dir_manager.config_names.is_empty() {
        if settings.current_config.is_some() {
            configs.push(config_choice_line(None))
        }

        dir_manager
            .config_names
            .names()
            .iter()
            .for_each(|name| configs.push(config_choice_line(Some(name.to_owned()))))
    }

    let mut expand_icon = expand_icon(expanded).height(ICON_LENGHT).width(ICON_LENGHT);

    if !configs.is_empty() {
        expand_icon =
            expand_icon.on_press(AppMsg::Toggle(crate::ToogleMsg::ChooseConfig(!expanded)));
    }

    let underlay = Row::new()
        .push(name)
        .push(expand_icon)
        .align_items(Alignment::Center);

    let overlay = Container::new(Column::with_children(configs).align_items(Alignment::Start))
        .style(theme::Container::Dropdown);

    let choose_config = drop_down::DropDown::new(underlay, overlay, expanded)
        .on_dismiss(AppMsg::Toggle(crate::ToogleMsg::ChooseConfig(false)))
        .into();

    elems.push(choose_config);

    let mut create_button = icon_button("add/40")
        .tooltip(fl!("create_config"));

    if dir_manager.config_names.is_valid_create(current_config) {
        create_button = create_button.on_press(ConfigMsg::Create(current_config.to_owned()).into());
    }

    elems.push(create_button.into());

    elems
}

fn config_choice_line<'a>(optional_name: Option<String>) -> Element<'a, AppMsg> {
    let name = optional_name.clone().unwrap_or(fl!("none"));

    let mut elements = vec![Button::new(Text::new(name.clone()))
        .on_press(ConfigMsg::Change(optional_name.clone()).into())
        .width(Length::Fill)
        .into()];

    if optional_name.is_some() {
        elements.push(
            icon_button("delete_forever/24")
                .on_press(ConfigMsg::Delete(name).into())
                .tooltip(fl!("delete_config"))
                .into(),
        );
    }
    Row::with_children(elements)
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .into()
}

pub fn header_end<'a>() -> Vec<Element<'a, AppMsg>> {
    let mut elems = vec![];

    let settings_button = icon_button("settings/40")
        .on_press(AppMsg::Toggle(ToogleMsg::Settings))
        .height(ICON_LENGHT)
        .width(ICON_LENGHT)
        .into();
    elems.push(settings_button);

    elems
}
