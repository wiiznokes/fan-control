use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
    rc::Rc,
};

use serde::Deserialize;

use crate::{
    ControlH, FanH, Hardware, HardwareBridge, HardwareBridgeT, HardwareError, InternalControlIndex,
    TempH, Value,
};

pub struct WindowsBridge {
    pub stream: TcpStream,
}

const IP: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 55555;

impl HardwareBridge for WindowsBridge {
    fn generate_hardware() -> (Hardware, HardwareBridgeT) {
        let mut hardware = Hardware::default();

        let stream = try_connect();

        let mut data = String::new();
        let mut buf_reader = BufReader::new(&stream);
        buf_reader.read_line(&mut data).unwrap();
        let base_hardware_list = serde_json::from_str::<Vec<BaseHardware>>(&data).unwrap();

        for base_hardware in base_hardware_list {
            match base_hardware.hardware_type {
                HardwareType::Control => hardware.controls.push(Rc::new(ControlH {
                    name: base_hardware.name,
                    hardware_id: base_hardware.id,
                    info: String::new(),
                    internal_index: base_hardware.index,
                })),
                HardwareType::Fan => hardware.fans.push(Rc::new(FanH {
                    name: base_hardware.name,
                    hardware_id: base_hardware.id,
                    info: String::new(),
                    internal_index: base_hardware.index,
                })),
                HardwareType::Temp => hardware.temps.push(Rc::new(TempH {
                    name: base_hardware.name,
                    hardware_id: base_hardware.id,
                    info: String::new(),
                    internal_index: base_hardware.index,
                })),
            }
        }

        let windows_bridge = WindowsBridge { stream };

        (hardware, Box::new(windows_bridge))
    }

    fn get_value(&mut self, internal_index: &usize) -> Result<Value, HardwareError> {
        info!("send command: {:?}", Command::GetValue);

        let command: &[u8; 4] = &From::from(Command::GetValue);
        self.stream.write_all(command).unwrap();

        info!("send index: {}", internal_index);
        let index: &[u8; 4] = &From::from(I32(*internal_index));
        self.stream.write_all(index).unwrap();

        info!("read value ...");
        let mut buf = [0u8; 4];
        self.stream.read_exact(&mut buf).unwrap();
        info!("read value!");

        let i32 = I32::from(buf);
        Ok(i32.0)
    }

    fn set_value(&mut self, internal_index: &usize, value: Value) -> Result<(), HardwareError> {
        todo!()
    }

    fn set_mode(&mut self, internal_index: &usize, value: Value) -> Result<(), HardwareError> {
        todo!()
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
#[repr(i32)]
enum Command {
    SetAuto = 1,
    SetValue = 2,
    GetValue = 3,
    Shutdown = 4,
}

impl From<Command> for [u8; 4] {
    #[inline]
    fn from(command: Command) -> Self {
        let command_value = command as u32;
        if is_little_endian() {
            command_value.to_le_bytes()
        } else {
            command_value.to_be_bytes()
        }
    }
}

struct I32<T>(T);

impl From<[u8; 4]> for I32<i32> {
    #[inline]
    fn from(bytes: [u8; 4]) -> Self {
        if is_little_endian() {
            I32(i32::from_le_bytes(bytes))
        } else {
            I32(i32::from_be_bytes(bytes))
        }
    }
}

impl From<I32<usize>> for [u8; 4] {
    #[inline]
    fn from(number: I32<usize>) -> Self {
        let index = number.0 as i32;
        if is_little_endian() {
            index.to_le_bytes()
        } else {
            index.to_be_bytes()
        }
    }
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

#[inline]
fn is_little_endian() -> bool {
    let test_value: u16 = 1;
    let test_ptr: *const u16 = &test_value;

    // Read the first byte of the u16 through the pointer
    let byte = unsafe { *test_ptr as u8 };

    // If the byte is 1, the system is little-endian; otherwise, it's big-endian
    byte == 1
}
