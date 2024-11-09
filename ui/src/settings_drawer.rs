use std::sync::LazyLock;

use cosmic::{
    iced_core::Alignment,
    iced_widget::PickList,
    widget::{self, button, About, Row, Text},
    Element,
};
use data::{dir_manager::DirManager, settings::AppTheme};
use utils::{APP, APP_ID};

use crate::{
    icon_button,
    message::{AppMsg, SettingsMsg, ToogleMsg},
};

pub fn settings_drawer(dir_manager: &DirManager) -> Element<'_, AppMsg> {
    widget::settings::view_column(vec![widget::settings::section()
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
        .add(
            widget::settings::item::builder(fl!("about"))
                .control(button::text("open").on_press(AppMsg::Toggle(ToogleMsg::About))),
        )
        .into()])
    .into()
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
        .push(icon_button!("remove/20").on_press_maybe(sub_message))
        .push(Text::new(fl!("update_delay_value", value = update_delay)))
        .push(icon_button!("add/20").on_press_maybe(plus_message))
        .align_y(Alignment::Center)
        .into()
}

static ABOUT: LazyLock<About> = LazyLock::new(|| {
    About::default()
        .set_application_name(APP)
        .set_application_icon(APP_ID)
        .set_license_type("GPL-3.0-only")
        .set_developer_name("wiiznokes")
        .set_repository_url("https://github.com/wiiznokes/fan-control")
});

pub fn about() -> Element<'static, AppMsg> {
    widget::about(&ABOUT, |url| AppMsg::OpenUrl(url)).into()
}
