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

use crate::{
    app_graph::{self, AppGraph},
    config::{
        control::Control, custom_temp::CustomTemp, fan::Fan, flat::Flat, graph::Graph,
        linear::Linear, target::Target, temp::Temp,
    },
};

use hardware::{Hardware, HardwareType};
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


pub trait HardwareId {
    
    fn hardware_id(&self) -> &Option<String>;
    fn hardware_id_mut(&mut self) -> &mut Option<String>;


    fn internal_index(&self) -> &Option<usize>;
    fn internal_index_mut(&mut self) -> &mut Option<usize>;

}

pub fn verif_hardware_id(
    node: &mut impl HardwareId,
    hardware: &Hardware,
    hardware_type: HardwareType,
) {
    let hardware_id_ref = node.hardware_id_mut();
    let internal_index_ref = node.internal_index_mut();

    match node.hardware_id() {
        Some(ref hardware_id) => {
            match hardware.get_internal_index(hardware_id, hardware_type) {
                Some(index) => {
                    let mut index_ref = node.internal_index_mut();
                    index_ref = &mut Some(index);
                },
                None => {
                    eprintln!(
                        "hardware {} from config not found. Fall back to no id",
                        hardware_id
                    );
                    let mut hardware_id_ref = node.hardware_id_mut();
                    hardware_id_ref = &mut None
                }
            }
        }
        None => {
            if node.internal_index().is_some() {
                eprintln!(
                    "Control to Node: Inconsistent internal index found: "
                );
                let mut index_ref = node.internal_index_mut();
                index_ref = &mut None;
            }
        }
    }
}