use std::fmt::Debug;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use const_format::formatcp;

use crate::conf::configs::{
    Config, Coord, CustomTemp, CustomTempType, Flat, Graph, Linear, Target,
};
use crate::conf::hardware::{Control, Fan, Hardware, Temp};
use crate::conf::settings::Settings;

const SETTINGS_DIR_PATH: &str = "./test/config/";

const SETTINGS_PATH: &str = formatcp!("{SETTINGS_DIR_PATH}settings.toml");
const HARDWARE_PATH: &str = formatcp!("{SETTINGS_DIR_PATH}hardware.toml");
const CONFIG_PATH_TOML: &str = formatcp!("{SETTINGS_DIR_PATH}config1.toml");
const CONFIG_PATH_JSON: &str = formatcp!("{SETTINGS_DIR_PATH}config1.json");

#[test]
fn serialize() {
    let _ = fs::create_dir_all(SETTINGS_DIR_PATH);

    write_file(Path::new(SETTINGS_PATH), || {
        let settings = Settings::default();
        toml::to_string_pretty(&settings)
    });

    write_file(Path::new(HARDWARE_PATH), || {
        let hardware1 = hardware1();
        toml::to_string_pretty(&hardware1)
    });

    let config1 = config1();

    write_file(Path::new(CONFIG_PATH_TOML), || {
        toml::to_string_pretty(&config1)
    });

    write_file(Path::new(CONFIG_PATH_JSON), || {
        toml::to_string_pretty(&config1)
    });
}

#[test]
fn check_deserialization() {
    if let Ok(content) = fs::read_to_string(Path::new(SETTINGS_PATH)) {
        let output: Settings = toml::from_str(&content).unwrap();
        dbg!(output);
    }

    if let Ok(content) = fs::read_to_string(Path::new(HARDWARE_PATH)) {
        let output: Hardware = toml::from_str(&content).unwrap();
        dbg!(output);
    }

    if let Ok(content) = fs::read_to_string(Path::new(CONFIG_PATH_TOML)) {
        let output: Config = toml::from_str(&content).unwrap();
        dbg!(output);
    }

    if let Ok(content) = fs::read_to_string(Path::new(CONFIG_PATH_JSON)) {
        let _: Config = toml::from_str(&content).unwrap();
    }
}

fn write_file<T: Debug>(path: &Path, content_generation: impl Fn() -> Result<String, T>) {
    println!("write file: {}", path.to_string_lossy());

    if path.exists() {
        fs::remove_file(path).unwrap();
    }

    let mut file = File::create(path).unwrap();

    let content = content_generation().unwrap();
    file.write_all(content.as_bytes()).unwrap();

    println!("file {} succesfully writed!", path.to_string_lossy());
}

fn hardware1() -> Hardware {
    Hardware {
        controls: vec![
            Control {
                name: "control1".into(),
            },
            Control {
                name: "control2".into(),
            },
            Control {
                name: "control3".into(),
            },
            Control {
                name: "control4".into(),
            },
        ],
        temps: vec![
            Temp {
                name: "temp1".into(),
            },
            Temp {
                name: "temp2".into(),
            },
            Temp {
                name: "temp3".into(),
            },
        ],
        fans: vec![Fan {
            name: "fan1".into(),
        }],
    }
}

pub fn config1() -> Config {
    Config {
        custom_temps: vec![CustomTemp {
            name: "max".into(),
            kind: CustomTempType::Max,
            input: vec!["temp1".into(), "temp2".into()],
        }],
        graphs: vec![Graph {
            name: "graph1".into(),
            coords: vec![
                Coord {
                    temp: 10,
                    percent: 10,
                },
                Coord {
                    temp: 50,
                    percent: 30,
                },
                Coord {
                    temp: 90,
                    percent: 100,
                },
            ],
            input: "max".into(),
            output: vec!["control1".into()],
        }],
        flats: vec![
            Flat {
                name: "flat1".into(),
                value: 50,
                output: vec!["control2".into()],
            },
            Flat {
                name: "flat2".into(),
                value: 100,
                output: vec![],
            },
        ],
        linears: vec![Linear {
            name: "graph1".into(),
            min_temp: 10,
            min_speed: 10,
            max_temp: 70,
            max_speed: 100,
            input: "temp3".into(),
            output: vec!["control3".into()],
        }],
        targets: vec![Target {
            name: "graph1".into(),

            idle_temp: 40,
            idle_speed: 10,
            load_temp: 70,
            load_speed: 100,
            input: "temp3".into(),
            output: vec!["control4".into()],
        }],
    }
}
