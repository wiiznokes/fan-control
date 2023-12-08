use cosmic::{
    iced_core::{Alignment, Length},
    iced_widget::{Button, Column},
    theme,
    widget::{Container, Row, Text, TextInput},
    Element,
};
use data::dir_manager::DirManager;

use crate::{
    my_widgets::drop_down,
    utils::{expand_icon, icon_button, my_icon},
    AppMsg, UiMsg,
};

static ICON_LENGHT: Length = Length::Fixed(35.0);

pub fn header_start<'a>() -> Vec<Element<'a, AppMsg>> {
    let mut elems = vec![];

    let app_icon = my_icon("app/toys_fan48").into();
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

    let mut save_button = icon_button("topBar/save40")
        .height(ICON_LENGHT)
        .width(ICON_LENGHT);

    if settings.current_config.is_some()
        && dir_manager
            .config_names
            .is_valid_name(&settings.current_config, current_config)
    {
        save_button = save_button.on_press(AppMsg::SaveConfig);
    }

    elems.push(save_button.into());

    let mut name = TextInput::new(fl!("config_name"), current_config)
        .on_input(AppMsg::RenameConfig)
        .width(Length::Fixed(150.0));

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
        expand_icon = expand_icon.on_press(AppMsg::Ui(crate::UiMsg::ToggleChooseConfig(!expanded)));
    }

    let underlay = Row::new()
        .push(name)
        .push(expand_icon)
        .align_items(Alignment::Center);

    let overlay = Container::new(Column::with_children(configs).align_items(Alignment::Center))
        .style(theme::Container::Dropdown);

    let choose_config = drop_down::DropDown::new(underlay, overlay)
        .expanded(expanded)
        .on_dismiss(Some(AppMsg::Ui(crate::UiMsg::ToggleChooseConfig(false))))
        .into();

    elems.push(choose_config);

    let mut new_button = icon_button("sign/plus/add40");

    if dir_manager.config_names.is_valid_create(current_config) {
        new_button = new_button.on_press(AppMsg::CreateConfig(current_config.to_owned()));
    }

    elems.push(new_button.into());

    elems
}

fn config_choice_line<'a>(optional_name: Option<String>) -> Element<'a, AppMsg> {
    let name = optional_name.clone().unwrap_or(fl!("none"));

    let mut elements = vec![Button::new(Text::new(name.clone()))
        .on_press(AppMsg::ChangeConfig(optional_name.clone()))
        .width(Length::Fill)
        .into()];

    if optional_name.is_some() {
        elements.push(
            icon_button("select/delete_forever24")
                .on_press(AppMsg::RemoveConfig(name))
                .into(),
        );
    }
    Row::with_children(elements)
        .align_items(Alignment::Center)
        // todo: control width with widget
        .width(Length::Fixed(150.0))
        .into()
}

pub fn header_end<'a>() -> Vec<Element<'a, AppMsg>> {
    let mut elems = vec![];

    let settings_button = icon_button("topBar/settings40")
        .on_press(AppMsg::Ui(UiMsg::ToggleSettings))
        .height(ICON_LENGHT)
        .width(ICON_LENGHT)
        .into();
    elems.push(settings_button);

    elems
}
