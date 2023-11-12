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

    pub fn graph(&mut self, nodes: &mut Nodes, root_nodes: &RootNodes) -> Result<(), UpdateError> {
        let mut updated: HashSet<Id> = HashSet::new();

        for node_id in root_nodes {
            Self::update_rec(nodes, node_id, &mut updated)?;
        }
        Ok(())
    }

    pub fn clear_cache(&mut self) {}

    fn update_rec(
        nodes: &mut Nodes,
        node_id: &Id,
        updated: &mut HashSet<Id>,
    ) -> Result<Option<Value>, UpdateError> {
        if updated.contains(node_id) {
            return match nodes.get(node_id) {
                Some(node) => Ok(node.value),
                None => Err(UpdateError::NodeNotFound),
            };
        }

        let input_ids: Vec<Id>;
        let mut input_values = Vec::new();
        {
            let Some(node) = nodes.get(node_id) else {
                return Err(UpdateError::NodeNotFound);
            };

            if !node.is_valid() {
                return Ok(None);
            }
            input_ids = node.inputs.iter().map(|i| i.0).collect();
        }

        for id in &input_ids {
            match Self::update_rec(nodes, id, updated)? {
                Some(value) => input_values.push(value),
                None => return Ok(None),
            }
        }

        let Some(node) = nodes.get_mut(node_id) else {
            return Err(UpdateError::NodeNotFound);
        };

        node.update(&input_values)?;
        updated.insert(node.id);

        Ok(node.value)
    }
}

impl Node {
    pub fn update(&mut self, input_values: &Vec<Value>) -> Result<(), UpdateError> {
        let value = match &mut self.node_type {
            crate::node::NodeType::Control(control) => control.set_value(input_values[0])?,

            crate::node::NodeType::Fan(fan) => fan.get_value()?,
            crate::node::NodeType::Temp(temp) => temp.get_value()?,
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

    pub fn validate(&self, nodes: &Nodes, trace: &mut Vec<Id>) -> Result<bool, UpdateError> {
        if !self.is_valid() {
            return Ok(false);
        }

        trace.push(self.id);

        for (id, _) in &self.inputs {
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
