use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug, Default)]
#[clap(author = "wiiznokes", version, about = "fan control app", long_about = None)]
pub struct Args {
    #[arg(short = 'p', long = "path", id = "path to the config directory")]
    pub config_dir_path: Option<PathBuf>,
    #[arg(
        short = 'c',
        long = "config",
        id = "existing config to use, within config_dir_path"
    )]
    pub config_name: Option<String>,
    #[arg(long = "cli", default_value_t = false)]
    pub cli: bool,
    #[arg(long = "debug", default_value_t = false)]
    pub debug: bool,
    #[arg(long = "info", default_value_t = false)]
    pub info: bool,
    #[arg(long = "log")]
    pub log_file: Option<PathBuf>,
}
