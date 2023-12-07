use iced::{
    widget::{
        scrollable::{Direction, Properties},
        Column, Container, Row, Scrollable, Slider, Space, Text, TextInput, Toggler,
    },
    Alignment, Element, Length, Padding,
};

use data::{
    app_graph::Nodes,
    config::custom_temp::CustomTempKind,
    node::{Node, NodeType, NodeTypeLight, ValueKind},
};
use hardware::Hardware;

use crate::{
    input_line::{input_line, InputLineUnit},
    my_widgets::drop_down,
    pick::{pick_hardware, pick_input, Pick},
    utils::{expand_icon, icon_button, icon_path_for_node_type, my_icon},
    AppMsg, ModifNodeMsg,
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

    let container = Container::new(content);

    Scrollable::new(container)
        .direction(Direction::Both {
            vertical: Properties::default(),
            horizontal: Properties::default(),
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn list_view(elements: Vec<Element<AppMsg>>) -> Element<AppMsg> {
    Column::with_children(elements)
        .spacing(20)
        .padding(25)
        .into()
}

fn item_view<'a>(node: &'a Node, bottom: impl Into<Element<'a, AppMsg>>) -> Element<'a, AppMsg> {
    let item_icon = my_icon(icon_path_for_node_type(&node.node_type.to_light()));

    let name = TextInput::new("name", &node.name_cached)
        .on_input(|s| ModifNodeMsg::Rename(s).to_app(node.id))
        .width(Length::Fill);

    if node.is_error_name {
        // todo
        //name = name.error("this name is already beeing use");
    }

    // todo: context menu
    let delete_button =
        icon_button("select/delete_forever24").on_press(AppMsg::DeleteNode(node.id));

    let top = Row::new()
        .push(item_icon)
        .push(name)
        //.push(Space::new(Length::Fill, 0.0))
        .push(delete_button)
        .align_items(Alignment::Center)
        .width(Length::Fill);

    let content = Column::new().push(top).push(bottom).spacing(5);

    Container::new(content)
        .width(Length::Fixed(200.0))
        .padding(Padding::new(10.0))
        .into()
}

#[derive(Debug, Clone)]
pub enum ControlMsg {
    Active(bool),
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
        pick_hardware(node, &hardware.controls, true).map(|m| m.to_app(node.id)),
        pick_input(node, nodes, &control.input, true, |pick| {
            ModifNodeMsg::ReplaceInput(pick).to_app(node.id)
        }),
        Row::new()
            .push(Text::new(node.value_text(&ValueKind::Porcentage)))
            .push(Space::new(Length::Fill, Length::Fixed(0.0)))
            .push(Toggler::new(None, control.active, |is_active| {
                ModifNodeMsg::Control(ControlMsg::Active(is_active)).to_app(node.id)
            }))
            .align_items(Alignment::Center)
            .width(Length::Fill)
            .into(),
    ];

    item_view(node, Column::with_children(content))
}

fn temp_view<'a>(node: &'a Node, hardware: &'a Hardware) -> Element<'a, AppMsg> {
    let content = vec![
        pick_hardware(node, &hardware.temps, false).map(|m| m.to_app(node.id)),
        Text::new(node.value_text(&ValueKind::Celsius)).into(),
    ];

    item_view(node, Column::with_children(content))
}

fn fan_view<'a>(node: &'a Node, hardware: &'a Hardware) -> Element<'a, AppMsg> {
    let content = vec![
        pick_hardware(node, &hardware.fans, false).map(|m| m.to_app(node.id)),
        Text::new(node.value_text(&ValueKind::RPM)).into(),
    ];

    item_view(node, Column::with_children(content))
}

#[derive(Debug, Clone)]
pub enum CustomTempMsg {
    Kind(CustomTempKind),
}

fn custom_temp_view<'a>(node: &'a Node, nodes: &'a Nodes) -> Element<'a, AppMsg> {
    let _custom_temp = node.to_custom_temp();
    let NodeType::CustomTemp(custom_temp) = &node.node_type else {
        panic!()
    };

    let inputs = node
        .inputs
        .iter()
        .map(|i| {
            Row::new()
                .push(Text::new(i.1.clone()))
                .push(Space::new(Length::Fill, Length::Fixed(0.0)))
                .push(
                    icon_button("select/close/close20")
                        .on_press(ModifNodeMsg::RemoveInput(Pick::new(&i.1, &i.0)).to_app(node.id)),
                )
                .align_items(Alignment::Center)
                .into()
        })
        .collect();

    /*
    let kind_options = CustomTempKind::VALUES
        .iter()
        .filter(|k| &custom_temp.kind != *k)
        .cloned()
        .collect::<Vec<_>>();

    let pick_kind = PickList::new(kind_options, Some(custom_temp.kind.clone()), |k| {
        ModifNodeMsg::CustomTemp(CustomTempMsg::Kind(k)).to_app(node.id)
    })
    .width(Length::Fill)
    .into();
     */

    let underlay = Row::new()
        .push(Text::new(custom_temp.kind.to_string()))
        .push(Space::new(Length::Fill, 0.0))
        .push(expand_icon(custom_temp.kind_expanded).on_press(AppMsg::Ui(
            crate::UiMsg::ToggleCustomTempKind(node.id, !custom_temp.kind_expanded),
        )))
        .align_items(Alignment::Center);

    /*
    let list = CustomTempKind::VALUES
    .iter()
    .filter(|k| &custom_temp.kind != *k)
    .map(|value| {
        Button::new(Text::new(value.to_string()))
            .on_press(
                ModifNodeMsg::CustomTemp(CustomTempMsg::Kind(value.clone()))
                    .to_app(node.id),
            )
            .into()
    })
    .collect();

    let overlay = Column::with_children(list);
    */
    let overlay = Text::new("hello la miff");

    let pick_kind = drop_down::DropDown::new(underlay, overlay)
        .expanded(custom_temp.kind_expanded)
        .on_dismiss(Some(AppMsg::Ui(crate::UiMsg::ToggleCustomTempKind(
            node.id, false,
        ))))
        .into();

    let content = vec![
        pick_kind,
        pick_input(node, nodes, &Some("Choose Temp".into()), false, |pick| {
            ModifNodeMsg::AddInput(pick).to_app(node.id)
        }),
        Column::with_children(inputs).into(),
        Text::new(node.value_text(&ValueKind::Celsius)).into(),
    ];

    item_view(node, Column::with_children(content))
}

#[derive(Debug, Clone)]
pub enum FlatMsg {
    Value(u16),
}

fn flat_view(node: &Node) -> Element<AppMsg> {
    let NodeType::Flat(flat) = &node.node_type else {
        panic!()
    };

    let mut sub_button = icon_button("sign/minus/remove24");
    if flat.value > 0 {
        sub_button =
            sub_button.on_press(ModifNodeMsg::Flat(FlatMsg::Value(flat.value - 1)).to_app(node.id));
    }

    let mut add_button = icon_button("sign/plus/add24");
    if flat.value < 100 {
        add_button =
            add_button.on_press(ModifNodeMsg::Flat(FlatMsg::Value(flat.value + 1)).to_app(node.id));
    }

    let buttons = Row::new()
        .push(sub_button)
        .push(add_button)
        .align_items(Alignment::Center);

    let buttons = Row::new()
        .push(Text::new(node.value_text(&ValueKind::Porcentage)))
        .push(Space::new(Length::Fill, Length::Fixed(0.0)))
        .push(buttons)
        .align_items(Alignment::Center)
        .into();

    let slider = Slider::new(0..=100, flat.value, |v| {
        ModifNodeMsg::Flat(FlatMsg::Value(v)).to_app(node.id)
    })
    .into();

    let content = vec![buttons, slider];

    item_view(node, Column::with_children(content))
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
        pick_input(node, nodes, &linear.input, true, |pick| {
            ModifNodeMsg::ReplaceInput(pick).to_app(node.id)
        }),
        Text::new(node.value_text(&ValueKind::Porcentage)).into(),
        input_line(
            "min temp",
            &linear.min_temp,
            &linear_cache.min_temp,
            InputLineUnit::Celcius,
            &(0..=255),
            |val, cached_val| ModifNodeMsg::Linear(LinearMsg::MinTemp(val, cached_val)),
        )
        .map(|m| m.to_app(node.id)),
        input_line(
            "min speed",
            &linear.min_speed,
            &linear_cache.min_speed,
            InputLineUnit::Porcentage,
            &(0..=100),
            |val, cached_val| ModifNodeMsg::Linear(LinearMsg::MinSpeed(val, cached_val)),
        )
        .map(|m| m.to_app(node.id)),
        input_line(
            "max temp",
            &linear.max_temp,
            &linear_cache.max_temp,
            InputLineUnit::Celcius,
            &(0..=255),
            |val, cached_val| ModifNodeMsg::Linear(LinearMsg::MaxTemp(val, cached_val)),
        )
        .map(|m| m.to_app(node.id)),
        input_line(
            "max speed",
            &linear.max_speed,
            &linear_cache.max_speed,
            InputLineUnit::Porcentage,
            &(0..=100),
            |val, cached_val| ModifNodeMsg::Linear(LinearMsg::MaxSpeed(val, cached_val)),
        )
        .map(|m| m.to_app(node.id)),
    ];

    item_view(node, Column::with_children(content))
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
        pick_input(node, nodes, &target.input, true, |pick| {
            ModifNodeMsg::ReplaceInput(pick).to_app(node.id)
        }),
        Text::new(node.value_text(&ValueKind::Porcentage)).into(),
        input_line(
            "idle temp",
            &target.idle_temp,
            &target_cache.idle_temp,
            InputLineUnit::Celcius,
            &(0..=255),
            |val, cached_val| ModifNodeMsg::Target(TargetMsg::IdleTemp(val, cached_val)),
        )
        .map(|m| m.to_app(node.id)),
        input_line(
            "idle speed",
            &target.idle_speed,
            &target_cache.idle_speed,
            InputLineUnit::Porcentage,
            &(0..=100),
            |val, cached_val| ModifNodeMsg::Target(TargetMsg::IdleSpeed(val, cached_val)),
        )
        .map(|m| m.to_app(node.id)),
        input_line(
            "load temp",
            &target.load_temp,
            &target_cache.load_temp,
            InputLineUnit::Celcius,
            &(0..=255),
            |val, cached_val| ModifNodeMsg::Target(TargetMsg::LoadTemp(val, cached_val)),
        )
        .map(|m| m.to_app(node.id)),
        input_line(
            "load speed",
            &target.load_speed,
            &target_cache.load_speed,
            InputLineUnit::Porcentage,
            &(0..=100),
            |val, cached_val| ModifNodeMsg::Target(TargetMsg::LoadSpeed(val, cached_val)),
        )
        .map(|m| m.to_app(node.id)),
    ];

    item_view(node, Column::with_children(content))
}
