use std::collections::HashMap;

use hardware::{Hardware, Value};
use light_enum::LightEnum;

use crate::config::{
    control::Control, custom_temp::CustomTemp, fan::Fan, flat::Flat, graph::Graph, linear::Linear,
    target::Target, temp::Temp,
};
use crate::config::{Config, IsValid};

use crate::id::{Id, IdGenerator};

pub type Nodes = HashMap<Id, Node>;
pub type RootNodes = Vec<Id>;

#[derive(Debug)]
pub struct AppGraph {
    pub nodes: Nodes,
    pub id_generator: IdGenerator,
    pub root_nodes: RootNodes,
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
            let control = Control {
                name: control_h.name.clone(),
                hardware_id: Some(control_h.hardware_id.clone()),
                input: None,
                auto: true,
                control_h: Some(control_h.clone()),
            };

            let node = Node {
                id: app_graph.id_generator.new_id(),
                node_type: NodeType::Control(control),
                max_input: NbInput::One,
                inputs: Vec::new(),
                value: None,
            };
            app_graph.root_nodes.push(node.id);
            app_graph.nodes.insert(node.id, node);
        }

        for fan_h in &hardware.fans {
            let fan = Fan {
                name: fan_h.name.clone(),
                hardware_id: Some(fan_h.hardware_id.clone()),
                hardware_index: Some(fan_h.internal_index),
            };

            let node = Node {
                id: app_graph.id_generator.new_id(),
                node_type: NodeType::Fan(fan),
                max_input: NbInput::Zero,
                inputs: Vec::new(),
                value: None,
            };
            app_graph.nodes.insert(node.id, node);
        }

        for temp_h in &hardware.temps {
            let temp = Temp {
                name: temp_h.name.clone(),
                hardware_id: Some(temp_h.hardware_id.clone()),
                hardware_index: Some(temp_h.internal_index),
            };

            let node = Node {
                id: app_graph.id_generator.new_id(),
                node_type: NodeType::Temp(temp),
                max_input: NbInput::Zero,
                inputs: Vec::new(),
                value: None,
            };
            app_graph.nodes.insert(node.id, node);
        }

        app_graph
    }

    pub fn from_config(config: Config, hardware: &Hardware) -> Self {
        let mut app_graph = AppGraph::new();

        // order: fan -> temp -> custom_temp -> behavior -> control

        for fan in config.fans {
            let node = fan.to_node(&mut app_graph.id_generator, hardware);
            app_graph.nodes.insert(node.id, node);
        }

        for temp in config.temps {
            let node = temp.to_node(&mut app_graph.id_generator, hardware);
            app_graph.nodes.insert(node.id, node);
        }

        for flat in config.flats {
            let node = flat.to_node(&mut app_graph.id_generator);
            app_graph.nodes.insert(node.id, node);
        }

        // TODO: other items

        for control in config.controls {
            let node = control.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.root_nodes.push(node.id);
            app_graph.nodes.insert(node.id, node);
        }

        app_graph
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: Id,
    pub node_type: NodeType,
    pub max_input: NbInput,
    pub inputs: Vec<Id>,

    pub value: Option<Value>,
}

#[derive(Debug, Clone, LightEnum)]
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
pub enum NbInput {
    Zero,
    One,
    Infinity,
}

impl Node {
    pub fn name(&self) -> &String {
        match &self.node_type {
            NodeType::Control(control) => &control.name,
            NodeType::Fan(fan) => &fan.name,
            NodeType::Temp(temp) => &temp.name,
            NodeType::CustomTemp(custom_temp) => &custom_temp.name,
            NodeType::Graph(graph) => &graph.name,
            NodeType::Flat(flat) => &flat.name,
            NodeType::Linear(linear) => &linear.name,
            NodeType::Target(target) => &target.name,
        }
    }
}

impl IsValid for Node {
    fn is_valid(&self) -> bool {
        match &self.node_type {
            NodeType::Control(control) => control.is_valid(),
            NodeType::Fan(fan) => fan.is_valid(),
            NodeType::Temp(temp) => temp.is_valid(),
            NodeType::CustomTemp(custom_temp) => custom_temp.is_valid(),
            NodeType::Graph(graph) => graph.is_valid(),
            NodeType::Flat(flat) => flat.is_valid(),
            NodeType::Linear(linear) => linear.is_valid(),
            NodeType::Target(target) => target.is_valid(),
        }
    }
}
