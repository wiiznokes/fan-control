use serde::{Deserialize, Serialize};

use crate::{
    app_graph::{NbInput, Node, NodeType, Nodes},
    id::IdGenerator,
};

use super::IntoNode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fan {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: Option<String>,
}

impl IntoNode for Fan {
    fn to_node(self, id_generator: &mut IdGenerator, _app_graph: &Nodes) -> Node {
        Node {
            id: id_generator.new_id(),
            node_type: NodeType::Fan(self),
            max_input: NbInput::Zero,
            inputs: Vec::new(),
            value: None,
        }
    }
}
