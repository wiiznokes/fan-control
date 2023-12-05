use std::vec;

use hardware::{Hardware, Value};
use light_enum::LightEnum;

use crate::app_graph::Nodes;
use crate::config::linear::LinearCache;
use crate::config::target::TargetCache;

use crate::config::{
    control::Control, custom_temp::CustomTemp, fan::Fan, flat::Flat, graph::Graph, linear::Linear,
    target::Target, temp::Temp,
};

use crate::id::{Id, IdGenerator};

#[derive(Debug, Clone)]
pub struct Node {
    pub id: Id,
    pub node_type: NodeType,
    pub inputs: Vec<(Id, String)>,

    pub value: Option<Value>,

    pub name_cached: String,
    pub is_error_name: bool,
}

#[derive(Debug, Clone, LightEnum)]
pub enum NodeType {
    Control(Control),
    Fan(Fan),
    Temp(Temp),
    CustomTemp(CustomTemp),
    Graph(Graph),
    Flat(Flat),
    Linear(Linear, LinearCache),
    Target(Target, TargetCache),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NbInput {
    Zero,
    One,
    Infinity,
}

#[derive(Debug, Clone)]
pub struct Sanitize {
    pub id: Id,
    item: Vec<String>,
    node: Vec<(Id, String)>,
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
        self.node.push((id, name.to_owned()));
    }
}

pub enum ToRemove {
    All,
    Specific(Vec<String>),
}

pub fn sanitize_inputs(node: &Node, nodes: &Nodes) -> Sanitize {
    let mut sanitize = Sanitize::new(node.id);

    match node.node_type.max_input() {
        NbInput::Zero => {
            return sanitize;
        }
        NbInput::One => {
            if node.inputs.len() > 1 || node.node_type.get_inputs().len() > 1 {
                eprintln!(
                    "{:?}: number of dep allowed == {:?}",
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
                            "incompatible node type: {:?} on {}",
                            n.node_type.to_light(),
                            name
                        );
                    }
                }
            }
            None => {
                warn!("can't find node {}", name);
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
    fn to_node(self, id_generator: &mut IdGenerator, nodes: &Nodes, hardware: &Hardware) -> Node;
}

pub trait IsValid {
    fn is_valid(&self) -> bool;
}

impl Node {
    pub fn new(id_generator: &mut IdGenerator, node_type: NodeType, nodes: &Nodes) -> Self {
        let name_cached = node_type.name().clone();
        let mut node = Self {
            id: id_generator.new_id(),
            node_type,
            inputs: Vec::new(),
            value: None,
            name_cached,
            is_error_name: false,
        };

        let sanitize = sanitize_inputs(&node, nodes);
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
        match &self.value {
            Some(value) => match kind {
                ValueKind::Fahrenheit => todo!(),
                ValueKind::Celsius => format!("{} °C", value),
                ValueKind::Porcentage => format!("{} %", value),
                ValueKind::RPM => format!("{} RPM", value),
            },
            None => "No value".into(),
        }
    }

    pub fn is_root(&self) -> bool {
        matches!(self.node_type, NodeType::Control(..))
    }
}

pub enum ValueKind {
    Fahrenheit,
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
    pub fn set_name(&mut self, name: &str) {
        let name_cloned = name.to_string();
        match self {
            NodeType::Control(i) => i.name = name_cloned,
            NodeType::Fan(i) => i.name = name_cloned,
            NodeType::Temp(i) => i.name = name_cloned,
            NodeType::CustomTemp(i) => i.name = name_cloned,
            NodeType::Graph(i) => i.name = name_cloned,
            NodeType::Flat(i) => i.name = name_cloned,
            NodeType::Linear(i, ..) => i.name = name_cloned,
            NodeType::Target(i, ..) => i.name = name_cloned,
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
            NodeType::CustomTemp(i) => i.input.clone(),
            NodeType::Graph(i) => i.input.clone().map_or(Vec::new(), |i| vec![i]),
            NodeType::Flat(_) => Vec::new(),
            NodeType::Linear(i, ..) => i.input.clone().map_or(Vec::new(), |i| vec![i]),
            NodeType::Target(i, ..) => i.input.clone().map_or(Vec::new(), |i| vec![i]),
        }
    }

    pub fn set_inputs(&mut self, inputs: Vec<String>) {
        match self {
            NodeType::Control(i) => match inputs.get(0) {
                Some(input) => {
                    let _ = i.input.insert(input.clone());
                }
                None => {
                    i.input.take();
                }
            },
            NodeType::CustomTemp(i) => {
                i.input = inputs;
            }
            NodeType::Graph(i) => match inputs.get(0) {
                Some(input) => {
                    let _ = i.input.insert(input.clone());
                }
                None => {
                    i.input.take();
                }
            },
            NodeType::Linear(i, ..) => match inputs.get(0) {
                Some(input) => {
                    let _ = i.input.insert(input.clone());
                }
                None => {
                    i.input.take();
                }
            },
            NodeType::Target(i, ..) => match inputs.get(0) {
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

    pub fn is_control(&self) -> bool {
        matches!(self, NodeType::Control(..))
    }
}
