use std::rc::Rc;

use hardware::{ControlH, Hardware, HardwareBridgeT, Mode, Value};
use serde::{Deserialize, Serialize};

use crate::{
    app_graph::Nodes,
    id::IdGenerator,
    node::{IsValid, Node, NodeType, ToNode},
    update::UpdateError,
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Control {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: Option<String>,
    pub input: Option<String>,
    pub active: bool,

    #[serde(skip)]
    pub control_h: Option<Rc<ControlH>>,

    #[serde(skip)]
    pub mode_set: Option<Mode>,
}

impl Control {
    pub fn new(
        name: String,
        hardware_id: Option<String>,
        input: Option<String>,
        active: bool,
        control_h: Option<Rc<ControlH>>,
    ) -> Self {
        Self {
            name: name.clone(),
            hardware_id,
            input,
            active,
            control_h,
            mode_set: None,
        }
    }

    pub fn set_value(
        &mut self,
        value: Value,
        bridge: &mut HardwareBridgeT,
    ) -> Result<Value, UpdateError> {
        if self.mode_set != Some(Mode::Manual) {
            self.set_mode(Mode::Manual, bridge)?;
        }

        match &self.control_h {
            Some(control_h) => {
                bridge.set_value(&control_h.internal_index, value)?;
                Ok(value)
            }
            None => Err(UpdateError::NodeIsInvalid),
        }
    }

    pub fn set_mode(
        &mut self,
        mode: Mode,
        bridge: &mut HardwareBridgeT,
    ) -> Result<(), UpdateError> {
        if let Some(mode_set) = &self.mode_set {
            if mode_set == &mode {
                debug!("mode already set: {}", mode);
                return Ok(());
            }
        }

        match &self.control_h {
            Some(control_h) => bridge.set_mode(&control_h.internal_index, &mode)?,
            None => return Err(UpdateError::NodeIsInvalid),
        };

        debug!("mode succefuly set to {}", mode);
        self.mode_set = Some(mode);
        Ok(())
    }

    pub fn get_value(&self, bridge: &mut HardwareBridgeT) -> Result<Value, UpdateError> {
        match &self.control_h {
            Some(control_h) => bridge
                .get_value(&control_h.internal_index)
                .map_err(UpdateError::Hardware),
            None => Err(UpdateError::NodeIsInvalid),
        }
    }
}

impl IsValid for Control {
    fn is_valid(&self) -> bool {
        self.active
            && self.hardware_id.is_some()
            && self.control_h.is_some()
            && self.input.is_some()
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
                        warn!("Control to Node, hardware_id not found. {} from config not found. Fall back to no id", hardware_id);
                        self.hardware_id.take();
                        self.control_h.take();
                    }
                }
            }
            None => {
                if self.control_h.is_some() {
                    warn!("Control to Node: inconsistent internal index");
                    self.control_h.take();
                }
            }
        }

        Node::new(id_generator, NodeType::Control(self), nodes)
    }
}
