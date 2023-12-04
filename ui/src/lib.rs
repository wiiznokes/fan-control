#![allow(dead_code)]
//#![allow(unused_imports)]
use std::time::Duration;

use data::{
    config::Config,
    id::Id,
    node::{validate_name, AppGraph, NodeType, NodeTypeLight},
    settings::AppTheme,
    utils::RemoveElem,
    AppState,
};

use cosmic::{
    app::{Command, Core},
    executor,
    iced::{self, time},
    iced_core::Length,
    iced_widget::PickList,
    theme,
    widget::{self, Column, Space, Text, TextInput},
    ApplicationExt, Element,
};

use item::{items_view, ControlMsg, CustomTempMsg, FlatMsg, LinearMsg, TargetMsg};
use pick::Pick;
use strum::IntoEnumIterator;
use utils::{icon_button, my_icon};

#[macro_use]
extern crate log;

mod input_line;
mod item;
pub mod localize;
mod pick;
//mod theme;
mod utils;
//mod widgets;

pub fn run_ui(app_state: AppState) -> Result<(), Box<dyn std::error::Error>> {
    let settings = cosmic::app::Settings::default();
    cosmic::app::run::<Ui>(settings, app_state)?;
    Ok(())
}
pub struct Ui {
    core: Core,
    app_state: AppState,
    cache: AppCache,
}

pub struct AppCache {
    current_config: String,
    theme_list: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    Tick,
    SaveConfig,
    RenameConfig(String),
    ChangeConfig(Option<String>),
    RemoveConfig(String),
    CreateConfig(CreateConfigMsg),
    ModifNode(Id, ModifNodeMsg),
    NewNode(NodeTypeLight),
    Settings(SettingsMsg),
}

#[derive(Debug, Clone)]
pub enum CreateConfigMsg {
    Init,
    Cancel,
    New(String),
}

#[derive(Debug, Clone)]
pub enum SettingsMsg {
    Toggle,
    ChangeTheme(AppTheme),
}

#[derive(Debug, Clone)]
pub enum ModifNodeMsg {
    Rename(String),
    ChangeHardware(Pick<String>),
    ReplaceInput(Pick<Id>),
    AddInput(Pick<Id>),
    RemoveInput(Pick<Id>),

    Control(ControlMsg),
    CustomTemp(CustomTempMsg),
    Flat(FlatMsg),
    Linear(LinearMsg),
    Target(TargetMsg),

    Delete,
}

impl cosmic::Application for Ui {
    type Executor = executor::Default;
    type Message = AppMsg;
    type Flags = AppState;

    const APP_ID: &'static str = "com.wiiznokes.fan-control";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let app_cache = AppCache {
            current_config: flags.dir_manager.settings.current_config_text().to_owned(),
            theme_list: AppTheme::iter().map(|e| e.to_string()).collect(),
        };

        let ui_state = Ui {
            cache: app_cache,
            app_state: flags,
            core,
        };
        (ui_state, Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMsg::Tick => {
                self.app_state.update.all(
                    &mut self.app_state.app_graph.nodes,
                    &self.app_state.app_graph.root_nodes,
                    &mut self.app_state.bridge,
                );
            }

            AppMsg::ModifNode(id, change_config) => {
                match change_config {
                    ModifNodeMsg::Rename(name) => {
                        let name_is_valid =
                            validate_name(&self.app_state.app_graph.nodes, &id, &name);

                        let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();

                        node.name_cached = name.clone();
                        if name_is_valid {
                            node.is_error_name = false;
                            let previous_name = node.name().clone();
                            node.node_type.set_name(&name);

                            let node_id = node.id;
                            // find nodes that depend on node.id
                            // change the name in input and item.input

                            for n in self.app_state.app_graph.nodes.values_mut() {
                                if let Some(node_input) = n
                                    .inputs
                                    .iter_mut()
                                    .find(|node_input| node_input.0 == node_id)
                                {
                                    node_input.1 = name.clone();
                                    let mut inputs = n.node_type.get_inputs();

                                    match inputs.iter().position(|n| n == &previous_name) {
                                        Some(index) => {
                                            inputs[index] = name.clone();
                                            n.node_type.set_inputs(inputs)
                                        }
                                        None => {
                                            error!("input id found in node inputs but the corresponding name was not found in item input")
                                        }
                                    }
                                }
                            }
                        } else {
                            node.is_error_name = true;
                        }
                    }
                    ModifNodeMsg::ChangeHardware(pick) => {
                        let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();
                        let hardware = &self.app_state.hardware;

                        match &mut node.node_type {
                            NodeType::Control(i) => {
                                i.hardware_id = pick.id();
                                i.control_h = match &i.hardware_id {
                                    Some(hardware_id) => hardware
                                        .controls
                                        .iter()
                                        .find(|h| &h.hardware_id == hardware_id)
                                        .cloned(),

                                    None => None,
                                }
                            }
                            NodeType::Fan(i) => {
                                i.hardware_id = pick.id();
                                i.fan_h = match &i.hardware_id {
                                    Some(hardware_id) => hardware
                                        .fans
                                        .iter()
                                        .find(|h| &h.hardware_id == hardware_id)
                                        .cloned(),

                                    None => None,
                                }
                            }
                            NodeType::Temp(i) => {
                                i.hardware_id = pick.id();
                                i.temp_h = match &i.hardware_id {
                                    Some(hardware_id) => hardware
                                        .temps
                                        .iter()
                                        .find(|h| &h.hardware_id == hardware_id)
                                        .cloned(),

                                    None => None,
                                }
                            }
                            _ => panic!("node have no hardware id"),
                        }
                    }
                    ModifNodeMsg::ReplaceInput(pick) => {
                        let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();
                        node.inputs.clear();

                        if let Some(id_name) = pick.to_couple() {
                            node.inputs.push(id_name)
                        }

                        match &mut node.node_type {
                            NodeType::Control(i) => i.input = pick.name(),
                            NodeType::Graph(i) => i.input = pick.name(),
                            NodeType::Linear(i, ..) => i.input = pick.name(),
                            NodeType::Target(i, ..) => i.input = pick.name(),
                            _ => panic!("node have not exactly one input"),
                        }
                    }
                    ModifNodeMsg::AddInput(pick) => {
                        let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();
                        node.inputs.push(pick.to_couple().unwrap());

                        match &mut node.node_type {
                            NodeType::CustomTemp(i) => i.input.push(pick.name().unwrap()),
                            _ => panic!("node have not multiple inputs"),
                        }
                    }
                    ModifNodeMsg::RemoveInput(pick) => {
                        let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();

                        node.inputs.remove_elem(|i| i.0 == pick.id().unwrap());

                        match &mut node.node_type {
                            NodeType::CustomTemp(i) => {
                                i.input.remove_elem(|n| n == &pick.name().unwrap());
                            }
                            _ => panic!("node have not multiple inputs"),
                        }
                    }
                    ModifNodeMsg::Control(control_msg) => match control_msg {
                        ControlMsg::Active(is_active) => {
                            let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();

                            let NodeType::Control(control) = &mut node.node_type else {
                                panic!()
                            };
                            control.active = is_active;
                        }
                    },
                    ModifNodeMsg::CustomTemp(custom_temp_msg) => match custom_temp_msg {
                        CustomTempMsg::Kind(kind) => {
                            let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();

                            let NodeType::CustomTemp(custom_temp) = &mut node.node_type else {
                                panic!()
                            };
                            custom_temp.kind = kind;
                        }
                    },
                    ModifNodeMsg::Flat(flat_msg) => match flat_msg {
                        FlatMsg::Value(value) => {
                            let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();

                            let NodeType::Flat(flat) = &mut node.node_type else {
                                panic!()
                            };
                            flat.value = value;
                            node.value = Some(value.into());
                        }
                    },
                    ModifNodeMsg::Linear(linear_msg) => {
                        let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();
                        let NodeType::Linear(linear, linear_cache) = &mut node.node_type else {
                            panic!()
                        };

                        match linear_msg {
                            LinearMsg::MinTemp(min_temp, cached_value) => {
                                linear.min_temp = min_temp;
                                linear_cache.min_temp = cached_value;
                            }
                            LinearMsg::MinSpeed(min_speed, cached_value) => {
                                linear.min_speed = min_speed;
                                linear_cache.min_speed = cached_value;
                            }
                            LinearMsg::MaxTemp(max_temp, cached_value) => {
                                linear.max_temp = max_temp;
                                linear_cache.max_temp = cached_value;
                            }
                            LinearMsg::MaxSpeed(max_speed, cached_value) => {
                                linear.max_speed = max_speed;
                                linear_cache.max_speed = cached_value;
                            }
                        }
                    }
                    ModifNodeMsg::Target(target_msg) => {
                        let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();
                        let NodeType::Target(target, target_cache) = &mut node.node_type else {
                            panic!()
                        };

                        match target_msg {
                            TargetMsg::IdleTemp(idle_temp, cached_value) => {
                                target.idle_temp = idle_temp;
                                target_cache.idle_temp = cached_value;
                            }
                            TargetMsg::IdleSpeed(idle_speed, cached_value) => {
                                target.idle_speed = idle_speed;
                                target_cache.idle_speed = cached_value;
                            }
                            TargetMsg::LoadTemp(load_temp, cached_value) => {
                                target.load_temp = load_temp;
                                target_cache.load_temp = cached_value;
                            }
                            TargetMsg::LoadSpeed(load_speed, cached_value) => {
                                target.load_speed = load_speed;
                                target_cache.load_speed = cached_value;
                            }
                        }
                    }
                    ModifNodeMsg::Delete => {}
                }

                self.app_state.update.config_changed();
            }
            AppMsg::SaveConfig => {
                let config = Config::from_app_graph(&self.app_state.app_graph);

                if let Err(e) = self
                    .app_state
                    .dir_manager
                    .save_config(&self.cache.current_config, &config)
                {
                    error!("{:?}", e);
                };
            }
            AppMsg::ChangeConfig(name) => match self.app_state.dir_manager.change_config(&name) {
                Ok(config) => match config {
                    Some((config_name, config)) => {
                        self.cache.current_config = config_name;
                        self.app_state.app_graph =
                            AppGraph::from_config(config, &self.app_state.hardware);
                    }
                    None => {
                        self.cache.current_config.clear();
                    }
                },
                Err(e) => {
                    error!("{:?}", e);
                }
            },
            AppMsg::RemoveConfig(name) => match self.app_state.dir_manager.remove_config(&name) {
                Ok(is_current_config) => {
                    if is_current_config {
                        self.cache.current_config.clear();
                    }
                }
                Err(e) => {
                    error!("can't remove config: {:?}", e);
                }
            },
            AppMsg::CreateConfig(create_config_msg) => match create_config_msg {
                CreateConfigMsg::Init => {}
                CreateConfigMsg::Cancel => {}
                CreateConfigMsg::New(new_name) => {
                    let config = Config::from_app_graph(&self.app_state.app_graph);

                    match self.app_state.dir_manager.create_config(&new_name, &config) {
                        Ok(_) => {
                            self.cache.current_config = new_name;
                        }
                        Err(e) => {
                            error!("can't create config: {:?}", e);
                        }
                    }
                }
            },

            AppMsg::RenameConfig(name) => {
                self.cache.current_config = name;
            }
            AppMsg::Settings(settings_msg) => match settings_msg {
                SettingsMsg::Toggle => {
                    self.core.window.show_context = !self.core.window.show_context;
                    self.set_context_title("Settings".into());
                }
                SettingsMsg::ChangeTheme(theme) => {
                    self.app_state.dir_manager.settings.theme = theme;
                    return cosmic::app::command::set_theme(to_cosmic_theme(
                        &self.app_state.dir_manager.settings.theme,
                    ));
                    // todo: save on fs
                }
            },
            AppMsg::NewNode(_) => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let app_state = &self.app_state;
        let app_graph = &app_state.app_graph;

        Column::new()
            .push(items_view(&app_graph.nodes, &app_state.hardware))
            .into()
    }

    fn header_start(&self) -> Vec<Element<Self::Message>> {
        let mut elems = vec![];

        let app_icon = my_icon("app/toys_fan48").into();
        elems.push(app_icon);

        elems.push(Space::new(Length::Fixed(10.0), 0.0).into());

        let app_name = Text::new("fan-control").into();
        elems.push(app_name);
        elems
    }

    fn header_center(&self) -> Vec<Element<Self::Message>> {
        let settings = &self.app_state.dir_manager.settings;
        let dir_manager = &self.app_state.dir_manager;

        let mut elems = vec![];

        if settings.current_config.is_some() {
            let save_button = icon_button("topBar/save40")
                .on_press(AppMsg::SaveConfig)
                .into();

            elems.push(save_button);
        }

        if !dir_manager.config_names.is_empty() {
            let choose_config = if settings.current_config.is_some() {
                TextInput::new("name", &self.cache.current_config)
                    .on_input(AppMsg::RenameConfig)
                    .width(Length::Fixed(200.0))
                    .into()
            } else {
                PickList::new(
                    &dir_manager.config_names,
                    Some(self.cache.current_config.to_owned()),
                    |name| AppMsg::ChangeConfig(Some(name)),
                )
                .into()
            };
            elems.push(choose_config);
        }

        let new_button = icon_button("sign/plus/add40")
            .on_press(AppMsg::CreateConfig(CreateConfigMsg::Init))
            .into();
        elems.push(new_button);

        elems
    }

    fn header_end(&self) -> Vec<Element<Self::Message>> {
        let mut elems = vec![];

        let settings_button = icon_button("topBar/settings40")
            .on_press(AppMsg::Settings(SettingsMsg::Toggle))
            .into();
        elems.push(settings_button);

        elems
    }

    fn context_drawer(&self) -> Option<Element<Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }
        let app_theme_selected = AppTheme::iter()
            .position(|e| e == self.app_state.dir_manager.settings.theme)
            .unwrap();

        let settings_context =
            widget::settings::view_column(vec![widget::settings::view_section("")
                .add(
                    widget::settings::item::builder("Theme").control(widget::dropdown(
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
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        time::every(Duration::from_millis(
            self.app_state.dir_manager.settings.update_delay,
        ))
        .map(|_| AppMsg::Tick)

        //cosmic::iced_futures::Subscription::none()
    }
}

fn to_cosmic_theme(theme: &AppTheme) -> theme::Theme {
    match theme {
        AppTheme::Dark => theme::Theme::dark(),
        AppTheme::Light => theme::Theme::light(),
        AppTheme::System => theme::system_preference(),
    }
}
