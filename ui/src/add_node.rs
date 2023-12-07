use iced::{
    widget::{Button, Column},
    Element,
};

use data::node::NodeTypeLight;

use crate::{
    utils::{icon_button, icon_path_for_node_type},
    AppMsg, UiMsg,
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
            .push(add_item(NodeTypeLight::Flat, fl!("add_flat")))
            .push(
                icon_button("select/close/close40")
                    .on_press(AppMsg::Ui(UiMsg::ToggleCreateButton(false))),
            )
            .into(),

        false => icon_button("sign/plus/add40")
            .on_press(AppMsg::Ui(UiMsg::ToggleCreateButton(true)))
            .into(),
    }
}

fn add_item<'a>(kind: NodeTypeLight, _desc: String) -> Button<'a, AppMsg> {
    let icon_path = icon_path_for_node_type(&kind);
    icon_button(icon_path).on_press(AppMsg::NewNode(kind.clone()))
}
