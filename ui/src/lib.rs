#![allow(dead_code)]
#![allow(unused_imports)]
use std::time::Duration;

use data::{
    id::Id,
    node::{validate_name, NodeType},
    AppState,
};
use iced::{
    self, executor, time,
    widget::{
        scrollable::{Direction, Properties},
        Column, Container, Row, Scrollable,
    },
    Application, Command, Element, Length,
};
use item::{control_view, fan_view, temp_view};
use pick::Pick;
use theme::{CustomContainerStyle, CustomScrollableStyle};

#[macro_use]
extern crate log;

mod item;
mod pick;
mod theme;
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
    NameChange(Id, String),
    HardwareIdChange(Id, Option<String>),
    InputReplaced(Id, Pick<Id>),
    ControlAutoChange(Id, bool),
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
            AppMsg::NameChange(id, name) => {
                let name_is_valid = validate_name(&self.app_state.app_graph.nodes, &id, &name);

                let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();
                node.name_cached = name.clone();
                if name_is_valid {
                    node.is_error_name = false;
                    match &mut node.node_type {
                        data::node::NodeType::Control(i) => i.name = name,
                        data::node::NodeType::Fan(i) => i.name = name,
                        data::node::NodeType::Temp(i) => i.name = name,
                        data::node::NodeType::CustomTemp(i) => i.name = name,
                        data::node::NodeType::Graph(i) => i.name = name,
                        data::node::NodeType::Flat(i) => i.name = name,
                        data::node::NodeType::Linear(i) => i.name = name,
                        data::node::NodeType::Target(i) => i.name = name,
                    }
                } else {
                    node.is_error_name = true;
                }
            }
            AppMsg::HardwareIdChange(id, hardware_id) => {
                let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();

                match &mut node.node_type {
                    data::node::NodeType::Control(i) => i.hardware_id = hardware_id,
                    data::node::NodeType::Fan(i) => i.hardware_id = hardware_id,
                    data::node::NodeType::Temp(i) => i.hardware_id = hardware_id,
                    _ => panic!("node have no hardware id"),
                }
            }
            AppMsg::InputReplaced(id, pick) => {
                let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();
                node.inputs.clear();
                if let Some(input_id) = pick.id {
                    node.inputs.push(input_id);
                }

                match &mut node.node_type {
                    data::node::NodeType::Control(i) => i.input = pick.name,
                    data::node::NodeType::Graph(i) => i.input = pick.name,
                    data::node::NodeType::Linear(i) => i.input = pick.name,
                    data::node::NodeType::Target(i) => i.input = pick.name,
                    _ => panic!("node have not exactly one input"),
                }
            }
            AppMsg::ControlAutoChange(id, auto) => {
                let node = self.app_state.app_graph.nodes.get_mut(&id).unwrap();

                let NodeType::Control(control) = &mut node.node_type else {
                    panic!()
                };
                control.auto = auto;
            }
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let mut controls = Vec::new();

        let mut temps = Vec::new();
        let mut fans = Vec::new();

        for node in self.app_state.app_graph.nodes.values() {
            match node.node_type.to_light() {
                data::node::NodeTypeLight::Control => controls.push(control_view(
                    node,
                    &self.app_state.app_graph.nodes,
                    &self.app_state.hardware,
                )),
                data::node::NodeTypeLight::Fan => {
                    fans.push(fan_view(node, &self.app_state.hardware))
                }
                data::node::NodeTypeLight::Temp => {
                    temps.push(temp_view(node, &self.app_state.hardware))
                }
                data::node::NodeTypeLight::CustomTemp => {}
                data::node::NodeTypeLight::Graph => {}
                data::node::NodeTypeLight::Flat => {}
                data::node::NodeTypeLight::Linear => {}
                data::node::NodeTypeLight::Target => {}
            }
        }

        let list_views = vec![list_view(controls), list_view(temps), list_view(fans)];

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
    }
}

fn list_view(elements: Vec<Element<AppMsg>>) -> Element<AppMsg> {
    Column::with_children(elements)
        .spacing(20)
        .padding(25)
        .into()
}
