use hardware::{Hardware, Value};
use light_enum::Values;
use serde::{Deserialize, Serialize};

use crate::{
    app_graph::Nodes,
    id::IdGenerator,
    node::{IsValid, Node, NodeType, ToNode},
    update::UpdateError,
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CustomTemp {
    pub name: String,
    pub kind: CustomTempKind,
    pub inputs: Vec<String>,

    #[serde(skip)]
    pub kind_expanded: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Values, Default, PartialEq, Eq)]
pub enum CustomTempKind {
    #[default]
    Average,
    Min,
    Max,
}

impl CustomTemp {
    pub fn new(name: String, kind: CustomTempKind, inputs: Vec<String>) -> Self {
        Self {
            name,
            kind,
            inputs,
            kind_expanded: false,
        }
    }

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

impl IsValid for CustomTemp {
    fn is_valid(&self) -> bool {
        !self.inputs.is_empty()
    }
}

impl ToNode for CustomTemp {
    fn to_node(self, id_generator: &mut IdGenerator, nodes: &Nodes, _hardware: &Hardware) -> Node {
        Node::new(id_generator, NodeType::CustomTemp(self), nodes)
    }
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
