use serde::{Deserialize, Serialize};

use crate::{
    app_graph::{NbInput, Node, NodeType},
    id::IdGenerator,
};

use super::IsValid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flat {
    pub name: String,
    pub value: u16,
}



impl Flat {
    pub fn to_node(self, id_generator: &mut IdGenerator) -> Node {
        Node {
            id: id_generator.new_id(),
            node_type: NodeType::Flat(self),
            max_input: NbInput::Zero,
            inputs: Vec::new(),
            value: None,
        }
    }
}

impl IsValid for Flat {
    fn is_valid(&self) -> bool {
        true
    }
}
