use cosmic::{
    iced_widget::PickList,
    widget::{self},
    Element,
};
use data::{dir_manager::DirManager, settings::AppTheme};

use crate::message::{AppMsg, SettingsMsg};

pub fn settings_drawer(show: bool, dir_manager: &DirManager) -> Option<Element<'_, AppMsg>> {
    if !show {
        return None;
    }

    let themes = AppTheme::VALUES.to_vec();

    let settings_context = widget::settings::view_column(vec![widget::settings::view_section("")
        .add(
            widget::settings::item::builder("Theme").control(PickList::new(
                themes,
                Some(dir_manager.settings().theme),
                move |theme| AppMsg::Settings(SettingsMsg::ChangeTheme(theme)),
            )),
        )
        .into()])
    .into();

    Some(settings_context)
}
