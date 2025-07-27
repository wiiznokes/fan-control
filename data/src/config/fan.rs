use std::rc::Rc;

use crate::{
    app_graph::AppGraph,
    node::{IsValid, Node, NodeType, ToNode},
    update::UpdateError,
};
use hardware::{HSensor, Hardware, HardwareBridge, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, Eq)]
pub struct Fan {
    // unique
    pub name: String,
    // E hardware.fans
    #[serde(rename = "id")]
    pub hardware_id: Option<String>,

    // E hardware.fans
    #[serde(skip)]
    pub fan_h: Option<Rc<HSensor>>,
}

impl PartialEq for Fan {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.hardware_id == other.hardware_id
    }
}

impl Fan {
    pub fn get_value<H: HardwareBridge>(&self, bridge: &mut H) -> Result<Value, UpdateError> {
        match &self.fan_h {
            Some(fan_h) => bridge
                .get_sensor_value(fan_h)
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
    fn to_node(mut self, app_graph: &mut AppGraph, hardware: &Hardware) -> Node {
        match &self.hardware_id {
            Some(hardware_id) => {
                match hardware
                    .fans
                    .iter()
                    .find(|fan_h| &fan_h.hardware_id == hardware_id)
                {
                    Some(fan_h) => self.fan_h = Some(fan_h.clone()),
                    None => {
                        warn!(
                            "Fan to Node, hardware_id not found. {hardware_id} from config not found. Fall back to no id"
                        );
                        self.hardware_id.take();
                        self.fan_h.take();
                    }
                }
            }
            None => {
                debug_assert!(self.fan_h.is_none())
            }
        }

        Node::new(NodeType::Fan(self), app_graph)
    }
}
