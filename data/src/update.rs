use std::collections::HashSet;

use hardware::{HardwareBridgeT, HardwareError, Value};

use crate::{
    id::Id,
    node::{Node, Nodes, RootNodes},
};

#[derive(Debug, Clone)]
pub enum UpdateError {
    NodeNotFound,
    ValueIsNone,
    NodeIsInvalid,
    Hardware(HardwareError),
    NoInputData,
    CantSetMode,
}

pub struct Update {}

impl Default for Update {
    fn default() -> Self {
        Self::new()
    }
}

impl Update {
    pub fn new() -> Self {
        Self {}
    }

    pub fn graph(
        &mut self,
        nodes: &mut Nodes,
        root_nodes: &RootNodes,
        bridge: &mut HardwareBridgeT,
    ) -> Result<(), UpdateError> {
        let mut updated: HashSet<Id> = HashSet::new();

        for node_id in root_nodes {
            Self::update_rec(nodes, node_id, &mut updated, bridge)?;
        }
        Ok(())
    }

    pub fn clear_cache(&mut self) {}

    fn update_rec(
        nodes: &mut Nodes,
        node_id: &Id,
        updated: &mut HashSet<Id>,
        bridge: &mut HardwareBridgeT,
    ) -> Result<Option<Value>, UpdateError> {
        if updated.contains(node_id) {
            return match nodes.get(node_id) {
                Some(node) => Ok(node.value),
                None => Err(UpdateError::NodeNotFound),
            };
        }

        let input_ids: Vec<Id>;
        {
            let Some(node) = nodes.get_mut(node_id) else {
                return Err(UpdateError::NodeNotFound);
            };

            if !node.node_type.is_valid() {
                node.value = None;
                return Ok(None);
            }
            input_ids = node.inputs.iter().map(|i| i.0).collect();
        }

        let mut input_values = Vec::new();
        for id in &input_ids {
            match Self::update_rec(nodes, id, updated, bridge)? {
                Some(value) => input_values.push(value),
                None => {
                    return match nodes.get_mut(node_id) {
                        Some(node) => {
                            node.value = None;
                            Ok(None)
                        }
                        None => Err(UpdateError::NodeNotFound),
                    }
                }
            }
        }

        let Some(node) = nodes.get_mut(node_id) else {
            return Err(UpdateError::NodeNotFound);
        };

        node.update(&input_values, bridge)?;
        updated.insert(node.id);

        Ok(node.value)
    }
}

impl Node {
    pub fn update(
        &mut self,
        input_values: &Vec<Value>,
        bridge: &mut HardwareBridgeT,
    ) -> Result<(), UpdateError> {
        let value = match &mut self.node_type {
            crate::node::NodeType::Control(control) => {
                control.set_value(input_values[0], bridge)?
            }
            crate::node::NodeType::Fan(fan) => fan.get_value(bridge)?,
            crate::node::NodeType::Temp(temp) => temp.get_value(bridge)?,
            crate::node::NodeType::CustomTemp(custom_temp) => custom_temp.update(input_values)?,
            crate::node::NodeType::Graph(_) => todo!(),
            crate::node::NodeType::Flat(flat) => flat.value.into(),
            crate::node::NodeType::Linear(linear, ..) => linear.update(input_values[0])?,
            crate::node::NodeType::Target(target, ..) => target.update(input_values[0])?,
        };
        debug!("{} set to {}", self.name(), value);
        self.value = Some(value);
        Ok(())
    }
}
