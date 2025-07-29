use crate::{Ui, message::AppMsg};
use cosmic::{
    Element, Task,
    iced::{clipboard, theme::Palette},
    widget::{
        button, dialog,
        markdown::{self, Url},
        scrollable,
    },
};
use hardware::HardwareBridge;

#[derive(Clone, Debug)]
pub enum Dialog {
    Udev,
}

#[derive(Clone, Debug)]
pub enum DialogMsg {
    Udev(UdevDialogMsg),
}

impl Dialog {
    pub fn view(&self) -> Element<AppMsg> {
        scrollable(
            match self {
                Dialog::Udev => view_udev_dialog(),
            }
            .map(AppMsg::Dialog),
        )
        .into()
    }

    pub fn update<H: HardwareBridge>(app: &mut Ui<H>, message: DialogMsg) -> Task<AppMsg> {
        match message {
            DialogMsg::Udev(flatpak_dialog_msg) => match flatpak_dialog_msg {
                UdevDialogMsg::Close => {
                    app.dialog = None;
                }
                UdevDialogMsg::CopyToClipboard(data) => return clipboard::write(data),
                UdevDialogMsg::CloseAndDontShowAgain => {
                    app.dialog = None;
                    app.app_state.dir_manager.update_state(|state| {
                        state.show_flatpak_dialog = false;
                    });
                }
                UdevDialogMsg::OpenUrl(url) => {
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
pub enum UdevDialogMsg {
    Close,
    CopyToClipboard(String),
    CloseAndDontShowAgain,
    OpenUrl(Url),
}

const UDEV_COMMANDS: &str = r#"wget https://raw.githubusercontent.com/wiiznokes/fan-control/master/res/linux/60-fan-control.rules
sudo mv 60-fan-control.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger"#;
fn view_udev_dialog() -> Element<'static, DialogMsg> {
    let content = udev_dialog_content();

    let items = markdown::parse(&content).collect::<Vec<_>>();

    let dialog: Element<_> = dialog()
        .control(
            markdown::view(
                items.iter(),
                markdown::Settings::default(),
                markdown::Style::from_palette(Palette::CATPPUCCIN_FRAPPE),
            )
            .map(UdevDialogMsg::OpenUrl),
        )
        .primary_action(
            button::text(fl!("udev_rules", "ok")).on_press(UdevDialogMsg::CloseAndDontShowAgain),
        )
        .secondary_action(
            button::text(fl!("udev_rules", "copy_to_clipboard"))
                .on_press(UdevDialogMsg::CopyToClipboard(UDEV_COMMANDS.into())),
        )
        .tertiary_action(
            button::text(fl!("udev_rules", "remind_later")).on_press(UdevDialogMsg::Close),
        )
        .into();

    dialog.map(DialogMsg::Udev)
}

fn udev_dialog_content() -> String {
    const UDEV_COMMANDS_MD: &str = constcat::concat!("```sh\n", UDEV_COMMANDS, "\n```");

    fn explain_command(command: &str, command_name: Option<&str>) -> String {
        let command_name = if let Some(c) = command_name {
            c
        } else {
            command
        };

        format!(
            "- _{command_name}_: {}",
            fl!("udev_rules", "explain_commands", cmd = command)
        )
    }

    const STEAM_OS_COMMANDS: &str = r#"sudo steamos-readonly disable
... commands
sudo steamos-readonly enable"#;

    const STEAM_OS_COMMANDS_MD: &str = constcat::concat!("```sh\n", STEAM_OS_COMMANDS, "\n```");

    format!(
        "# {}\n\n{}\n\n{}\n\n#### _{}_\n\n{}\n{}\n{}\n\n### {}\n\n{}\n\n{}\n",
        fl!("udev_rules", "title"),
        fl!("udev_rules", "info"),
        UDEV_COMMANDS_MD,
        fl!("udev_rules", "explain_commands_title"),
        explain_command("wget", None),
        explain_command("sudomv", Some("sudo mv")),
        explain_command("udevadm", None),
        "Steam Os",
        fl!("udev_rules", "streamos"),
        STEAM_OS_COMMANDS_MD
    )
}

#[cfg(test)]
mod test {

    use crate::udev_dialog::udev_dialog_content;
    use pretty_assertions::assert_eq;

    #[test]
    fn udev_dialog_test() {
        unsafe {
            std::env::set_var("LANG", "");
        }

        let str = udev_dialog_content();

        let md_file = include_str!("../../res/linux/udev_rules.md");

        assert_eq!(str, md_file);
    }
}
