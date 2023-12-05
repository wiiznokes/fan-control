use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use data::app_graph::AppGraph;
use data::cli::Args;
use data::dir_manager::DirManager;

use data::{update::Update, AppState};
use hardware::{fake_hardware, HardwareBridge};

#[test]
fn test_config() {
    env_logger::init();

    let args = Args {
        config_dir_path: Some(PathBuf::from("./.config")),
        config_name: Some("fake".into()),
    };

    let dir_manager = DirManager::new(args);

    let (hardware, bridge) = fake_hardware::FakeHardwareBridge::generate_hardware();

    let config = dir_manager.get_config().unwrap();

    let app_graph = AppGraph::from_config(config, &hardware);

    let mut app_state = AppState {
        dir_manager,
        hardware,
        app_graph,
        update: Update::new(),
        bridge,
    };

    for _ in 0..20 {
        app_state.update.optimized(
            &mut app_state.app_graph.nodes,
            &app_state.app_graph.root_nodes,
            &mut app_state.bridge,
        );
        debug!("\n");
        thread::sleep(Duration::from_millis(50));
    }
}
