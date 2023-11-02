//#![feature(return_position_impl_trait_in_trait)]
#![allow(dead_code)]

pub mod cli;
pub mod config;
pub mod directories;
pub mod id;
pub mod node;
pub mod settings;
pub mod update;

use hardware::{Hardware, HardwareBridge};
use node::AppGraph;
use update::Update;

use crate::{directories::DirManager, settings::Settings};

pub type BoxedHardwareBridge = Box<dyn HardwareBridge>;

pub struct AppState {
    pub dir_manager: DirManager,
    pub settings: Settings,
    pub hardware: Hardware,
    pub app_graph: AppGraph,
    pub update: Update,
}
