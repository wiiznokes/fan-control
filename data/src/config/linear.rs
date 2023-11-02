use crate::{
    id::IdGenerator,
    node::{sanitize_inputs, Inputs, IsValid, Node, NodeType, NodeTypeLight, Nodes, ToNode},
    update::UpdateError,
};
use hardware::{Hardware, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Linear {
    pub name: String,
    #[serde(rename = "minTemp", alias = "min_temp")]
    pub min_temp: u8,
    #[serde(rename = "minSpeed", alias = "min_speed")]
    pub min_speed: u8,
    #[serde(rename = "maxTemp", alias = "max_temp")]
    pub max_temp: u8,
    #[serde(rename = "maxSpeed", alias = "max_speed")]
    pub max_speed: u8,
    pub input: Option<String>,
}

impl IsValid for Linear {
    fn is_valid(&self) -> bool {
        self.input.is_some() && self.max_temp > self.min_temp && self.max_speed > self.min_speed
    }
}

impl Inputs for Linear {
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

struct Affine {
    a: Value,
    b: Value,
}

impl Linear {
    pub fn update(&self, value: Value) -> Result<Value, UpdateError> {
        if value <= self.min_temp.into() {
            return Ok(self.min_speed.into());
        }

        if value >= self.max_temp.into() {
            return Ok(self.max_speed.into());
        }

        let affine = self.calcule_affine();

        Ok(affine.a * value + affine.b)
    }

    fn calcule_affine(&self) -> Affine {
        let a = (self.max_speed - self.min_speed) / (self.max_temp - self.min_temp);

        Affine {
            a: a.into(),
            b: (self.min_speed - a * self.min_temp).into(),
        }
    }
}

impl ToNode for Linear {
    fn to_node(
        mut self,
        id_generator: &mut IdGenerator,
        nodes: &Nodes,
        _hardware: &Hardware,
    ) -> Node {
        let inputs = sanitize_inputs(&mut self, nodes, NodeTypeLight::Linear);
        Node::new(id_generator, NodeType::Linear(self), inputs)
    }
}
