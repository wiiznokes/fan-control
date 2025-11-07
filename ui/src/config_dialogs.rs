use cosmic::{
    Apply, Element, Task,
    widget::{button, dialog, text_input},
};
use data::{config::Config, dir_manager::DirManager};

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
                let config = Config::from_app_graph(&app.app_state.app_graph);

                if let Err(e) = app.app_state.dir_manager.create_config(&new_name, &config) {
                    error!("can't create config: {e}");
                }

                app.dialog = None;

                app.reload_nav_bar_model();
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

#[derive(Debug)]
pub struct RenameConfigDialog {
    previous_name: String,
    name: String,
}

#[derive(Clone, Debug)]
pub enum RenameConfigDialogMsg {
    Cancel,
    Rename(String),
    Input(String),
}

impl RenameConfigDialog {
    pub fn new(previous_name: &str) -> Self {
        Self {
            previous_name: previous_name.to_string(),
            name: String::new(),
        }
    }

    pub fn view(&self, dir_manager: &DirManager) -> Element<'_, DialogMsg> {
        dialog()
            .title("Rename configuration")
            .control(text_input("New name", &self.name).on_input(RenameConfigDialogMsg::Input))
            .primary_action(
                button::text("Rename").on_press_maybe(
                    dir_manager
                        .config_names
                        .is_valid_create(&self.name)
                        .then_some(RenameConfigDialogMsg::Rename(self.name.clone())),
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
            RenameConfigDialogMsg::Rename(new_name) => {
                if let Some(Dialog::RenameConfig(dialog)) = &mut app.dialog
                    && let Err(e) = app
                        .app_state
                        .dir_manager
                        .rename_config(&dialog.previous_name, &new_name)
                {
                    error!("can't rename config: {e}");
                }

                app.reload_nav_bar_model();
                app.dialog = None;
            }
            RenameConfigDialogMsg::Input(input) => {
                if let Some(Dialog::RenameConfig(dialog)) = &mut app.dialog {
                    dialog.name = input;
                }
            }
        }
        Task::none()
    }
}
