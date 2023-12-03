use std::collections::HashMap;
use std::vec;

use hardware::{Hardware, Value};
use light_enum::LightEnum;

use crate::config::linear::LinearCache;
use crate::config::target::TargetCache;
use crate::config::Config;
use crate::config::{
    control::Control, custom_temp::CustomTemp, fan::Fan, flat::Flat, graph::Graph, linear::Linear,
    target::Target, temp::Temp,
};

use crate::id::{Id, IdGenerator};

pub type Nodes = HashMap<Id, Node>;
pub type RootNodes = Vec<Id>;

#[derive(Debug)]
pub struct AppGraph {
    pub nodes: Nodes,
    pub id_generator: IdGenerator,
    pub root_nodes: RootNodes,
}

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

pub fn sanitize_inputs(mut node: Node, nodes: &Nodes) -> Node {
    node.inputs.clear();
    match node.node_type.max_input() {
        NbInput::Zero => {
            if !node.node_type.get_inputs().is_empty() {
                eprintln!(
                    "{:?}: number of dep allowed == {:?}",
                    node.node_type.to_light(),
                    node.node_type.max_input()
                );
                node.node_type.clear_inputs();
            };
            return node;
        }
        NbInput::One => {
            if node.node_type.get_inputs().len() > 1 {
                eprintln!(
                    "{:?}: number of dep allowed == {:?}",
                    node.node_type.to_light(),
                    node.node_type.max_input()
                );
                node.node_type.clear_inputs();
                return node;
            }
        }
        _ => {}
    };

    for name in node.node_type.get_inputs() {
        if let Some(n) = nodes.values().find(|n| n.name() == &name) {
            if !node
                .node_type
                .allowed_dep()
                .contains(&n.node_type.to_light())
            {
                eprintln!(
                    "sanitize_inputs: incompatible node type. {:?} <- {}. Fall back: remove all",
                    n.node_type.to_light(),
                    name
                );
                node.node_type.clear_inputs();
                node.inputs.clear();
                return node;
            }
            node.inputs.push((n.id, name.clone()))
        } else {
            eprintln!(
                "sanitize_inputs: can't find {} in app_graph. Fall back: remove all",
                name
            );
            node.node_type.clear_inputs();
            node.inputs.clear();
            return node;
        }
    }

    if node.node_type.max_input() == NbInput::One && node.inputs.len() > 1 {
        node.node_type.clear_inputs();
        node.inputs.clear();
        return node;
    }
    node
}

pub fn validate_name(nodes: &Nodes, id: &Id, name: &String) -> bool {
    if name.is_empty() {
        return false;
    };

    for node in nodes.values() {
        if node.name() == name && &node.id != id {
            return false;
        }
    }

    true
}

pub trait ToNode {
    fn to_node(self, id_generator: &mut IdGenerator, nodes: &Nodes, hardware: &Hardware) -> Node;
}

pub trait IsValid {
    fn is_valid(&self) -> bool;
}

impl AppGraph {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            id_generator: IdGenerator::new(),
            root_nodes: Vec::new(),
        }
    }

    pub fn default(hardware: &Hardware) -> Self {
        let mut app_graph = AppGraph::new();

        for control_h in &hardware.controls {
            let control = Control::new(
                control_h.name.clone(),
                Some(control_h.hardware_id.clone()),
                None,
                true,
                Some(control_h.clone()),
            );

            let node = Node::new(
                &mut app_graph.id_generator,
                NodeType::Control(control),
                Vec::new(),
            );
            app_graph.root_nodes.push(node.id);
            app_graph.nodes.insert(node.id, node);
        }

        for fan_h in &hardware.fans {
            let fan = Fan {
                name: fan_h.name.clone(),
                hardware_id: Some(fan_h.hardware_id.clone()),
                fan_h: Some(fan_h.clone()),
            };

            let node = Node::new(&mut app_graph.id_generator, NodeType::Fan(fan), Vec::new());
            app_graph.nodes.insert(node.id, node);
        }

        for temp_h in &hardware.temps {
            let temp = Temp {
                name: temp_h.name.clone(),
                hardware_id: Some(temp_h.hardware_id.clone()),
                temp_h: Some(temp_h.clone()),
            };

            let node = Node::new(
                &mut app_graph.id_generator,
                NodeType::Temp(temp),
                Vec::new(),
            );
            app_graph.nodes.insert(node.id, node);
        }

        app_graph
    }

    pub fn from_config(config: Config, hardware: &Hardware) -> Self {
        let mut app_graph = AppGraph::new();

        // order: fan -> temp -> custom_temp -> behavior -> control

        for fan in config.fans {
            let node = fan.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.nodes.insert(node.id, node);
        }

        for temp in config.temps {
            let node = temp.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.nodes.insert(node.id, node);
        }

        for custom_temp in config.custom_temps {
            let node = custom_temp.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.nodes.insert(node.id, node);
        }

        for flat in config.flats {
            let node = flat.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.nodes.insert(node.id, node);
        }

        for linear in config.linears {
            let node = linear.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.nodes.insert(node.id, node);
        }

        for target in config.targets {
            let node = target.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.nodes.insert(node.id, node);
        }

        for control in config.controls {
            let node = control.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.root_nodes.push(node.id);
            app_graph.nodes.insert(node.id, node);
        }

        app_graph
    }
}

impl Node {
    pub fn new(
        id_generator: &mut IdGenerator,
        node_type: NodeType,
        inputs: Vec<(Id, String)>,
    ) -> Self {
        let name_cached = node_type.name().clone();
        Self {
            id: id_generator.new_id(),
            node_type,
            inputs,
            value: None,
            name_cached,
            is_error_name: false,
        }
    }

    pub fn name(&self) -> &String {
        self.node_type.name()
    }

    #[allow(clippy::result_unit_err)]
    pub fn hardware_id(&self) -> Result<&Option<String>, ()> {
        match &self.node_type {
            NodeType::Control(i) => Ok(&i.hardware_id),
            NodeType::Fan(i) => Ok(&i.hardware_id),
            NodeType::Temp(i) => Ok(&i.hardware_id),
            _ => Err(()),
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

    pub fn clear_inputs(&mut self) {
        match self {
            NodeType::Control(i) => {
                i.input.take();
            }
            NodeType::CustomTemp(i) => {
                i.input.clear();
            }
            NodeType::Graph(i) => {
                i.input.take();
            }
            NodeType::Linear(i, ..) => {
                i.input.take();
            }
            NodeType::Target(i, ..) => {
                i.input.take();
            }
            NodeType::Fan(_) => {}
            NodeType::Temp(_) => {}
            NodeType::Flat(_) => {}
        };
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
}
