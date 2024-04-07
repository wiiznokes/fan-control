use cosmic::{
    iced_core::{Alignment, Length},
    iced_widget::PickList,
    widget::{Column, Row, Space, Text},
    Element,
};
use data::{
    app_graph::Nodes,
    config::graph::Graph,
    node::{Input, Node, ValueKind},
};

use crate::{
    icon::icon_button, message::{
        AppMsg, GraphMsg, ModifNodeMsg,
    }, node_cache::GraphC, pick_list_utils::{self, MyOption}
};

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

    // todo: add scrollable ?
    let coords = Column::with_children(coords).into();

    let content = vec![
        pick_input,
        Text::new(node.value_text(&ValueKind::Porcentage)).into(),
        coords,
    ];

    Column::with_children(content).into()
}
