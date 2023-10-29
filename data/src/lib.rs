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

    pub struct Id {
        prec_id: u32,
    }

    impl Id {
        pub fn new_id(&mut self) -> u32 {
            self.prec_id += 1;

            self.prec_id
        }
    }
}

pub mod config {
    use crate::items::{Control, CustomTemp, Fan, Flat, Graph, Linear, Target, Temp};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Config {
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

    #[derive(Serialize, Deserialize, Debug, Clone, Default)]
    pub struct Hardware {
        #[serde(default, rename = "Control")]
        pub controls: Vec<Control>,
        #[serde(default, rename = "Fan")]
        pub fans: Vec<Fan>,
        #[serde(default, rename = "Temp")]
        pub temps: Vec<Temp>,
    }
}
