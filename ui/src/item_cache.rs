use std::collections::HashMap;

use data::{
    id::Id,
    node::{Node, NodeType},
};

pub struct NodesC {
    data: HashMap<Id, NodeC>,
}

#[derive(Debug, Clone)]
pub struct NodeC {
    pub name: String,
    pub is_error_name: bool,
    pub context_menu_expanded: bool,
    pub node_type_c: NodeTypeC,
}

#[derive(Debug, Clone)]
pub enum NodeTypeC {
    Control(ControlC),
    Fan(FanC),
    Temp(TempC),
    CustomTemp(CustomTempC),
    Graph(GraphC),
    Flat(FlatC),
    Linear(LinearC),
    Target(TargetC),
}

#[derive(Debug, Clone)]
pub struct ControlC {}

#[derive(Debug, Clone)]
pub struct FanC {}

#[derive(Debug, Clone)]
pub struct TempC {}

#[derive(Debug, Clone)]
pub struct CustomTempC {}

#[derive(Debug, Clone)]
pub struct GraphC {}

#[derive(Debug, Clone)]
pub struct FlatC {}

#[derive(Debug, Clone)]
pub struct LinearC {
    pub min_temp: String,
    pub min_speed: String,
    pub max_temp: String,
    pub max_speed: String,
}

#[derive(Debug, Clone)]
pub struct TargetC {
    pub idle_temp: String,
    pub idle_speed: String,
    pub load_temp: String,
    pub load_speed: String,
}

impl NodesC {
    pub fn new<'a>(nodes: impl Iterator<Item = &'a Node>) -> Self {
        let mut data = HashMap::new();

        for node in nodes {
            data.insert(node.id, NodeC::new(node));
        }

        Self { data }
    }

    pub fn get_mut(&mut self, id: &Id) -> &mut NodeC {
        self.data.get_mut(id).unwrap()
    }

    pub fn get(&self, id: &Id) -> &NodeC {
        self.data.get(id).unwrap()
    }

    pub fn insert(&mut self, id: Id, node_c: NodeC) {
        self.data.insert(id, node_c);
    }

    pub fn remove(&mut self, id: &Id) {
        self.data.remove(id);
    }
}

impl NodeC {
    pub fn new(node: &Node) -> Self {
        Self {
            name: node.name().clone(),
            context_menu_expanded: false,
            node_type_c: NodeTypeC::new(&node.node_type),
            is_error_name: false,
        }
    }
}

impl NodeTypeC {
    pub fn new(node_type: &NodeType) -> Self {
        match node_type {
            data::node::NodeType::Control(_) => NodeTypeC::Control(ControlC {}),
            data::node::NodeType::Fan(_) => NodeTypeC::Fan(FanC {}),
            data::node::NodeType::Temp(_) => NodeTypeC::Temp(TempC {}),
            data::node::NodeType::CustomTemp(_) => NodeTypeC::CustomTemp(CustomTempC {}),
            data::node::NodeType::Graph(_) => NodeTypeC::Graph(GraphC {}),
            data::node::NodeType::Flat(_) => NodeTypeC::Flat(FlatC {}),
            data::node::NodeType::Linear(linear) => NodeTypeC::Linear(LinearC {
                min_temp: linear.min_temp.to_string(),
                min_speed: linear.min_speed.to_string(),
                max_temp: linear.max_temp.to_string(),
                max_speed: linear.max_speed.to_string(),
            }),
            data::node::NodeType::Target(target) => NodeTypeC::Target(TargetC {
                idle_temp: target.idle_temp.to_string(),
                idle_speed: target.idle_speed.to_string(),
                load_temp: target.load_temp.to_string(),
                load_speed: target.load_speed.to_string(),
            }),
        }
    }
}
