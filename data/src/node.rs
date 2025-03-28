use std::cmp::Ordering;
use std::vec;

use derive_more::{Display, Unwrap};
use hardware::{Hardware, Value};
use light_enum::LightEnum;
use std::fmt::Display;

use crate::app_graph::{AppGraph, Nodes};

use crate::config::{
    control::Control, custom_temp::CustomTemp, fan::Fan, flat::Flat, graph::Graph, linear::Linear,
    target::Target, temp::Temp,
};

use crate::id::Id;

#[derive(Debug, Clone, LightEnum, Unwrap)]
#[unwrap(ref, ref_mut)]
pub enum NodeType {
    Control(Control),
    Fan(Fan),
    Temp(Temp),
    CustomTemp(CustomTemp),
    Graph(Graph),
    Flat(Flat),
    Linear(Linear),
    Target(Target),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    pub id: Id,
    pub name: String,
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: Id,
    pub node_type: NodeType,
    pub inputs: Vec<Input>,
    pub value: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum NbInput {
    Zero,
    One,
    Infinity,
}

#[derive(Debug, Clone)]
pub struct Sanitize {
    pub id: Id,
    node: Vec<Input>,
    item: Vec<String>,
}

impl Sanitize {
    fn new(id: Id) -> Self {
        Sanitize {
            id,
            item: Default::default(),
            node: Default::default(),
        }
    }

    fn add(&mut self, id: Id, name: &str) {
        self.item.push(name.to_owned());
        let input = Input {
            id,
            name: name.to_owned(),
        };
        self.node.push(input);
    }
}

pub fn sanitize_inputs(node: &Node, nodes: &Nodes, log: bool) -> Sanitize {
    let mut sanitize = Sanitize::new(node.id);

    match node.node_type.max_input() {
        NbInput::Zero => {
            return sanitize;
        }
        NbInput::One => {
            if node.inputs.len() > 1 || node.node_type.get_inputs().len() > 1 {
                // todo: remove this debug print
                error!(
                    "sanitize_inputs {}: {:?} number of dep != {}",
                    node.name(),
                    node.node_type.to_light(),
                    node.node_type.max_input()
                );
                return sanitize;
            }
        }
        NbInput::Infinity => {}
    };

    for name in node.node_type.get_inputs() {
        match nodes.values().find(|n| n.name() == &name) {
            Some(n) => {
                match node
                    .node_type
                    .allowed_dep()
                    .contains(&n.node_type.to_light())
                {
                    true => {
                        sanitize.add(n.id, &name);
                    }
                    false => {
                        warn!(
                            "sanitize_inputs {}: dep {} have an unauthorized node type: {:?}",
                            node.name(),
                            name,
                            n.node_type.to_light(),
                        );
                    }
                }
            }
            None => {
                if log {
                    warn!("sanitize_inputs {}: can't find node {}", node.name(), name);
                }
            }
        }
    }
    sanitize
}

pub fn validate_name(nodes: &Nodes, id: &Id, name: &String) -> bool {
    if name.trim().is_empty() {
        return false;
    };

    !nodes.values().any(|n| n.name() == name && &n.id != id)
}

pub trait ToNode {
    fn to_node(self, app_graph: &mut AppGraph, hardware: &Hardware) -> Node;
}

pub trait IsValid {
    fn is_valid(&self) -> bool;
}

impl Node {
    pub fn new(node_type: NodeType, app_graph: &mut AppGraph) -> Self {
        let mut node = Self {
            id: app_graph.id_generator.new_id(),
            node_type,
            inputs: Vec::new(),
            value: None,
        };

        if app_graph.is_name_taken(node.name()) {
            warn!("Name {} is already taken", node.name());
            node.node_type
                .set_name(app_graph.generate_new_name(node.name()));
        }

        let sanitize = self::sanitize_inputs(&node, &app_graph.nodes, true);
        node.set_inputs(sanitize);
        node
    }

    pub fn name(&self) -> &String {
        self.node_type.name()
    }

    pub fn set_inputs(&mut self, sanitize: Sanitize) {
        self.inputs = sanitize.node;
        self.node_type.set_inputs(sanitize.item);
    }

    pub fn hardware_id(&self) -> &Option<String> {
        match &self.node_type {
            NodeType::Control(i) => &i.hardware_id,
            NodeType::Fan(i) => &i.hardware_id,
            NodeType::Temp(i) => &i.hardware_id,
            _ => panic!(),
        }
    }

    pub fn value_text(&self, kind: &ValueKind) -> String {
        match self.value {
            Some(val) => match kind {
                ValueKind::Celsius => fl!("value_celsius", value = val),
                ValueKind::Porcentage => fl!("value_percentage", value = val),
                ValueKind::RPM => fl!("value_rpm", value = val),
            },
            None => fl!("no_value"),
        }
    }

    pub fn is_root(&self) -> bool {
        self.node_type.is_root()
    }
}

pub enum ValueKind {
    Celsius,
    Porcentage,
    RPM,
}

impl NodeType {
    pub fn name(&self) -> &String {
        match self {
            NodeType::Control(control) => &control.name,
            NodeType::Fan(fan) => &fan.name,
            NodeType::Temp(temp) => &temp.name,
            NodeType::CustomTemp(custom_temp) => &custom_temp.name,
            NodeType::Graph(graph) => &graph.name,
            NodeType::Flat(flat) => &flat.name,
            NodeType::Linear(linear, ..) => &linear.name,
            NodeType::Target(target, ..) => &target.name,
        }
    }
    pub fn set_name(&mut self, name: String) {
        match self {
            NodeType::Control(i) => i.name = name,
            NodeType::Fan(i) => i.name = name,
            NodeType::Temp(i) => i.name = name,
            NodeType::CustomTemp(i) => i.name = name,
            NodeType::Graph(i) => i.name = name,
            NodeType::Flat(i) => i.name = name,
            NodeType::Linear(i, ..) => i.name = name,
            NodeType::Target(i, ..) => i.name = name,
        }
    }

    pub fn is_valid(&self) -> bool {
        match self {
            NodeType::Control(control) => control.is_valid(),
            NodeType::Fan(fan) => fan.is_valid(),
            NodeType::Temp(temp) => temp.is_valid(),
            NodeType::CustomTemp(custom_temp) => custom_temp.is_valid(),
            NodeType::Graph(graph) => graph.is_valid(),
            NodeType::Flat(flat) => flat.is_valid(),
            NodeType::Linear(linear, ..) => linear.is_valid(),
            NodeType::Target(target, ..) => target.is_valid(),
        }
    }

    pub fn get_inputs(&self) -> Vec<String> {
        match self {
            NodeType::Control(i) => i.input.clone().map_or(Vec::new(), |i| vec![i]),
            NodeType::Fan(_) => Vec::new(),
            NodeType::Temp(_) => Vec::new(),
            NodeType::CustomTemp(i) => i.inputs.clone(),
            NodeType::Graph(i) => i.input.clone().map_or(Vec::new(), |i| vec![i]),
            NodeType::Flat(_) => Vec::new(),
            NodeType::Linear(i, ..) => i.input.clone().map_or(Vec::new(), |i| vec![i]),
            NodeType::Target(i, ..) => i.input.clone().map_or(Vec::new(), |i| vec![i]),
        }
    }

    pub fn set_inputs(&mut self, inputs: Vec<String>) {
        match self {
            NodeType::Control(i) => match inputs.first() {
                Some(input) => {
                    let _ = i.input.insert(input.clone());
                }
                None => {
                    i.input.take();
                }
            },
            NodeType::CustomTemp(i) => {
                i.inputs = inputs;
            }
            NodeType::Graph(i) => match inputs.first() {
                Some(input) => {
                    let _ = i.input.insert(input.clone());
                }
                None => {
                    i.input.take();
                }
            },
            NodeType::Linear(i, ..) => match inputs.first() {
                Some(input) => {
                    let _ = i.input.insert(input.clone());
                }
                None => {
                    i.input.take();
                }
            },
            NodeType::Target(i, ..) => match inputs.first() {
                Some(input) => {
                    let _ = i.input.insert(input.clone());
                }
                None => {
                    i.input.take();
                }
            },
            NodeType::Fan(_) => {}
            NodeType::Temp(_) => {}
            NodeType::Flat(_) => {}
        };
    }

    pub fn allowed_dep(&self) -> &'static [NodeTypeLight] {
        match self {
            NodeType::Control(..) => &[
                NodeTypeLight::Flat,
                NodeTypeLight::Graph,
                NodeTypeLight::Target,
                NodeTypeLight::Linear,
            ],
            NodeType::Fan(..) => &[],
            NodeType::Temp(..) => &[],
            NodeType::CustomTemp(..) => &[NodeTypeLight::Temp],
            NodeType::Graph(..) => &[NodeTypeLight::Temp, NodeTypeLight::CustomTemp],
            NodeType::Flat(..) => &[],
            NodeType::Linear(..) => &[NodeTypeLight::Temp, NodeTypeLight::CustomTemp],
            NodeType::Target(..) => &[NodeTypeLight::Temp, NodeTypeLight::CustomTemp],
        }
    }

    pub fn max_input(&self) -> NbInput {
        match self {
            NodeType::Control(..) => NbInput::One,
            NodeType::Fan(..) => NbInput::Zero,
            NodeType::Temp(..) => NbInput::Zero,
            NodeType::CustomTemp(..) => NbInput::Infinity,
            NodeType::Graph(..) => NbInput::One,
            NodeType::Flat(..) => NbInput::Zero,
            NodeType::Linear(..) => NbInput::One,
            NodeType::Target(..) => NbInput::One,
        }
    }

    pub fn is_sensor(&self) -> bool {
        matches!(self, NodeType::Fan(..) | NodeType::Temp(..))
    }

    pub fn is_behavior(&self) -> bool {
        matches!(
            self,
            NodeType::Graph(..) | NodeType::Flat(..) | NodeType::Linear(..) | NodeType::Target(..)
        )
    }

    pub fn is_root(&self) -> bool {
        matches!(self, NodeType::Control(..))
    }

    pub fn compare_update_priority(&self, other: &Self) -> Ordering {
        match self {
            NodeType::Control(_) => match other {
                NodeType::Control(_) => Ordering::Equal,
                _ => Ordering::Greater,
            },
            NodeType::Fan(_) => {
                if other.is_sensor() {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            }
            NodeType::Temp(_) => {
                if other.is_sensor() {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            }
            NodeType::CustomTemp(_) => match other {
                NodeType::CustomTemp(_) => Ordering::Equal,
                NodeType::Fan(_) => Ordering::Greater,
                NodeType::Temp(_) => Ordering::Greater,
                _ => Ordering::Less,
            },
            NodeType::Flat(_) => Ordering::Equal,

            NodeType::Graph(_) | NodeType::Linear(..) | NodeType::Target(..) => match other {
                NodeType::Control(_) => Ordering::Less,
                NodeType::Fan(_) => Ordering::Greater,
                NodeType::Temp(_) => Ordering::Greater,
                NodeType::CustomTemp(_) => Ordering::Greater,
                _ => Ordering::Equal,
            },
        }
    }
}
