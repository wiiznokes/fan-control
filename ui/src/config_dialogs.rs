use cosmic::{
    Apply, Element, Task,
    widget::{button, dialog, text_input},
};
use data::dir_manager::DirManager;

use crate::{Dialog, DialogMsg, Ui, message::AppMsg};
use hardware::HardwareBridge;

#[derive(Debug)]
pub struct CreateConfigDialog {
    name: String,
}

#[derive(Clone, Debug)]
pub enum CreateConfigDialogMsg {
    Cancel,
    Create(String),
    Input(String),
}

impl CreateConfigDialog {
    pub fn new() -> Self {
        Self {
            name: String::new(),
        }
    }

    pub fn view(&self, dir_manager: &DirManager) -> Element<'_, DialogMsg> {
        dialog()
            .title("Create configuration")
            .control(text_input("Name", &self.name).on_input(CreateConfigDialogMsg::Input))
            .primary_action(
                button::text("Create").on_press_maybe(
                    dir_manager
                        .config_names
                        .is_valid_create(&self.name)
                        .then_some(CreateConfigDialogMsg::Create(self.name.clone())),
                ),
            )
            .secondary_action(button::text("Cancel").on_press(CreateConfigDialogMsg::Cancel))
            .apply(Element::from)
            .map(DialogMsg::CreateConfig)
    }

    pub fn update<H: HardwareBridge>(
        app: &mut Ui<H>,
        message: CreateConfigDialogMsg,
    ) -> Task<AppMsg> {
        match message {
            CreateConfigDialogMsg::Cancel => {
                app.dialog = None;
            }
            CreateConfigDialogMsg::Create(new_name) => {
                app.create_config(new_name);
                app.dialog = None;
            }
            CreateConfigDialogMsg::Input(input) => {
                if let Some(Dialog::CreateConfig(dialog)) = &mut app.dialog {
                    dialog.name = input;
                }
            }
        }
        Task::none()
    }
}

#[derive(Debug, Clone)]
pub struct RenameConfigDialog {
    prev: String,
    new: String,
}

#[derive(Clone, Debug)]
pub enum RenameConfigDialogMsg {
    Cancel,
    Rename { prev: String, new: String },
    Input(String),
}

impl RenameConfigDialog {
    pub fn new(previous_name: &str) -> Self {
        Self {
            prev: previous_name.to_string(),
            new: String::new(),
        }
    }

    pub fn view(&self, dir_manager: &DirManager) -> Element<'_, DialogMsg> {
        dialog()
            .title("Rename configuration")
            .control(text_input("New name", &self.new).on_input(RenameConfigDialogMsg::Input))
            .primary_action(
                button::text("Rename").on_press_maybe(
                    dir_manager
                        .config_names
                        .is_valid_create(&self.new)
                        .then_some(RenameConfigDialogMsg::Rename {
                            prev: self.prev.clone(),
                            new: self.new.clone(),
                        }),
                ),
            )
            .secondary_action(button::text("Cancel").on_press(RenameConfigDialogMsg::Cancel))
            .apply(Element::from)
            .map(DialogMsg::RenameConfig)
    }

    pub fn update<H: HardwareBridge>(
        app: &mut Ui<H>,
        message: RenameConfigDialogMsg,
    ) -> Task<AppMsg> {
        match message {
            RenameConfigDialogMsg::Cancel => {
                app.dialog = None;
            }
            RenameConfigDialogMsg::Rename { prev, new } => {
                app.rename_config(&prev, &new);
                app.dialog = None;
            }
            RenameConfigDialogMsg::Input(input) => {
                if let Some(Dialog::RenameConfig(dialog)) = &mut app.dialog {
                    dialog.new = input;
                }
            }
        }
        Task::none()
    }
}
