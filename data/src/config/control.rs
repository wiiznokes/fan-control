use std::{rc::Rc, vec};

use hardware::{ControlH, Hardware, Value};
use serde::{Deserialize, Serialize};

use crate::{
    app_graph::{NbInput, Node, NodeType, NodeTypeLight, Nodes},
    id::IdGenerator,
    update::UpdateError,
};

use super::{sanitize_inputs, Inputs, IsValid};

static CONTROL_ALLOWED_DEP: &[NodeTypeLight] = &[
    NodeTypeLight::Flat,
    NodeTypeLight::Graph,
    NodeTypeLight::Target,
    NodeTypeLight::Linear,
];

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Control {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: Option<String>,
    pub input: Option<String>,
    pub auto: bool,

    #[serde(skip)]
    pub control_h: Option<Rc<ControlH>>,
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

impl Control {
    pub fn to_node(
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

        let inputs = sanitize_inputs(&mut self, nodes, NbInput::One, CONTROL_ALLOWED_DEP);

        Node {
            id: id_generator.new_id(),
            node_type: NodeType::Control(self),
            max_input: NbInput::One,
            inputs,
            value: None,
        }
    }
}

impl IsValid for Control {
    fn is_valid(&self) -> bool {
        !self.auto && self.hardware_id.is_some() && self.control_h.is_some() && self.input.is_some()
    }
}

impl Control {
    pub fn update(&self, _value: Value) -> Result<i32, UpdateError> {
        match &self.control_h {
            Some(control_h) => control_h.bridge.value().map_err(UpdateError::Hardware),
            None => Err(UpdateError::NodeIsInvalid),
        }
    }

    pub fn enable(&self, auto: bool) -> Result<(), UpdateError> {
        match &self.control_h {
            Some(control_h) => control_h
                .bridge
                .set_value(!(auto as i32))
                .map_err(UpdateError::Hardware),
            None => Err(UpdateError::NodeIsInvalid),
        }
    }
}
