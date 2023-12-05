use std::{
    fs::{self},
    path::{Path, PathBuf},
    vec,
};

use directories::ProjectDirs;

use crate::{cli::Args, config::Config, name_sorter, serde_helper, settings::Settings};

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
    current_index: usize,
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

        let mut config_names = ConfigNames::new(&config_dir_path);

        if let Some(config_name) = args.config_name {
            let config_name = remove_toml_ext(&config_name).to_owned();
            if config_names.contains(&config_name) {
                settings.current_config = Some(config_name);
            }
        };

        if let Some(config_name) = &settings.current_config {
            config_names.set_index(config_name);
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
        let Some(previous_name) = self.config_names.get_current() else {
            return Err("can't save config: no name".to_string());
        };

        let previous_path = self.config_file_path(previous_name);
        if let Err(e) = fs::remove_file(previous_path) {
            warn!("{:?}", e);
        }

        let new_path = self.config_file_path(new_name);

        serde_helper::serialize(&new_path, config)?;

        self.config_names.remove_current();
        self.config_names.add(new_name);

        self.settings.current_config = Some(new_name.to_owned());

        Ok(())
    }

    pub fn change_config(&mut self, index: usize) -> Result<Option<(String, Config)>, String> {
        self.config_names.current_index = index;

        match self.config_names.get(index) {
            Some(new_config_name) => {
                let new_config_path = self.config_file_path(new_config_name);
                let config: Config = serde_helper::deserialize(&new_config_path)?;
                self.settings.current_config = Some(new_config_name.to_owned());
                Ok(Some((new_config_name.to_owned(), config)))
            }
            None => {
                self.settings.current_config = None;
                Ok(None)
            }
        }
    }

    /// return true if it's the current config whitch has been removed
    pub fn remove_config(&mut self, index: usize) -> Result<bool, String> {
        let config_name = self.config_names.remove(index);

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
        if self.config_names.contains(new_config_name) {
            return Err(format!(
                "can't create config: the name {} is already taken",
                new_config_name
            ));
        }

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
            data: vec!["None".to_owned()],
            current_index: 0,
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

    fn set_index(&mut self, config_name: &str) {
        if let Some(index) = self.data.iter().position(|n| n == config_name) {
            self.current_index = index;
        }
    }

    fn remove_current(&mut self) -> String {
        self.remove(self.current_index)
    }
    fn remove(&mut self, index: usize) -> String {
        if index == 0 {
            panic!()
        }
        if self.current_index == index {
            self.current_index = 0;
        }
        self.data.remove(index)
    }

    fn add(&mut self, name: &str) {
        let name = remove_toml_ext(name).to_owned();
        let index = name_sorter::add_sorted(&mut self.data, name);
        self.current_index = index;
    }

    pub fn contains(&self, name: &str) -> bool {
        let name = remove_toml_ext(name).to_owned();
        self.data.contains(&name)
    }

    pub fn get(&self, index: usize) -> Option<&String> {
        if index == 0 {
            None
        } else {
            Some(&self.data[index])
        }
    }

    pub fn get_current(&self) -> Option<&String> {
        self.get(self.current_index)
    }

    pub fn names(&self) -> &[String] {
        

        let (_left, right) = self.data.split_at(self.current_index);
        let (_, right) = right.split_at(1); // Exclude the element at index n

        
        right

        /*
        self.data.iter().enumerate().filter(|(i, _)|{
            i != &self.current_index
        }).map(|(_,e)|{
            e
        }).collect()
         */
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() > 1
    }

    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.data.iter().position(|n| n == name)
    }

    pub fn index(&self) -> usize {
        self.current_index
    }

    pub fn is_valid(&self, _name: &str) -> bool {
        todo!()
    }
}

static NONE: &str = "none";

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
