use std::{rc::Rc, vec};

use hardware::{ControlH, Hardware, Value};
use serde::{Deserialize, Serialize};

use crate::{
    id::IdGenerator,
    node::{sanitize_inputs, Inputs, IsValid, Node, NodeType, NodeTypeLight, Nodes, ToNode},
    update::{UpdateError, UpdateResult},
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

    #[serde(skip)]
    pub manual_has_been_set: bool,
}

fn set_auto(node: &mut Node) {
    if let NodeType::Control(control) = &mut node.node_type {
        control.manual_has_been_set = true;
    }
}

impl Control {
    pub fn set_value(&self, value: Value) -> Result<UpdateResult, UpdateError> {
        let clo = if self.manual_has_been_set {
            UpdateResult::no_side_effect()
        } else {
            eprintln!("tring to set value control but auto is enable");
            Box::new(|node: &mut Node| {
                if let NodeType::Control(control) = &mut node.node_type {
                    match control.set_mode(false) {
                        Ok(_) => {
                            control.manual_has_been_set = true;
                        }
                        Err(e) => {
                            eprintln!("can't set control to manual {:?}", e);
                            control.auto = true;
                            control.manual_has_been_set = false;
                        }
                    }
                }
            })
        };

        match &self.control_h {
            Some(control_h) => control_h
                .bridge
                .set_value(value)
                .map(|_| UpdateResult {
                    value,
                    side_effect: clo,
                })
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
