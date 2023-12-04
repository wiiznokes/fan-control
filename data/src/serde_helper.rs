use std::{
    fs::{self, File},
    path::PathBuf,
};

use serde::{de::DeserializeOwned, Serialize};

pub fn deserialize<T: DeserializeOwned>(path: &PathBuf) -> Result<T, String> {
    match fs::read_to_string(path) {
        Ok(content) => match toml::from_str(&content) {
            Ok(t) => Ok(t),
            Err(e) => Err(format!("{:?}", e)),
        },
        Err(e) => Err(format!("{:?}", e)),
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
