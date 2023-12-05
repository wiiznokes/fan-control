//#![feature(return_position_impl_trait_in_trait)]
//#![feature(type_alias_impl_trait)]
#![allow(dead_code)]
//#![allow(clippy::match_like_matches_macro)]

#[macro_use]
extern crate log;

pub mod cli;
pub mod config;
pub mod directories;
pub mod id;
#[macro_use]
pub mod localize;
pub mod app_graph;
mod name_sorter;
pub mod node;
pub mod serde_helper;
pub mod settings;
pub mod update;
pub mod utils;

use crate::app_graph::AppGraph;
use hardware::{Hardware, HardwareBridge, HardwareBridgeT};
use update::Update;

use crate::directories::DirManager;

pub type BoxedHardwareBridge = Box<dyn HardwareBridge>;

pub struct AppState {
    pub dir_manager: DirManager,
    pub hardware: Hardware,
    pub bridge: HardwareBridgeT,
    pub app_graph: AppGraph,
    pub update: Update,
}
