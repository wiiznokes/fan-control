use std::{fs::{File, self}, io::Write, path::Path};

use example::hardware1;

use crate::example::config1;

mod config;
mod example;

fn main() {
    let hardware1 = hardware1();

    let res = serde_json::to_string_pretty(&hardware1).unwrap();
    write_file(Path::new("./config/hardware.json"), res);
    let res = toml::to_string_pretty(&hardware1).unwrap();
    write_file(Path::new("./config/hardware.toml"), res);

    let config1 = config1();

    let res = serde_json::to_string_pretty(&config1).unwrap();
    write_file(Path::new("./config/config1.json"), res);
    let res = toml::to_string_pretty(&config1).unwrap();
    write_file(Path::new("./config/config1.toml"), res);
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
