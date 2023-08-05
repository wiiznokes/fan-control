use std::{fs::File, io::Write, path::Path};

use crate::example::config1;

mod config;
mod example;

fn main() {
    let config1 = config1();

    let res = serde_json::to_string(&config1).unwrap();
    write_file(Path::new("./config/config.json"), res);
    let res = toml::to_string(&config1).unwrap();
    write_file(Path::new("./config/config.toml"), res);
}

fn write_file(path: &Path, content: String) {
    if path.exists() {
        eprintln!("path {} already exist.", path.to_string_lossy());
        return;
    }

    let mut file = File::create(path).unwrap();

    file.write_all(content.as_bytes()).unwrap();
    println!("config succesfully writed!");
}
