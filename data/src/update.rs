use std::{cmp::Ordering, collections::HashSet};

use hardware::{HardwareBridgeT, HardwareError, Value};

use crate::{
    app_graph::{Nodes, RootNodes},
    id::Id,
    node::{Node, NodeType},
};

#[derive(Debug)]
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

    pub fn optimized(
        &mut self,
        nodes: &mut Nodes,
        root_nodes: &RootNodes,
        bridge: &mut HardwareBridgeT,
    ) {
        if let Err(e) = bridge.update() {
            error!("{:?}", e);
            return;
        }

        let mut updated: HashSet<Id> = HashSet::new();
        for node_id in root_nodes {
            if let Err(e) = Self::update_rec(nodes, node_id, &mut updated, bridge) {
                error!("{:?}", e);
            }
        }
    }

    pub fn all(&mut self, nodes: &mut Nodes, root_nodes: &RootNodes, bridge: &mut HardwareBridgeT) {
        if let Err(e) = bridge.update() {
            error!("{:?}", e);
            return;
        }

        for id in root_nodes {
            match nodes.get_mut(id) {
                Some(node) => {
                    if let NodeType::Control(control) = &node.node_type {
                        if control.control_h.is_none() {
                            continue;
                        }
                        match control.get_value(bridge) {
                            Ok(value) => {
                                node.value = Some(value);
                            }
                            Err(e) => {
                                node.value.take();
                                error!("{:?}", e);
                            }
                        }
                    }
                }
                None => {
                    error!("{:?}", UpdateError::NodeNotFound);
                }
            }
        }

        let ids: Vec<Id>;
        {
            let mut key_values = nodes.iter().collect::<Vec<_>>();

            key_values.sort_by(|(_, first), (_, other)| match first.node_type {
                NodeType::Control(_) => match other.node_type {
                    NodeType::Control(_) => Ordering::Equal,
                    _ => Ordering::Greater,
                },
                NodeType::Fan(_) => {
                    if other.node_type.is_sensor() {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                }
                NodeType::Temp(_) => {
                    if other.node_type.is_sensor() {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                }
                NodeType::CustomTemp(_) => match other.node_type {
                    NodeType::CustomTemp(_) => Ordering::Equal,
                    NodeType::Fan(_) => Ordering::Greater,
                    NodeType::Temp(_) => Ordering::Greater,
                    _ => Ordering::Less,
                },
                NodeType::Graph(_) => todo!(),
                NodeType::Flat(_) => Ordering::Equal,
                NodeType::Linear(..) => match other.node_type {
                    NodeType::Control(_) => Ordering::Less,
                    NodeType::Fan(_) => Ordering::Greater,
                    NodeType::Temp(_) => Ordering::Greater,
                    NodeType::CustomTemp(_) => Ordering::Greater,
                    _ => Ordering::Equal,
                },
                NodeType::Target(..) => match other.node_type {
                    NodeType::Control(_) => Ordering::Less,
                    NodeType::Fan(_) => Ordering::Greater,
                    NodeType::Temp(_) => Ordering::Greater,
                    NodeType::CustomTemp(_) => Ordering::Greater,
                    _ => Ordering::Equal,
                },
            });

            ids = key_values.iter().map(|e| *e.0).collect();
        }

        let mut updated = HashSet::new();
        for id in ids {
            if let Err(e) = Self::update_rec(nodes, &id, &mut updated, bridge) {
                error!("{:?}", e);
            }
        }
    }

    pub fn set_all_control_to_auto(
        &mut self,
        nodes: &mut Nodes,
        root_nodes: &RootNodes,
        bridge: &mut HardwareBridgeT,
    ) {
        for node_id in root_nodes {
            let Some(node) = nodes.get_mut(node_id) else {
                warn!("node not found in set_all_control_to_auto function");
                continue;
            };
            if let NodeType::Control(control) = &mut node.node_type {
                if let Err(e) = control.set_mode(false, bridge) {
                    error!("{:?}", e);
                }
            }
        }
    }

    pub fn set_invalid_controls_to_auto(
        &mut self,
        nodes: &mut Nodes,
        root_nodes: &RootNodes,
        bridge: &mut HardwareBridgeT,
    ) {
        for node_id in root_nodes {
            if !Self::validate_rec(nodes, node_id) {
                let Some(node) = nodes.get_mut(node_id) else {
                    warn!("node not found in set_invalid_controls_to_auto function");
                    continue;
                };
                if let NodeType::Control(control) = &mut node.node_type {
                    if let Err(e) = control.set_mode(false, bridge) {
                        error!("{:?}", e);
                    }
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

        for (id, _) in &node.inputs {
            if !Self::validate_rec(nodes, id) {
                return false;
            }
        }
        true
    }

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
            updated.insert(node.id);

            if !node.node_type.is_valid() {
                if !node.node_type.is_control() {
                    node.value = None;
                }
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
                            if !node.node_type.is_control() {
                                node.value = None;
                            }
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
            crate::node::NodeType::Control(control) => control.set_value(input_values[0], bridge),
            crate::node::NodeType::Fan(fan) => fan.get_value(bridge),
            crate::node::NodeType::Temp(temp) => temp.get_value(bridge),
            crate::node::NodeType::CustomTemp(custom_temp) => custom_temp.update(input_values),
            crate::node::NodeType::Graph(_) => todo!(),
            crate::node::NodeType::Flat(flat) => Ok(flat.value.into()),
            crate::node::NodeType::Linear(linear, ..) => linear.update(input_values[0]),
            crate::node::NodeType::Target(target, ..) => target.update(input_values[0]),
        };

        match value {
            Ok(value) => {
                debug!("{} set to {}", self.name(), value);
                self.value = Some(value);
                Ok(())
            }
            Err(e) => {
                error!("{:?}", e);
                self.value = None;
                Err(e)
            }
        }
    }
}
