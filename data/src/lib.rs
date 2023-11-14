//#![feature(return_position_impl_trait_in_trait)]
//#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

#[macro_use]
extern crate log;

pub mod cli;
pub mod config;
pub mod directories;
pub mod id;
pub mod localize;
pub mod node;
pub mod settings;
pub mod update;

use hardware::{Hardware, HardwareBridge, HardwareBridgeT};
use node::AppGraph;
use update::Update;

use crate::{directories::DirManager, settings::Settings};

pub type BoxedHardwareBridge = Box<dyn HardwareBridge>;

pub struct AppState {
    pub dir_manager: DirManager,
    pub settings: Settings,
    pub hardware: Hardware,
    pub bridge: HardwareBridgeT,
    pub app_graph: AppGraph,
    pub update: Update,
}
