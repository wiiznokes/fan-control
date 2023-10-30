use std::collections::HashSet;

use crate::{
    app_graph::{Node, Nodes, RootNodes},
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
                continue;
            };

            if let Some(ids) = node.find_nodes_to_update(nodes) {
                to_update.extend(ids);
            };
        }

        let mut updated: HashSet<Id> = HashSet::new();

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

    pub fn find_nodes_to_update(&self, _nodes: &Nodes) -> Option<Vec<Id>> {
        todo!()
    }
}
