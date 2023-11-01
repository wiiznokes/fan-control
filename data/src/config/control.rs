use std::vec;

use hardware::{Hardware, InternalControlIndex, Value};
use serde::{Deserialize, Serialize};

use crate::{
    app_graph::{NbInput, Node, NodeType, NodeTypeLight, Nodes},
    id::IdGenerator,
    update::UpdateError,
    BoxedHardwareBridge,
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
    pub hardware_index: Option<InternalControlIndex>,
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
                    Some(control_h) => self.hardware_index = Some(control_h.internal_index.clone()),
                    None => {
                        eprintln!("Control to Node, hardware_id not found. {} from config not found. Fall back to no id", hardware_id);
                        self.hardware_id.take();
                        self.hardware_index.take();
                    }
                }
            }
            None => {
                if self.hardware_index.is_some() {
                    eprintln!("Control to Node: inconsistent internal index");
                    self.hardware_index.take();
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
        !self.auto
            && self.hardware_id.is_some()
            && self.hardware_index.is_some()
            && self.input.is_some()
    }
}

impl Control {
    pub fn update(
        &self,
        _value: Value,
        hardware_bridge: &BoxedHardwareBridge,
    ) -> Result<i32, UpdateError> {
        match &self.hardware_index {
            Some(indexes) => hardware_bridge
                .value(&indexes.io)
                .map_err(UpdateError::Hardware),
            None => Err(UpdateError::NodeIsInvalid),
        }
    }

    pub fn enable(
        &self,
        auto: bool,
        hardware_bridge: &BoxedHardwareBridge,
    ) -> Result<(), UpdateError> {
        match &self.hardware_index {
            Some(indexes) => hardware_bridge
                .set_value(&indexes.enable, !(auto as i32))
                .map_err(UpdateError::Hardware),
            None => Err(UpdateError::NodeIsInvalid),
        }
    }
}
