use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use hardware::Hardware;

use thiserror::Error;
use utils::{APP, ORG, QUALIFIER};

use crate::{
    config::Config,
    settings::{Settings, SettingsState},
    utils::RemoveElem,
};

use self::helper::{deserialize, serialize};

#[derive(Debug)]
pub struct ConfigNames {
    pub data: Vec<String>,
}

#[derive(Debug)]
pub struct DirManager {
    pub config_dir_path: PathBuf,
    pub state_dir_path: PathBuf,
    pub config_names: ConfigNames,
    settings: Settings,
    state: SettingsState,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    TomlDeserialization(#[from] toml::de::Error),
    #[error(transparent)]
    TomlSerialization(#[from] toml::ser::Error),
    #[error("There is no name")]
    NoName,
}

type Result<T> = std::result::Result<T, ConfigError>;

static SETTINGS_FILENAME: &str = "settings.toml";
static STATE_FILENAME: &str = "state.toml";
static HARDWARE_FILENAME: &str = "hardware.toml";

impl DirManager {
    pub fn new(
        custom_config_dir_path: &Option<PathBuf>,
        custom_config_name: &Option<String>,
    ) -> DirManager {
        let project_dirs = ProjectDirs::from(QUALIFIER, ORG, APP).unwrap();

        let config_dir_path = match custom_config_dir_path {
            Some(config_dir_path) => {
                if config_dir_path.exists() {
                    if config_dir_path.is_dir() {
                        config_dir_path.clone()
                    } else {
                        error!(
                            "The path {} is not a directory. Fall back to default directory.",
                            config_dir_path.display()
                        );
                        project_dirs.config_dir().to_path_buf()
                    }
                } else {
                    warn!(
                        "The directory {} does not yet exist.",
                        config_dir_path.display()
                    );
                    config_dir_path.clone()
                }
            }
            None => project_dirs.config_dir().to_path_buf(),
        };

        if !config_dir_path.exists() {
            if let Err(e) = fs::create_dir_all(&config_dir_path) {
                error!("Can't create config directories: {e}.")
            }
        }

        let mut settings = {
            let settings_file_path = config_dir_path.join(SETTINGS_FILENAME);

            if !settings_file_path.exists() {
                Settings::default()
            } else {
                match deserialize(&settings_file_path) {
                    Ok(t) => t,
                    Err(e) => {
                        error!("can't deserialize settings at init: {}", e);
                        Settings::default()
                    }
                }
            }
        };

        let config_names = ConfigNames::new(&config_dir_path);

        if let Some(config_name) = custom_config_name {
            let config_name = helper::remove_toml_extension(config_name).to_owned();
            settings.current_config = if config_names.contains(&config_name) {
                Some(config_name)
            } else {
                warn!("Config gave as parameter not exist.");
                None
            }
        };

        let state_dir_path = project_dirs.data_local_dir().to_path_buf();

        if !state_dir_path.exists() {
            if let Err(e) = fs::create_dir_all(&state_dir_path) {
                error!("Can't create config directories: {e}.")
            }
        }

        let state = {
            let state_file_path = state_dir_path.join(STATE_FILENAME);

            if !state_file_path.exists() {
                SettingsState::default()
            } else {
                match deserialize(&state_file_path) {
                    Ok(t) => t,
                    Err(e) => {
                        error!("can't deserialize settings at init: {}", e);
                        SettingsState::default()
                    }
                }
            }
        };

        DirManager {
            config_names,
            config_dir_path,
            settings,
            state,
            state_dir_path,
        }
    }

    fn settings_file_path(&self) -> PathBuf {
        self.config_dir_path.join(SETTINGS_FILENAME)
    }

    fn hardware_file_path(&self) -> PathBuf {
        self.config_dir_path.join(HARDWARE_FILENAME)
    }

    fn state_file_path(&self) -> PathBuf {
        self.state_dir_path.join(STATE_FILENAME)
    }

    fn config_file_path(&self, name: &str) -> PathBuf {
        self.config_dir_path
            .join(helper::add_toml_extension(name).into_owned())
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn update_settings(&mut self, mut f: impl FnMut(&mut Settings)) {
        f(&mut self.settings);

        if let Err(e) = serialize(&self.settings_file_path(), &self.settings) {
            error!("{e}");
        }
    }

    pub fn state(&self) -> &SettingsState {
        &self.state
    }

    pub fn update_state(&mut self, mut f: impl FnMut(&mut SettingsState)) {
        f(&mut self.state);

        if let Err(e) = serialize(&self.state_file_path(), &self.state) {
            error!("{e}");
        }
    }

    pub fn get_config(&self) -> Option<Config> {
        match &self.settings().current_config {
            Some(config_name) => match deserialize::<Config>(&self.config_file_path(config_name)) {
                Ok(config) => Some(config),
                Err(e) => {
                    warn!("{}", e);
                    None
                }
            },
            None => None,
        }
    }

    pub fn serialize_hardware(&self, hardware: &Hardware) {
        let hardware_file_path = self.hardware_file_path();

        if let Err(e) = serialize(&hardware_file_path, hardware) {
            warn!("{}", e);
        } else {
            println!(
                "hardware file successfully written in {}",
                hardware_file_path.display()
            )
        }
    }
}

impl DirManager {
    pub fn save_config(&mut self, new_name: &str, config: &Config) -> Result<()> {
        let Some(previous_name) = &self.settings().current_config else {
            return Err(ConfigError::NoName);
        };

        let previous_path = self.config_file_path(previous_name);
        if let Err(e) = fs::remove_file(previous_path) {
            warn!("Can't remove file while saving config: {}.", e);
        }

        let new_path = self.config_file_path(new_name);

        serialize(&new_path, config)?;

        self.config_names.remove(&previous_name.clone());
        self.config_names.add(new_name);

        self.update_settings(|settings| {
            settings.current_config = Some(new_name.to_owned());
        });

        Ok(())
    }

    /// Return the config and her name
    pub fn change_config(
        &mut self,
        new_config_name: Option<String>,
    ) -> Result<Option<(String, Config)>> {
        match new_config_name {
            Some(new_config_name) => {
                let new_config_path = self.config_file_path(&new_config_name);
                let config = deserialize::<Config>(&new_config_path)?;
                self.update_settings(|settings| {
                    settings.current_config = Some(new_config_name.to_owned());
                });
                Ok(Some((new_config_name, config)))
            }
            None => {
                self.update_settings(|settings| {
                    settings.current_config = None;
                });
                Ok(None)
            }
        }
    }

    /// return true if it's the current config whitch has been removed
    pub fn remove_config(&mut self, config_name: String) -> Result<bool> {
        self.config_names.remove(&config_name);

        let config_path = self.config_file_path(&config_name);
        if let Err(e) = fs::remove_file(config_path) {
            warn!("{}", e);
        }

        if let Some(current_config) = &self.settings().current_config {
            if current_config == &config_name {
                self.update_settings(|settings| {
                    settings.current_config.take();
                });
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn create_config(&mut self, new_config_name: &str, new_config: &Config) -> Result<()> {
        let new_path = self.config_file_path(new_config_name);
        serialize(&new_path, new_config)?;

        self.config_names.add(new_config_name);
        self.update_settings(|settings| {
            settings.current_config = Some(new_config_name.to_owned());
        });

        Ok(())
    }
}

impl ConfigNames {
    fn new(config_dir_path: &Path) -> Self {
        let mut config_names = ConfigNames { data: Vec::new() };

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

            if let Err(e) = deserialize::<Config>(&file.path()) {
                warn!("can't deserialize potential config: {}", e);
                continue;
            }

            let file_name =
                helper::remove_toml_extension(&file.file_name().to_string_lossy()).to_owned();
            config_names.data.push(file_name);
        }

        config_names
            .data
            .sort_by(|first, second| lexical_sort::natural_lexical_cmp(first, second));

        config_names
    }

    fn remove(&mut self, name: &str) {
        let name = helper::remove_toml_extension(name);
        if self.data.remove_elem(|e| e == name).is_none() {
            warn!("no element to remove")
        }
    }

    fn add(&mut self, name: &str) {
        let name = helper::remove_toml_extension(name).to_owned();

        let insert_position = match self
            .data
            .binary_search_by(|e| lexical_sort::natural_lexical_cmp(&name, e))
        {
            Ok(position) => position,  // Element already exists at this position
            Err(position) => position, // Element doesn't exist, insert at this position
        };

        self.data.insert(insert_position, name);
    }

    pub fn is_valid_create(&self, name: &str) -> bool {
        if name.trim() != name || name.is_empty() {
            return false;
        }
        let name = helper::remove_toml_extension(name).to_owned();
        !self.data.contains(&name)
    }

    pub fn contains(&self, name: &str) -> bool {
        let name = helper::remove_toml_extension(name).to_owned();
        self.data.contains(&name)
    }

    pub fn names(&self) -> &Vec<String> {
        &self.data
    }

    pub fn index_of(&self, name: &str) -> Option<usize> {
        let name = helper::remove_toml_extension(name).to_owned();
        self.data.iter().position(|n| n == &name)
    }

    pub fn is_valid_name(&self, previous_name: &Option<String>, new_name: &str) -> bool {
        if new_name.trim() != new_name || new_name.is_empty() {
            return false;
        }
        let new_name = helper::remove_toml_extension(new_name);

        let is_same_name = match previous_name {
            Some(previous_name) => {
                let previous_name = helper::remove_toml_extension(previous_name);
                previous_name == new_name
            }
            None => false,
        };

        let nb_occurence = self
            .data
            .iter()
            .filter(|n| n == &new_name)
            .collect::<Vec<_>>()
            .len();

        nb_occurence == 0 || (nb_occurence == 1 && is_same_name)
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

mod helper {
    use std::{borrow::Cow, fs, path::Path};

    use serde::{de::DeserializeOwned, Serialize};

    static TOML_EXT: &str = ".toml";

    pub fn add_toml_extension(input: &str) -> Cow<'_, str> {
        if !input.ends_with(TOML_EXT) {
            let val = format!("{}{}", input, TOML_EXT);
            Cow::Owned(val)
        } else {
            Cow::Borrowed(input)
        }
    }

    pub fn remove_toml_extension(str: &str) -> &str {
        if let Some(str) = str.strip_suffix(TOML_EXT) {
            str
        } else {
            str
        }
    }

    pub fn deserialize<T: DeserializeOwned>(path: &Path) -> super::Result<T> {
        let str = fs::read_to_string(path)?;
        let t = toml::from_str(&str)?;
        Ok(t)
    }

    pub fn serialize<T: Serialize>(path: &Path, rust_struct: &T) -> super::Result<()> {
        let str = toml::to_string_pretty(rust_struct)?;
        fs::write(path, str)?;
        Ok(())
    }
}
