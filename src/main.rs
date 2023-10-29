use clap::Parser;
use data::{
    app_graph::AppGraph, cli::Args, config::Config, directories::DirManager, id::IdGenerator,
    AppState,
};
use hardware::{self, HardwareBridge};
use ui::run_ui;

fn main() {
    let args = Args::parse();

    let dir_manager = DirManager::new(args.config_dir_path);
    let settings = dir_manager.init_settings();

    #[cfg(target_os = "linux")]
    let hardware_bridge = hardware::linux::LinuxBridge::new();

    #[cfg(target_os = "windows")]
    let hardware_bridge = hardware::windows::WindowsBridge::new();

    let hardware_file_path = dir_manager.hardware_file_path();

    let hardware = hardware_bridge.hardware();
    if let Err(e) = DirManager::serialize(&hardware_file_path, &hardware) {
        eprintln!("{}", e);
    }

    let config = match &settings.current_config {
        Some(config_name) => {
            DirManager::deserialize::<Config>(&dir_manager.config_file_path(config_name), true)
        }
        None => None,
    };

    let mut id_generator = IdGenerator::new();

    let app_graph = match config {
        Some(config) => AppGraph::from_config(config, &hardware, &mut id_generator),
        None => AppGraph::default(),
    };

    let app_state = AppState {
        dir_manager,
        settings,
        hardware_bridge: Box::new(hardware_bridge),
        hardware,
        app_graph,
        id_generator,
    };

    run_ui(app_state).unwrap();
}
