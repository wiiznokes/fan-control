use std::{rc::Rc, vec};

use hardware::{ControlH, Hardware, Value};
use serde::{Deserialize, Serialize};

use crate::{
    id::IdGenerator,
    node::{sanitize_inputs, Inputs, IsValid, Node, NodeType, NodeTypeLight, Nodes, ToNode},
    update::UpdateError,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Control {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: Option<String>,
    pub input: Option<String>,
    pub auto: bool,

    #[serde(skip)]
    pub control_h: Option<Rc<ControlH>>,

    pub is_set_auto: bool,
}

impl Control {
    pub fn set_value(&self, value: Value) -> Result<Value, UpdateError> {
        match &self.control_h {
            Some(control_h) => control_h
                .bridge
                .set_value(value)
                .map(|_| value)
                .map_err(UpdateError::Hardware),
            None => Err(UpdateError::NodeIsInvalid),
        }
    }

    pub fn set_mode(&self, auto: bool) -> Result<(), UpdateError> {
        match &self.control_h {
            Some(control_h) => control_h
                .bridge
                .set_mode(!(auto as i32))
                .map_err(UpdateError::Hardware),
            None => Err(UpdateError::NodeIsInvalid),
        }
    }
}

impl IsValid for Control {
    fn is_valid(&self) -> bool {
        !self.auto && self.hardware_id.is_some() && self.control_h.is_some() && self.input.is_some()
    }
}

impl Inputs for Control {
    fn clear_inputs(&mut self) {
        self.input.take();
    }

    fn get_inputs(&self) -> Vec<&String> {
        match &self.input {
            Some(input) => vec![input],
            None => Vec::new(),
        }
    }
}

impl ToNode for Control {
    fn to_node(
        mut self,
        id_generator: &mut IdGenerator,
        nodes: &Nodes,
        hardware: &Hardware,
    ) -> Node {
        match &self.hardware_id {
            Some(hardware_id) => {
                match hardware
                    .controls
                    .iter()
                    .find(|control_h| &control_h.hardware_id == hardware_id)
                {
                    Some(control_h) => self.control_h = Some(control_h.clone()),
                    None => {
                        eprintln!("Control to Node, hardware_id not found. {} from config not found. Fall back to no id", hardware_id);
                        self.hardware_id.take();
                        self.control_h.take();
                    }
                }
            }
            None => {
                if self.control_h.is_some() {
                    eprintln!("Control to Node: inconsistent internal index");
                    self.control_h.take();
                }
            }
        }

        let inputs = sanitize_inputs(&mut self, nodes, NodeTypeLight::Control);
        Node::new(id_generator, NodeType::Control(self), inputs)
    }
}
