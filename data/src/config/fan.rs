use std::rc::Rc;

use crate::{
    app_graph::Nodes,
    id::IdGenerator,
    node::{IsValid, Node, NodeType, ToNode},
    update::UpdateError,
};
use hardware::{FanH, Hardware, HardwareBridge, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Fan {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: Option<String>,

    #[serde(skip)]
    pub fan_h: Option<Rc<FanH>>,
}

impl Fan {
    pub fn get_value(&self, bridge: &mut (impl HardwareBridge + ?Sized)) -> Result<Value, UpdateError> {
        match &self.fan_h {
            Some(fan_h) => bridge
                .get_value(&fan_h.internal_index)
                .map_err(UpdateError::Hardware),
            None => Err(UpdateError::NodeIsInvalid(self.name.clone())),
        }
    }
}

impl IsValid for Fan {
    fn is_valid(&self) -> bool {
        self.hardware_id.is_some() && self.fan_h.is_some()
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
                        warn!("Fan to Node, hardware_id not found. {} from config not found. Fall back to no id", hardware_id);
                        self.hardware_id.take();
                        self.fan_h.take();
                    }
                }
            }
            None => {
                if self.fan_h.is_some() {
                    warn!("Fan to Node: inconsistent internal index");
                    self.fan_h.take();
                }
            }
        }

        Node::new(id_generator, NodeType::Fan(self), nodes)
    }
}
