use std::vec;

use hardware::{Hardware, HardwareType, Value};
use serde::{Deserialize, Serialize};

use crate::{
    app_graph::{NbInput, Node, NodeType, NodeTypeLight, Nodes},
    id::IdGenerator,
    update::UpdateError,
    BoxedHardwareBridge,
};

use super::{sanitize_hardware_id, sanitize_inputs, HardwareId, Inputs, IsValid};

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
    pub hardware_internal_index: Option<usize>,
}

impl HardwareId for Control {
    fn hardware_id(&self) -> &Option<String> {
        &self.hardware_id
    }

    fn hardware_id_mut(&mut self) -> &mut Option<String> {
        &mut self.hardware_id
    }

    fn internal_index(&self) -> &Option<usize> {
        &self.hardware_internal_index
    }

    fn internal_index_mut(&mut self) -> &mut Option<usize> {
        &mut self.hardware_internal_index
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

impl Control {
    pub fn to_node(
        mut self,
        id_generator: &mut IdGenerator,
        nodes: &Nodes,
        hardware: &Hardware,
    ) -> Node {
        sanitize_hardware_id(&mut self, hardware, HardwareType::Control);
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
            && self.hardware_internal_index.is_some()
            && self.input.is_some()
    }
}

impl Control {
    pub fn update(
        &self,
        value: Value,
        hardware_bridge: &BoxedHardwareBridge,
    ) -> Result<i32, UpdateError> {
        match &self.hardware_internal_index {
            Some(index) => {
                hardware_bridge
                    .set_value(index, value)
                    .map_err(UpdateError::Hardware)?;
                hardware_bridge.value(index).map_err(UpdateError::Hardware)
            }
            None => Err(UpdateError::NodeIsInvalid),
        }
    }
}
