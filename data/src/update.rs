use std::collections::HashSet;

use crate::{
    app_graph::{Node, Nodes, RootNodes},
    config::IsValid,
    id::Id,
    BoxedHardwareBridge,
};

#[derive(Debug, Clone)]
pub enum UpdateError {
    NodeNotFound,
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
        hardware_bridge: &BoxedHardwareBridge,
        root_nodes: &RootNodes,
    ) -> Result<(), UpdateError> {
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

                let value = node.update(nodes, hardware_bridge)?;

                let Some(node) = nodes.get_mut(&node_id) else {
                    return Err(UpdateError::NodeNotFound);
                };
                node.value = Some(value);

                updated.insert(node.id);
            }
        }

        Ok(())
    }

    pub fn clear_cache(&mut self) {}
}

impl Node {
    pub fn update(
        &self,
        _nodes: &Nodes,
        _hardware_bridge: &BoxedHardwareBridge,
    ) -> Result<i32, UpdateError> {
        todo!()
    }

    pub fn validate(&self, nodes: &Nodes, trace: &mut Vec<Id>) -> Result<bool, UpdateError> {
        match &self.node_type {
            crate::app_graph::NodeType::Control(control) => {
                if !control.is_valid() {
                    return Ok(false);
                }
            }
            crate::app_graph::NodeType::Fan(fan) => {
                if !fan.is_valid() {
                    return Ok(false);
                }
            }
            crate::app_graph::NodeType::Temp(temp) => {
                if !temp.is_valid() {
                    return Ok(false);
                }
            }
            crate::app_graph::NodeType::CustomTemp(custom_temp) => {
                if !custom_temp.is_valid() {
                    return Ok(false);
                }
            }
            crate::app_graph::NodeType::Graph(graph) => {
                if !graph.is_valid() {
                    return Ok(false);
                }
            }
            crate::app_graph::NodeType::Flat(flat) => {
                if !flat.is_valid() {
                    return Ok(false);
                }
            }
            crate::app_graph::NodeType::Linear(linear) => {
                if !linear.is_valid() {
                    return Ok(false);
                }
            }
            crate::app_graph::NodeType::Target(target) => {
                if !target.is_valid() {
                    return Ok(false);
                }
            }
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
