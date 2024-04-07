use std::collections::HashSet;

use hardware::{HardwareBridge, Mode, Value};

use thiserror::Error;

use crate::{
    app_graph::{Nodes, RootNodes},
    id::Id,
    node::{Node, NodeType},
};

#[derive(Error, Debug)]
pub enum UpdateError {
    #[error("Node id \"{0}\" was not found")]
    NodeNotFound(Id),
    #[error("Value was none")]
    ValueIsNone,
    #[error("Node {0} was invalid")]
    NodeIsInvalid(String),
    #[error("No input data")]
    NoInputData,
    #[error("Can't set mode")]
    CantSetMode,
    #[error(transparent)]
    Hardware(#[from] hardware::HardwareError),
}

type Result<T> = std::result::Result<T, UpdateError>;

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

    // todo: remember what nodes are valid
    /// Update graph in an optimal way. This shouln't be use
    /// with a graphical interface.
    /// Warning: doesn't call update from the bridge, it's the role of the caller.
    pub fn optimized<H: HardwareBridge>(
        &mut self,
        nodes: &mut Nodes,
        root_nodes: &RootNodes,
        bridge: &mut H,
    ) -> Result<()> {
        let mut updated: HashSet<Id> = HashSet::new();
        for node_id in root_nodes {
            if let Err(e) = Self::update_rec(nodes, node_id, &mut updated, bridge) {
                error!("Can't update node: {}.", e);
            }
        }
        Ok(())
    }

    /// Doesn't update root nodes and doesn't re update nodes that could have been updated (fans).
    /// Warning: doesn't call update from the bridge, it's the role of the caller.
    pub fn all<H: HardwareBridge>(&mut self, nodes: &mut Nodes, bridge: &mut H) -> Result<()> {
        let ids_to_update_sorted: Vec<Id>;
        {
            let mut key_values = nodes.iter().collect::<Vec<_>>();

            key_values.sort_by(|(_, first), (_, other)| {
                first.node_type.compare_update_priority(&other.node_type)
            });

            ids_to_update_sorted = key_values.iter().map(|(id, _)| **id).collect();
        }

        let mut updated = HashSet::new();
        for id in ids_to_update_sorted {
            if let Err(e) = Self::update_rec(nodes, &id, &mut updated, bridge) {
                error!("can't update node: {}", e);
            }
        }

        Ok(())
    }

    pub fn nodes_which_update_can_change<H: HardwareBridge>(
        &mut self,
        nodes: &mut Nodes,
        bridge: &mut H,
    ) -> Result<()> {
        for node in nodes.values_mut() {
            let value = match &mut node.node_type {
                crate::node::NodeType::Control(control) => Some(control.get_value(bridge)),
                crate::node::NodeType::Fan(fan) => Some(fan.get_value(bridge)),
                _ => None,
            };

            if let Some(value) = value {
                match value {
                    Ok(value) => {
                        debug!("Node {} value is {}.", node.name(), value);
                        node.value.replace(value);
                    }
                    Err(UpdateError::NodeIsInvalid(_)) => {
                        node.value.take();
                    }
                    Err(e) => {
                        node.value.take();
                        error!(
                            "Can't get the value of the root node {}: {}.",
                            node.name(),
                            e
                        );
                    }
                }
            }
        }

        Ok(())
    }

    fn set_node_to_auto<H: HardwareBridge>(
        &mut self,
        nodes: &mut Nodes,
        node_id: &Id,
        bridge: &mut H,
    ) -> Result<()> {
        let Some(node) = nodes.get_mut(node_id) else {
            return Err(UpdateError::NodeNotFound(*node_id));
        };

        match &mut node.node_type {
            NodeType::Control(control) => control.set_mode(Mode::Auto, bridge),
            _ => Err(UpdateError::NodeIsInvalid(node.name().to_owned())),
        }
    }

    pub fn set_valid_root_nodes_to_auto<H: HardwareBridge>(
        &mut self,
        nodes: &mut Nodes,
        root_nodes: &RootNodes,
        bridge: &mut H,
    ) {
        for node_id in root_nodes {
            if Self::validate_rec(nodes, node_id) {
                if let Err(e) = self.set_node_to_auto(nodes, node_id, bridge) {
                    error!(
                        "Can't set control to auto in set_valid_controls_to_auto fn: {}",
                        e
                    );
                }
            }
        }
    }

    pub fn set_invalid_root_nodes_to_auto<H: HardwareBridge>(
        &mut self,
        nodes: &mut Nodes,
        root_nodes: &RootNodes,
        bridge: &mut H,
    ) {
        for node_id in root_nodes {
            if !Self::validate_rec(nodes, node_id) {
                if let Err(e) = self.set_node_to_auto(nodes, node_id, bridge) {
                    error!(
                        "Can't set control to auto in set_invalid_controls_to_auto fn: {}",
                        e
                    );
                }
            }
        }
    }

    fn validate_rec(nodes: &Nodes, node_id: &Id) -> bool {
        let Some(node) = nodes.get(node_id) else {
            return false;
        };

        if !node.node_type.is_valid() {
            return false;
        };

        for input in &node.inputs {
            if !Self::validate_rec(nodes, &input.id) {
                return false;
            }
        }
        true
    }

    fn update_rec<H: HardwareBridge>(
        nodes: &mut Nodes,
        node_id: &Id,
        updated: &mut HashSet<Id>,
        bridge: &mut H,
    ) -> Result<Option<Value>> {
        if updated.contains(node_id) {
            return match nodes.get(node_id) {
                Some(node) => Ok(node.value),
                None => Err(UpdateError::NodeNotFound(*node_id)),
            };
        }

        let input_ids: Vec<Id>;
        {
            let Some(node) = nodes.get_mut(node_id) else {
                return Err(UpdateError::NodeNotFound(*node_id));
            };
            updated.insert(node.id);

            if !node.node_type.is_valid() {
                if !node.is_root() {
                    node.value = None;
                }
                return Ok(None);
            }
            input_ids = node.inputs.iter().map(|i| i.id).collect();
        }

        let mut input_values = Vec::new();
        for id in &input_ids {
            match Self::update_rec(nodes, id, updated, bridge)? {
                Some(value) => input_values.push(value),
                None => {
                    return match nodes.get_mut(node_id) {
                        Some(node) => {
                            if !node.is_root() {
                                node.value = None;
                            }
                            Ok(None)
                        }
                        None => Err(UpdateError::NodeNotFound(*node_id)),
                    }
                }
            }
        }

        let Some(node) = nodes.get_mut(node_id) else {
            return Err(UpdateError::NodeNotFound(*node_id));
        };

        node.update(&input_values, bridge)?;

        Ok(node.value)
    }
}

impl Node {
    fn update<H: HardwareBridge>(&mut self, input_values: &[Value], bridge: &mut H) -> Result<()> {
        let value = match &mut self.node_type {
            crate::node::NodeType::Control(control) => {
                let input_value = input_values[0];
                return if self.value == Some(input_value) {
                    debug!("Control {} already set to {}", control.name, input_value);
                    Ok(())
                } else {
                    debug!("Before setting control {} to {}", control.name, input_value);
                    control.set_value(input_value, bridge).map(|_| ())
                };
            }
            crate::node::NodeType::Fan(fan) => fan.get_value(bridge),
            crate::node::NodeType::Temp(temp) => temp.get_value(bridge),
            crate::node::NodeType::CustomTemp(custom_temp) => custom_temp.get_value(input_values),
            crate::node::NodeType::Graph(graph) => graph.get_value(input_values[0]),
            crate::node::NodeType::Flat(flat) => Ok(flat.value.into()),
            crate::node::NodeType::Linear(linear, ..) => linear.get_value(input_values[0]),
            crate::node::NodeType::Target(target, ..) => target.get_value(input_values[0]),
        };

        match value {
            Ok(value) => {
                self.value = Some(value);
                Ok(())
            }
            Err(e) => {
                self.value = None;
                Err(e)
            }
        }
    }
}
