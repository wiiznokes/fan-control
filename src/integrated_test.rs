use data::args::Args;
use data::dir_manager::DirManager;
use hardware::{self, HardwareBridge, HardwareBridgeT};
use std::path::PathBuf;

pub fn init_test_logging() {
    let _ = env_logger::builder()
        .format_timestamp(None)
        .is_test(false)
        .try_init();
}

#[test]
fn test_init() {
    init_test_logging();

    let args = Args {
        config_dir_path: Some(PathBuf::from("./.config")),
        config_name: Some("fake".into()),
        ..Default::default()
    };

    let _dir_manager = DirManager::new(&args);

    let mut bridge = HardwareBridgeT::new().unwrap();
    let hardware = bridge.generate_hardware().unwrap();

    info!("Controls: {}", hardware.controls.len());
    info!("Fans: {}", hardware.fans.len());
    info!("Temps: {}", hardware.temps.len());

    bridge.shutdown().unwrap();
}
