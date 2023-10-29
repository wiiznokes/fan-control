use clap::Parser;
use data::{
    cli::Args,
    config::Config,
    directories::SettingsManager,
    node::{AppGraph, AppState},
};
use hardware::{self, HardwareBridge};
use ui::run_ui;

fn main() {
    let args = Args::parse();

    let settings_manager = SettingsManager::new(args.config_dir_path);
    let settings = settings_manager.init_settings();

    #[cfg(target_os = "linux")]
    let hardware_bridge = hardware::linux::LinuxBridge::new();

    #[cfg(target_os = "windows")]
    let hardware_bridge = hardware::windows::WindowsBridge::new();

    let hardware_file_path = settings_manager.hardware_file_path();

    let hardware = hardware_bridge.hardware();
    if let Err(e) = SettingsManager::serialize(&hardware_file_path, &hardware) {
        eprintln!("{}", e);
    }

    let config = match &settings.current_config {
        Some(config_name) => SettingsManager::deserialize::<Config>(
            &settings_manager.config_file_path(config_name),
            true,
        ),
        None => None,
    };

    let app_graph = match config {
        Some(config) => config.to_app_graph(&hardware),
        None => AppGraph::new(),
    };

    let app_state = AppState {
        settings_manager,
        settings,
        hardware_bridge: Box::new(hardware_bridge),
        hardware,
        app_graph,
    };

    run_ui(app_state).unwrap();
}
