use data::args::Args;
use data::dir_manager::DirManager;
use hardware::{self, HardwareBridge};
use std::path::PathBuf;

#[test]
fn test_init() {
    env_logger::init();

    let args = Args {
        config_dir_path: Some(PathBuf::from("./.config")),
        config_name: Some("fake".into()),
        ..Default::default()
    };

    let _dir_manager = DirManager::new(&args);

    #[cfg(target_os = "linux")]
    let (hardware, bridge) = hardware::linux::LinuxBridge::generate_hardware().unwrap();

    #[cfg(target_os = "windows")]
    let (_hardware, mut bridge) = hardware::windows::WindowsBridge::generate_hardware().unwrap();

    bridge.shutdown().unwrap();
}
