use std::sync::LazyLock;

use cosmic::{
    iced_core::Alignment,
    iced_widget::PickList,
    widget::{self, about::About, button, Row, Text},
    Element,
};
use data::{dir_manager::DirManager, settings::AppTheme};
use utils::{APP, APP_ID};

use crate::{
    icon_button,
    message::{AppMsg, SettingsMsg, ToogleMsg},
};

pub enum Drawer {
    Settings,
    About,
}

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
        .name(APP)
        .icon(APP_ID)
        .license("GPL-3.0-only")
        .author("wiiznokes")
        .links([
            (
                fl!("repository"),
                "https://github.com/wiiznokes/fan-control",
            ),
            (
                fl!("donate"),
                "https://www.paypal.com/donate/?hosted_button_id=HV84HZ4G63HQ6",
            ),
            (
                fl!("issues_tracker"),
                "https://github.com/wiiznokes/fan-control/issues",
            ),
        ])
        .developers([("wiiznokes", "wiiznokes2@gmail.com")])
        .version(format!(
            "{}-{}",
            env!("FAN_CONTROL_VERSION"),
            env!("FAN_CONTROL_COMMIT")
        ))
});

pub fn about() -> Element<'static, AppMsg> {
    widget::about(&ABOUT, AppMsg::OpenUrl)
}
