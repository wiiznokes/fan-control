use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use crate::args::Args;
use crate::integrated_test::init_test_logging;
use data::app_graph::AppGraph;
use data::dir_manager::DirManager;
use data::{update::Update, AppState};
use hardware::HardwareBridge;

#[test]
fn test_config() {
    init_test_logging();

    let args = Args {
        config_dir_path: Some(PathBuf::from("./configs-examples")),
        config_name: Some("fake".into()),
        ..Default::default()
    };

    let dir_manager = DirManager::new(&args.config_dir_path, &args.config_name);

    let bridge = hardware::fake_hardware::FakeHardwareBridge::new().unwrap();

    let config = dir_manager.get_config().unwrap();

    let app_graph = AppGraph::from_config(config, bridge.hardware());

    let app_state = AppState {
        dir_manager,
        app_graph,
        update: Update::new(),
        bridge,
    };

    run(app_state)
}

fn run<H: HardwareBridge>(mut app_state: AppState<H>) {
    for _ in 0..20 {
        if let Err(e) = app_state.bridge.update() {
            error!("{}", e);
            break;
        }

        app_state
            .update
            .optimized(
                &mut app_state.app_graph.nodes,
                &app_state.app_graph.root_nodes,
                &mut app_state.bridge,
            )
            .unwrap();
        debug!("\n");
        thread::sleep(Duration::from_millis(50));
    }
}
