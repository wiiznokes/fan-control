// to not launch a console on Windows, only in release
// because it blocks all logs, from C# AND Rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, fs};

use clap::Parser;
use data::{app_graph::AppGraph, args::Args, dir_manager::DirManager, update::Update, AppState};
use hardware::{self, HardwareBridge};
use log::LevelFilter;
use thiserror::Error;

#[allow(unused_imports)]
#[macro_use]
extern crate log;

#[cfg(all(test, feature = "fake_hardware"))]
mod fake_integrated_test;

#[cfg(test)]
mod integrated_test;

mod cli;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Hardware(#[from] hardware::HardwareError),
}

pub type Result<T> = std::result::Result<T, Error>;

fn setup_logs(args: &Args) {
    let mut env_logger_builder = env_logger::builder();

    if args.info {
        env_logger_builder.filter_level(LevelFilter::Info);
    };

    if args.debug {
        env_logger_builder.filter_level(LevelFilter::Debug);
    };

    if let Some(log_file_path) = &args.log_file {
        env::set_var("FAN_CONTROL_LOG_FILE", log_file_path);
        match fs::File::create(log_file_path) {
            Ok(log_file) => {
                if let Err(e) = log_file.set_len(0) {
                    warn!("can't clear the content of log file: {e}");
                };

                let pipe = env_logger::Target::Pipe(Box::new(log_file));
                env_logger_builder.target(pipe);
            }
            Err(e) => {
                error!("can't create/open log file: {e}");
            }
        };
    }

    env_logger_builder.format_timestamp(None).init();
}

fn try_run() -> Result<()> {
    let args = Args::parse();
    setup_logs(&args);

    ui::localize::localize();
    data::localize::localize();

    let dir_manager = DirManager::new(&args);

    #[cfg(feature = "fake_hardware")]
    let (hardware, bridge) = hardware::fake_hardware::FakeHardwareBridge::generate_hardware()?;

    #[cfg(all(not(feature = "fake_hardware"), target_os = "linux"))]
    let (hardware, bridge) = hardware::linux::LinuxBridge::generate_hardware()?;

    #[cfg(all(not(feature = "fake_hardware"), target_os = "windows"))]
    let (hardware, bridge) = hardware::windows::WindowsBridge::generate_hardware()?;

    debug!("sensors found: {:?}", hardware);

    dir_manager.serialize_hardware(&hardware);

    let app_graph = match dir_manager.get_config() {
        Some(config) => AppGraph::from_config(config, &hardware),
        None => AppGraph::default(&hardware),
    };

    let app_state = AppState {
        dir_manager,
        hardware,
        bridge,
        app_graph,
        update: Update::new(),
    };

    match args.cli {
        true => cli::run_cli(app_state),
        false => ui::run_ui(app_state),
    };

    Ok(())
}

fn main() {
    if let Err(e) = try_run() {
        error!("{}", e);
    }
}
