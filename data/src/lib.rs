#[macro_use]
extern crate log;

#[macro_use]
pub mod localize;

pub mod app_graph;
pub mod config;
pub mod dir_manager;
pub mod id;
pub mod node;
pub mod settings;
pub mod update;
pub mod utils;

use crate::app_graph::AppGraph;
use hardware::HardwareBridge;
use update::Update;

use crate::dir_manager::DirManager;

pub struct AppState<H: HardwareBridge> {
    pub dir_manager: DirManager,
    pub bridge: H,
    pub app_graph: AppGraph,
    pub update: Update,
}
