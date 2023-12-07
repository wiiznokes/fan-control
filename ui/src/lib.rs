#![allow(dead_code)]
//#![allow(unused_imports)]
use std::time::Duration;

use data::{
    app_graph::AppGraph,
    config::Config,
    id::Id,
    node::{validate_name, NodeType, NodeTypeLight},
    settings::AppTheme,
    utils::RemoveElem,
    AppState,
};

use iced::{
    alignment::{Horizontal, Vertical},
    executor, time,
    widget::{Column, Row, Space},
    Application, Command, Element, Length,
};
use item::{items_view, ControlMsg, CustomTempMsg, FlatMsg, LinearMsg, TargetMsg};
use pick::Pick;
use strum::IntoEnumIterator;

use crate::{
    add_node::add_node_button_view,
    headers::{header_center, header_end, header_start, header_wrapper},
};

#[macro_use]
extern crate log;

mod input_line;
mod item;
#[macro_use]
pub mod localize;
mod add_node;
//mod drawer;
mod headers;
mod my_widgets;
mod pick;
mod utils;

pub fn run_ui(app_state: AppState) -> Result<(), Box<dyn std::error::Error>> {
    let settings = iced::Settings::with_flags(app_state);
    Ui::run(settings)?;
    Ok(())
}
pub struct Ui {
    app_state: AppState,
    cache: AppCache,
    create_button_expanded: bool,
    settings_expanded: bool,
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
    CreateConfig(String),
    ModifNode(Id, ModifNodeMsg),
    NewNode(NodeTypeLight),
    DeleteNode(Id),
    Settings(SettingsMsg),

    Ui(UiMsg),
}

#[derive(Debug, Clone)]
pub enum UiMsg {
    ToggleCreateButton(bool),
    ToggleCustomTempKind(Id, bool),
    ToggleSettings(bool),
}

#[derive(Debug, Clone)]
pub enum SettingsMsg {
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
}

impl ModifNodeMsg {
    pub fn to_app(self, id: Id) -> AppMsg {
        AppMsg::ModifNode(id, self)
    }
}

impl iced::Application for Ui {
    type Executor = executor::Default;
    type Message = AppMsg;
    type Flags = AppState;
    type Theme = iced::Theme;

    fn title(&self) -> String {
        "fan-control".into()
    }

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let app_cache = AppCache {
            current_config: flags
                .dir_manager
                .settings()
                .current_config_text()
                .to_owned(),
            theme_list: AppTheme::iter().map(|e| e.to_string()).collect(),
        };

        let ui_state = Ui {
            cache: app_cache,
            app_state: flags,
            create_button_expanded: false,
            settings_expanded: false,
        };
        (ui_state, Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let dir_manager = &mut self.app_state.dir_manager;

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
                            NodeType::CustomTemp(i) => i.inputs.push(pick.name().unwrap()),
                            _ => panic!("node have not multiple inputs"),
                        }
                    }
                    ModifNodeMsg::RemoveInput(pick) => {
                        let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();

                        node.inputs.remove_elem(|i| i.0 == pick.id().unwrap());

                        match &mut node.node_type {
                            NodeType::CustomTemp(i) => {
                                i.inputs.remove_elem(|n| n == &pick.name().unwrap());
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
                }

                self.app_state.update.set_invalid_controls_to_auto(
                    &mut self.app_state.app_graph.nodes,
                    &self.app_state.app_graph.root_nodes,
                    &mut self.app_state.bridge,
                );
            }
            AppMsg::SaveConfig => {
                let config = Config::from_app_graph(&self.app_state.app_graph);

                if let Err(e) = dir_manager.save_config(&self.cache.current_config, &config) {
                    error!("{:?}", e);
                };
            }
            AppMsg::ChangeConfig(selected) => {
                self.app_state.update.set_all_control_to_auto(
                    &mut self.app_state.app_graph.nodes,
                    &self.app_state.app_graph.root_nodes,
                    &mut self.app_state.bridge,
                );

                match dir_manager.change_config(selected) {
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
                }
            }
            AppMsg::RemoveConfig(index) => match dir_manager.remove_config(index) {
                Ok(is_current_config) => {
                    if is_current_config {
                        self.cache.current_config.clear();
                    }
                }
                Err(e) => {
                    error!("can't remove config: {:?}", e);
                }
            },
            AppMsg::CreateConfig(new_name) => {
                let config = Config::from_app_graph(&self.app_state.app_graph);

                match dir_manager.create_config(&new_name, &config) {
                    Ok(_) => {
                        self.cache.current_config = new_name;
                    }
                    Err(e) => {
                        error!("can't create config: {:?}", e);
                    }
                }
            }
            AppMsg::RenameConfig(name) => {
                self.cache.current_config = name;
            }
            AppMsg::Settings(settings_msg) => match settings_msg {
                SettingsMsg::ChangeTheme(theme) => {
                    dir_manager.update_settings(|settings| {
                        settings.theme = theme;
                    });
                }
            },
            AppMsg::NewNode(node_type_light) => {
                self.app_state.app_graph.add_new_node(node_type_light);
            }
            AppMsg::DeleteNode(id) => {
                if let Some(mut node) = self.app_state.app_graph.remove_node(id) {
                    if let NodeType::Control(control) = &mut node.node_type {
                        if let Err(e) = control.set_mode(false, &mut self.app_state.bridge) {
                            error!("{:?}", e);
                        }
                    }
                }

                self.app_state.app_graph.sanitize_inputs()
            }
            AppMsg::Ui(ui_msg) => match ui_msg {
                UiMsg::ToggleCreateButton(expanded) => self.create_button_expanded = expanded,
                UiMsg::ToggleCustomTempKind(id, expanded) => {
                    self.app_state
                        .app_graph
                        .get_custom_temp_mut(id)
                        .kind_expanded = expanded;
                }
                UiMsg::ToggleSettings(expanded) => {
                    self.settings_expanded = expanded;
                }
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        use my_widgets::floating_element;

        let app_state = &self.app_state;
        let app_graph = &app_state.app_graph;

        let header_bar = Row::new()
            .push(header_wrapper(header_start()))
            .push(Space::new(Length::Fill, 0.0))
            .push(header_wrapper(header_center(
                &app_state.dir_manager,
                &self.cache.current_config,
            )))
            .push(header_wrapper(header_end(self.settings_expanded)))
            .align_items(iced::Alignment::Center)
            .width(Length::Fill);

        let content = Column::new()
            .push(header_bar)
            .push(items_view(&app_graph.nodes, &app_state.hardware));

        let mut content_with_floating_button = floating_element::FloatingElement::new(
            content,
            add_node_button_view(self.create_button_expanded),
            floating_element::Anchor::new(Vertical::Bottom, Horizontal::Right),
        )
        .offset(floating_element::Offset::new(15.0, 10.0));

        if self.create_button_expanded {
            content_with_floating_button = content_with_floating_button
                .on_dismiss(Some(AppMsg::Ui(UiMsg::ToggleCreateButton(false))));
        }

        content_with_floating_button.into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        time::every(Duration::from_millis(
            self.app_state.dir_manager.settings().update_delay,
        ))
        .map(|_| AppMsg::Tick)

        //cosmic::iced_futures::Subscription::none()
    }
}
