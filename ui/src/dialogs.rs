use cosmic::{
    iced::{clipboard, theme::Palette},
    widget::{
        button, dialog,
        markdown::{self, Url},
    },
    Element, Task,
};
use hardware::HardwareBridge;

use crate::{message::AppMsg, Ui};

#[derive(Clone, Debug)]
pub enum Dialog {
    Flatpak,
}

#[derive(Clone, Debug)]
pub enum DialogMsg {
    Flatpak(FlatpakDialogMsg),
}

impl Dialog {
    pub fn view(&self) -> Element<AppMsg> {
        match self {
            Dialog::Flatpak => view_flatpak_dialog(),
        }
        .map(AppMsg::Dialog)
    }

    pub fn update<H: HardwareBridge>(app: &mut Ui<H>, message: DialogMsg) -> Task<AppMsg> {
        match message {
            DialogMsg::Flatpak(flatpak_dialog_msg) => match flatpak_dialog_msg {
                FlatpakDialogMsg::Close => {
                    app.dialog = None;
                }
                FlatpakDialogMsg::CopyToClipboard(data) => return clipboard::write(data),
                FlatpakDialogMsg::CloseAndDontShowAgain => {
                    app.dialog = None;
                    app.app_state.dir_manager.update_state(|state| {
                        state.show_flatpak_dialog = false;
                    });
                }
                FlatpakDialogMsg::OpenUrl(url) => {
                    if let Err(e) = open::that(url.as_str()) {
                        error!("{e}");
                    }
                }
            },
        }

        Task::none()
    }
}

#[derive(Clone, Debug)]
pub enum FlatpakDialogMsg {
    Close,
    CopyToClipboard(String),
    CloseAndDontShowAgain,
    OpenUrl(Url),
}

fn view_flatpak_dialog() -> Element<'static, DialogMsg> {
    let items = markdown::parse(include_str!("../../res/linux/udev_rules.md")).collect::<Vec<_>>();

    let commands = r#"
        wget https://raw.githubusercontent.com/wiiznokes/fan-control/master/res/linux/60-fan-control.rules
        sudo mv 60-fan-control.rules /etc/udev/rules.d/
        sudo udevadm control --reload-rules && sudo udevadm trigger
    "#;

    let dialog: Element<_> = dialog()
        .control(
            markdown::view(
                items.iter(),
                markdown::Settings::default(),
                markdown::Style::from_palette(Palette::CATPPUCCIN_FRAPPE),
            )
            .map(FlatpakDialogMsg::OpenUrl),
        )
        .primary_action(
            button::text(fl!("udev_rules_dialog_ok"))
                .on_press(FlatpakDialogMsg::CloseAndDontShowAgain),
        )
        .secondary_action(
            button::text(fl!("udev_rules_dialog_copy_to_clipboard"))
                .on_press(FlatpakDialogMsg::CopyToClipboard(commands.into())),
        )
        .tertiary_action(
            button::text(fl!("udev_rules_dialog_remind_later")).on_press(FlatpakDialogMsg::Close),
        )
        .into();

    dialog.map(DialogMsg::Flatpak)
}
