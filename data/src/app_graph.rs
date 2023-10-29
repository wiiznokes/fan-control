use std::collections::HashMap;

use hardware::Hardware;

use crate::config::{
    control::Control, custom_temp::CustomTemp, fan::Fan, flat::Flat, graph::Graph, linear::Linear,
    target::Target, temp::Temp,
};
use crate::config::{Config, IntoNode};
use crate::BoxedHardwareBridge;

use crate::id::{Id, IdGenerator};

#[derive(Default)]
pub struct AppGraph(pub HashMap<Id, Node>);

impl AppGraph {
    pub fn from_config(
        config: Config,
        hardware: &Hardware,
        id_generator: &mut IdGenerator,
    ) -> Self {
        let mut app_graph = AppGraph::default();

        for fan in config.fans {
            let node = fan.to_node(id_generator, &app_graph, hardware);
            app_graph.0.insert(node.id, node);
        }

        app_graph
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: Id,
    pub node_type: NodeType,
    pub nb_input: NbInput,
    pub input_ids: Vec<Id>,

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
    Fixed(u32),
    Infinity,
}

impl Node {
    pub fn update(&self, hardware_bridge: &BoxedHardwareBridge) -> Option<(Id, i32)> {
        todo!()
    }
}
