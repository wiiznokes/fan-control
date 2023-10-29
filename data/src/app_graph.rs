use std::collections::HashMap;

use crate::config::{
    control::Control, custom_temp::CustomTemp, fan::Fan, flat::Flat, graph::Graph, linear::Linear,
    target::Target, temp::Temp,
};

use crate::id::{Id, IdGenerator};

pub type Nodes = HashMap<Id, Node>;

pub struct AppGraph {
    pub id_generator: IdGenerator,
    pub nodes: Nodes,
}

impl Default for AppGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl AppGraph {
    pub fn new() -> Self {
        AppGraph {
            id_generator: IdGenerator::new(),
            nodes: HashMap::new(),
        }
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
