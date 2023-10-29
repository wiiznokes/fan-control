pub mod control;
pub mod custom_temp;
pub mod fan;
pub mod flat;
pub mod graph;
pub mod linear;
pub mod target;
pub mod temp;

#[cfg(test)]
mod serde_test;

use crate::config::{
    control::Control, custom_temp::CustomTemp, fan::Fan, flat::Flat, graph::Graph, linear::Linear,
    target::Target, temp::Temp,
};
use hardware::Hardware;
use serde::{Deserialize, Serialize};

use crate::app_graph::{AppGraph, Node};

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub fn to_app_graph(self, hardware: &Hardware) -> AppGraph {
        let mut nodes = AppGraph::new();

        for fan in self.fans {
            let node = fan.to_node(&mut nodes, hardware);
            nodes.nodes.insert(node.id, node);
        }

        nodes
    }
}

pub trait IntoNode {
    fn to_node(self, app_graph: &mut AppGraph, hardware: &Hardware) -> Node;
}

pub trait Update {
    //fn update(&self, &)
}
