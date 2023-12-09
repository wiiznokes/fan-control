use cosmic::{
    iced_core::Alignment,
    iced_widget::PickList,
    widget::{self, Row, Text},
    Element,
};
use data::{dir_manager::DirManager, settings::AppTheme};

use crate::{
    message::{AppMsg, SettingsMsg},
    utils::icon_button,
};

pub fn settings_drawer(show: bool, dir_manager: &DirManager) -> Option<Element<'_, AppMsg>> {
    if !show {
        return None;
    }

    let settings_context = widget::settings::view_column(vec![widget::settings::view_section("")
        .add(
            widget::settings::item::builder(fl!("theme")).control(PickList::new(
                AppTheme::VALUES.to_vec(),
                Some(dir_manager.settings().theme),
                move |theme| AppMsg::Settings(SettingsMsg::Theme(theme)),
            )),
        )
        .add(
            widget::settings::item::builder(fl!("update_delay")).control(update_delay(dir_manager)),
        )
        .into()])
    .into();

    Some(settings_context)
}

fn update_delay(dir_manager: &DirManager) -> Element<'_, AppMsg> {
    let update_delay = dir_manager.settings().update_delay;

    let add_value = 100;

    let sub_message = match update_delay.checked_sub(add_value) {
        Some(value) => {
            if value < 100 {
                None
            } else {
                Some(SettingsMsg::UpdateDelay(value).into())
            }
        }
        None => None,
    };

    let plus_message = update_delay
        .checked_add(add_value)
        .map(|value| SettingsMsg::UpdateDelay(value).into());

    Row::new()
        .push(icon_button("remove/20").on_press_maybe(sub_message))
        .push(Text::new(fl!("update_delay_value", value = update_delay)))
        .push(icon_button("add/20").on_press_maybe(plus_message))
        .align_items(Alignment::Center)
        .into()
}
