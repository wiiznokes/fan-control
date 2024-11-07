use std::rc::Rc;

use hardware::{HControl, Hardware, HardwareBridge, Mode, Value};
use serde::{Deserialize, Serialize};

use crate::{
    app_graph::AppGraph,
    node::{IsValid, Node, NodeType, ToNode},
    update::UpdateError,
};

#[derive(Serialize, Deserialize, Debug, Clone, Default, Eq)]
pub struct Control {
    // unique
    pub name: String,
    // E hardware.controls, only one ref in all controls
    #[serde(rename = "id")]
    pub hardware_id: Option<String>,
    pub input: Option<String>,
    pub active: bool,

    // E hardware.controls, only one ref in all controls
    #[serde(skip)]
    pub control_h: Option<Rc<HControl>>,

    #[serde(skip)]
    pub mode_set: Option<Mode>,
}

impl PartialEq for Control {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.hardware_id == other.hardware_id
            && self.input == other.input
            && self.active == other.active
    }
}

impl Control {
    pub fn new(
        name: String,
        hardware_id: Option<String>,
        input: Option<String>,
        active: bool,
        control_h: Option<Rc<HControl>>,
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

    pub fn set_value<H: HardwareBridge>(
        &mut self,
        value: Value,
        bridge: &mut H,
    ) -> Result<Value, UpdateError> {
        if self.mode_set != Some(Mode::Manual) {
            self.set_mode(Mode::Manual, bridge)?;
        }

        match &self.control_h {
            Some(control_h) => {
                bridge.set_value(control_h, value)?;
                Ok(value)
            }
            None => Err(UpdateError::NodeIsInvalid(self.name.clone())),
        }
    }

    pub fn set_mode<H: HardwareBridge>(
        &mut self,
        mode: Mode,
        bridge: &mut H,
    ) -> Result<(), UpdateError> {
        if let Some(mode_set) = &self.mode_set {
            if mode_set == &mode {
                info!("Mode {} is already set for {}.", mode, self.name);
                return Ok(());
            }
        }

        match &self.control_h {
            Some(control_h) => bridge.set_mode(control_h, &mode)?,
            None => return Err(UpdateError::NodeIsInvalid(self.name.clone())),
        };

        info!("Mode {} succefuly set for {}.", mode, self.name);
        self.mode_set = Some(mode);
        Ok(())
    }

    pub fn get_value<H: HardwareBridge>(&self, bridge: &mut H) -> Result<Value, UpdateError> {
        match &self.control_h {
            Some(control_h) => bridge
                .get_control_value(control_h)
                .map_err(UpdateError::Hardware),
            None => Err(UpdateError::NodeIsInvalid(self.name.clone())),
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
    fn to_node(mut self, app_graph: &mut AppGraph, hardware: &Hardware) -> Node {
        match &self.hardware_id {
            Some(hardware_id) => {
                match hardware
                    .controls
                    .iter()
                    .find(|control_h| &control_h.hardware_id == hardware_id)
                {
                    Some(control_h) => {
                        if Rc::strong_count(control_h) > 1 {
                            warn!("Control to Node, hardware id \"{}\" is already use by another control. {}.", hardware_id, self.name);
                            self.hardware_id.take();
                            self.control_h.take();
                        } else {
                            self.control_h = Some(control_h.clone());
                        }
                    }
                    None => {
                        warn!("Control to Node, hardware id \"{}\" was not found for {}. Fall back: hardware not used.", hardware_id, self.name);
                        self.hardware_id.take();
                        self.control_h.take();
                    }
                }
            }
            None => {
                if self.control_h.is_some() {
                    warn!(
                        "Control to Node: inconsistent internal index for {}.",
                        self.name
                    );
                    self.control_h.take();
                }
            }
        }

        Node::new(NodeType::Control(self), app_graph)
    }
}
