use data::{
    id::Id,
    node::{Node, NodeType, Nodes},
};
use hardware::Hardware;
use iced::{
    widget::{Column, Container, PickList, Row, Text, TextInput, Toggler},
    Alignment, Element, Length, Padding,
};

use crate::{
    theme::{CustomContainerStyle, CustomTextInputStyle},
    AppMsg, InputReplaced,
};

fn item_view<'a>(node: &'a Node, content: Element<'a, AppMsg>) -> Element<'a, AppMsg> {
    let mut name =
        TextInput::new("name", &node.name_cached).on_input(|str| AppMsg::NameChange(node.id, str));

    if node.is_error_name {
        name = name.style(iced::theme::TextInput::Custom(Box::new(
            CustomTextInputStyle::Error,
        )));
    }

    let column = Column::new().push(name).push(content);

    Container::new(column)
        .width(Length::Fixed(150.0))
        .padding(Padding::new(10.0))
        .style(iced::theme::Container::Custom(Box::new(
            CustomContainerStyle::Item,
        )))
        .into()
}

fn hardware_view<'a>(
    node: &'a Node,
    hardware: &'a Hardware,
    content: Element<'a, AppMsg>,
) -> Element<'a, AppMsg> {
    let mut h_control_option = vec![Pick::none()];
    h_control_option.extend(hardware.controls.iter().map(|h| Pick::with_name(&h.name)));

    let pick_h_control = PickList::new(
        h_control_option,
        Pick::selected(node.hardware_id().unwrap()),
        |pick| AppMsg::HardwareIdChange(node.id, pick.name),
    )
    .width(Length::Fill);

    let column = Column::new().push(pick_h_control).push(content).into();

    item_view(node, column)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pick {
    name: Option<String>,
    id: Option<Id>,
}

impl Pick {
    fn none() -> Self {
        Pick {
            name: None,
            id: None,
        }
    }
    fn with_name(name: &str) -> Self {
        Pick {
            name: Some(name.to_string()),
            id: None,
        }
    }
    fn with_name_id(name: &str, id: Id) -> Self {
        Pick {
            name: Some(name.to_string()),
            id: Some(id),
        }
    }
    fn selected(option: &Option<String>) -> Option<Self> {
        match option {
            Some(str) => Some(Self::with_name(str)),
            None => Some(Self::none()),
        }
    }
}

impl ToString for Pick {
    fn to_string(&self) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => "None".into(),
        }
    }
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

    let mut input_option = vec![Pick::none()];
    input_option.extend(
        nodes
            .values()
            .filter(|n| {
                node.node_type
                    .to_light()
                    .allowed_dep()
                    .contains(&n.node_type.to_light())
            })
            .map(|n| Pick {
                name: Some(n.name().clone()),
                id: Some(n.id),
            }),
    );
    let pick_behavior = PickList::new(input_option, Pick::selected(&control.input), |pick| {
        AppMsg::InputReplaced(
            node.id,
            InputReplaced {
                input_id: pick.id,
                input_name: pick.name,
            },
        )
    })
    .width(Length::Fill);

    let content = Column::new()
        .push(pick_behavior)
        .push(
            Row::new()
                .push(Text::new(format!("{} %", node.value.unwrap_or(0))))
                .push(Toggler::new(None, !control.auto, |is_active| {
                    AppMsg::ControlAutoChange(node.id, !is_active)
                }))
                // todo: need space_between here
                .align_items(Alignment::End)
                .width(Length::Fill),
        )
        .width(Length::Fill)
        .into();

    hardware_view(node, hardware, content)
}

pub fn temp_view<'a>(node: &'a Node, hardware: &'a Hardware) -> Element<'a, AppMsg> {
    let content = Text::new(format!("{} °C", node.value.unwrap_or(0))).into();

    hardware_view(node, hardware, content)
}
