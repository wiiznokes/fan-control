use const_format::formatcp;
use hardware::{ControlH, FanH, Hardware, HardwareItem, TempH};
use serial_test::serial;
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::config::Config;

use crate::settings::Settings;

use super::control::Control;
use super::custom_temp::{CustomTemp, CustomTempKind};

use super::fan::Fan;
use super::flat::Flat;
use super::graph::{Coord, Graph};
use super::linear::Linear;
use super::target::Target;
use super::temp::Temp;

const SETTINGS_DIR_PATH: &str = "./test/config/";

const SETTINGS_PATH: &str = formatcp!("{SETTINGS_DIR_PATH}settings.toml");
const HARDWARE_PATH: &str = formatcp!("{SETTINGS_DIR_PATH}hardware.toml");
const CONFIG_PATH_TOML: &str = formatcp!("{SETTINGS_DIR_PATH}config1.toml");
const CONFIG_PATH_JSON: &str = formatcp!("{SETTINGS_DIR_PATH}config1.json");

#[test]
#[serial]
fn check_deserialization() {
    parse_file(SETTINGS_PATH, false, |content| {
        toml::from_str::<Settings>(content)
    });
    parse_file(CONFIG_PATH_TOML, false, |content| {
        toml::from_str::<Config>(content)
    });

    parse_file(CONFIG_PATH_JSON, false, |content| {
        serde_json::from_str::<Config>(content)
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

#[derive(Debug)]
struct T {}

impl HardwareItem for T {
    fn get_value(&self) -> Result<hardware::Value, hardware::HardwareError> {
        todo!()
    }

    fn set_value(&self, _value: hardware::Value) -> Result<(), hardware::HardwareError> {
        todo!()
    }

    fn set_mode(&self, _value: hardware::Value) -> Result<(), hardware::HardwareError> {
        todo!()
    }
}

fn hardware1() -> Hardware {
    Hardware {
        controls: vec![ControlH {
            name: "ControlH".into(),
            hardware_id: "ControlH".into(),
            info: "ControlH".into(),
            bridge: Box::new(T {}),
        }
        .into()],
        temps: vec![TempH {
            name: "TempH".into(),
            hardware_id: "TempH".into(),
            info: "TempH".into(),
            bridge: Box::new(T {}),
        }
        .into()],
        fans: vec![FanH {
            name: "FanH".into(),
            hardware_id: "FanH".into(),
            info: "FanH".into(),
            bridge: Box::new(T {}),
        }
        .into()],
    }
}

fn config1() -> Config {
    Config {
        controls: vec![Control::new(
            "Control".into(),
            Some("Control".into()),
            None,
            true,
            None,
        )],
        temps: vec![Temp {
            name: "Temp".into(),
            hardware_id: Some("temp".into()),
            temp_h: None,
        }],
        fans: vec![Fan {
            name: "Fan".into(),
            hardware_id: None,
            fan_h: None,
        }],
        custom_temps: vec![CustomTemp {
            name: "CustomTemp".into(),
            kind: CustomTempKind::Max,
            input: vec!["temp1".into(), "temp2".into()],
        }],
        graphs: vec![Graph {
            name: "Graph".into(),
            coords: vec![
                Coord {
                    temp: 10,
                    percent: 10,
                },
                Coord {
                    temp: 50,
                    percent: 30,
                },
            ],
            input: Some("max".into()),
        }],
        flats: vec![Flat {
            name: "flat1".into(),
            value: 50,
        }],
        linears: vec![Linear::new(
            "Linear".into(),
            10,
            10,
            70,
            100,
            Some("temp3".into()),
        )],
        targets: vec![Target {
            name: "Target".into(),
            idle_temp: 40,
            idle_speed: 10,
            load_temp: 70,
            load_speed: 100,
            input: Some("temp3".into()),
            idle_has_been_reatch: false,
        }],
    }
}
