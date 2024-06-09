use crate::{
    app_graph::AppGraph,
    node::{IsValid, Node, NodeType, ToNode},
    update::UpdateError,
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
    pub fn get_value(&mut self, value: Value) -> Result<Value, UpdateError> {
        if self.idle_has_been_reatch {
            if value < self.load_temp.into() {
                return Ok(self.idle_speed.into());
            }

            self.idle_has_been_reatch = false;
            return Ok(self.load_speed.into());
        }

        if value > self.idle_temp.into() {
            return Ok(self.load_speed.into());
        }

        self.idle_has_been_reatch = true;
        Ok(self.idle_speed.into())
    }
}

impl IsValid for Target {
    fn is_valid(&self) -> bool {
        self.input.is_some()
    }
}

impl ToNode for Target {
    fn to_node(mut self, app_graph: &mut AppGraph, _hardware: &Hardware) -> Node {
        let default = Self::default();

        if self.idle_temp > 100 {
            self.idle_temp = default.idle_temp;
        }
        if self.idle_speed > 100 {
            self.idle_speed = default.idle_speed;
        }
        if self.load_temp > 100 {
            self.load_temp = default.load_temp;
        }
        if self.load_speed > 100 {
            self.load_speed = default.load_speed;
        }

        Node::new(NodeType::Target(self), app_graph)
    }
}

impl Default for Target {
    fn default() -> Self {
        Self {
            name: Default::default(),
            idle_temp: 40,
            idle_speed: 10,
            load_temp: 70,
            load_speed: 100,
            input: Default::default(),
            idle_has_been_reatch: false,
        }
    }
}

#[cfg(test)]
mod test {

    use crate::utils::init_test_logging;

    use super::Target;

    #[test]
    fn test_update() {
        init_test_logging();

        let mut target = Target {
            name: "linear".to_string(),
            input: Some("temp1".into()),
            idle_temp: 40,
            idle_speed: 10,
            load_temp: 70,
            load_speed: 100,
            idle_has_been_reatch: false,
        };

        assert!(target.get_value(55).unwrap() == 100);
        assert!(target.get_value(30).unwrap() == 10);
        assert!(target.get_value(55).unwrap() == 10);
        assert!(target.get_value(70).unwrap() == 100);
    }
}
