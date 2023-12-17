//#![allow(dead_code)]
//#![allow(unused_imports)]
use std::time::Duration;

use data::{
    app_graph::AppGraph,
    config::Config,
    node::{validate_name, NodeType},
    settings::AppTheme,
    utils::RemoveElem,
    AppState,
};
use item::items_view;
use message::{ConfigMsg, ModifNodeMsg, SettingsMsg, ToogleMsg};
use node_cache::{NodeC, NodeTypeC, NodesC};

use crate::settings_drawer::settings_drawer;

use cosmic::{
    app::{command, Command, Core},
    executor,
    iced::{self, time},
    iced_core::Length,
    theme,
    widget::{Column, Row, Space},
    ApplicationExt, Element,
};

use crate::message::{AppMsg, ControlMsg, CustomTempMsg, FlatMsg, LinearMsg, TargetMsg};

use crate::add_node::add_node_button_view;

#[macro_use]
extern crate log;

#[macro_use]
pub mod localize;

mod add_node;
mod headers;
mod input_line;
mod item;
mod message;
mod my_widgets;
mod node_cache;
mod settings_drawer;
mod utils;

pub fn run_ui(app_state: AppState) -> Result<(), Box<dyn std::error::Error>> {
    let settings = cosmic::app::Settings::default();
    cosmic::app::run::<Ui>(settings, app_state)?;
    Ok(())
}
pub struct Ui {
    core: Core,
    app_state: AppState,
    current_config_cached: String,
    create_button_expanded: bool,
    choose_config_expanded: bool,
    nodes_c: NodesC,
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
        let current_config_cached = flags
            .dir_manager
            .settings()
            .current_config_text()
            .to_owned();

        let commands = Command::batch([
            command::set_theme(to_cosmic_theme(&flags.dir_manager.settings().theme)),
            command::message(cosmic::app::Message::App(AppMsg::Tick)),
        ]);

        let ui_state = Ui {
            nodes_c: NodesC::new(flags.app_graph.nodes.values()),
            app_state: flags,
            core,
            create_button_expanded: false,
            choose_config_expanded: false,
            current_config_cached,
        };
        (ui_state, commands)
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

            AppMsg::ModifNode(id, modif_node_msg) => {
                match modif_node_msg {
                    ModifNodeMsg::ChangeHardware(hardware_id) => {
                        let node = self.app_state.app_graph.get_mut(&id);
                        let hardware = &self.app_state.hardware;

                        match &mut node.node_type {
                            NodeType::Control(i) => {
                                i.hardware_id = hardware_id;
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
                                i.hardware_id = hardware_id;
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
                                i.hardware_id = hardware_id;
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
                    ModifNodeMsg::ReplaceInput(input) => {
                        let node = self.app_state.app_graph.get_mut(&id);
                        node.inputs.clear();

                        if let Some(input) = &input {
                            node.inputs.push(input.clone())
                        }

                        let optional_name = match input {
                            Some(input) => Some(input.name),
                            None => None,
                        };
                        match &mut node.node_type {
                            NodeType::Control(i) => i.input = optional_name,
                            NodeType::Graph(i) => i.input = optional_name,
                            NodeType::Linear(i, ..) => i.input = optional_name,
                            NodeType::Target(i, ..) => i.input = optional_name,
                            _ => panic!("node have not exactly one input"),
                        }
                    }
                    ModifNodeMsg::AddInput(input) => {
                        let node = self.app_state.app_graph.get_mut(&id);
                        node.inputs.push(input.clone());

                        match &mut node.node_type {
                            NodeType::CustomTemp(i) => i.inputs.push(input.name),
                            _ => panic!("node have not multiple inputs"),
                        }
                    }
                    ModifNodeMsg::RemoveInput(input) => {
                        let node = self.app_state.app_graph.get_mut(&id);

                        node.inputs.remove_elem(|i| i.id == input.id);

                        match &mut node.node_type {
                            NodeType::CustomTemp(i) => {
                                i.inputs.remove_elem(|n| n == &input.name);
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
                        let NodeType::Linear(linear) = &mut node.node_type else {
                            panic!()
                        };

                        let NodeTypeC::Linear(linear_c) =
                            &mut self.nodes_c.get_mut(&id).node_type_c
                        else {
                            panic!()
                        };

                        match linear_msg {
                            LinearMsg::MinTemp(min_temp, cached_value) => {
                                linear.min_temp = min_temp;
                                linear_c.min_temp = cached_value;
                            }
                            LinearMsg::MinSpeed(min_speed, cached_value) => {
                                linear.min_speed = min_speed;
                                linear_c.min_speed = cached_value;
                            }
                            LinearMsg::MaxTemp(max_temp, cached_value) => {
                                linear.max_temp = max_temp;
                                linear_c.max_temp = cached_value;
                            }
                            LinearMsg::MaxSpeed(max_speed, cached_value) => {
                                linear.max_speed = max_speed;
                                linear_c.max_speed = cached_value;
                            }
                        }
                    }
                    ModifNodeMsg::Target(target_msg) => {
                        let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();
                        let NodeType::Target(target) = &mut node.node_type else {
                            panic!()
                        };

                        let NodeTypeC::Target(target_c) =
                            &mut self.nodes_c.get_mut(&id).node_type_c
                        else {
                            panic!()
                        };

                        match target_msg {
                            TargetMsg::IdleTemp(idle_temp, cached_value) => {
                                target.idle_temp = idle_temp;
                                target_c.idle_temp = cached_value;
                            }
                            TargetMsg::IdleSpeed(idle_speed, cached_value) => {
                                target.idle_speed = idle_speed;
                                target_c.idle_speed = cached_value;
                            }
                            TargetMsg::LoadTemp(load_temp, cached_value) => {
                                target.load_temp = load_temp;
                                target_c.load_temp = cached_value;
                            }
                            TargetMsg::LoadSpeed(load_speed, cached_value) => {
                                target.load_speed = load_speed;
                                target_c.load_speed = cached_value;
                            }
                        }
                    }
                    ModifNodeMsg::Delete => {
                        if let Some(mut node) = self.app_state.app_graph.remove_node(id) {
                            if let NodeType::Control(control) = &mut node.node_type {
                                if let Err(e) = control.set_mode(false, &mut self.app_state.bridge)
                                {
                                    error!("{:?}", e);
                                }
                            }
                        }

                        self.nodes_c.remove(&id);
                        self.app_state.app_graph.sanitize_inputs(false)
                    }
                }

                self.app_state.update.set_invalid_controls_to_auto(
                    &mut self.app_state.app_graph.nodes,
                    &self.app_state.app_graph.root_nodes,
                    &mut self.app_state.bridge,
                );
            }

            AppMsg::Settings(settings_msg) => match settings_msg {
                SettingsMsg::Theme(theme) => {
                    dir_manager.update_settings(|settings| {
                        settings.theme = theme;
                    });
                    return cosmic::app::command::set_theme(to_cosmic_theme(&theme));
                }
                SettingsMsg::UpdateDelay(update_delay) => dir_manager.update_settings(|settings| {
                    settings.update_delay = update_delay;
                }),
            },
            AppMsg::NewNode(node_type_light) => {
                let node = self.app_state.app_graph.create_new_node(node_type_light);
                let node_c = NodeC::new(&node);
                self.nodes_c.insert(node.id, node_c);
                self.app_state.app_graph.insert_node(node);
            }
            AppMsg::Toggle(ui_msg) => match ui_msg {
                ToogleMsg::CreateButton(expanded) => self.create_button_expanded = expanded,
                ToogleMsg::Settings => {
                    self.core.window.show_context = !self.core.window.show_context;
                    self.set_context_title(fl!("settings"));
                }
                ToogleMsg::ChooseConfig(expanded) => {
                    self.choose_config_expanded = expanded;
                }
                ToogleMsg::NodeContextMenu(id, expanded) => {
                    let node_c = self.nodes_c.get_mut(&id);
                    node_c.context_menu_expanded = expanded;
                }
            },
            AppMsg::Config(config_msg) => match config_msg {
                ConfigMsg::Save => {
                    let config = Config::from_app_graph(&self.app_state.app_graph);

                    if let Err(e) = dir_manager.save_config(&self.current_config_cached, &config) {
                        error!("{:?}", e);
                    };
                }
                ConfigMsg::Change(selected) => {
                    self.choose_config_expanded = false;
                    self.app_state.update.set_all_control_to_auto(
                        &mut self.app_state.app_graph.nodes,
                        &self.app_state.app_graph.root_nodes,
                        &mut self.app_state.bridge,
                    );

                    match dir_manager.change_config(selected) {
                        Ok(config) => match config {
                            Some((config_name, config)) => {
                                self.current_config_cached = config_name;
                                self.app_state.app_graph =
                                    AppGraph::from_config(config, &self.app_state.hardware);
                                self.nodes_c = NodesC::new(self.app_state.app_graph.nodes.values());
                                self.app_state.update.all(
                                    &mut self.app_state.app_graph.nodes,
                                    &self.app_state.app_graph.root_nodes,
                                    &mut self.app_state.bridge,
                                );
                            }
                            None => {
                                self.current_config_cached.clear();
                            }
                        },
                        Err(e) => {
                            error!("{:?}", e);
                        }
                    }
                }
                ConfigMsg::Delete(name) => match dir_manager.remove_config(name) {
                    Ok(is_current_config) => {
                        if is_current_config {
                            self.current_config_cached.clear();
                        }
                    }
                    Err(e) => {
                        error!("can't remove config: {:?}", e);
                    }
                },
                ConfigMsg::Create(new_name) => {
                    let config = Config::from_app_graph(&self.app_state.app_graph);

                    match dir_manager.create_config(&new_name, &config) {
                        Ok(_) => {
                            self.current_config_cached = new_name;
                        }
                        Err(e) => {
                            error!("can't create config: {:?}", e);
                        }
                    }
                }
                ConfigMsg::Rename(name) => {
                    self.current_config_cached = name;
                }
            },
            AppMsg::Rename(id, name) => {
                let name_is_valid = validate_name(&self.app_state.app_graph.nodes, &id, &name);

                let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();
                let nodec = self.nodes_c.get_mut(&id);

                nodec.name = name.clone();
                if name_is_valid {
                    nodec.is_error_name = false;
                    let previous_name = node.name().clone();
                    node.node_type.set_name(&name);

                    let node_id = node.id;
                    // find nodes that depend on node.id
                    // change the name in input and item.input

                    for n in self.app_state.app_graph.nodes.values_mut() {
                        if let Some(node_input) = n
                            .inputs
                            .iter_mut()
                            .find(|node_input| node_input.id == node_id)
                        {
                            node_input.name = name.clone();
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
                    nodec.is_error_name = true;
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let app_state = &self.app_state;
        let app_graph = &app_state.app_graph;

        let content = items_view(&app_graph.nodes, &self.nodes_c, &app_state.hardware);

        let floating_button = Column::new()
            .push(Space::new(0.0, Length::Fill))
            .push(add_node_button_view(self.create_button_expanded));

        Row::new().push(content).push(floating_button).into()
    }

    fn header_start(&self) -> Vec<Element<Self::Message>> {
        headers::header_start()
    }

    fn header_center(&self) -> Vec<Element<Self::Message>> {
        headers::header_center(
            &self.app_state.dir_manager,
            &self.current_config_cached,
            self.choose_config_expanded,
        )
    }

    fn header_end(&self) -> Vec<Element<Self::Message>> {
        headers::header_end()
    }

    fn context_drawer(&self) -> Option<Element<Self::Message>> {
        settings_drawer(self.core.window.show_context, &self.app_state.dir_manager)
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        time::every(Duration::from_millis(
            self.app_state.dir_manager.settings().update_delay,
        ))
        .map(|_| AppMsg::Tick)

        //cosmic::iced_futures::Subscription::none()
    }

    fn on_app_exit(&mut self) {
        if let Err(e) = self.app_state.bridge.shutdown() {
            error!("{:?}", e);
        }
    }

    fn on_close_requested(&self, _id: iced::window::Id) -> Option<Self::Message> {
        // todo pop up. Need to use settings to not close auto
        None
    }
}

fn to_cosmic_theme(theme: &AppTheme) -> theme::Theme {
    match theme {
        AppTheme::Dark => theme::Theme::dark(),
        AppTheme::Light => theme::Theme::light(),
        AppTheme::System => theme::system_preference(),
    }
}
