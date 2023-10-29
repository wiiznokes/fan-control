//#![feature(return_position_impl_trait_in_trait)]
#![allow(dead_code)]

pub mod cli;
pub mod directories;
pub mod items;
pub mod node;
pub mod settings;

#[cfg(test)]
mod serde_test;

pub mod id {

    pub type Id = u32;

    pub struct IdGenerator {
        prec_id: Id,
    }

    impl Default for IdGenerator {
        fn default() -> Self {
            Self::new()
        }
    }

    impl IdGenerator {
        pub fn new() -> Self {
            Self { prec_id: 0 }
        }

        pub fn new_id(&mut self) -> Id {
            self.prec_id += 1;

            self.prec_id
        }
    }
}

pub mod config {

    use crate::{
        items::{Control, CustomTemp, Fan, Flat, Graph, IntoNode, Linear, Target, Temp},
        node::AppGraph,
    };
    use hardware::Hardware;
    use serde::{Deserialize, Serialize};

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
}
