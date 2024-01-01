use std::rc::Rc;

use hardware::{Hardware, HardwareBridgeT, HardwareBridge, TempH, Value};
use serde::{Deserialize, Serialize};

use crate::{
    app_graph::Nodes,
    id::IdGenerator,
    node::{IsValid, Node, NodeType, ToNode},
    update::UpdateError,
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Temp {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: Option<String>,

    #[serde(skip)]
    pub temp_h: Option<Rc<TempH>>,
}

impl Temp {
    pub fn get_value(&self, bridge: &mut HardwareBridge) -> Result<Value, UpdateError> {
        match &self.temp_h {
            Some(temp_h) => bridge
                .get_value(&temp_h.internal_index)
                .map_err(UpdateError::Hardware),
            None => Err(UpdateError::NodeIsInvalid(self.name.clone())),
        }
    }
}

impl IsValid for Temp {
    fn is_valid(&self) -> bool {
        self.hardware_id.is_some() && self.temp_h.is_some()
    }
}

impl ToNode for Temp {
    fn to_node(
        mut self,
        id_generator: &mut IdGenerator,
        nodes: &Nodes,
        hardware: &Hardware,
    ) -> Node {
        match &self.hardware_id {
            Some(hardware_id) => {
                match hardware
                    .temps
                    .iter()
                    .find(|temp_h| &temp_h.hardware_id == hardware_id)
                {
                    Some(temp_h) => self.temp_h = Some(temp_h.clone()),
                    None => {
                        warn!("Temp to Node, hardware_id not found. {} from config not found. Fall back to no id", hardware_id);
                        self.hardware_id.take();
                        self.temp_h.take();
                    }
                }
            }
            None => {
                if self.temp_h.is_some() {
                    warn!("Temp to Node: inconsistent internal index");
                    self.temp_h.take();
                }
            }
        }

        Node::new(id_generator, NodeType::Temp(self), nodes)
    }
}
