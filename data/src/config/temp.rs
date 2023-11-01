use std::rc::Rc;

use hardware::{Hardware, TempH};
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
    pub temp_h: Option<Rc<TempH>>,
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
                    Some(temp_h) => self.temp_h = Some(temp_h.clone()),
                    None => {
                        eprintln!("Temp to Node, hardware_id not found. {} from config not found. Fall back to no id", hardware_id);
                        self.hardware_id.take();
                        self.temp_h.take();
                    }
                }
            }
            None => {
                if self.temp_h.is_some() {
                    eprintln!("Temp to Node: inconsistent internal index");
                    self.temp_h.take();
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
        self.hardware_id.is_some() && self.temp_h.is_some()
    }
}
