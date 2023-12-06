// todo: make a macro for this pattern

use crate::app_graph::AppGraph;
use crate::config::custom_temp::CustomTemp;
use crate::config::flat::Flat;
use crate::config::linear::Linear;
use crate::config::target::Target;

use crate::config::{control::Control, fan::Fan, temp::Temp};

use crate::id::Id;
use crate::node::{Node, NodeType};

impl AppGraph {
    pub fn get_control(&self, id: Id) -> &Control {
        self.nodes.get(&id).expect("node not found").to_control()
    }
    pub fn get_temp(&self, id: Id) -> &Temp {
        self.nodes.get(&id).expect("node not found").to_temp()
    }
    pub fn get_fan(&self, id: Id) -> &Fan {
        self.nodes.get(&id).expect("node not found").to_fan()
    }

    pub fn get_custom_temp(&self, id: Id) -> &CustomTemp {
        self.nodes
            .get(&id)
            .expect("node not found")
            .to_custom_temp()
    }

    pub fn get_flat(&self, id: Id) -> &Flat {
        self.nodes.get(&id).expect("node not found").to_flat()
    }

    pub fn get_linear(&self, id: Id) -> &Linear {
        self.nodes.get(&id).expect("node not found").to_linear()
    }

    pub fn get_target(&self, id: Id) -> &Target {
        self.nodes.get(&id).expect("node not found").to_target()
    }

    pub fn get_control_mut(&mut self, id: Id) -> &mut Control {
        self.nodes
            .get_mut(&id)
            .expect("node not found")
            .to_control_mut()
    }
    pub fn get_custom_temp_mut(&mut self, id: Id) -> &mut CustomTemp {
        self.nodes
            .get_mut(&id)
            .expect("node not found")
            .to_custom_temp_mut()
    }

    pub fn get_temp_mut(&mut self, id: Id) -> &mut Temp {
        self.nodes
            .get_mut(&id)
            .expect("node not found")
            .to_temp_mut()
    }
    pub fn get_fan_mut(&mut self, id: Id) -> &mut Fan {
        self.nodes
            .get_mut(&id)
            .expect("node not found")
            .to_fan_mut()
    }

    pub fn get_flat_mut(&mut self, id: Id) -> &mut Flat {
        self.nodes
            .get_mut(&id)
            .expect("node not found")
            .to_flat_mut()
    }

    pub fn get_linear_mut(&mut self, id: Id) -> &mut Linear {
        self.nodes
            .get_mut(&id)
            .expect("node not found")
            .to_linear_mut()
    }

    pub fn get_target_mut(&mut self, id: Id) -> &mut Target {
        self.nodes
            .get_mut(&id)
            .expect("node not found")
            .to_target_mut()
    }
}

impl Node {
    pub fn to_control_mut(&mut self) -> &mut Control {
        let NodeType::Control(item) = &mut self.node_type else {
            panic!("node found but not the good type");
        };
        item
    }
    pub fn to_custom_temp_mut(&mut self) -> &mut CustomTemp {
        let NodeType::CustomTemp(item) = &mut self.node_type else {
            panic!("node found but not the good type");
        };
        item
    }
    pub fn to_temp_mut(&mut self) -> &mut Temp {
        let NodeType::Temp(item) = &mut self.node_type else {
            panic!("node found but not the good type");
        };
        item
    }
    pub fn to_fan_mut(&mut self) -> &mut Fan {
        let NodeType::Fan(item) = &mut self.node_type else {
            panic!("node found but not the good type");
        };
        item
    }

    pub fn to_flat_mut(&mut self) -> &mut Flat {
        let NodeType::Flat(item) = &mut self.node_type else {
            panic!("node found but not the good type");
        };
        item
    }

    pub fn to_linear_mut(&mut self) -> &mut Linear {
        let NodeType::Linear(item, ..) = &mut self.node_type else {
            panic!("node found but not the good type");
        };
        item
    }

    pub fn to_target_mut(&mut self) -> &mut Target {
        let NodeType::Target(item, ..) = &mut self.node_type else {
            panic!("node found but not the good type");
        };
        item
    }

    pub fn to_control(&self) -> &Control {
        let NodeType::Control(item) = &self.node_type else {
            panic!("node found but not the good type");
        };
        item
    }
    pub fn to_custom_temp(&self) -> &CustomTemp {
        let NodeType::CustomTemp(item) = &self.node_type else {
            panic!("node found but not the good type");
        };
        item
    }
    pub fn to_temp(&self) -> &Temp {
        let NodeType::Temp(item) = &self.node_type else {
            panic!("node found but not the good type");
        };
        item
    }
    pub fn to_fan(&self) -> &Fan {
        let NodeType::Fan(item) = &self.node_type else {
            panic!("node found but not the good type");
        };
        item
    }

    pub fn to_flat(&self) -> &Flat {
        let NodeType::Flat(item) = &self.node_type else {
            panic!("node found but not the good type");
        };
        item
    }

    pub fn to_linear(&self) -> &Linear {
        let NodeType::Linear(item, ..) = &self.node_type else {
            panic!("node found but not the good type");
        };
        item
    }

    pub fn to_target(&self) -> &Target {
        let NodeType::Target(item, ..) = &self.node_type else {
            panic!("node found but not the good type");
        };
        item
    }
}
