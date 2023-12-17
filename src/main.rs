use clap::Parser;
use data::{app_graph::AppGraph, args::Args, dir_manager::DirManager, update::Update, AppState};
use hardware::{self, HardwareBridge};
use thiserror::Error;

#[allow(unused_imports)]
#[macro_use]
extern crate log;

#[cfg(all(test, feature = "fake_hardware"))]
mod integrated_test;

mod cli;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Hardware(#[from] hardware::HardwareError),
}

pub type Result<T> = std::result::Result<T, Error>;

fn start() -> Result<()> {
    env_logger::init();
    ui::localize::localize();
    data::localize::localize();

    let args = Args::parse();

    let dir_manager = DirManager::new(&args);

    #[cfg(feature = "fake_hardware")]
    let (hardware, bridge) = hardware::fake_hardware::FakeHardwareBridge::generate_hardware()?;

    #[cfg(all(not(feature = "fake_hardware"), target_os = "linux"))]
    let (hardware, bridge) = hardware::linux::LinuxBridge::generate_hardware();

    #[cfg(all(not(feature = "fake_hardware"), target_os = "windows"))]
    let (hardware, bridge) = hardware::windows::WindowsBridge::generate_hardware()?;

    dir_manager.serialize_hardware(&hardware);

    let app_graph = match dir_manager.get_config() {
        Some(config) => AppGraph::from_config(config, &hardware),
        None => AppGraph::default(&hardware),
    };

    let app_state = AppState {
        dir_manager,
        hardware,
        bridge,
        app_graph,
        update: Update::new(),
    };

    match args.cli {
        true => cli::run_cli(app_state),
        false => ui::run_ui(app_state).unwrap(),
    };

    Ok(())
}

fn main() {
    // todo
    start().unwrap()
}
