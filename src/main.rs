use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use example::hardware1;

use crate::{
    conf::{configs::Config, hardware::Hardware},
    example::config1,
};

mod conf;
mod example;

const HARDWARE_PATH_TOML: &str = "./config/hardware.toml";
const HARDWARE_PATH_JSON: &str = "./config/hardware.json";

const CONFIG_PATH_TOML: &str = "./config/config1.toml";
const CONFIG_PATH_JSON: &str = "./config/config1.json";

fn main() {
    if let Ok(content) = fs::read_to_string(Path::new(CONFIG_PATH_TOML)) {
        let output: Config = toml::from_str(&content).unwrap();
        dbg!(output);
    }

    if let Ok(content) = fs::read_to_string(Path::new(HARDWARE_PATH_TOML)) {
        let output: Hardware = toml::from_str(&content).unwrap();
        dbg!(output);
    }

    let hardware1 = hardware1();

    let res = serde_json::to_string_pretty(&hardware1).unwrap();
    write_file(Path::new(HARDWARE_PATH_JSON), res);
    let res = toml::to_string_pretty(&hardware1).unwrap();
    write_file(Path::new(HARDWARE_PATH_TOML), res);

    let config1 = config1();

    let res = serde_json::to_string_pretty(&config1).unwrap();
    write_file(Path::new(CONFIG_PATH_JSON), res);
    let res = toml::to_string_pretty(&config1).unwrap();
    write_file(Path::new(CONFIG_PATH_TOML), res);
}

fn write_file(path: &Path, content: String) {
    if path.exists() {
        eprintln!("path {} already exist.", path.to_string_lossy());
        fs::remove_file(path).unwrap();
    }

    let mut file = File::create(path).unwrap();

    file.write_all(content.as_bytes()).unwrap();
    println!("config succesfully writed!");
}
