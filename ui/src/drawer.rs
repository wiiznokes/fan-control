use data::dir_manager::DirManager;
use iced::Element;

use crate::AppMsg;

pub fn settings_drawer(expanded: bool, _dir_manager: &DirManager) -> Option<Element<AppMsg>> {
    if !expanded {
        return None;
    }
    None
    /*
    let app_theme_selected = AppTheme::iter()
        .position(|e| e == dir_manager.settings().theme)
        .unwrap();

    let settings_context =
        widget::settings::view_column(vec![widget::settings::view_section("")
            .add(
                widget::settings::item::builder("Theme").control(Dropdown::new(
                    &self.cache.theme_list,
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
     */
}
