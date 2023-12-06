use cosmic::{iced_widget::Column, Element};
use data::node::NodeTypeLight;

use crate::{
    utils::{icon_button, icon_path_for_node_type},
    AppMsg,
};

pub fn add_node_button_view(expanded: bool) -> Element<'static, AppMsg> {
    match expanded {
        true => {
            let mut icons: Vec<Element<_>> = NodeTypeLight::VALUES
                .iter()
                .map(|node_type| {
                    let icon_path = icon_path_for_node_type(node_type);
                    icon_button(icon_path)
                        .on_press(AppMsg::NewNode(node_type.clone()))
                        .into()
                })
                .collect();

            icons.push(
                icon_button("select/close/close40")
                    .on_press(AppMsg::CreateButton(false))
                    .into(),
            );
            Column::with_children(icons).into()
        }
        false => icon_button("sign/plus/add40")
            .on_press(AppMsg::CreateButton(true))
            .into(),
    }
}
