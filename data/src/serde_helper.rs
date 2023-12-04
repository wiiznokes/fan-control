use std::{
    fs::{self},
    path::Path,
};

use serde::{de::DeserializeOwned, Serialize};

pub fn deserialize<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    match fs::read_to_string(path) {
        Ok(content) => toml::from_str(&content).map_err(|e| format!("{:?}", e)),
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub fn serialize<T: Serialize>(path: &Path, rust_struct: &T) -> Result<(), String> {
    match toml::to_string_pretty(rust_struct) {
        Ok(content) => fs::write(path, content).map_err(|e| format!("serialize: {:?}", e)),
        Err(e) => Err(format!("serialize {}, {:?}", path.display(), e)),
    }
}
