#![allow(dead_code)]
#![allow(unused_imports)]
use std::time::Duration;

use data::{
    config::custom_temp::CustomTempKind,
    id::Id,
    node::{validate_name, NodeType},
    AppState,
};

use iced::{self, executor, time, Application, Command, Element};
use item::{items_view, LinearMsg, TargetMsg};
use pick::Pick;

use utils::RemoveElem;

#[macro_use]
extern crate log;

mod input_line;
mod item;
mod pick;
mod theme;
mod utils;
mod widgets;

pub fn run_ui(app_state: AppState) -> Result<(), iced::Error> {
    let settings = iced::Settings::with_flags(app_state);

    Ui::run(settings)
}
pub struct Ui {
    app_state: AppState,
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    Rename(Id, String),
    ChangeHardware(Id, Pick<String>),
    ReplaceInput(Id, Pick<Id>),
    AddInput(Id, Pick<Id>),
    RemoveInput(Id, Pick<Id>),
    ChangeControlAuto(Id, bool),
    ChangeCustomTempKind(Id, CustomTempKind),
    ChangeFlatValue(Id, u16),
    ChangeLinear(Id, LinearMsg),
    ChangeTarget(Id, TargetMsg),
    Tick,
}

impl Application for Ui {
    type Executor = executor::Default;
    type Message = AppMsg;
    type Theme = iced::Theme;
    type Flags = AppState;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let ui_state = Ui { app_state: flags };
        (ui_state, Command::none())
    }

    fn title(&self) -> String {
        String::from("fan-control")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            AppMsg::Tick => {
                match self.app_state.update.graph(
                    &mut self.app_state.app_graph.nodes,
                    &self.app_state.app_graph.root_nodes,
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{:?}", e);
                        self.app_state.update.clear_cache();
                    }
                }
            }
            AppMsg::Rename(id, name) => {
                let name_is_valid = validate_name(&self.app_state.app_graph.nodes, &id, &name);

                let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();
                node.name_cached = name.clone();
                if name_is_valid {
                    node.is_error_name = false;
                    match &mut node.node_type {
                        NodeType::Control(i) => i.name = name,
                        NodeType::Fan(i) => i.name = name,
                        NodeType::Temp(i) => i.name = name,
                        NodeType::CustomTemp(i) => i.name = name,
                        NodeType::Graph(i) => i.name = name,
                        NodeType::Flat(i) => i.name = name,
                        NodeType::Linear(i, ..) => i.name = name,
                        NodeType::Target(i, ..) => i.name = name,
                    }
                } else {
                    node.is_error_name = true;
                }
            }
            AppMsg::ChangeHardware(id, pick) => {
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
            AppMsg::ReplaceInput(id, pick) => {
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
            AppMsg::ChangeControlAuto(id, auto) => {
                let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();

                let NodeType::Control(control) = &mut node.node_type else {
                    panic!()
                };
                control.auto = auto;
            }
            AppMsg::AddInput(id, pick) => {
                let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();
                node.inputs.push(pick.to_couple().unwrap());

                match &mut node.node_type {
                    NodeType::CustomTemp(i) => i.input.push(pick.name().unwrap()),
                    _ => panic!("node have not multiple inputs"),
                }
            }
            AppMsg::RemoveInput(id, pick) => {
                let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();

                node.inputs.remove_elem(|i| i.0 == pick.id().unwrap());

                match &mut node.node_type {
                    NodeType::CustomTemp(i) => {
                        i.input.remove_elem(|n| n == &pick.name().unwrap());
                    }
                    _ => panic!("node have not multiple inputs"),
                }
            }
            AppMsg::ChangeCustomTempKind(id, kind) => {
                let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();

                let NodeType::CustomTemp(custom_temp) = &mut node.node_type else {
                    panic!()
                };
                custom_temp.kind = kind;
            }
            AppMsg::ChangeFlatValue(id, value) => {
                let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();

                let NodeType::Flat(flat) = &mut node.node_type else {
                    panic!()
                };
                flat.value = value;
            }
            AppMsg::ChangeLinear(id, linear_msg) => {
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
            AppMsg::ChangeTarget(id, target_msg) => {
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

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        items_view(&self.app_state.app_graph.nodes, &self.app_state.hardware)
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        time::every(Duration::from_millis(1000)).map(|_| AppMsg::Tick)
        //Subscription::none()
    }
}
