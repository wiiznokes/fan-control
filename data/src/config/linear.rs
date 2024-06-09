use crate::{
    app_graph::AppGraph,
    node::{IsValid, Node, NodeType, ToNode},
    update::UpdateError,
};
use hardware::{Hardware, Value};
use serde::{Deserialize, Serialize};

use super::utils::affine::Affine;

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
        self.input.is_some()
    }
}

impl Linear {
    pub fn get_value(&self, value: Value) -> Result<Value, UpdateError> {
        if value <= self.min_temp.into() {
            return Ok(self.min_speed.into());
        }

        if value >= self.max_temp.into() {
            return Ok(self.max_speed.into());
        }

        let res = Affine {
            xa: self.min_temp.into(),
            ya: self.min_speed.into(),
            xb: self.max_temp.into(),
            yb: self.max_speed.into(),
        }
        .calcule(value) as Value;

        Ok(res)
    }
}

impl ToNode for Linear {
    fn to_node(mut self, app_graph: &mut AppGraph, _hardware: &Hardware) -> Node {
        let default = Self::default();

        if self.max_temp < self.min_temp {
            self.min_temp = default.min_temp;
            self.max_temp = default.max_temp;
        }

        if self.max_speed < self.min_speed {
            self.min_speed = default.min_speed;
            self.max_speed = default.max_speed;
        }

        if self.min_temp > 100 {
            self.min_temp = default.min_temp;
        }
        if self.min_speed > 100 {
            self.min_speed = default.min_speed;
        }
        if self.max_temp > 100 {
            self.max_temp = default.max_temp;
        }
        if self.max_speed > 100 {
            self.max_speed = default.max_speed;
        }

        Node::new(NodeType::Linear(self), app_graph)
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
    use crate::utils::init_test_logging;

    use super::Linear;

    #[test]
    fn test_update() {
        init_test_logging();

        let linear = Linear {
            name: "Linear".into(),
            min_temp: 10,
            min_speed: 10,
            max_temp: 70,
            max_speed: 100,
            input: Some("temp1".into()),
        };

        assert!(linear.get_value(9).unwrap() == 10);
        assert!(linear.get_value(70).unwrap() == 100);
        assert!(linear.get_value(40).unwrap() == 55);
    }
}
