use hardware::Hardware;
use serde::{Deserialize, Serialize};

use crate::{
    app_graph::{NbInput, Node, NodeType},
    id::IdGenerator,
};

use super::IsValid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Temp {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: Option<String>,

    #[serde(skip)]
    pub hardware_index: Option<usize>,
}

impl Temp {
    pub fn to_node(mut self, id_generator: &mut IdGenerator, hardware: &Hardware) -> Node {
        match &self.hardware_id {
            Some(hardware_id) => {
                match hardware
                    .temps
                    .iter()
                    .find(|temp_h| &temp_h.hardware_id == hardware_id)
                {
                    Some(temp_h) => self.hardware_index = Some(temp_h.internal_index),
                    None => {
                        eprintln!("Temp to Node, hardware_id not found. {} from config not found. Fall back to no id", hardware_id);
                        self.hardware_id.take();
                        self.hardware_index.take();
                    }
                }
            }
            None => {
                if self.hardware_index.is_some() {
                    eprintln!("Temp to Node: inconsistent internal index");
                    self.hardware_index.take();
                }
            }
        }

        Node {
            id: id_generator.new_id(),
            node_type: NodeType::Temp(self),
            max_input: NbInput::Zero,
            inputs: Vec::new(),
            value: None,
        }
    }
}

impl IsValid for Temp {
    fn is_valid(&self) -> bool {
        self.hardware_id.is_some() && self.hardware_index.is_some()
    }
}
