use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use directories::ProjectDirs;

use crate::{cli::Args, config::Config, serde_helper, settings::Settings, utils::RemoveElem};

static QUALIFIER: &str = "com";
static ORG: &str = "wiiznokes";
static APP: &str = "fan-control";

static SETTINGS_FILENAME: &str = "settings.toml";
static HARDWARE_FILENAME: &str = "hardware.toml";

pub struct DirManager {
    pub config_dir_path: PathBuf,
    pub config_names: Vec<String>,
    pub settings: Settings,
}

impl DirManager {
    pub fn new(args: Args) -> DirManager {
        fn default_config_dir_path() -> PathBuf {
            ProjectDirs::from(QUALIFIER, ORG, APP)
                .unwrap()
                .config_dir()
                .to_path_buf()
        }

        let config_dir_path = match args.config_dir_path {
            Some(config_dir_path) => {
                if !config_dir_path.is_dir() {
                    error!("{} is not a directory", config_dir_path.display());
                }
                default_config_dir_path()
            }
            None => default_config_dir_path(),
        };

        if !config_dir_path.exists() {
            if let Err(e) = fs::create_dir_all(&config_dir_path) {
                error!("can't create config directories: {e}")
            }
        }

        let mut settings = init_settings(&config_dir_path);

        let config_names = config_names(&config_dir_path);

        if let Some(config_name) = args.config_name {
            if config_names.contains(&config_name) {
                settings.current_config = Some(config_name)
            }
        };

        DirManager {
            config_names,
            config_dir_path,
            settings,
        }
    }

    pub fn settings_file_path(&self) -> PathBuf {
        self.config_dir_path.join(SETTINGS_FILENAME)
    }

    pub fn hardware_file_path(&self) -> PathBuf {
        self.config_dir_path.join(HARDWARE_FILENAME)
    }

    pub fn config_file_path(&self, name: &str) -> PathBuf {
        self.config_dir_path.join(name)
    }
}

impl DirManager {
    pub fn save_config(
        &mut self,
        previous_name: &str,
        new_name: &str,
        config: &Config,
    ) -> Result<(), String> {
        let previous_path = self.config_file_path(previous_name);
        if let Err(e) = fs::remove_file(previous_path) {
            return Err(format!("{:?}", e));
        }

        let new_path = self.config_file_path(new_name);

        serde_helper::serialize(&new_path, config)?;

        if self
            .config_names
            .remove_elem(|e| previous_name == e)
            .is_none()
        {
            warn!("can't find {} in config_names to remove it", previous_name);
        }

        self.config_names.push(new_name.to_owned());

        self.settings.current_config = Some(new_name.to_owned());

        Ok(())
    }

    pub fn remove() {}
}

fn config_names(config_dir_path: &Path) -> Vec<String> {
    let mut filenames = Vec::new();

    let Ok(files) = config_dir_path.read_dir() else {
        return filenames;
    };

    for file in files {
        let Ok(file) = file else {
            continue;
        };

        let Ok(metadata) = file.metadata() else {
            continue;
        };

        if !metadata.is_file() {
            continue;
        }
        if serde_helper::deserialize::<Config>(&file.path()).is_err() {
            continue;
        }
        filenames.push(file.file_name().to_string_lossy().to_string())
    }

    filenames
}

fn init_settings(config_dir_path: &Path) -> Settings {
    let settings_file_path = config_dir_path.join(SETTINGS_FILENAME);

    if !settings_file_path.exists() {
        let default_settings = Settings::default();

        if let Err(e) = serde_helper::serialize(&settings_file_path, &default_settings) {
            error!("{e}");
        }

        default_settings
    } else {
        match serde_helper::deserialize(&settings_file_path) {
            Ok(t) => t,
            Err(e) => {
                error!("{:?}", e);
                Settings::default()
            }
        }
    }
}
