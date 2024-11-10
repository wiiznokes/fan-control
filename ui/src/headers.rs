use cosmic::{
    iced_core::{Alignment, Length},
    iced_widget::{text, Button, Column},
    theme,
    widget::{tooltip, Container, Row, Text, TextInput},
    Element,
};
use data::{config::Config, AppState};
use hardware::HardwareBridge;

use crate::{
    icon, icon::expand_icon, icon_button, message::ConfigMsg, my_widgets::drop_down, AppMsg,
    ToogleMsg,
};

static ICON_LENGHT: Length = Length::Fixed(33.0);

pub fn header_start<'a>() -> Vec<Element<'a, AppMsg>> {
    let mut elems = vec![];

    let app_icon = icon!("toys_fan/48").into();
    elems.push(app_icon);

    // elems.push(Space::new(Length::Fixed(10.0), 0.0).into());
    // let app_name = Text::new("fan-control").into();
    // elems.push(app_name);

    elems
}

pub fn header_center<'a, H: HardwareBridge>(
    app_state: &'a AppState<H>,
    current_config_cached: &'a String,
    expanded: bool,
) -> Vec<Element<'a, AppMsg>> {
    let dir_manager = &app_state.dir_manager;
    let settings = dir_manager.settings();

    let mut elems = Vec::new();

    // configuration not saved
    if match &settings.current_config {
        Some(current_config) => {
            if current_config != current_config_cached {
                true
            } else {
                match dir_manager.get_config() {
                    Some(config) => config != Config::from_app_graph(&app_state.app_graph),
                    None => true,
                }
            }
        }
        None => true,
    } {
        elems.push(
            tooltip(
                icon!("warning/40"),
                text(fl!("config_not_saved")),
                tooltip::Position::Bottom,
            )
            .into(),
        );
    }

    // save button
    if settings.current_config.is_some() {
        elems.push(
            tooltip(
                icon_button!("save/40")
                    .height(ICON_LENGHT)
                    .width(ICON_LENGHT)
                    .on_press_maybe(
                        dir_manager
                            .config_names
                            .is_valid_name(&settings.current_config, current_config_cached)
                            .then_some(ConfigMsg::Save.into()),
                    ),
                text(fl!("save_config")),
                tooltip::Position::Bottom,
            )
            .into(),
        );
    }

    let mut name = TextInput::new(fl!("config_name"), current_config_cached)
        .on_input(|name| ConfigMsg::Rename(name).into())
        .width(Length::Fixed(180.0));

    if !dir_manager
        .config_names
        .is_valid_name(&settings.current_config, current_config_cached)
    {
        //let error_text = fl!("already_used_error");
        name = name.error("This name is already being use");
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

    let expand_icon = if !configs.is_empty() {
        let expand_icon = expand_icon(expanded)
            .height(ICON_LENGHT)
            .width(ICON_LENGHT)
            .on_press(AppMsg::Toggle(crate::ToogleMsg::ChooseConfig(!expanded)));
        Some(expand_icon)
    } else {
        None
    };

    let underlay = Row::new()
        .push(name)
        .push_maybe(expand_icon)
        .align_y(Alignment::Center);

    let overlay = Container::new(Column::with_children(configs).align_x(Alignment::Start))
        .class(theme::Container::Dropdown);

    let choose_config = drop_down::DropDown::new(underlay, overlay, expanded)
        .on_dismiss(AppMsg::Toggle(crate::ToogleMsg::ChooseConfig(false)))
        .into();

    elems.push(choose_config);

    // create button
    elems.push(
        tooltip(
            icon_button!("add/40")
                .height(ICON_LENGHT)
                .width(ICON_LENGHT)
                .on_press_maybe(
                    dir_manager
                        .config_names
                        .is_valid_create(current_config_cached)
                        .then_some(ConfigMsg::Create(current_config_cached.to_owned()).into()),
                ),
            text(fl!("create_config")),
            tooltip::Position::Bottom,
        )
        .into(),
    );

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
            tooltip(
                icon_button!("delete_forever/40")
                    .height(ICON_LENGHT)
                    .width(ICON_LENGHT)
                    .on_press(ConfigMsg::Delete(name).into()),
                text(fl!("delete_config")),
                tooltip::Position::Right,
            )
            .into(),
        );
    }
    Row::with_children(elements)
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .into()
}

pub fn header_end<'a>() -> Vec<Element<'a, AppMsg>> {
    let mut elems = vec![];

    let settings_button = icon_button!("settings/40")
        .on_press(AppMsg::Toggle(ToogleMsg::Settings))
        .height(ICON_LENGHT)
        .width(ICON_LENGHT)
        .into();
    elems.push(settings_button);

    elems
}
