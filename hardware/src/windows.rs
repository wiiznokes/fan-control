use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    rc::Rc,
};

use serde::Deserialize;

use crate::{ControlH, Hardware, HardwareBridge, HardwareError, HardwareItem, Value};

pub struct WindowsBridge {}

const IP: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 55555;

impl HardwareBridge for WindowsBridge {
    fn generate_hardware() -> Hardware {
        let mut hardware = Hardware::default();

        let stream = try_connect();

        let mut data = String::new();
        let mut buff_reader = BufReader::new(&stream);
        buff_reader.read_line(&mut data).unwrap();
        let base_hardware_list = serde_json::from_str::<Vec<BaseHardware>>(&data).unwrap();

        for base_hardware in base_hardware_list {
            match base_hardware.hardware_type {
                HardwareType::Control => hardware.controls.push(Rc::new(ControlH {
                    name: base_hardware.name,
                    hardware_id: base_hardware.id,
                    info: String::new(),
                    bridge: Box::new(InternalControl {
                        index: base_hardware.index,
                    }),
                })),
                HardwareType::Fan => hardware.controls.push(Rc::new(ControlH {
                    name: base_hardware.name,
                    hardware_id: base_hardware.id,
                    info: String::new(),
                    bridge: Box::new(InternalSensor {
                        index: base_hardware.index,
                    }),
                })),
                HardwareType::Temp => hardware.controls.push(Rc::new(ControlH {
                    name: base_hardware.name,
                    hardware_id: base_hardware.id,
                    info: String::new(),
                    bridge: Box::new(InternalSensor {
                        index: base_hardware.index,
                    }),
                })),
            }
        }

        hardware
    }
}

fn try_connect() -> TcpStream {
    for port in DEFAULT_PORT..65535 {
        match TcpStream::connect((IP, port)) {
            Ok(stream) => {
                info!("connected to {}:{}", IP, port);
                return stream;
            }
            Err(_) => continue,
        }
    }
    panic!("can't find connection")
}

#[derive(Deserialize, Debug, Clone)]
enum HardwareType {
    Control = 1,
    Fan = 2,
    Temp = 3,
}

#[derive(Debug, Clone)]
enum Command {
    SetAuto = 1,
    SetValue = 2,

    // command -> type -> index -> value
    GetValue = 3,
    Shutdown = 4,
}

#[derive(Deserialize, Debug, Clone)]
struct BaseHardware {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Index")]
    index: usize,
    #[serde(rename = "Type")]
    hardware_type: HardwareType,
}

#[derive(Debug)]
struct InternalSensor {
    index: usize,
}

#[derive(Debug)]
struct InternalControl {
    index: usize,
}

impl Drop for InternalControl {
    fn drop(&mut self) {
        info!("pwm sould be set to auto");
        // TODO: set to auto
    }
}

impl HardwareItem for InternalSensor {
    fn get_value(&self) -> Result<Value, crate::HardwareError> {
        Ok(4)
    }

    fn set_value(&self, value: Value) -> Result<(), crate::HardwareError> {
        panic!("can't set the value of a sensor");
    }

    fn set_mode(&self, value: Value) -> Result<(), HardwareError> {
        panic!("can't set the mode of a sensor");
    }
}

impl HardwareItem for InternalControl {
    fn get_value(&self) -> Result<Value, crate::HardwareError> {
        panic!("can't get the value of a control");
    }

    fn set_value(&self, value: Value) -> Result<(), crate::HardwareError> {
        debug!("set value {} to a control", value);
        Ok(())
    }

    fn set_mode(&self, value: Value) -> Result<(), HardwareError> {
        debug!("set mode {} to a control", value);
        Ok(())
    }
}
