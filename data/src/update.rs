use std::collections::HashSet;

use hardware::{HardwareBridgeT, HardwareError, Value};

use crate::{
    id::Id,
    node::{Node, NodeType, Nodes, RootNodes},
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

pub struct Update {
    config_changed: bool,
}

impl Default for Update {
    fn default() -> Self {
        Self::new()
    }
}

impl Update {
    pub fn new() -> Self {
        Self {
            config_changed: false,
        }
    }

    pub fn config_changed(&mut self) {
        self.config_changed = true;
    }
}

impl Update {
    pub fn optimized(
        &mut self,
        nodes: &mut Nodes,
        root_nodes: &RootNodes,
        bridge: &mut HardwareBridgeT,
    ) -> Result<(), UpdateError> {
        self.update_root_nodes(nodes, root_nodes, bridge, &false)
    }

    pub fn all(
        &mut self,
        nodes: &mut Nodes,
        root_nodes: &RootNodes,
        bridge: &mut HardwareBridgeT,
    ) -> Result<(), UpdateError> {
        let _v = vec![];
        for node in nodes.values_mut() {
            if node.node_type.is_sensor() {
                let _ = node.update(&_v, bridge, &false);
            };
        }

        self.update_root_nodes(nodes, root_nodes, bridge, &true)
    }

    pub fn clear_cache(&mut self) {}

    fn update_root_nodes(
        &mut self,
        nodes: &mut Nodes,
        root_nodes: &RootNodes,
        bridge: &mut HardwareBridgeT,
        skip_sensors: &bool,
    ) -> Result<(), UpdateError> {
        let mut updated: HashSet<Id> = HashSet::new();

        if self.config_changed {
            for node_id in root_nodes {
                let value = Self::update_rec(nodes, node_id, &mut updated, bridge, skip_sensors)?;

                match value {
                    Some(_) => {}
                    None => {
                        let Some(node) = nodes.get_mut(node_id) else {
                            return Err(UpdateError::NodeNotFound);
                        };
                        if let NodeType::Control(control) = &mut node.node_type {
                            control.set_mode(false, bridge)?;
                        }
                    }
                }
            }
            self.config_changed = false;
        } else {
            for node_id in root_nodes {
                Self::update_rec(nodes, node_id, &mut updated, bridge, skip_sensors)?;
            }
        }

        Ok(())
    }

    fn update_rec(
        nodes: &mut Nodes,
        node_id: &Id,
        updated: &mut HashSet<Id>,
        bridge: &mut HardwareBridgeT,
        skip_sensors: &bool,
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
                node.value = None;
                return Ok(None);
            }
            input_ids = node.inputs.iter().map(|i| i.0).collect();
        }

        let mut input_values = Vec::new();
        for id in &input_ids {
            match Self::update_rec(nodes, id, updated, bridge, skip_sensors)? {
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

        node.update(&input_values, bridge, skip_sensors)?;

        Ok(node.value)
    }
}

impl Node {
    pub fn update(
        &mut self,
        input_values: &Vec<Value>,
        bridge: &mut HardwareBridgeT,
        skip_sensors: &bool,
    ) -> Result<(), UpdateError> {
        if *skip_sensors && self.node_type.is_sensor() {
            return Ok(());
        }

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
