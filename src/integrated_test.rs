use hardware::{self, HardwareBridgeT, HardwareBridge};

pub fn init_test_logging() {
    let _ = env_logger::builder()
        .format_timestamp(None)
        .is_test(false)
        .try_init();
}

#[test]
fn test_init() {
    init_test_logging();

    let mut bridge = HardwareBridge::new().unwrap();
    let hardware = bridge.hardware();

    info!("Controls: {}", hardware.controls.len());
    info!("Fans: {}", hardware.fans.len());
    info!("Temps: {}", hardware.temps.len());

    bridge.shutdown().unwrap();
}
