#![allow(dead_code)]
#![allow(unused_imports)]
use std::time::Duration;

use data::{
    config::custom_temp::CustomTempKind,
    id::Id,
    node::{validate_name, NodeType, NodeTypeLight},
    AppState,
};
use hardware::Value;
use iced::{
    self, executor, subscription, time,
    widget::{
        scrollable::{Direction, Properties},
        Column, Container, Row, Scrollable,
    },
    Application, Command, Element, Length, Subscription,
};
use item::{
    control_view, custom_temp_view, fan_view, flat_view, linear_view, temp_view, LinearMsg,
};
use pick::{IdName, Pick};
use theme::{CustomContainerStyle, CustomScrollableStyle};
use utils::RemoveElem;

#[macro_use]
extern crate log;

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
    Tick,
}

impl Application for Ui {
    type Executor = executor::Default;
    type Message = AppMsg;
    type Theme = iced::Theme;
    type Flags = AppState;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let ui_state = Ui { app_state: flags };

        //dbg!(&ui_state.app_state.app_graph);

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
                        NodeType::Linear((i, _)) => i.name = name,
                        NodeType::Target(i) => i.name = name,
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
                    NodeType::Linear((i, _)) => i.input = pick.name(),
                    NodeType::Target(i) => i.input = pick.name(),
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
                let NodeType::Linear((linear, linear_cache)) = &mut node.node_type else {
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
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let mut controls = Vec::new();
        let mut behaviors = Vec::new();
        let mut temps = Vec::new();
        let mut fans = Vec::new();

        let nodes = &self.app_state.app_graph.nodes;
        let hardware = &self.app_state.hardware;

        for node in self.app_state.app_graph.nodes.values() {
            match node.node_type.to_light() {
                NodeTypeLight::Control => controls.push(control_view(node, nodes, hardware)),
                NodeTypeLight::Fan => fans.push(fan_view(node, hardware)),
                NodeTypeLight::Temp => temps.push(temp_view(node, hardware)),
                NodeTypeLight::CustomTemp => temps.push(custom_temp_view(node, nodes)),
                NodeTypeLight::Graph => {}
                NodeTypeLight::Flat => behaviors.push(flat_view(node)),
                NodeTypeLight::Linear => behaviors.push(linear_view(node, nodes)),
                NodeTypeLight::Target => {}
            }
        }

        let list_views = vec![
            list_view(controls),
            list_view(behaviors),
            list_view(temps),
            list_view(fans),
        ];

        let content = Row::with_children(list_views).spacing(20).padding(25);

        let container = Container::new(content)
            .style(iced::theme::Container::Custom(Box::new(
                CustomContainerStyle::Background,
            )))
            .width(Length::Fill)
            .height(Length::Fill);

        Scrollable::new(container)
            .direction(Direction::Both {
                vertical: Properties::default(),
                horizontal: Properties::default(),
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .style(iced::theme::Scrollable::Custom(Box::new(
                CustomScrollableStyle::Background,
            )))
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        time::every(Duration::from_millis(1000)).map(|_| AppMsg::Tick)
        //Subscription::none()
    }
}

fn list_view(elements: Vec<Element<AppMsg>>) -> Element<AppMsg> {
    Column::with_children(elements)
        .spacing(20)
        .padding(25)
        .into()
}
