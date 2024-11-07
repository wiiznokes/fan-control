use cosmic::{
    iced_widget::{text, Column},
    widget::tooltip,
    Element,
};
use data::node::NodeTypeLight;

use crate::{icon::icon_button_from_handle, icon_button, node_icon_handle, AppMsg, ToogleMsg};

pub fn add_node_button_view(expanded: bool) -> Element<'static, AppMsg> {
    match expanded {
        true => Column::new()
            .push(add_item(NodeTypeLight::Control, fl!("add_control")))
            .push(add_item(NodeTypeLight::Fan, fl!("add_fan")))
            .push(add_item(NodeTypeLight::Temp, fl!("add_temp")))
            .push(add_item(NodeTypeLight::CustomTemp, fl!("add_custom_temp")))
            .push(add_item(NodeTypeLight::Linear, fl!("add_linear")))
            .push(add_item(NodeTypeLight::Target, fl!("add_target")))
            .push(add_item(NodeTypeLight::Graph, fl!("add_graph")))
            .push(add_item(NodeTypeLight::Flat, fl!("add_flat")))
            .push(icon_button!("close/40").on_press(AppMsg::Toggle(ToogleMsg::CreateButton(false))))
            .into(),

        false => tooltip(
            icon_button!("add/40").on_press(AppMsg::Toggle(ToogleMsg::CreateButton(true))),
            text(fl!("add_item")),
            tooltip::Position::Top,
        )
        .into(),
    }
}

fn add_item<'a>(kind: NodeTypeLight, desc: String) -> Element<'a, AppMsg> {
    tooltip(
        icon_button_from_handle(node_icon_handle!(kind)).on_press(AppMsg::NewNode(kind.clone())),
        text(desc),
        tooltip::Position::Left,
    )
    .into()
}
