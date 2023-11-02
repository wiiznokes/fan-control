use serde::{Deserialize, Serialize};

use hardware::Hardware;

use crate::{
    id::IdGenerator,
    node::{sanitize_inputs, Inputs, IsValid, Node, NodeType, NodeTypeLight, Nodes, ToNode},
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

impl Inputs for Flat {
    fn clear_inputs(&mut self) {}

    fn get_inputs(&self) -> Vec<&String> {
        Vec::new()
    }
}

impl ToNode for Flat {
    fn to_node(
        mut self,
        id_generator: &mut IdGenerator,
        nodes: &Nodes,
        _hardware: &Hardware,
    ) -> Node {
        let inputs = sanitize_inputs(&mut self, nodes, NodeTypeLight::Flat);

        Node {
            id: id_generator.new_id(),
            node_type: NodeType::Flat(self),
            inputs,
            value: None,
        }
    }
}
