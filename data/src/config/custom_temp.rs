use hardware::Value;
use serde::{Deserialize, Serialize};

use crate::{app_graph::{Node, Nodes, NbInput, NodeType}, id::IdGenerator, update::UpdateError};

use super::IsValid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CustomTempType {
    Min,
    Max,
    Average,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomTemp {
    pub name: String,
    pub kind: CustomTempType,
    pub input: Vec<String>,
}


impl CustomTemp {
    pub fn to_node(
        mut self,
        id_generator: &mut IdGenerator,
        nodes: &Nodes
    ) -> Node {

        let mut inputs = Vec::new();

        for name in &self.input {
            if let Some(node) = nodes.values().find(|node| node.name() == name) {
                inputs.push(node.id)
            } else {
                eprintln!(
                    "CustomTemp to Node: can't find {} in app_graph. Fall back: remove",
                    name
                );
                self.input.clear();
                inputs.clear();
                break;
            }
        }

        Node {
            id: id_generator.new_id(),
            node_type: NodeType::CustomTemp(self),
            max_input: NbInput::Infinity,
            inputs,
            value: None,
        }
    }
}

impl IsValid for CustomTemp {
    fn is_valid(&self) -> bool {
        !self.input.is_empty()
    }
}


impl CustomTemp {
    pub fn update(
        &self,
        values: Vec<Value>,
    ) -> Result<i32, UpdateError> {

        if values.is_empty() {
            return Err(UpdateError::NoInputData);
        }

        let value = match self.kind {
            CustomTempType::Min => *values.iter().min().unwrap(),
            CustomTempType::Max => *values.iter().min().unwrap(),
            CustomTempType::Average => values.iter().sum::<i32>() / values.len() as i32,
        };

        Ok(value)
    }
}
