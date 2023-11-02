use std::rc::Rc;

use hardware::{FanH, Hardware, Value};
use serde::{Deserialize, Serialize};

use crate::{
    id::IdGenerator,
    node::{sanitize_inputs, Inputs, IsValid, Node, NodeType, NodeTypeLight, Nodes, ToNode},
    update::UpdateError,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fan {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: Option<String>,

    #[serde(skip)]
    pub fan_h: Option<Rc<FanH>>,
}

impl Fan {
    pub fn get_value(&self) -> Result<Value, UpdateError> {
        match &self.fan_h {
            Some(fan_h) => fan_h.bridge.get_value().map_err(UpdateError::Hardware),
            None => Err(UpdateError::NodeIsInvalid),
        }
    }
}

impl IsValid for Fan {
    fn is_valid(&self) -> bool {
        self.hardware_id.is_some() && self.fan_h.is_some()
    }
}

impl Inputs for Fan {
    fn clear_inputs(&mut self) {}

    fn get_inputs(&self) -> Vec<&String> {
        Vec::new()
    }
}

impl ToNode for Fan {
    fn to_node(
        mut self,
        id_generator: &mut IdGenerator,
        nodes: &Nodes,
        hardware: &Hardware,
    ) -> Node {
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

        let inputs = sanitize_inputs(&mut self, nodes, NodeTypeLight::Fan);
        Node::new(id_generator, NodeType::Fan(self), inputs)
    }
}
