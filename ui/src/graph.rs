use cosmic::{
    iced::{window, Size},
    iced_core::{Alignment, Length},
    iced_widget::{button, PickList},
    widget::{button::text, container, Column, Row, Space, Text, TextInput},
    Element,
};
use data::{
    app_graph::Nodes,
    config::graph::{Coord, Graph},
    id::Id,
    node::{Input, Node, ValueKind},
};

use crate::{
    icon::icon_button,
    message::{AppMsg, GraphMsg, ModifNodeMsg},
    node_cache::GraphC,
    pick_list_utils::{self, MyOption},
};

pub fn window_settings() -> window::Settings {
    window::Settings {
        size: Size::new(300.0, 200.0),
        resizable: false,
        ..Default::default()
    }
}

pub fn graph_view<'a>(
    node: &'a Node,
    graph: &'a Graph,
    _graph_c: &'a GraphC,
    nodes: &'a Nodes,
) -> Element<'a, AppMsg> {
    let input_options =
        pick_list_utils::input::optional_availlable_inputs(nodes, node, graph.input.is_some());
    let current_input: MyOption<Input> = graph.input.clone().into();
    let pick_input = PickList::new(input_options, Some(current_input), |input| {
        ModifNodeMsg::ReplaceInput(input.into()).to_app(node.id)
    })
    .width(Length::Fill)
    .into();

    let coords = graph.coords.0.iter().map(|coord| {
        let text = format!("{}°C = {}%", coord.temp, coord.percent);

        Row::new()
            .push(Text::new(text).width(Length::Fixed(100.0)))
            .push(Space::new(Length::Fill, Length::Fixed(0.0)))
            .push(icon_button("close/20").on_press(
                ModifNodeMsg::Graph(GraphMsg::RemoveCoord(coord.clone())).to_app(node.id),
            ))
            .align_items(Alignment::Center)
            .into()
    });

    let launch_window = Row::new()
        .push(Text::new(fl!("launch_graph_window")).width(Length::Fixed(100.0)))
        .push(Space::new(Length::Fill, Length::Fixed(0.0)))
        .push(icon_button("add/20").on_press(GraphWindowMsg::Toogle(Some(node.id)).into()))
        .align_items(Alignment::Center)
        .into();

    // todo: add scrollable ?
    let coords = Column::with_children(coords).into();

    let content = vec![
        pick_input,
        launch_window,
        Text::new(node.value_text(&ValueKind::Porcentage)).into(),
        coords,
    ];

    Column::with_children(content).into()
}

#[derive(Debug, Clone)]
pub enum GraphWindowMsg {
    Toogle(Option<Id>),
    ChangeTemp(String),
    ChangePercent(String),
}

pub struct GraphWindow {
    pub window_id: window::Id,
    pub node_id: Id,
    pub temp_c: String,
    pub percent_c: String,
}

pub fn graph_window_view<'a>(
    graph_window: &'a GraphWindow,
    _nodes: &'a Nodes,
) -> Element<'a, AppMsg> {
    let temp_input = Row::new()
        .push(
            TextInput::new("temp", &graph_window.temp_c)
                .on_input(|s| GraphWindowMsg::ChangeTemp(s).into())
                .width(Length::Fixed(70.0)),
        )
        .push(text("°C"))
        .spacing(5)
        .align_items(Alignment::Center);

    let percent_input = Row::new()
        .push(
            TextInput::new("percent", &graph_window.percent_c)
                .on_input(|s| GraphWindowMsg::ChangePercent(s).into())
                .width(Length::Fixed(70.0)),
        )
        .push(text("%"))
        .spacing(5)
        .align_items(Alignment::Center);

    let coord = Coord::try_from((
        graph_window.temp_c.as_ref(),
        graph_window.percent_c.as_ref(),
    ));

    let mut add_button = button("add");

    if let Ok(coord) = coord {
        add_button = add_button
            .on_press(ModifNodeMsg::Graph(GraphMsg::AddCoord(coord)).to_app(graph_window.node_id));
    }

    let inputs_row = Row::new()
        .push(temp_input)
        .push(text("="))
        .push(percent_input)
        .align_items(Alignment::Center)
        .spacing(5);

    let close_button = button("close").on_press(GraphWindowMsg::Toogle(None).into());
    let actions_row = Row::new()
        .push(close_button)
        .push(add_button)
        .spacing(20)
        .align_items(Alignment::Center);

    let content = Column::new()
        .push(inputs_row)
        .push(actions_row)
        .align_items(Alignment::Center)
        .spacing(20);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}
