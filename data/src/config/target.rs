use crate::{
    id::IdGenerator,
    node::{sanitize_inputs, Inputs, IsValid, Node, NodeType, NodeTypeLight, Nodes, ToNode},
    update::{UpdateError, UpdateResult},
};
use hardware::{Hardware, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Target {
    pub name: String,
    #[serde(rename = "idleTemp", alias = "idle_temp")]
    pub idle_temp: u8,
    #[serde(rename = "idleSpeed", alias = "idle_speed")]
    pub idle_speed: u8,
    #[serde(rename = "loadTemp", alias = "load_temp")]
    pub load_temp: u8,
    #[serde(rename = "loadSpeed", alias = "load_speed")]
    pub load_speed: u8,
    pub input: Option<String>,

    #[serde(skip)]
    pub idle_has_been_reatch: bool,
}

impl Target {
    pub fn update(&self, value: Value) -> Result<UpdateResult, UpdateError> {
        if self.idle_has_been_reatch {
            if value < self.load_temp.into() {
                return UpdateResult::without_side_effect(self.idle_speed.into()).into();
            }

            let load_reatch = |node: &mut Node| {
                if let NodeType::Target(target) = &mut node.node_type {
                    target.idle_has_been_reatch = false;
                }
            };
            return UpdateResult {
                value: self.load_speed.into(),
                side_effect: Box::new(load_reatch),
            }
            .into();
        }

        if value > self.idle_temp.into() {
            return UpdateResult::without_side_effect(self.load_speed.into()).into();
        }

        let idle_reatch = |node: &mut Node| {
            if let NodeType::Target(target) = &mut node.node_type {
                target.idle_has_been_reatch = true;
            }
        };
        UpdateResult {
            value: self.load_speed.into(),
            side_effect: Box::new(idle_reatch),
        }
        .into()
    }
}

impl IsValid for Target {
    fn is_valid(&self) -> bool {
        self.input.is_some()
    }
}

impl Inputs for Target {
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

impl ToNode for Target {
    fn to_node(
        mut self,
        id_generator: &mut IdGenerator,
        nodes: &Nodes,
        _hardware: &Hardware,
    ) -> Node {
        let inputs = sanitize_inputs(&mut self, nodes, NodeTypeLight::Target);
        Node::new(id_generator, NodeType::Target(self), inputs)
    }
}
