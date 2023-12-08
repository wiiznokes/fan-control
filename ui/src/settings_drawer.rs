use cosmic::{
    widget::{self, Dropdown},
    Element,
};
use data::{dir_manager::DirManager, settings::AppTheme};
use strum::IntoEnumIterator;

use crate::{AppCache, AppMsg, SettingsMsg};

pub fn settings_drawer<'a>(
    show: bool,
    dir_manager: &'a DirManager,
    cache: &'a AppCache,
) -> Option<Element<'a, AppMsg>> {
    if !show {
        return None;
    }
    let app_theme_selected = AppTheme::iter()
        .position(|e| e == dir_manager.settings().theme)
        .unwrap();

    let settings_context = widget::settings::view_column(vec![widget::settings::view_section("")
        .add(
            widget::settings::item::builder("Theme").control(Dropdown::new(
                &cache.theme_list,
                Some(app_theme_selected),
                move |index| {
                    let theme = AppTheme::iter().nth(index).unwrap();
                    AppMsg::Settings(SettingsMsg::ChangeTheme(theme))
                },
            )),
        )
        .into()])
    .into();

    Some(settings_context)
}
