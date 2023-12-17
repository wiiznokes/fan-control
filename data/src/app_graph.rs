use std::collections::HashMap;

use hardware::Hardware;

use crate::config::Config;
use crate::config::{control::Control, fan::Fan, temp::Temp};

use crate::id::{Id, IdGenerator};
use crate::node::{Node, NodeType, NodeTypeLight, ToNode, self};
use crate::utils::RemoveElem;

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
            nodes: Nodes::new(),
            id_generator: IdGenerator::new(),
            root_nodes: Vec::new(),
        }
    }

    pub fn insert_node(&mut self, node: Node) {
        if node.is_root() {
            self.root_nodes.push(node.id);
        }
        self.nodes.insert(node.id, node);
    }

    pub fn remove_node(&mut self, id: Id) -> Option<Node> {
        let node = self.nodes.remove(&id);
        if let Some(node) = &node {
            if node.is_root() {
                self.root_nodes.remove_elem(|e| e == &id);
            }
        }
        node
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
                &app_graph.nodes,
            );
            app_graph.insert_node(node);
        }

        for fan_h in &hardware.fans {
            let fan = Fan {
                name: fan_h.name.clone(),
                hardware_id: Some(fan_h.hardware_id.clone()),
                fan_h: Some(fan_h.clone()),
            };

            let node = Node::new(
                &mut app_graph.id_generator,
                NodeType::Fan(fan),
                &app_graph.nodes,
            );
            app_graph.insert_node(node);
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
                &app_graph.nodes,
            );
            app_graph.insert_node(node);
        }

        app_graph
    }

    pub fn from_config(config: Config, hardware: &Hardware) -> Self {
        let mut app_graph = AppGraph::new();

        // order: fan -> temp -> custom_temp -> behavior -> control

        for fan in config.fans {
            let node = fan.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.insert_node(node);
        }

        for temp in config.temps {
            let node = temp.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.insert_node(node);
        }

        for custom_temp in config.custom_temps {
            let node = custom_temp.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.insert_node(node);
        }

        for flat in config.flats {
            let node = flat.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.insert_node(node);
        }

        for linear in config.linears {
            let node = linear.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.insert_node(node);
        }

        for target in config.targets {
            let node = target.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.insert_node(node);
        }

        for control in config.controls {
            let node = control.to_node(&mut app_graph.id_generator, &app_graph.nodes, hardware);
            app_graph.insert_node(node);
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

    pub fn create_new_node(&mut self, node_type_light: NodeTypeLight) -> Node {
        let mut node_type = match node_type_light {
            NodeTypeLight::Control => NodeType::Control(Default::default()),
            NodeTypeLight::Fan => NodeType::Fan(Default::default()),
            NodeTypeLight::Temp => NodeType::Temp(Default::default()),
            NodeTypeLight::CustomTemp => NodeType::CustomTemp(Default::default()),
            NodeTypeLight::Graph => NodeType::Graph(Default::default()),
            NodeTypeLight::Flat => NodeType::Flat(Default::default()),
            NodeTypeLight::Linear => NodeType::Linear(Default::default()),
            NodeTypeLight::Target => NodeType::Target(Default::default()),
        };

        let new_name = self.generate_default_name(node_type_light);
        node_type.set_name(&new_name);

        Node::new(&mut self.id_generator, node_type, &self.nodes)
    }

    pub fn sanitize_inputs(&mut self, log: bool) {
        let mut sanitizes = Vec::new();
        for node in self.nodes.values() {
            sanitizes.push(node::sanitize_inputs(node, &self.nodes, log));
        }

        for inputs in sanitizes {
            let node = self.nodes.get_mut(&inputs.id).unwrap();
            node.set_inputs(inputs);
        }
    }

    pub fn get(&self, k: &Id) -> & Node {
        self.nodes.get(k).expect(&format!("can't find {} in nodes", k))
    }

    pub fn get_mut(&mut self, k: &Id) -> &mut Node {
        self.nodes.get_mut(k).expect(&format!("can't find {} in nodes", k))
    }
}
