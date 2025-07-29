use cosmic::{
    Element,
    iced::padding,
    iced_widget::text,
    widget::{column, tooltip},
};
use data::node::NodeTypeLight;

use crate::{AppMsg, ToogleMsg, icon::icon_button_from_handle, icon_button, node_icon_handle};

const ICON_SIZE: u16 = 25;

pub fn add_node_button_view(expanded: bool) -> Element<'static, AppMsg> {
    let column = column().padding(padding::bottom(15));

    match expanded {
        true => column
            .push(add_item(NodeTypeLight::Control, fl!("add_control")))
            .push(add_item(NodeTypeLight::Fan, fl!("add_fan")))
            .push(add_item(NodeTypeLight::Temp, fl!("add_temp")))
            .push(add_item(NodeTypeLight::CustomTemp, fl!("add_custom_temp")))
            .push(add_item(NodeTypeLight::Linear, fl!("add_linear")))
            .push(add_item(NodeTypeLight::Target, fl!("add_target")))
            .push(add_item(NodeTypeLight::Graph, fl!("add_graph")))
            .push(add_item(NodeTypeLight::Flat, fl!("add_flat")))
            .push(
                icon_button!("close/24")
                    .on_press(AppMsg::Toggle(ToogleMsg::CreateButton(false)))
                    .icon_size(ICON_SIZE),
            )
            .into(),

        false => column
            .push(tooltip(
                icon_button!("add/24")
                    .on_press(AppMsg::Toggle(ToogleMsg::CreateButton(true)))
                    .icon_size(ICON_SIZE),
                text(fl!("add_item")),
                tooltip::Position::Top,
            ))
            .into(),
    }
}

fn add_item<'a>(kind: NodeTypeLight, desc: String) -> Element<'a, AppMsg> {
    tooltip(
        icon_button_from_handle(node_icon_handle!(kind))
            .on_press(AppMsg::NewNode(kind.clone()))
            .icon_size(ICON_SIZE),
        text(desc),
        tooltip::Position::Left,
    )
    .into()
}
