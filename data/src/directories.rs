use std::{
    fs::{self, File},
    path::PathBuf,
};

use directories::ProjectDirs;
use serde::{de::DeserializeOwned, Serialize};

use crate::{cli::Args, config::Config, settings::Settings};

static QUALIFIER: &str = "com";
static ORG: &str = "wiiznokes";
static APP: &str = "fan-control";

static SETTINGS_FILENAME: &str = "settings.toml";
static HARDWARE_FILENAME: &str = "hardware.toml";

fn default_config_dir_path() -> PathBuf {
    ProjectDirs::from(QUALIFIER, ORG, APP)
        .unwrap()
        .config_dir()
        .to_path_buf()
}

pub struct DirManager {
    pub config_dir_path: PathBuf,
}

impl DirManager {
    pub fn new(config_dir_path_opt: Option<PathBuf>) -> DirManager {
        let config_dir_path = if let Some(config_dir_path) = config_dir_path_opt {
            match Args::validate_config_dir_path(&config_dir_path) {
                Ok(_) => config_dir_path,
                Err(e) => {
                    eprintln!("{}", e);
                    default_config_dir_path()
                }
            }
        } else {
            default_config_dir_path()
        };

        if !config_dir_path.exists() {
            if let Err(e) = fs::create_dir_all(&config_dir_path) {
                eprintln!("can't create config directories: {e}")
            }
        }

        DirManager { config_dir_path }
    }

    pub fn settings_file_path(&self) -> PathBuf {
        self.config_dir_path.join(SETTINGS_FILENAME)
    }

    pub fn hardware_file_path(&self) -> PathBuf {
        self.config_dir_path.join(HARDWARE_FILENAME)
    }

    pub fn config_file_path(&self, name: &String) -> PathBuf {
        self.config_dir_path.join(name)
    }

    pub fn init_settings(&self) -> Settings {
        let settings_file_path = self.settings_file_path();

        if !settings_file_path.exists() {
            if let Err(e) = File::create(&settings_file_path) {
                eprintln!("can't create settings file: {e}");
                return Settings::default();
            }

            if let Err(e) = fs::write(
                settings_file_path,
                toml::to_string(&Settings::default())
                    .expect("can't serialise default Settings struct"),
            ) {
                eprintln!("can't write to settings file: {e}");
                return Settings::default();
            }

            Settings::default()
        } else {
            Self::deserialize(&settings_file_path, true).unwrap_or_default()
        }
    }

    pub fn list_config_filenames(&self) -> Vec<String> {
        let mut filenames = Vec::new();

        let Ok(files) = self.config_dir_path.read_dir() else {
            return filenames;
        };

        for file in files {
            let Ok(file) = file else {
                continue;
            };

            match file.metadata() {
                Ok(metadata) => {
                    if !metadata.is_file() {
                        continue;
                    }
                }
                Err(_) => continue,
            }

            if Self::deserialize::<Config>(&file.path(), false).is_none() {
                continue;
            }
            filenames.push(file.file_name().to_string_lossy().to_string())
        }

        filenames
    }

    pub fn deserialize<T: DeserializeOwned>(path: &PathBuf, log: bool) -> Option<T> {
        match fs::read_to_string(path) {
            Ok(content) => match toml::from_str(&content) {
                Ok(t) => t,
                Err(e) => {
                    if log {
                        eprintln!("deserialize: {:?}", e);
                    }
                    None
                }
            },
            Err(e) => {
                if log {
                    eprintln!("deserialize: can't read file {}, {:?}", path.display(), e);
                }
                None
            }
        }
    }

    pub fn serialize<T: Serialize>(path: &PathBuf, rust_struct: &T) -> Result<(), String> {
        match toml::to_string_pretty(rust_struct) {
            Ok(content) => {
                if !path.exists() {
                    if let Err(e) = File::create(path) {
                        return Err(format!("can't create file {}, {}", path.display(), e));
                    }
                }

                match fs::write(path, content) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("serialize: {:?}", e)),
                }
            }
            Err(e) => Err(format!("serialize {}, {:?}", path.display(), e)),
        }
    }
}
