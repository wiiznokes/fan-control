use serde::{Deserialize, Serialize};

use crate::{
    app_graph::{NbInput, Node, NodeType, Nodes},
    id::IdGenerator,
};

use super::{IntoNode, IsValid};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Control {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: Option<String>,
    pub input: Option<String>,
    pub auto: bool,
}

impl IntoNode for Control {
    fn to_node(self, id_generator: &mut IdGenerator, nodes: &Nodes) -> Node {
        let inputs = match &self.input {
            Some(input) => {
                let Some(node) = nodes.values().find(|node| node.name() == input) else {
                    panic!("Control to Node: can't find {} in app_graph", input)
                };
                vec![node.id]
            }
            None => Vec::new(),
        };

        Node {
            id: id_generator.new_id(),
            node_type: NodeType::Control(self),
            max_input: NbInput::One,
            inputs,
            value: None,
        }
    }
}

impl IsValid for Control {
    fn is_valid(&self) -> bool {
        !self.auto && self.hardware_id.is_some() && self.input.is_some()
    }
}
