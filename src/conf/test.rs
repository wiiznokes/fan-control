use const_format::formatcp;
use serial_test::serial;
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::conf::configs::{
    Config, Coord, CustomTemp, CustomTempType, Flat, Graph, Linear, Target,
};
use crate::conf::hardware::{Control, Fan, Hardware, Temp};
use crate::conf::settings::Settings;

use super::hardware::FetchHardware;

const SETTINGS_DIR_PATH: &str = "./test/config/";

const SETTINGS_PATH: &str = formatcp!("{SETTINGS_DIR_PATH}settings.toml");
const HARDWARE_PATH: &str = formatcp!("{SETTINGS_DIR_PATH}hardware.toml");
const CONFIG_PATH_TOML: &str = formatcp!("{SETTINGS_DIR_PATH}config1.toml");
const CONFIG_PATH_JSON: &str = formatcp!("{SETTINGS_DIR_PATH}config1.json");

#[test]
#[serial]
fn check_deserialization() {
    parse_file(SETTINGS_PATH, false, |content| {
        toml::from_str::<Settings>(&content)
    });
    parse_file(HARDWARE_PATH, false, |content| {
        toml::from_str::<Hardware<String, String>>(&content)
    });
    parse_file(CONFIG_PATH_TOML, false, |content| {
        toml::from_str::<Config>(&content)
    });

    parse_file(CONFIG_PATH_JSON, false, |content| {
        serde_json::from_str::<Config>(&content)
    });
}

fn parse_file<T: Debug, E: Debug>(
    path: &str,
    print: bool,
    struct_generation: impl Fn(&String) -> Result<T, E>,
) {
    println!("read file: {}", path);
    if let Ok(content) = fs::read_to_string(Path::new(path)) {
        let output: T = struct_generation(&content).unwrap();
        if print {
            dbg!(output);
        }
    }
    println!("file {} succesfully parsed!", path);
}

#[test]
#[serial]
fn serialize() {
    let _ = fs::create_dir_all(SETTINGS_DIR_PATH);

    write_file(SETTINGS_PATH, || {
        let settings = Settings::default();
        toml::to_string_pretty(&settings)
    });

    write_file(HARDWARE_PATH, || {
        let hardware1 = hardware1();
        toml::to_string_pretty(&hardware1)
    });

    let config1 = config1();

    write_file(CONFIG_PATH_TOML, || toml::to_string_pretty(&config1));

    write_file(CONFIG_PATH_JSON, || serde_json::to_string_pretty(&config1));
}

fn write_file<E: Debug>(path: &str, content_generation: impl Fn() -> Result<String, E>) {
    println!("write file: {}", path);

    let path_fs = Path::new(path);
    if path_fs.exists() {
        fs::remove_file(path).unwrap();
    }

    let mut file = File::create(path).unwrap();

    let content = content_generation().unwrap();
    file.write_all(content.as_bytes()).unwrap();

    println!("file {} succesfully writed!", path);
}

fn hardware1<S: FetchHardware>() -> Hardware<S> {
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
                sensor: S::new("temp1".into())
            },
            Temp {
                name: "temp2".into(),
                sensor: "".into(),
            },
            Temp {
                name: "temp3".into(),
                sensor: "".into(),
            },
        ],
        fans: vec![Fan {
            name: "fan1".into(),
            sensor: "".into(),
        }],
    }
}

fn config1() -> Config {
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
