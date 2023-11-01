use std::rc::Rc;

use hardware::{FanH, Hardware};
use serde::{Deserialize, Serialize};

use crate::{
    app_graph::{NbInput, Node, NodeType},
    id::IdGenerator,
};

use super::IsValid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fan {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: Option<String>,

    #[serde(skip)]
    pub fan_h: Option<Rc<FanH>>,
}

impl Fan {
    pub fn to_node(mut self, id_generator: &mut IdGenerator, hardware: &Hardware) -> Node {
        match &self.hardware_id {
            Some(hardware_id) => {
                match hardware
                    .fans
                    .iter()
                    .find(|fan_h| &fan_h.hardware_id == hardware_id)
                {
                    Some(fan_h) => self.fan_h = Some(fan_h.clone()),
                    None => {
                        eprintln!("Fan to Node, hardware_id not found. {} from config not found. Fall back to no id", hardware_id);
                        self.hardware_id.take();
                        self.fan_h.take();
                    }
                }
            }
            None => {
                if self.fan_h.is_some() {
                    eprintln!("Fan to Node: inconsistent internal index");
                    self.fan_h.take();
                }
            }
        }

        Node {
            id: id_generator.new_id(),
            node_type: NodeType::Fan(self),
            max_input: NbInput::Zero,
            inputs: Vec::new(),
            value: None,
        }
    }
}

impl IsValid for Fan {
    fn is_valid(&self) -> bool {
        self.hardware_id.is_some() && self.fan_h.is_some()
    }
}
