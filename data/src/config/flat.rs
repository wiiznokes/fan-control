use serde::{Deserialize, Serialize};

use hardware::Hardware;

use crate::{
    app_graph::Nodes,
    id::IdGenerator,
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
    fn to_node(self, id_generator: &mut IdGenerator, nodes: &Nodes, _hardware: &Hardware) -> Node {
        Node::new(id_generator, NodeType::Flat(self), nodes)
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
