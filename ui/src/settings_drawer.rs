use cosmic::{
    iced_widget::PickList,
    widget::{self},
    Element,
};
use data::{dir_manager::DirManager, settings::AppTheme};
use strum::IntoEnumIterator;

use crate::message::{AppMsg, SettingsMsg};

pub fn settings_drawer(show: bool, dir_manager: &DirManager) -> Option<Element<'_, AppMsg>> {
    if !show {
        return None;
    }

    let themes: Vec<_> = AppTheme::iter().collect();

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
