use cosmic::{iced_widget::Column, widget::IconButton, Element};
use data::node::NodeTypeLight;

use crate::{
    icon::{icon_button, icon_path_for_node_type},
    AppMsg, ToogleMsg,
};

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
            .push(icon_button("close/40").on_press(AppMsg::Toggle(ToogleMsg::CreateButton(false))))
            .into(),

        false => icon_button("add/40")
            .on_press(AppMsg::Toggle(ToogleMsg::CreateButton(true)))
            .tooltip(fl!("add_item"))
            .into(),
    }
}

fn add_item<'a>(kind: NodeTypeLight, desc: String) -> IconButton<'a, AppMsg> {
    let icon_path = icon_path_for_node_type(&kind);
    icon_button(icon_path)
        .on_press(AppMsg::NewNode(kind.clone()))
        .tooltip(desc)
}
