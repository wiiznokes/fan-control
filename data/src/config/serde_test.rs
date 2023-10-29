use const_format::formatcp;
use hardware::Hardware;
use serial_test::serial;
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::config::Config;

use crate::settings::Settings;

use super::custom_temp::CustomTempType;

use super::flat::Flat;
use super::graph::{Coord, Graph};
use super::linear::Linear;
use super::target::Target;

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

fn hardware1() -> Hardware {
    Hardware {
        controls: vec![
            test_helper::control_h("control1"),
            test_helper::control_h("control2"),
            test_helper::control_h("control3"),
            test_helper::control_h("control4"),
        ],
        temps: vec![
            test_helper::temp_h("temp1"),
            test_helper::temp_h("temp2"),
            test_helper::temp_h("temp3"),
        ],
        fans: vec![test_helper::fan_h("fan1"), test_helper::fan_h("fan2")],
    }
}

fn config1() -> Config {
    Config {
        controls: vec![
            test_helper::control("control1", None, true),
            test_helper::control("control1", Some("flat1"), true),
            test_helper::control("control1", Some("target1"), false),
        ],
        temps: vec![test_helper::temp("temp1"), test_helper::temp("temp2")],
        fans: vec![test_helper::fan("fan1"), test_helper::fan("fan2")],
        custom_temps: vec![test_helper::custom_temp(
            "max",
            CustomTempType::Max,
            vec!["temp1", "temp2"],
        )],
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
            input: Some("max".into()),
        }],
        flats: vec![
            Flat {
                name: "flat1".into(),
                value: 50,
            },
            Flat {
                name: "flat2".into(),
                value: 100,
            },
        ],
        linears: vec![Linear {
            name: "graph1".into(),
            min_temp: 10,
            min_speed: 10,
            max_temp: 70,
            max_speed: 100,
            input: Some("temp3".into()),
        }],
        targets: vec![Target {
            name: "graph1".into(),

            idle_temp: 40,
            idle_speed: 10,
            load_temp: 70,
            load_speed: 100,
            input: Some("temp3".into()),
        }],
    }
}

mod test_helper {
    use hardware::{ControlH, FanH, TempH};

    use crate::config::{
        control::Control,
        custom_temp::{CustomTemp, CustomTempType},
        fan::Fan,
        temp::Temp,
    };

    pub fn control_h(name: &str) -> ControlH {
        ControlH {
            name: name.into(),
            hardware_id: name.into(),
            info: name.into(),
        }
    }
    pub fn temp_h(name: &str) -> TempH {
        TempH {
            name: name.into(),
            hardware_id: name.into(),
            info: name.into(),
        }
    }
    pub fn fan_h(name: &str) -> FanH {
        FanH {
            name: name.into(),
            hardware_id: name.into(),
            info: name.into(),
        }
    }

    pub fn control(name: &str, input: Option<&str>, auto: bool) -> Control {
        Control {
            name: name.into(),
            hardware_id: Some(name.into()),
            input: input.map(|i| i.into()),
            auto,
        }
    }
    pub fn temp(name: &str) -> Temp {
        Temp {
            name: name.into(),
            hardware_id: Some(name.into()),
        }
    }
    pub fn fan(name: &str) -> Fan {
        Fan {
            name: name.into(),
            hardware_id: Some(name.into()),
        }
    }

    pub fn custom_temp(name: &str, kind: CustomTempType, input: Vec<&str>) -> CustomTemp {
        CustomTemp {
            name: name.into(),
            kind,
            input: vec_to_string(input),
        }
    }

    pub fn vec_to_string(v: Vec<&str>) -> Vec<String> {
        v.iter().map(|e| e.to_string()).collect()
    }
}
