use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use data::directories::DirManager;

use data::{config::Config, node::AppGraph, update::Update, AppState};
use hardware::{hardware_test, HardwareBridge};

#[test]
fn test_config() {
    env_logger::init();

    let dir_manager = DirManager::new(Some(PathBuf::from("./config")));
    let settings = dir_manager.init_settings();

    let hardware = hardware_test::TestBridge::generate_hardware();
    DirManager::serialize(&dir_manager.hardware_file_path(), &hardware).unwrap();

    let config = DirManager::deserialize::<Config>(
        &dir_manager.config_file_path(&settings.current_config.clone().unwrap()),
        true,
    )
    .unwrap();

    let app_graph = AppGraph::from_config(config, &hardware);

    let mut app_state = AppState {
        dir_manager,
        settings,
        hardware,
        app_graph,
        update: Update::new(),
    };

    for _ in 0..20 {
        if let Err(e) = app_state.update.graph(
            &mut app_state.app_graph.nodes,
            &app_state.app_graph.root_nodes,
        ) {
            error!("{:?}", e);
        }
        debug!("\n");
        thread::sleep(Duration::from_millis(50));
    }
}
