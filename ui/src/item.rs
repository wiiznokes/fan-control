use std::rc::Rc;

use cosmic::{
    iced_core::{Alignment, Length, Padding},
    iced_widget::{
        scrollable::{Direction, Properties},
        Button, PickList, Scrollable, Toggler,
    },
    style, theme,
    widget::{Column, Container, Row, Slider, Space, Text, TextInput},
    Element,
};
use data::{
    app_graph::Nodes,
    config::{
        control::Control,
        custom_temp::{CustomTemp, CustomTempKind},
        flat::Flat,
        linear::Linear,
        target::Target,
    },
    node::{Input, Node, NodeTypeLight, ValueKind},
};
use hardware::{HItem, Hardware};

use crate::{
    graph::graph_view,
    icon::{icon_button, icon_path_for_node_type, my_icon},
    input_line::{input_line, InputLineUnit},
    message::{
        AppMsg, ControlMsg, CustomTempMsg, FlatMsg, LinearMsg, ModifNodeMsg, TargetMsg,
        ToogleMsg,
    },
    my_widgets::{self, drop_down::DropDown, offset::Offset},
    node_cache::{LinearC, NodeC, NodesC, TargetC},
    pick_list_utils::{self, MyOption},
};

pub fn items_view<'a>(
    nodes: &'a Nodes,
    nodes_c: &'a NodesC,
    hardware: &'a Hardware,
) -> Element<'a, AppMsg> {
    let mut controls = Vec::new();
    let mut behaviors = Vec::new();
    let mut custom_temps = Vec::new();
    let mut temps = Vec::new();
    let mut fans = Vec::new();

    for node in nodes.values() {
        let node_c = nodes_c.get(&node.id);
        let content = item_view(node, node_c, nodes, hardware);

        match node.node_type.to_light() {
            NodeTypeLight::Control => controls.push(content),
            NodeTypeLight::Fan => fans.push(content),
            NodeTypeLight::Temp => temps.push(content),
            NodeTypeLight::Graph
            | NodeTypeLight::Flat
            | NodeTypeLight::Linear
            | NodeTypeLight::Target => behaviors.push(content),
            NodeTypeLight::CustomTemp => custom_temps.push(content),
        }
    }

    let mut list_views = Vec::new();

    if !controls.is_empty() {
        list_views.push(list_view(controls))
    }
    if !behaviors.is_empty() {
        list_views.push(list_view(behaviors))
    }
    if !custom_temps.is_empty() {
        list_views.push(list_view(custom_temps))
    }
    if !temps.is_empty() {
        list_views.push(list_view(temps))
    }
    if !fans.is_empty() {
        list_views.push(list_view(fans))
    }

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

fn item_view<'a>(
    node: &'a Node,
    node_c: &'a NodeC,
    nodes: &'a Nodes,
    hardware: &'a Hardware,
) -> Element<'a, AppMsg> {
    let item_icon = my_icon(icon_path_for_node_type(&node.node_type.to_light()));

    let mut name = TextInput::new(fl!("name"), &node_c.name)
        .on_input(|s| AppMsg::Rename(node.id, s))
        .width(Length::Fill);

    if node_c.is_error_name {
        name = name.error("This name is already being use");
    }

    fn action_line<'a>(action: String, message: AppMsg) -> Element<'a, AppMsg> {
        Button::new(Text::new(action))
            .on_press(message)
            .width(Length::Fill)
            .into()
    }

    let overlay = Container::new(Column::new().push(action_line(
        fl!("delete"),
        ModifNodeMsg::Delete.to_app(node.id),
    )))
    .style(theme::Container::Dropdown);

    let context_menu = DropDown::new(
        icon_button("more_vert/24")
            .on_press(ToogleMsg::NodeContextMenu(node.id, !node_c.context_menu_expanded).into()),
        overlay,
        node_c.context_menu_expanded,
    )
    .on_dismiss(ToogleMsg::NodeContextMenu(node.id, false).into())
    .width(130.0)
    .alignment(my_widgets::alignment::Alignment::BottomEnd)
    .offset(Offset::new(5.0, 0.0));

    let top = Row::new()
        .push(item_icon)
        .push(Space::new(5.0, 0.0))
        .push(name)
        .push(context_menu)
        .align_items(Alignment::Center);

    let node_specific_content = match &node.node_type {
        data::node::NodeType::Control(control) => control_view(node, control, nodes, hardware),
        data::node::NodeType::Fan(_fan) => fan_view(node, hardware),
        data::node::NodeType::Temp(_temp) => temp_view(node, hardware),
        data::node::NodeType::CustomTemp(custom_temp) => custom_temp_view(node, custom_temp, nodes),
        data::node::NodeType::Graph(graph) => {
            graph_view(node, graph, node_c.node_type_c.unwrap_graph_ref(), nodes)
        }
        data::node::NodeType::Flat(flat) => flat_view(node, flat),
        data::node::NodeType::Linear(linear) => {
            linear_view(node, linear, node_c.node_type_c.unwrap_linear_ref(), nodes)
        }
        data::node::NodeType::Target(target) => {
            target_view(node, target, node_c.node_type_c.unwrap_target_ref(), nodes)
        }
    };

    let content = Column::new()
        .push(top)
        .push(node_specific_content)
        .align_items(Alignment::Center)
        .spacing(5);

    Container::new(content)
        .width(Length::Fixed(200.0))
        .padding(Padding::new(10.0))
        .style(style::Container::Card)
        .into()
}

fn pick_hardware<'a, H: HItem>(
    node: &'a Node,
    hardwares: &'a [Rc<H>],
    one_ref: bool,
) -> Element<'a, AppMsg> {
    let hardware_id = node.hardware_id().clone();
    let (selected_hardware_info, input_hardware) =
        pick_list_utils::hardware::availlable_hardware(&hardware_id, hardwares, one_ref);

    PickList::new(input_hardware, Some(selected_hardware_info), |selected| {
        let message_content = match selected {
            MyOption::Some(selected) => Some(selected.id),
            MyOption::None => None,
        };

        ModifNodeMsg::ChangeHardware(message_content).to_app(node.id)
    })
    .width(Length::Fill)
    .into()
}

fn control_view<'a>(
    node: &'a Node,
    control: &'a Control,
    nodes: &'a Nodes,
    hardware: &'a Hardware,
) -> Element<'a, AppMsg> {
    let input_options =
        pick_list_utils::input::optional_availlable_inputs(nodes, node, control.input.is_some());
    let current_input: MyOption<Input> = control.input.clone().into();

    let pick_input = PickList::new(input_options, Some(current_input), |input| {
        ModifNodeMsg::ReplaceInput(input.into()).to_app(node.id)
    })
    .width(Length::Fill)
    .into();

    let content = vec![
        pick_hardware(node, &hardware.controls, true),
        pick_input,
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

    Column::with_children(content).into()
}

fn fan_view<'a>(node: &'a Node, hardware: &'a Hardware) -> Element<'a, AppMsg> {
    let content = vec![
        pick_hardware(node, &hardware.fans, false),
        Text::new(node.value_text(&ValueKind::RPM)).into(),
    ];

    Column::with_children(content).into()
}

fn temp_view<'a>(node: &'a Node, hardware: &'a Hardware) -> Element<'a, AppMsg> {
    let content = vec![
        pick_hardware(node, &hardware.temps, false),
        Text::new(node.value_text(&ValueKind::Celsius)).into(),
    ];

    Column::with_children(content).into()
}

fn custom_temp_view<'a>(
    node: &'a Node,
    custom_temp: &'a CustomTemp,
    nodes: &'a Nodes,
) -> Element<'a, AppMsg> {
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

    let inputs = node.inputs.iter().map(|input| {
        Row::new()
            .push(Text::new(input.name.clone()).width(Length::Fixed(100.0)))
            .push(Space::new(Length::Fill, Length::Fixed(0.0)))
            .push(
                icon_button("close/20")
                    .on_press(ModifNodeMsg::RemoveInput(input.clone()).to_app(node.id)),
            )
            .align_items(Alignment::Center)
            .into()
    });

    let input_options: Vec<Input> =
        pick_list_utils::input::availlable_inputs(nodes, node).collect();

    let current_input = Input {
        id: Default::default(),
        name: fl!("temp_selection"),
    };

    let pick_input = PickList::new(input_options, Some(current_input), |input| {
        ModifNodeMsg::AddInput(input).to_app(node.id)
    })
    .width(Length::Fill)
    .into();

    let content = vec![
        pick_kind,
        pick_input,
        Column::with_children(inputs).into(),
        Text::new(node.value_text(&ValueKind::Celsius)).into(),
    ];

    Column::with_children(content).into()
}

fn flat_view<'a>(node: &'a Node, flat: &'a Flat) -> Element<'a, AppMsg> {
    let mut sub_button = icon_button("remove/24");
    if flat.value > 0 {
        sub_button =
            sub_button.on_press(ModifNodeMsg::Flat(FlatMsg::Value(flat.value - 1)).to_app(node.id));
    }

    let mut add_button = icon_button("add/24");
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

    Column::with_children(content).into()
}

fn linear_view<'a>(
    node: &'a Node,
    linear: &'a Linear,
    linear_c: &'a LinearC,
    nodes: &'a Nodes,
) -> Element<'a, AppMsg> {
    let input_options =
        pick_list_utils::input::optional_availlable_inputs(nodes, node, linear.input.is_some());
    let current_input: MyOption<Input> = linear.input.clone().into();
    let pick_input = PickList::new(input_options, Some(current_input), |input| {
        ModifNodeMsg::ReplaceInput(input.into()).to_app(node.id)
    })
    .width(Length::Fill)
    .into();

    let content = vec![
        pick_input,
        Text::new(node.value_text(&ValueKind::Porcentage)).into(),
        input_line(
            fl!("min_temp"),
            &linear.min_temp,
            &linear_c.min_temp,
            InputLineUnit::Celcius,
            &(0..=255),
            |val, cached_val| ModifNodeMsg::Linear(LinearMsg::MinTemp(val, cached_val)),
        )
        .map(|m| m.to_app(node.id)),
        input_line(
            fl!("min_speed"),
            &linear.min_speed,
            &linear_c.min_speed,
            InputLineUnit::Porcentage,
            &(0..=100),
            |val, cached_val| ModifNodeMsg::Linear(LinearMsg::MinSpeed(val, cached_val)),
        )
        .map(|m| m.to_app(node.id)),
        input_line(
            fl!("max_temp"),
            &linear.max_temp,
            &linear_c.max_temp,
            InputLineUnit::Celcius,
            &(0..=255),
            |val, cached_val| ModifNodeMsg::Linear(LinearMsg::MaxTemp(val, cached_val)),
        )
        .map(|m| m.to_app(node.id)),
        input_line(
            fl!("max_speed"),
            &linear.max_speed,
            &linear_c.max_speed,
            InputLineUnit::Porcentage,
            &(0..=100),
            |val, cached_val| ModifNodeMsg::Linear(LinearMsg::MaxSpeed(val, cached_val)),
        )
        .map(|m| m.to_app(node.id)),
    ];

    Column::with_children(content).into()
}

fn target_view<'a>(
    node: &'a Node,
    target: &'a Target,
    target_c: &'a TargetC,
    nodes: &'a Nodes,
) -> Element<'a, AppMsg> {
    let input_options =
        pick_list_utils::input::optional_availlable_inputs(nodes, node, target.input.is_some());
    let current_input: MyOption<Input> = target.input.clone().into();
    let pick_input = PickList::new(input_options, Some(current_input), |input| {
        ModifNodeMsg::ReplaceInput(input.into()).to_app(node.id)
    })
    .width(Length::Fill)
    .into();

    let content = vec![
        pick_input,
        Text::new(node.value_text(&ValueKind::Porcentage)).into(),
        input_line(
            fl!("idle_temp"),
            &target.idle_temp,
            &target_c.idle_temp,
            InputLineUnit::Celcius,
            &(0..=255),
            |val, cached_val| ModifNodeMsg::Target(TargetMsg::IdleTemp(val, cached_val)),
        )
        .map(|m| m.to_app(node.id)),
        input_line(
            fl!("idle_speed"),
            &target.idle_speed,
            &target_c.idle_speed,
            InputLineUnit::Porcentage,
            &(0..=100),
            |val, cached_val| ModifNodeMsg::Target(TargetMsg::IdleSpeed(val, cached_val)),
        )
        .map(|m| m.to_app(node.id)),
        input_line(
            fl!("load_temp"),
            &target.load_temp,
            &target_c.load_temp,
            InputLineUnit::Celcius,
            &(0..=255),
            |val, cached_val| ModifNodeMsg::Target(TargetMsg::LoadTemp(val, cached_val)),
        )
        .map(|m| m.to_app(node.id)),
        input_line(
            fl!("load_speed"),
            &target.load_speed,
            &target_c.load_speed,
            InputLineUnit::Porcentage,
            &(0..=100),
            |val, cached_val| ModifNodeMsg::Target(TargetMsg::LoadSpeed(val, cached_val)),
        )
        .map(|m| m.to_app(node.id)),
    ];

    Column::with_children(content).into()
}
