use serde::{Deserialize, Serialize};

use hardware::Hardware;

use crate::{
    id::IdGenerator,
    node::{sanitize_inputs, IsValid, Node, NodeType, Nodes, ToNode},
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
        sanitize_inputs(
            Node::new(id_generator, NodeType::Flat(self), Vec::new()),
            nodes,
        )
    }
}
