use std::collections::{HashMap, HashSet};

use crate::config::{
    control::Control, custom_temp::CustomTemp, fan::Fan, flat::Flat, graph::Graph, linear::Linear,
    target::Target, temp::Temp,
};
use crate::config::{Config, IntoNode};
use crate::BoxedHardwareBridge;

use crate::id::{Id, IdGenerator};

pub type Nodes = HashMap<Id, Node>;

#[derive(Default)]
pub struct AppGraph {
    pub nodes: Nodes,
    pub id_generator: IdGenerator,
    pub root_nodes: Vec<Id>,
}

impl AppGraph {
    pub fn from_config(config: Config) -> Self {
        let mut app_graph = AppGraph::default();

        for fan in config.fans {
            let node = fan.to_node(&mut app_graph.id_generator, &app_graph.nodes);
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

    pub value: Option<i32>,
}

#[derive(Debug, Clone)]
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

impl AppGraph {
    pub fn update(
        &self,
        _hardware_bridge: &BoxedHardwareBridge,
        app_graph: &AppGraph,
        controls_id: Vec<Id>,
    ) -> Vec<(Id, i32)> {
        let mut to_update: HashSet<Id> = HashSet::new();

        let update = Vec::new();

        for control_id in controls_id {
            let Some(control) = app_graph.nodes.get(&control_id) else {
                continue;
            };

            if let Some(ids) = control.find_nodes_to_update(app_graph) {
                to_update.extend(&ids);
            };
        }

        update
    }
}

impl Node {
    pub fn update(
        &self,
        _hardware_bridge: &BoxedHardwareBridge,
        _app_graph: &AppGraph,
    ) -> Vec<(Id, i32)> {
        todo!()
    }

    pub fn find_nodes_to_update(&self, _app_graph: &AppGraph) -> Option<HashSet<Id>> {
        todo!()
    }
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
