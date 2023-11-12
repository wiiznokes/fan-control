use std::ops::{Add, RangeInclusive, Sub};

use data::{
    config::custom_temp::CustomTempKind,
    node::{Node, NodeType, Nodes},
};
use hardware::Hardware;
use iced::{
    widget::{Button, Column, Container, PickList, Row, Slider, Text, TextInput, Toggler},
    Alignment, Element, Length, Padding,
};

use crate::{
    pick::{pick_hardware, pick_input, Pick},
    theme::{CustomContainerStyle, CustomTextInputStyle},
    AppMsg,
};

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

pub fn temp_view<'a>(node: &'a Node, hardware: &'a Hardware) -> Element<'a, AppMsg> {
    let content = vec![
        pick_hardware(node, &hardware.temps, false),
        Text::new(format!("{} °C", node.value.unwrap_or(0))).into(),
    ];

    item_view(node, content)
}

pub fn fan_view<'a>(node: &'a Node, hardware: &'a Hardware) -> Element<'a, AppMsg> {
    let content = vec![
        pick_hardware(node, &hardware.fans, false),
        Text::new(format!("{} RPM", node.value.unwrap_or(0))).into(),
    ];

    item_view(node, content)
}

pub fn custom_temp_view<'a>(node: &'a Node, nodes: &'a Nodes) -> Element<'a, AppMsg> {
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

pub fn flat_view(node: &Node) -> Element<AppMsg> {
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
    MinSpeed(u8),
    MaxTemp(u8),
    MaxSpeed(u8),
}

pub fn linear_view<'a>(node: &'a Node, nodes: &'a Nodes) -> Element<'a, AppMsg> {
    let NodeType::Linear(linear) = &node.node_type else {
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
            &linear.min_temp_cached,
            "°C",
            &(0..=255),
            &|val, cached_val| AppMsg::ChangeLinear(node.id, LinearMsg::MinTemp(val, cached_val)),
        ),
    ];

    item_view(node, content)
}



trait MyFrom<T> {
    fn from(value: T) -> Self;
}

impl MyFrom<i32> for u8 {
    fn from(value: i32) -> Self {
        value as u8
    }
}

impl MyFrom<String> for Option<u8> {
    fn from(value: String) -> Self {
        match value.parse::<u8>() {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
}


fn input_line<'a, V, F>(
    info: &'a str,
    value: &'a V,
    cached_value: &str,
    unit: &'a str,
    range: &'a RangeInclusive<V>,
    map_value: &'a F,
) -> Element<'a, AppMsg>
where
    V: Add<V, Output = V>,
    V: Sub<V, Output = V>,
    V: MyFrom<i32>,
    V: PartialOrd + Clone,
    Option<V>: MyFrom<String>,
    F: Fn(V, String) -> AppMsg + 'a,
{
    let input = TextInput::new("value", cached_value).on_input(|s| {
        let final_value = if let Some(final_value) = <Option<V> as MyFrom<_>>::from(s.clone()) {
            if range.contains(&final_value) {
                value.clone()
            } else {
                final_value
            }
        } else {
            value.clone()
        };

        map_value(final_value, s.clone())
    });

    let one: V = MyFrom::from(1);

    let plus_message = if range.contains(&(value.clone() + one.clone())) {
        Some(map_value(value.clone() + one.clone(), info.into()))
    } else {
        None
    };

    let sub_message = if range.contains(&(value.clone() - one.clone())) {
        Some(map_value(value.clone() - one.clone(), info.into()))
    } else {
        None
    };

    let plus_button = Button::new("+").on_press_maybe(plus_message);

    let sub_button = Button::new("-").on_press_maybe(sub_message);

    Row::new()
        .push(Text::new(info))
        .push(
            Row::new()
                .push(Text::new(":"))
                .push(input)
                .push(Text::new(unit))
                .push(Row::new().push(plus_button).push(sub_button)),
        )
        .into()
}
