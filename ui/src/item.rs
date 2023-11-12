use data::{
    config::custom_temp::CustomTempKind,
    node::{Node, NodeType, NodeTypeLight, Nodes},
};
use hardware::Hardware;
use iced::{
    widget::{
        scrollable::{Direction, Properties},
        Button, Column, Container, PickList, Row, Scrollable, Slider, Text, TextInput, Toggler,
    },
    Alignment, Element, Length, Padding,
};

use crate::{
    input_line::input_line,
    pick::{pick_hardware, pick_input, Pick},
    theme::{CustomContainerStyle, CustomScrollableStyle, CustomTextInputStyle},
    AppMsg,
};

pub fn items_view<'a>(nodes: &'a Nodes, hardware: &'a Hardware) -> Element<'a, AppMsg> {
    let mut controls = Vec::new();
    let mut behaviors = Vec::new();
    let mut temps = Vec::new();
    let mut fans = Vec::new();

    for node in nodes.values() {
        match node.node_type.to_light() {
            NodeTypeLight::Control => controls.push(control_view(node, nodes, hardware)),
            NodeTypeLight::Fan => fans.push(fan_view(node, hardware)),
            NodeTypeLight::Temp => temps.push(temp_view(node, hardware)),
            NodeTypeLight::CustomTemp => temps.push(custom_temp_view(node, nodes)),
            NodeTypeLight::Graph => {}
            NodeTypeLight::Flat => behaviors.push(flat_view(node)),
            NodeTypeLight::Linear => behaviors.push(linear_view(node, nodes)),
            NodeTypeLight::Target => behaviors.push(target_view(node, nodes)),
        }
    }

    let list_views = vec![
        list_view(controls),
        list_view(behaviors),
        list_view(temps),
        list_view(fans),
    ];

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

fn list_view(elements: Vec<Element<AppMsg>>) -> Element<AppMsg> {
    Column::with_children(elements)
        .spacing(20)
        .padding(25)
        .into()
}

fn item_view<'a>(node: &'a Node, mut content: Vec<Element<'a, AppMsg>>) -> Element<'a, AppMsg> {
    let mut name =
        TextInput::new("name", &node.name_cached).on_input(|str| AppMsg::Rename(node.id, str));

    if node.is_error_name {
        name = name.style(iced::theme::TextInput::Custom(Box::new(
            CustomTextInputStyle::Error,
        )));
    }

    content.insert(0, name.into());

    let column = Column::with_children(content).spacing(5);

    Container::new(column)
        .width(Length::Fixed(200.0))
        .padding(Padding::new(10.0))
        .style(iced::theme::Container::Custom(Box::new(
            CustomContainerStyle::Item,
        )))
        .into()
}

fn control_view<'a>(
    node: &'a Node,
    nodes: &'a Nodes,
    hardware: &'a Hardware,
) -> Element<'a, AppMsg> {
    let NodeType::Control(control) = &node.node_type else {
        panic!()
    };

    let content = vec![
        pick_hardware(node, &hardware.controls, true),
        pick_input(
            node,
            nodes,
            &control.input,
            true,
            Box::new(AppMsg::ReplaceInput),
        ),
        Row::new()
            .push(Text::new(format!("{} %", node.value.unwrap_or(0))))
            .push(Toggler::new(None, !control.auto, |is_active| {
                AppMsg::ChangeControlAuto(node.id, !is_active)
            }))
            // todo: need space_between here
            .align_items(Alignment::End)
            .width(Length::Fill)
            .into(),
    ];

    item_view(node, content)
}

fn temp_view<'a>(node: &'a Node, hardware: &'a Hardware) -> Element<'a, AppMsg> {
    let content = vec![
        pick_hardware(node, &hardware.temps, false),
        Text::new(format!("{} °C", node.value.unwrap_or(0))).into(),
    ];

    item_view(node, content)
}

fn fan_view<'a>(node: &'a Node, hardware: &'a Hardware) -> Element<'a, AppMsg> {
    let content = vec![
        pick_hardware(node, &hardware.fans, false),
        Text::new(format!("{} RPM", node.value.unwrap_or(0))).into(),
    ];

    item_view(node, content)
}

fn custom_temp_view<'a>(node: &'a Node, nodes: &'a Nodes) -> Element<'a, AppMsg> {
    let NodeType::CustomTemp(custom_temp) = &node.node_type else {
        panic!()
    };

    let inputs = node
        .inputs
        .iter()
        .map(|i| {
            Row::new()
                .push(Text::new(i.1.clone()))
                // todo: icon
                .push(
                    Button::new(Text::new("x"))
                        .on_press(AppMsg::RemoveInput(node.id, Pick::new(&i.1, &i.0))),
                )
                .into()
        })
        .collect();

    let kind_options = CustomTempKind::VALUES
        .iter()
        .filter(|k| &custom_temp.kind != *k)
        .cloned()
        .collect::<Vec<_>>();

    let pick_kind = PickList::new(kind_options, Some(custom_temp.kind.clone()), |k| {
        AppMsg::ChangeCustomTempKind(node.id, k)
    })
    .into();
    let content = vec![
        pick_kind,
        pick_input(
            node,
            nodes,
            &Some("Choose Temp".into()),
            false,
            Box::new(AppMsg::AddInput),
        ),
        Column::with_children(inputs).into(),
        Text::new(format!("{} °C", node.value.unwrap_or(0))).into(),
    ];

    item_view(node, content)
}

fn flat_view(node: &Node) -> Element<AppMsg> {
    let NodeType::Flat(flat) = &node.node_type else {
        panic!()
    };

    let mut sub_button = Button::new("-");
    if flat.value > 0 {
        sub_button = sub_button.on_press(AppMsg::ChangeFlatValue(node.id, flat.value - 1));
    }

    let mut add_button = Button::new("+");
    if flat.value < 100 {
        add_button = add_button.on_press(AppMsg::ChangeFlatValue(node.id, flat.value + 1));
    }

    let buttons = Row::new()
        .push(Text::new("fan speed"))
        .push(Row::new().push(sub_button).push(add_button))
        .into();

    let slider = Row::new()
        .push(Slider::new(0..=100, flat.value, |v| {
            AppMsg::ChangeFlatValue(node.id, v)
        }))
        .push(Text::new(format!("{} %", flat.value)))
        .into();

    let content = vec![buttons, slider];

    item_view(node, content)
}

#[derive(Debug, Clone)]
pub enum LinearMsg {
    MinTemp(u8, String),
    MinSpeed(u8, String),
    MaxTemp(u8, String),
    MaxSpeed(u8, String),
}

fn linear_view<'a>(node: &'a Node, nodes: &'a Nodes) -> Element<'a, AppMsg> {
    let NodeType::Linear(linear, linear_cache) = &node.node_type else {
        panic!()
    };

    let content = vec![
        pick_input(
            node,
            nodes,
            &linear.input,
            true,
            Box::new(AppMsg::ReplaceInput),
        ),
        Text::new(format!("{} %", node.value.unwrap_or(0))).into(),
        input_line(
            "min temp",
            &linear.min_temp,
            &linear_cache.min_temp,
            "°C",
            &(0..=255),
            |val, cached_val| AppMsg::ChangeLinear(node.id, LinearMsg::MinTemp(val, cached_val)),
        ),
        input_line(
            "min speed",
            &linear.min_speed,
            &linear_cache.min_speed,
            "%",
            &(0..=100),
            |val, cached_val| AppMsg::ChangeLinear(node.id, LinearMsg::MinSpeed(val, cached_val)),
        ),
        input_line(
            "max temp",
            &linear.max_temp,
            &linear_cache.max_temp,
            "°C",
            &(0..=255),
            |val, cached_val| AppMsg::ChangeLinear(node.id, LinearMsg::MaxTemp(val, cached_val)),
        ),
        input_line(
            "max speed",
            &linear.max_speed,
            &linear_cache.max_speed,
            "%",
            &(0..=100),
            |val, cached_val| AppMsg::ChangeLinear(node.id, LinearMsg::MaxSpeed(val, cached_val)),
        ),
    ];

    item_view(node, content)
}

#[derive(Debug, Clone)]
pub enum TargetMsg {
    IdleTemp(u8, String),
    IdleSpeed(u8, String),
    LoadTemp(u8, String),
    LoadSpeed(u8, String),
}

fn target_view<'a>(node: &'a Node, nodes: &'a Nodes) -> Element<'a, AppMsg> {
    let NodeType::Target(target, target_cache) = &node.node_type else {
        panic!()
    };

    let content = vec![
        pick_input(
            node,
            nodes,
            &target.input,
            true,
            Box::new(AppMsg::ReplaceInput),
        ),
        Text::new(format!("{} %", node.value.unwrap_or(0))).into(),
        input_line(
            "idle temp",
            &target.idle_temp,
            &target_cache.idle_temp,
            "°C",
            &(0..=255),
            |val, cached_val| AppMsg::ChangeTarget(node.id, TargetMsg::IdleTemp(val, cached_val)),
        ),
        input_line(
            "idle speed",
            &target.idle_speed,
            &target_cache.idle_speed,
            "%",
            &(0..=100),
            |val, cached_val| AppMsg::ChangeTarget(node.id, TargetMsg::IdleSpeed(val, cached_val)),
        ),
        input_line(
            "load temp",
            &target.load_temp,
            &target_cache.load_temp,
            "°C",
            &(0..=255),
            |val, cached_val| AppMsg::ChangeTarget(node.id, TargetMsg::LoadTemp(val, cached_val)),
        ),
        input_line(
            "load speed",
            &target.load_speed,
            &target_cache.load_speed,
            "%",
            &(0..=100),
            |val, cached_val| AppMsg::ChangeTarget(node.id, TargetMsg::LoadSpeed(val, cached_val)),
        ),
    ];

    item_view(node, content)
}
