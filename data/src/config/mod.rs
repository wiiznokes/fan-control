pub mod control;
pub mod custom_temp;
pub mod fan;
pub mod flat;
pub mod graph;
pub mod linear;
pub mod target;
pub mod temp;

mod utils;

#[cfg(test)]
mod serde_test;

use crate::{
    app_graph::AppGraph,
    config::{
        control::Control, custom_temp::CustomTemp, fan::Fan, flat::Flat, graph::Graph,
        linear::Linear, target::Target, temp::Temp,
    },
    node::{self},
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
                node::NodeType::Control(control) => config.controls.push(control.clone()),
                node::NodeType::Fan(fan) => config.fans.push(fan.clone()),
                node::NodeType::Temp(temp) => config.temps.push(temp.clone()),
                node::NodeType::CustomTemp(custom_temp) => {
                    config.custom_temps.push(custom_temp.clone())
                }
                node::NodeType::Graph(graph) => config.graphs.push(graph.clone()),
                node::NodeType::Flat(flat) => config.flats.push(flat.clone()),
                node::NodeType::Linear(linear, ..) => config.linears.push(linear.clone()),
                node::NodeType::Target(target, ..) => config.targets.push(target.clone()),
            }
        }
        config
    }
}
