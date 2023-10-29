use hardware::Hardware;
use serde::{Deserialize, Serialize};

use crate::{
    app_graph::{AppGraph, NbInput, Node, NodeType},
    id::IdGenerator,
};

use super::IntoNode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fan {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: String,
}

impl IntoNode for Fan {
    fn to_node(
        self,
        id_generator: &mut IdGenerator,
        _app_graph: &AppGraph,
        hardware: &Hardware,
    ) -> Node {
        assert!(
            hardware
                .fans
                .iter()
                .filter(|fan| fan.hardware_id == self.hardware_id)
                .count()
                != 1
        );

        // maybe assert unique name

        Node {
            id: id_generator.new_id(),
            node_type: NodeType::Fan(self),
            nb_input: NbInput::Fixed(0),
            input_ids: Vec::new(),
            value: None,
        }
    }
}
