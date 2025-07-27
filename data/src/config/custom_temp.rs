use hardware::{Hardware, Value};
use light_enum::Values;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::{
    app_graph::AppGraph,
    node::{IsValid, Node, NodeType, ToNode},
    update::UpdateError,
};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct CustomTemp {
    pub name: String,
    pub kind: CustomTempKind,
    pub inputs: Vec<String>,
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
        Self { name, kind, inputs }
    }

    pub fn get_value(&self, values: &[Value]) -> Result<Value, UpdateError> {
        let value = match self.kind {
            CustomTempKind::Min => match values.iter().min() {
                Some(min) => *min,
                None => return Err(UpdateError::NoInputData),
            },
            CustomTempKind::Max => match values.iter().max() {
                Some(max) => *max,
                None => return Err(UpdateError::NoInputData),
            },
            CustomTempKind::Average => {
                if values.is_empty() {
                    return Err(UpdateError::NoInputData);
                }

                values.iter().sum::<i32>() / (values.len() as i32)
            }
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
    fn to_node(self, app_graph: &mut AppGraph, _hardware: &Hardware) -> Node {
        Node::new(NodeType::CustomTemp(self), app_graph)
    }
}

impl Display for CustomTempKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CustomTempKind::Average => fl!("average"),
            CustomTempKind::Max => fl!("max"),
            CustomTempKind::Min => fl!("min"),
        };
        write!(f, "{str}")
    }
}
