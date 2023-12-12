use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use hardware::Hardware;

use crate::{
    args::Args, config::Config, name_sorter, serde_helper, settings::Settings, utils::RemoveElem,
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
    settings: Settings,
}

#[derive(Debug)]
pub struct ConfigNames {
    pub data: Vec<String>,
}

impl DirManager {
    pub fn new(args: &Args) -> DirManager {
        fn default_config_dir_path() -> PathBuf {
            ProjectDirs::from(QUALIFIER, ORG, APP)
                .unwrap()
                .config_dir()
                .to_path_buf()
        }

        let config_dir_path = match &args.config_dir_path {
            Some(config_dir_path) => {
                if !config_dir_path.is_dir() {
                    error!("{} is not a directory", config_dir_path.display());
                    default_config_dir_path()
                } else {
                    config_dir_path.clone()
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

        if let Some(config_name) = &args.config_name {
            let config_name = remove_toml_ext(config_name).to_owned();
            settings.current_config = if config_names.contains(&config_name) {
                Some(config_name)
            } else {
                warn!("config gave as parameter not exist");
                None
            }
        };

        DirManager {
            config_names,
            config_dir_path,
            settings,
        }
    }

    fn settings_file_path(&self) -> PathBuf {
        self.config_dir_path.join(SETTINGS_FILENAME)
    }

    fn hardware_file_path(&self) -> PathBuf {
        self.config_dir_path.join(HARDWARE_FILENAME)
    }

    fn config_file_path(&self, name: &str) -> PathBuf {
        self.config_dir_path.join(add_toml_ext(name).into_owned())
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn update_settings(&mut self, mut f: impl FnMut(&mut Settings)) {
        f(&mut self.settings);

        if let Err(e) = serde_helper::serialize(&self.settings_file_path(), &self.settings()) {
            error!("{e}");
        }
    }

    pub fn get_config(&self) -> Option<Config> {
        match &self.settings().current_config {
            Some(config_name) => {
                match serde_helper::deserialize::<Config>(&self.config_file_path(config_name)) {
                    Ok(config) => Some(config),
                    Err(e) => {
                        warn!("{:?}", e);
                        None
                    }
                }
            }
            None => None,
        }
    }

    pub fn serialize_hardware(&self, hardware: &Hardware) {
        if let Err(e) = serde_helper::serialize(&self.hardware_file_path(), hardware) {
            warn!("{}", e);
        }
    }
}

impl DirManager {
    pub fn save_config(&mut self, new_name: &str, config: &Config) -> Result<(), String> {
        let Some(previous_name) = &self.settings().current_config else {
            return Err("can't save config: no name".to_string());
        };

        let previous_path = self.config_file_path(previous_name);
        if let Err(e) = fs::remove_file(previous_path) {
            warn!("{:?}", e);
        }

        let new_path = self.config_file_path(new_name);

        serde_helper::serialize(&new_path, config)?;

        self.config_names.remove(&previous_name.clone());
        self.config_names.add(new_name);

        self.update_settings(|settings| {
            settings.current_config = Some(new_name.to_owned());
        });

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
    pub fn remove_config(&mut self, config_name: String) -> Result<bool, String> {
        self.config_names.remove(&config_name);

        let config_path = self.config_file_path(&config_name);
        if let Err(e) = fs::remove_file(config_path) {
            warn!("{:?}", e);
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

    pub fn create_config(
        &mut self,
        new_config_name: &str,
        new_config: &Config,
    ) -> Result<(), String> {
        let new_path = self.config_file_path(new_config_name);
        serde_helper::serialize(&new_path, new_config)?;

        self.config_names.add(new_config_name);
        self.update_settings(|settings| {
            settings.current_config = Some(new_config_name.to_owned());
        });

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

            if let Err(e) = serde_helper::deserialize::<Config>(&file.path()) {
                warn!("error while deserialize conifg in config dir: {}", e);
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

    pub fn names(&self) -> &Vec<String> {
        &self.data
    }

    pub fn index_of(&self, name: &str) -> Option<usize> {
        let name = remove_toml_ext(name).to_owned();
        self.data.iter().position(|n| n == &name)
    }

    pub fn is_valid_name(&self, previous_name: &Option<String>, new_name: &str) -> bool {
        if new_name.trim() != new_name || new_name.is_empty() {
            return false;
        }
        let new_name = remove_toml_ext(new_name);

        let is_same_name = match previous_name {
            Some(previous_name) => {
                let previous_name = remove_toml_ext(previous_name);
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
