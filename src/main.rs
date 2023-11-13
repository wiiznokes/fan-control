use clap::Parser;
use data::{
    cli::Args, config::Config, directories::DirManager, node::AppGraph, update::Update, AppState,
};
use hardware::{self, HardwareBridge};

use ui::run_ui;

#[macro_use]
extern crate log;

#[cfg(all(test, feature = "fake_hardware"))]
mod integrated_test;

fn main() {
    env_logger::init();

    let args = Args::parse();

    let dir_manager = DirManager::new(args.config_dir_path);
    let settings = dir_manager.init_settings();

    #[cfg(feature = "fake_hardware")]
    let hardware = hardware::fake_hardware::FakeHardwareBridge::generate_hardware();

    #[cfg(all(not(feature = "fake_hardware"), target_os = "linux"))]
    let hardware = hardware::linux::LinuxBridge::generate_hardware();

    #[cfg(all(not(feature = "fake_hardware"), target_os = "windows"))]
    let hardware = hardware::windows::WindowsBridge::generate_hardware();

    let hardware_file_path = dir_manager.hardware_file_path();

    if let Err(e) = DirManager::serialize(&hardware_file_path, &hardware) {
        warn!("{}", e);
    }

    let config = match &settings.current_config {
        Some(config_name) => {
            DirManager::deserialize::<Config>(&dir_manager.config_file_path(config_name), true)
        }
        None => None,
    };

    let app_graph = match config {
        Some(config) => AppGraph::from_config(config, &hardware),
        None => AppGraph::default(&hardware),
    };

    let app_state = AppState {
        dir_manager,
        settings,
        hardware,
        app_graph,
        update: Update::new(),
    };

    run_ui(app_state).unwrap();
}
