use data::node::{Node, NodeType, Nodes};
use hardware::Hardware;
use iced::{
    widget::{Column, Container, PickList, Row, Text, TextInput, Toggler},
    Alignment, Element, Length, Padding,
};

use crate::{
    theme::{CustomContainerStyle, CustomTextInputStyle},
    AppMsg, InputReplaced,
};

fn item_view(content: Element<AppMsg>) -> Element<AppMsg> {
    Container::new(content)
        .width(Length::Fixed(150.0))
        .padding(Padding::new(10.0))
        .style(iced::theme::Container::Custom(Box::new(
            CustomContainerStyle::Item,
        )))
        .into()
}

pub fn control_view<'a>(
    node: &'a Node,
    nodes: &'a Nodes,
    hardware: &'a Hardware,
) -> Element<'a, AppMsg> {
    debug!("build control ui");

    let NodeType::Control(control) = &node.node_type else {
        panic!()
    };

    let mut name = TextInput::new("control name", &node.name_cached)
        .on_input(|str| AppMsg::NameChange(node.id, str));

    if node.is_error_name {
        name = name.style(iced::theme::TextInput::Custom(Box::new(
            CustomTextInputStyle::Error,
        )));
    }

    let mut h_control_option = hardware
        .controls
        .iter()
        .map(|h| Some(h.name.clone()))
        .collect::<Vec<_>>();

    h_control_option.insert(0, None);

    let pick_h_control = PickList::new(
        h_control_option
            .iter()
            .map(|h| match &h {
                Some(name) => name.clone(),
                None => "None".into(),
            })
            .collect::<Vec<_>>(),
        if let Some(hardware_id) = &control.hardware_id {
            Some(hardware_id.clone())
        } else {
            Some("None".into())
        },
        |hardware_id| AppMsg::HardwareIdChange(node.id, Some(hardware_id)),
    )
    .width(Length::Fill);

    let mut input_option = nodes
        .values()
        .filter(|n| {
            node.node_type
                .to_light()
                .allowed_dep()
                .contains(&n.node_type.to_light())
        })
        .map(|n| (Some(n.id), Some(n.name().clone())))
        .collect::<Vec<_>>();

    input_option.insert(0, (None, None));

    // todo: fork pick list and to allow giving a complex object as options
    // similar to drop down menu in kotlin
    let pick_behavior = PickList::new(
        input_option
            .iter()
            .map(|i| match &i.1 {
                Some(name) => name.clone(),
                None => "None".into(),
            })
            .collect::<Vec<_>>(),
        if let Some(input) = &control.input {
            Some(input.clone())
        } else {
            Some("None".into())
        },
        move |node_name| {
            let input_replaced = if node_name == "None" {
                InputReplaced {
                    input_id: None,
                    input_name: None,
                }
            } else {
                let f = input_option
                    .iter()
                    .find(|i| match &i.1 {
                        Some(name) => name == &node_name,
                        None => false,
                    })
                    .unwrap();
                InputReplaced {
                    input_id: f.0,
                    input_name: f.1.clone(),
                }
            };
            AppMsg::InputReplaced(node.id, input_replaced)
        },
    )
    .width(Length::Fill);

    let content = Column::new()
        .push(name)
        .push(pick_h_control)
        .push(pick_behavior)
        .push(
            Row::new()
                .push(Text::new(format!("{} %", node.value.unwrap_or(0))))
                .push(Toggler::new(None, !control.auto, |is_active| {
                    AppMsg::ControlAutoChange(node.id, !is_active)
                }))
                // todo: need space between here
                .align_items(Alignment::End)
                .width(Length::Fill),
        )
        .width(Length::Fill);

    item_view(content.into())
}
