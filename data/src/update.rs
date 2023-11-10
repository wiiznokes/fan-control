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

    
    fn update2(nodes: &mut Nodes, node_id: &Id, updated: &mut HashSet<Id> ) -> Result<Option<Value>, UpdateError> {

        let Some(node) = nodes.get_mut(node_id) else {
            return Err(UpdateError::NodeNotFound);
        };

        if !updated.contains(&node_id) {

            if !node.is_valid() {
                return Ok(None);
            }

            let mut input_values = Vec::new();
            for id in &node.inputs {
                match Self::update2(nodes, id, updated)? {
                    Some(value) => input_values.push(value),
                    None => return Ok(None),
                }
            }
            
            node.update(&input_values)?;
            updated.insert(node.id);
        }

        return Ok(node.value);
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

        let mut input_values = Vec::new();
        for node_id in to_update {
            if !updated.contains(&node_id) {
                let Some(node) = nodes.get(&node_id) else {
                    return Err(UpdateError::NodeNotFound);
                };

                input_values.clear();
                for id in &node.inputs {
                    match nodes.get(id) {
                        Some(node) => match node.value {
                            Some(value) => input_values.push(value),
                            None => return Err(UpdateError::ValueIsNone),
                        },
                        None => return Err(UpdateError::NodeNotFound),
                    }
                }

                let Some(node) = nodes.get_mut(&node_id) else {
                    return Err(UpdateError::NodeNotFound);
                };

                node.update(&input_values)?;

                updated.insert(node.id);
            }
        }

        Ok(())
    }

    pub fn clear_cache(&mut self) {}
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
            crate::node::NodeType::Linear(linear) => linear.update(input_values[0])?,
            crate::node::NodeType::Target(target) => target.update(input_values[0])?,
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
