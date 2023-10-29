//#![feature(return_position_impl_trait_in_trait)]
#![allow(dead_code)]

pub mod app_graph;
pub mod cli;
pub mod config;
pub mod directories;
pub mod id;
pub mod settings;

use app_graph::AppGraph;
use hardware::{Hardware, HardwareBridge};

use crate::{directories::DirManager, settings::Settings};

pub type BoxedHardwareBridge = Box<dyn HardwareBridge>;

pub struct AppState {
    pub dir_manager: DirManager,
    pub settings: Settings,
    pub hardware_bridge: BoxedHardwareBridge,
    pub hardware: Hardware,
    pub app_graph: AppGraph,
}
