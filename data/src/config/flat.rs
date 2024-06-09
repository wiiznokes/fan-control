use serde::{Deserialize, Serialize};

use hardware::Hardware;

use crate::{
    app_graph::AppGraph,
    node::{IsValid, Node, NodeType, ToNode},
};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flat {
    pub name: String,
    pub value: u16,
}

impl IsValid for Flat {
    fn is_valid(&self) -> bool {
        true
    }
}

impl ToNode for Flat {
    fn to_node(mut self, app_graph: &mut AppGraph, _hardware: &Hardware) -> Node {
        if self.value > 100 {
            self.value = 50;
        }

        Node::new(NodeType::Flat(self), app_graph)
    }
}
impl Default for Flat {
    fn default() -> Self {
        Self {
            name: Default::default(),
            value: 50u16,
        }
    }
}
