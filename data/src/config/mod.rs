pub mod control;
pub mod custom_temp;
pub mod fan;
pub mod flat;
pub mod graph;
pub mod linear;
pub mod target;
pub mod temp;

//#[cfg(test)]
//mod serde_test;

use crate::{
    app_graph::{self, AppGraph, NbInput, NodeTypeLight, Nodes},
    config::{
        control::Control, custom_temp::CustomTemp, fan::Fan, flat::Flat, graph::Graph,
        linear::Linear, target::Target, temp::Temp,
    },
    id::Id,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    #[serde(default, rename = "Control")]
    pub controls: Vec<Control>,
    #[serde(default, rename = "Fan")]
    pub fans: Vec<Fan>,
    #[serde(default, rename = "Temp")]
    pub temps: Vec<Temp>,
    #[serde(default, rename = "CustomTemp")]
    pub custom_temps: Vec<CustomTemp>,
    #[serde(default, rename = "Graph")]
    pub graphs: Vec<Graph>,
    #[serde(default, rename = "Flat")]
    pub flats: Vec<Flat>,
    #[serde(default, rename = "Linear")]
    pub linears: Vec<Linear>,
    #[serde(default, rename = "Target")]
    pub targets: Vec<Target>,
}

impl Config {
    pub fn from_app_graph(app_graph: &AppGraph) -> Self {
        let mut config = Config::default();
        for node in app_graph.nodes.values() {
            match &node.node_type {
                app_graph::NodeType::Control(control) => config.controls.push(control.clone()),
                app_graph::NodeType::Fan(fan) => config.fans.push(fan.clone()),
                app_graph::NodeType::Temp(temp) => config.temps.push(temp.clone()),
                app_graph::NodeType::CustomTemp(custom_temp) => {
                    config.custom_temps.push(custom_temp.clone())
                }
                app_graph::NodeType::Graph(graph) => config.graphs.push(graph.clone()),
                app_graph::NodeType::Flat(flat) => config.flats.push(flat.clone()),
                app_graph::NodeType::Linear(linear) => config.linears.push(linear.clone()),
                app_graph::NodeType::Target(target) => config.targets.push(target.clone()),
            }
        }
        config
    }
}

pub trait IsValid {
    fn is_valid(&self) -> bool;
}

pub trait Inputs {
    fn clear_inputs(&mut self);
    fn get_inputs(&self) -> Vec<&String>;
}

pub fn sanitize_inputs(
    item: &mut impl Inputs,
    nodes: &Nodes,
    max_input: NbInput,
    allowed_dep: &[NodeTypeLight],
) -> Vec<Id> {
    let mut inputs = Vec::new();

    match max_input {
        NbInput::Zero => {
            if !item.get_inputs().is_empty() {
                item.clear_inputs();
            };
            return inputs;
        }
        NbInput::One => {
            if !item.get_inputs().len() > 1 {
                item.clear_inputs();
                return inputs;
            }
        }
        _ => {}
    };

    for name in item.get_inputs() {
        if let Some(node) = nodes.values().find(|node| node.name() == name) {
            if !allowed_dep.contains(&node.node_type.to_light()) {
                eprintln!(
                    "sanitize_inputs: incompatible node type. {:?} <- {}. Fall back: remove all",
                    node.node_type.to_light(),
                    name
                );
                item.clear_inputs();
                inputs.clear();
                return inputs;
            }
            inputs.push(node.id)
        } else {
            eprintln!(
                "sanitize_inputs: can't find {} in app_graph. Fall back: remove all",
                name
            );
            item.clear_inputs();
            inputs.clear();
            return inputs;
        }
    }

    if max_input == NbInput::One && !inputs.len() > 1 {
        item.clear_inputs();
        inputs.clear();
        return inputs;
    }
    inputs
}
