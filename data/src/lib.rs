#[macro_use]
extern crate log;

#[macro_use]
pub mod localize;

pub mod app_graph;
pub mod args;
pub mod config;
pub mod dir_manager;
pub mod id;
mod name_sorter;
pub mod node;
pub mod settings;
pub mod update;
pub mod utils;

use crate::app_graph::AppGraph;
use hardware::{Hardware, HardwareBridgeT};
use update::Update;

use crate::dir_manager::DirManager;

pub struct AppState {
    pub dir_manager: DirManager,
    pub hardware: Hardware,
    pub bridge: HardwareBridgeT,
    pub app_graph: AppGraph,
    pub update: Update,
}
