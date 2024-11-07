use cosmic::{
    iced::{clipboard, theme::Palette},
    widget::{
        button, dialog,
        markdown::{self, Url},
        text,
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
                    // todo
                },
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

    let dialog: Element<_> = dialog("Udev rules")
        .body("body")
        .control(
            markdown::view(
                items.iter(),
                markdown::Settings::default(),
                markdown::Style::from_palette(Palette::CATPPUCCIN_FRAPPE),
            )
            .map(|url| FlatpakDialogMsg::OpenUrl(url)),
        )
        .primary_action(button::text("Remind me latter").on_press(FlatpakDialogMsg::Close))
        .secondary_action(
            button::text("Copy command to clipboard")
                .on_press(FlatpakDialogMsg::CopyToClipboard("todo".into())),
        )
        .tertiary_action(
            button::text("Already done it").on_press(FlatpakDialogMsg::CloseAndDontShowAgain),
        )
        .into();

    dialog.map(DialogMsg::Flatpak)
}
