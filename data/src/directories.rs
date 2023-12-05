use std::{
    fs::{self},
    path::{Path, PathBuf},
    vec,
};

use directories::ProjectDirs;

use crate::{
    cli::Args, config::Config, fl, name_sorter, serde_helper, settings::Settings, utils::RemoveElem,
};

use self::toml::{add_toml_ext, remove_toml_ext};

static QUALIFIER: &str = "com";
static ORG: &str = "wiiznokes";
static APP: &str = "fan-control";

static SETTINGS_FILENAME: &str = "settings.toml";
static HARDWARE_FILENAME: &str = "hardware.toml";

#[derive(Debug)]
pub struct DirManager {
    pub config_dir_path: PathBuf,
    pub config_names: ConfigNames,
    pub settings: Settings,
}

#[derive(Debug)]
pub struct ConfigNames {
    pub data: Vec<String>,
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
                    default_config_dir_path()
                } else {
                    config_dir_path
                }
            }
            None => default_config_dir_path(),
        };

        if !config_dir_path.exists() {
            if let Err(e) = fs::create_dir_all(&config_dir_path) {
                error!("can't create config directories: {e}")
            }
        }

        let mut settings = init_settings(&config_dir_path);

        let config_names = ConfigNames::new(&config_dir_path);

        if let Some(config_name) = args.config_name {
            let config_name = remove_toml_ext(&config_name).to_owned();
            if config_names.contains(&config_name) {
                settings.current_config = Some(config_name);
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
        self.config_dir_path.join(add_toml_ext(name).into_owned())
    }
}

impl DirManager {
    pub fn save_config(&mut self, new_name: &str, config: &Config) -> Result<(), String> {
        let Some(previous_name) = &self.settings.current_config else {
            return Err("can't save config: no name".to_string());
        };

        let previous_path = self.config_file_path(previous_name);
        if let Err(e) = fs::remove_file(previous_path) {
            warn!("{:?}", e);
        }

        let new_path = self.config_file_path(new_name);

        serde_helper::serialize(&new_path, config)?;

        self.config_names.remove(previous_name);
        self.config_names.add(new_name);

        self.settings.current_config = Some(new_name.to_owned());

        Ok(())
    }

    pub fn change_config(
        &mut self,
        new_config_name: Option<String>,
    ) -> Result<Option<(String, Config)>, String> {
        match new_config_name {
            Some(new_config_name) => {
                let new_config_path = self.config_file_path(&new_config_name);
                let config = serde_helper::deserialize::<Config>(&new_config_path)?;
                self.settings.current_config = Some(new_config_name.to_owned());
                Ok(Some((new_config_name, config)))
            }
            None => {
                self.settings.current_config = None;
                Ok(None)
            }
        }
    }

    /// return true if it's the current config whitch has been removed
    pub fn remove_config(&mut self, config_name: String) -> Result<bool, String> {
        self.config_names.remove(&config_name);

        let config_path = self.config_file_path(&config_name);
        if let Err(e) = fs::remove_file(config_path) {
            warn!("{:?}", e);
        }

        if let Some(current_config) = &self.settings.current_config {
            if current_config == &config_name {
                self.settings.current_config.take();
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn create_config(
        &mut self,
        new_config_name: &str,
        new_config: &Config,
    ) -> Result<(), String> {
        let new_path = self.config_file_path(new_config_name);
        serde_helper::serialize(&new_path, new_config)?;

        self.config_names.add(new_config_name);
        self.settings.current_config = Some(new_config_name.to_owned());

        Ok(())
    }
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

impl ConfigNames {
    fn new(config_dir_path: &Path) -> Self {
        let mut config_names = ConfigNames {
            data: vec![fl!("none")],
        };

        let Ok(files) = config_dir_path.read_dir() else {
            return config_names;
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

            let file_name = file.file_name();

            if file_name == SETTINGS_FILENAME || file_name == HARDWARE_FILENAME {
                continue;
            }

            if serde_helper::deserialize::<Config>(&file.path()).is_err() {
                continue;
            }

            let file_name = remove_toml_ext(&file.file_name().to_string_lossy()).to_owned();
            config_names.data.push(file_name);
        }

        config_names
            .data
            .sort_by(|first, second| name_sorter::compare_names(first, second));

        config_names
    }

    fn remove(&mut self, name: &str) {
        let name = remove_toml_ext(name);
        if self.data.remove_elem(|e| e == name).is_none() {
            warn!("no element to remove")
        }
    }

    fn add(&mut self, name: &str) {
        let name = remove_toml_ext(name).to_owned();
        name_sorter::add_sorted(&mut self.data, name);
    }

    pub fn is_valid_create(&self, name: &str) -> bool {
        if name.trim() != name || name.is_empty() {
            return false;
        }
        let name = remove_toml_ext(name).to_owned();
        !self.data.contains(&name)
    }

    pub fn contains(&self, name: &str) -> bool {
        let name = remove_toml_ext(name).to_owned();
        self.data.contains(&name)
    }

    pub fn names(&self, without: &Option<String>) -> Vec<String> {
        let mut names = self.data.clone();

        let without_index = match without {
            Some(name) => self.data.iter().position(|n| n == name).unwrap(),
            None => 0,
        };

        names.remove(without_index);
        names
    }

    pub fn index_of(&self, name: &str) -> Option<usize> {
        let name = remove_toml_ext(name).to_owned();
        self.data.iter().position(|n| n == &name)
    }

    pub fn is_valid_name(&self, name: &str) -> bool {
        if name.trim() != name || name.is_empty() {
            return false;
        }
        let name = remove_toml_ext(name);
        self.data
            .iter()
            .filter(|n| n == &name)
            .collect::<Vec<_>>()
            .len()
            > 1
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 1
    }
}

pub fn filter_none(str: String) -> Option<String> {
    if str == fl!("none") {
        None
    } else {
        Some(str)
    }
}

mod toml {
    use std::borrow::Cow;

    static TOML_EXT: &str = ".toml";

    pub fn add_toml_ext(input: &str) -> Cow<'_, str> {
        if !input.ends_with(TOML_EXT) {
            let val = format!("{}{}", input, TOML_EXT);
            Cow::Owned(val)
        } else {
            Cow::Borrowed(input)
        }
    }

    pub fn remove_toml_ext(str: &str) -> &str {
        if let Some(str) = str.strip_suffix(TOML_EXT) {
            str
        } else {
            str
        }
    }
}
