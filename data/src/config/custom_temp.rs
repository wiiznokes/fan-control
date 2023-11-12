use hardware::{Hardware, Value};
use light_enum::Values;
use serde::{Deserialize, Serialize};

use crate::{
    id::IdGenerator,
    node::{sanitize_inputs, IsValid, Node, NodeType, Nodes, ToNode},
    update::UpdateError,
};

#[derive(Serialize, Deserialize, Debug, Clone, Values, Default, PartialEq, Eq)]
pub enum CustomTempKind {
    #[default]
    Average,
    Min,
    Max,
}

impl ToString for CustomTempKind {
    fn to_string(&self) -> String {
        match self {
            CustomTempKind::Average => "Average".into(),
            CustomTempKind::Max => "Max".into(),
            CustomTempKind::Min => "Min".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomTemp {
    pub name: String,
    pub kind: CustomTempKind,
    pub input: Vec<String>,
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
    fn to_node(self, id_generator: &mut IdGenerator, nodes: &Nodes, _hardware: &Hardware) -> Node {
        sanitize_inputs(
            Node::new(id_generator, NodeType::CustomTemp(self), Vec::new()),
            nodes,
        )
    }
}
