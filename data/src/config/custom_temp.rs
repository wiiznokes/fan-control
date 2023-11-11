use hardware::{Hardware, Value};
use serde::{Deserialize, Serialize};

use crate::{
    id::IdGenerator,
    node::{sanitize_inputs, Inputs, IsValid, Node, NodeType, NodeTypeLight, Nodes, ToNode},
    update::UpdateError,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CustomTempKind {
    Min,
    Max,
    Average,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomTemp {
    pub name: String,
    pub kind: CustomTempKind,
    pub input: Vec<String>,
}

impl Inputs for CustomTemp {
    fn clear_inputs(&mut self) {
        self.input.clear();
    }

    fn get_inputs(&self) -> Vec<&String> {
        let mut v = Vec::with_capacity(self.input.len());
        for input in &self.input {
            v.push(input)
        }
        v
    }
}

impl IsValid for CustomTemp {
    fn is_valid(&self) -> bool {
        !self.input.is_empty()
    }
}

impl CustomTemp {
    pub fn update(&self, values: &Vec<Value>) -> Result<Value, UpdateError> {
        if values.is_empty() {
            return Err(UpdateError::NoInputData);
        }

        let value = match self.kind {
            CustomTempKind::Min => *values.iter().min().unwrap(),
            CustomTempKind::Max => *values.iter().min().unwrap(),
            CustomTempKind::Average => values.iter().sum::<i32>() / values.len() as i32,
        };
        Ok(value)
    }
}

impl ToNode for CustomTemp {
    fn to_node(
        mut self,
        id_generator: &mut IdGenerator,
        nodes: &Nodes,
        _hardware: &Hardware,
    ) -> Node {
        let inputs = sanitize_inputs(&mut self, nodes, NodeTypeLight::CustomTemp);
        Node::new(id_generator, NodeType::CustomTemp(self), inputs)
    }
}
