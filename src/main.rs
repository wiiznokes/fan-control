use clap::Parser;
use data::{
    cli::Args,
    config::{Config, Hardware},
    directories::SettingsManager,
};
use hardware::{self, HardwareGenerator};

fn main() {
    let args = Args::parse();

    let settings_manager = SettingsManager::new(args.config_dir_path);
    let settings = settings_manager.init_settings();

    let hardware_generator = hardware::linux::LinuxGenerator::new();

    let hardware_file_path = settings_manager.hardware_file_path();

    let _hardware = match SettingsManager::deserialize::<Hardware>(&hardware_file_path, true) {
        Some(hardware) => hardware,
        None => {
            let hardware = hardware_generator.hardware();
            if let Err(e) = SettingsManager::serialize(&hardware_file_path, &hardware) {
                eprintln!("{}", e);
            }
            hardware
        },
    };

    let _config = match settings.current_config {
        Some(config_name) => SettingsManager::deserialize::<Config>(
            &settings_manager.config_file_path(config_name),
            true,
        ),
        None => None,
    };


    
}
