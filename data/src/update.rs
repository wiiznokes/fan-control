use std::collections::HashSet;

use hardware::{HardwareError, Value};

use crate::{
    id::Id,
    node::{IsValid, Node, Nodes, RootNodes},
};

#[derive(Debug, Clone)]
pub enum UpdateError {
    NodeNotFound,
    ValueIsNone,
    NodeIsInvalid,
    Hardware(HardwareError),
    NoInputData,
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

    pub fn graph(&mut self, nodes: &mut Nodes, root_nodes: &RootNodes) -> Result<(), UpdateError> {
        let mut to_update: Vec<Id> = Vec::new();

        for node_id in root_nodes {
            let Some(node) = nodes.get(node_id) else {
                return Err(UpdateError::NodeNotFound);
            };

            let mut ids = Vec::new();
            if node.validate(nodes, &mut ids)? {
                to_update.extend(ids);
            };
        }

        let mut updated: HashSet<Id> = HashSet::new();

        to_update.reverse();

        for node_id in to_update {
            if !updated.contains(&node_id) {
                let Some(node) = nodes.get(&node_id) else {
                    return Err(UpdateError::NodeNotFound);
                };

                let update_result = node.update(nodes)?;

                let Some(node) = nodes.get_mut(&node_id) else {
                    return Err(UpdateError::NodeNotFound);
                };

                node.value = Some(update_result.value);
                debug!("{} set to {}", node.name(), update_result.value);
                (update_result.side_effect)(node);

                updated.insert(node.id);
            }
        }

        Ok(())
    }

    pub fn clear_cache(&mut self) {}
}

pub struct UpdateResult {
    pub value: Value,
    pub side_effect: Box<dyn Fn(&mut Node)>,
}

impl UpdateResult {
    pub fn without_side_effect(value: Value) -> UpdateResult {
        UpdateResult {
            value,
            side_effect: UpdateResult::no_side_effect(),
        }
    }

    pub fn no_side_effect() -> Box<dyn Fn(&mut Node)> {
        Box::new(|_| {})
    }
}

impl From<UpdateResult> for Result<UpdateResult, UpdateError> {
    fn from(value: UpdateResult) -> Self {
        Ok(value)
    }
}

impl Node {
    pub fn update(&self, nodes: &Nodes) -> Result<UpdateResult, UpdateError> {
        let mut input_values = Vec::new();

        for id in &self.inputs {
            match nodes.get(id) {
                Some(node) => match node.value {
                    Some(value) => input_values.push(value),
                    None => return Err(UpdateError::ValueIsNone),
                },
                None => return Err(UpdateError::NodeNotFound),
            }
        }

        match &self.node_type {
            crate::node::NodeType::Control(control) => control.set_value(input_values[0]),

            crate::node::NodeType::Fan(fan) => fan.get_value(),
            crate::node::NodeType::Temp(temp) => temp.get_value(),
            crate::node::NodeType::CustomTemp(custom_temp) => custom_temp.update(input_values),
            crate::node::NodeType::Graph(_) => todo!(),
            crate::node::NodeType::Flat(flat) => {
                UpdateResult::without_side_effect(flat.value.into()).into()
            }
            crate::node::NodeType::Linear(linear) => linear.update(input_values[0]),
            crate::node::NodeType::Target(target) => target.update(input_values[0]),
        }
    }

    pub fn validate(&self, nodes: &Nodes, trace: &mut Vec<Id>) -> Result<bool, UpdateError> {
        if !self.is_valid() {
            return Ok(false);
        }

        trace.push(self.id);

        for id in &self.inputs {
            match nodes.get(id) {
                Some(node) => {
                    if !node.validate(nodes, trace)? {
                        return Ok(false);
                    }
                }
                None => return Err(UpdateError::NodeNotFound),
            }
        }

        Ok(true)
    }
}
