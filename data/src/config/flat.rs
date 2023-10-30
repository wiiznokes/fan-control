use serde::{Deserialize, Serialize};

use crate::{id::IdGenerator, app_graph::{Nodes, Node, NodeType, NbInput}};

use super::{IsValid, IntoNode};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flat {
    pub name: String,
    pub value: u16,
}


impl IntoNode for Flat {
    fn to_node(self, id_generator: &mut IdGenerator, _nodes: &Nodes) -> Node {
    

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
