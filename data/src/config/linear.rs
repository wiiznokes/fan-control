use crate::{
    app_graph::Nodes,
    id::IdGenerator,
    node::{sanitize_inputs, IsValid, Node, NodeType, ToNode},
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

#[derive(Debug, Clone)]
pub struct LinearCache {
    pub min_temp: String,
    pub min_speed: String,
    pub max_temp: String,
    pub max_speed: String,
}

impl IsValid for Linear {
    fn is_valid(&self) -> bool {
        self.input.is_some() && self.max_temp > self.min_temp && self.max_speed > self.min_speed
    }
}

#[derive(Debug)]
struct Affine {
    a: f32,
    b: f32,
}

impl Linear {
    pub fn cache(&self) -> LinearCache {
        LinearCache {
            min_temp: self.min_temp.to_string(),
            min_speed: self.min_speed.to_string(),
            max_temp: self.max_temp.to_string(),
            max_speed: self.max_speed.to_string(),
        }
    }

    pub fn update(&self, value: Value) -> Result<Value, UpdateError> {
        if value <= self.min_temp.into() {
            return Ok(self.min_speed.into());
        }

        if value >= self.max_temp.into() {
            return Ok(self.max_speed.into());
        }

        let affine = self.calcule_affine();

        let final_value: f32 = affine.a * value as f32 + affine.b;
        Ok(final_value as Value)
    }

    fn calcule_affine(&self) -> Affine {
        let xa: f32 = self.min_temp.into();
        let ya: f32 = self.min_speed.into();
        let xb: f32 = self.max_temp.into();
        let yb: f32 = self.max_speed.into();

        let a = (yb - ya) / (xb - xa);

        Affine { a, b: ya - a * xa }
    }
}

impl ToNode for Linear {
    fn to_node(self, id_generator: &mut IdGenerator, nodes: &Nodes, _hardware: &Hardware) -> Node {
        let cache = self.cache();
        sanitize_inputs(
            Node::new(id_generator, NodeType::Linear(self, cache), Vec::new()),
            nodes,
        )
    }
}

impl Default for Linear {
    fn default() -> Self {
        Self {
            name: Default::default(),
            min_temp: 10,
            min_speed: 10,
            max_temp: 70,
            max_speed: 100,
            input: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Linear;

    #[test]
    fn test_update() {
        let _ = env_logger::try_init();

        let linear = Linear {
            name: "Linear".into(),
            min_temp: 10,
            min_speed: 10,
            max_temp: 70,
            max_speed: 100,
            input: Some("temp1".into()),
        };

        assert!(linear.update(9).unwrap() == 10);
        assert!(linear.update(70).unwrap() == 100);
        assert!(linear.update(40).unwrap() == 55);
    }
}
