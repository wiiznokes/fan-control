// to not launch a console on Windows, only in release
// because it blocks all logs, from C# AND Rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, fs};

use args::Args;
use clap::Parser;
use data::{app_graph::AppGraph, dir_manager::DirManager, update::Update, AppState};
use hardware::{self, HardwareBridge, HardwareBridgeT};
use log::LevelFilter;
use thiserror::Error;

#[allow(unused_imports)]
#[macro_use]
extern crate log;

pub mod args;
mod cli;

#[cfg(all(test, feature = "fake_hardware"))]
mod fake_integrated_test;

#[cfg(test)]
mod integrated_test;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Hardware(#[from] hardware::HardwareError),
}

pub type Result<T> = std::result::Result<T, Error>;

fn setup_logs(args: &Args) {
    let mut env_logger_builder = env_logger::builder();

    fn filter_workspace_crates(
        env_logger_builder: &mut env_logger::Builder,
        level_filter: LevelFilter,
    ) -> &mut env_logger::Builder {
        // allow other crate to show warn level of error
        env_logger_builder.filter_level(LevelFilter::Warn);
        env_logger_builder.filter_module("hardware", level_filter);
        env_logger_builder.filter_module("data", level_filter);
        env_logger_builder.filter_module("ui", level_filter);
        env_logger_builder.filter_module("fan-control", level_filter);
        env_logger_builder
    }

    if args.info {
        filter_workspace_crates(&mut env_logger_builder, LevelFilter::Info);
    };

    if args.debug {
        filter_workspace_crates(&mut env_logger_builder, LevelFilter::Debug);
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

    if args.log_file.is_some() {
        env_logger_builder.format_timestamp_secs();
    } else {
        env_logger_builder.format_timestamp(None);
    }
    env_logger_builder.init();
}

fn try_run() -> Result<()> {
    let args = Args::parse();
    setup_logs(&args);

    ui::localize::localize();
    data::localize::localize();

    let dir_manager = DirManager::new(&args.config_dir_path, &args.config_name);

    let bridge = HardwareBridge::new()?;
    let hardware = bridge.hardware();

    debug!("sensors found: {:?}", hardware);

    dir_manager.serialize_hardware(hardware);

    let app_graph = match dir_manager.get_config() {
        Some(config) => AppGraph::from_config(config, hardware),
        None => AppGraph::default(hardware),
    };

    let app_state = AppState {
        dir_manager,
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
