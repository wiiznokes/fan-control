#![allow(dead_code)]
//#![allow(unused_imports)]
use std::time::Duration;

use data::{
    id::Id,
    node::{validate_name, NodeType},
    AppState,
};

use cosmic::{
    app::{Command, Core},
    executor,
    iced::{self, time},
    Element,
};

use item::{items_view, ControlMsg, CustomTempMsg, FlatMsg, LinearMsg, TargetMsg};
use pick::Pick;
use utils::RemoveElem;

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
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    Tick,
    ChangeConfig(Id, ChangeConfigMsg),
}

#[derive(Debug, Clone)]
pub enum ChangeConfigMsg {
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
        let ui_state = Ui {
            app_state: flags,
            core,
        };
        (ui_state, Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMsg::Tick => {
                match self.app_state.update.graph(
                    &mut self.app_state.app_graph.nodes,
                    &self.app_state.app_graph.root_nodes,
                    &mut self.app_state.bridge,
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{:?}", e);
                        self.app_state.update.clear_cache();
                    }
                }
            }

            AppMsg::ChangeConfig(id, change_config) => {
                match change_config {
                    ChangeConfigMsg::Rename(name) => {
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
                    ChangeConfigMsg::ChangeHardware(pick) => {
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
                    ChangeConfigMsg::ReplaceInput(pick) => {
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
                    ChangeConfigMsg::AddInput(pick) => {
                        let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();
                        node.inputs.push(pick.to_couple().unwrap());

                        match &mut node.node_type {
                            NodeType::CustomTemp(i) => i.input.push(pick.name().unwrap()),
                            _ => panic!("node have not multiple inputs"),
                        }
                    }
                    ChangeConfigMsg::RemoveInput(pick) => {
                        let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();

                        node.inputs.remove_elem(|i| i.0 == pick.id().unwrap());

                        match &mut node.node_type {
                            NodeType::CustomTemp(i) => {
                                i.input.remove_elem(|n| n == &pick.name().unwrap());
                            }
                            _ => panic!("node have not multiple inputs"),
                        }
                    }
                    ChangeConfigMsg::Control(control_msg) => match control_msg {
                        ControlMsg::Auto(auto) => {
                            let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();

                            let NodeType::Control(control) = &mut node.node_type else {
                                panic!()
                            };
                            control.auto = auto;
                        }
                    },
                    ChangeConfigMsg::CustomTemp(custom_temp_msg) => match custom_temp_msg {
                        CustomTempMsg::Kind(kind) => {
                            let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();

                            let NodeType::CustomTemp(custom_temp) = &mut node.node_type else {
                                panic!()
                            };
                            custom_temp.kind = kind;
                        }
                    },
                    ChangeConfigMsg::Flat(flat_msg) => match flat_msg {
                        FlatMsg::Value(value) => {
                            let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();

                            let NodeType::Flat(flat) = &mut node.node_type else {
                                panic!()
                            };
                            flat.value = value;
                        }
                    },
                    ChangeConfigMsg::Linear(linear_msg) => {
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
                    ChangeConfigMsg::Target(target_msg) => {
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

                self.app_state.update.config_changed();
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        items_view(&self.app_state.app_graph.nodes, &self.app_state.hardware)
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        time::every(Duration::from_millis(
            self.app_state.settings.update_delay as u64,
        ))
        .map(|_| AppMsg::Tick)
        //Subscription::none()
    }
}
