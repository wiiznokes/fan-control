use std::collections::HashMap;

use hardware::Hardware;

use crate::config::linear::Linear;
use crate::config::target::Target;
use crate::config::Config;
use crate::config::{control::Control, fan::Fan, temp::Temp};

use crate::id::{Id, IdGenerator};
use crate::node::{Node, NodeType, NodeTypeLight, ToNode};

pub type Nodes = HashMap<Id, Node>;
pub type RootNodes = Vec<Id>;

#[derive(Debug)]
pub struct AppGraph {
    pub nodes: Nodes,
    pub id_generator: IdGenerator,
    pub root_nodes: RootNodes,
}

impl AppGraph {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            id_generator: IdGenerator::new(),
            root_nodes: Vec::new(),
        }
    }

    pub fn default(hardware: &Hardware) -> Self {
        let mut app_graph = AppGraph::new();

        for control_h in &hardware.controls {
            let control = Control::new(
                control_h.name.clone(),
                Some(control_h.hardware_id.clone()),
                None,
                true,
                Some(control_h.clone()),
            );

            let node = Node::new(
                &mut app_graph.id_generator,
                NodeType::Control(control),
                Vec::new(),
            );
            app_graph.root_nodes.push(node.id);
            app_graph.nodes.insert(node.id, node);
        }

        for fan_h in &hardware.fans {
            let fan = Fan {
                name: fan_h.name.clone(),
                hardware_id: Some(fan_h.hardware_id.clone()),
                fan_h: Some(fan_h.clone()),
            };

            let node = Node::new(&mut app_graph.id_generator, NodeType::Fan(fan), Vec::new());
            app_graph.nodes.insert(node.id, node);
        }

        for temp_h in &hardware.temps {
            let temp = Temp {
                name: temp_h.name.clone(),
                hardware_id: Some(temp_h.hardware_id.clone()),
                temp_h: Some(temp_h.clone()),
            };

            let node = Node::new(
                &mut app_graph.id_generator,
                NodeType::Temp(temp),
                Vec::new(),
            );
            app_graph.nodes.insert(node.id, node);
        }

        app_graph
    }

    pub fn from_config(config: Config, hardware: &Hardware) -> Self {
        let mut app_graph = AppGraph::new();

        // order: fan -> temp -> custom_temp -> behavior -> control

        for fan in config.fans {
            let node = fan.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.nodes.insert(node.id, node);
        }

        for temp in config.temps {
            let node = temp.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.nodes.insert(node.id, node);
        }

        for custom_temp in config.custom_temps {
            let node = custom_temp.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.nodes.insert(node.id, node);
        }

        for flat in config.flats {
            let node = flat.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.nodes.insert(node.id, node);
        }

        for linear in config.linears {
            let node = linear.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.nodes.insert(node.id, node);
        }

        for target in config.targets {
            let node = target.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.nodes.insert(node.id, node);
        }

        for control in config.controls {
            let node = control.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.root_nodes.push(node.id);
            app_graph.nodes.insert(node.id, node);
        }

        app_graph
    }

    pub fn generate_default_name(&self, node_type: NodeTypeLight) -> String {
        let default_name = match node_type {
            NodeTypeLight::Control => fl!("default_control"),
            NodeTypeLight::Fan => fl!("default_fan"),
            NodeTypeLight::Temp => fl!("default_temp"),
            NodeTypeLight::CustomTemp => fl!("default_custom_temp"),
            NodeTypeLight::Graph => fl!("default_graph"),
            NodeTypeLight::Flat => fl!("default_flat"),
            NodeTypeLight::Linear => fl!("default_linear"),
            NodeTypeLight::Target => fl!("default_target"),
        };

        fn find_unused_name(nodes: &Nodes, default_name: &str, i: u32) -> String {
            let new_name = format!("{} {}", default_name, i);
            if nodes.values().any(|n| n.name() == &new_name) {
                find_unused_name(nodes, default_name, i + 1)
            } else {
                new_name
            }
        }

        find_unused_name(&self.nodes, &default_name, 1)
    }

    pub fn add_new_node(&mut self, node_type_light: NodeTypeLight) {
        let mut node_type = match node_type_light {
            NodeTypeLight::Control => NodeType::Control(Default::default()),
            NodeTypeLight::Fan => NodeType::Fan(Default::default()),
            NodeTypeLight::Temp => NodeType::Temp(Default::default()),
            NodeTypeLight::CustomTemp => NodeType::CustomTemp(Default::default()),
            NodeTypeLight::Graph => NodeType::Graph(Default::default()),
            NodeTypeLight::Flat => NodeType::Flat(Default::default()),
            NodeTypeLight::Linear => {
                let linear = Linear::default();
                let cache = linear.cache();
                NodeType::Linear(linear, cache)
            }
            NodeTypeLight::Target => {
                let target = Target::default();
                let cache = target.cache();
                NodeType::Target(target, cache)
            }
        };

        let new_name = self.generate_default_name(node_type_light);
        node_type.set_name(&new_name);

        let node = Node::new(&mut self.id_generator, node_type, Vec::new());
        self.nodes.insert(node.id, node);
    }
}
