use std::{io::Read, net::TcpStream};

use serde::Deserialize;

use crate::{Hardware, HardwareBridge};

pub struct WindowsBridge {}

const IP: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 55555;

#[derive(Deserialize, Debug, Clone)]
enum HardwareType {
    Control,
    Fan,
    Temp,
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

impl HardwareBridge for WindowsBridge {
    fn generate_hardware() -> Hardware {
        let hardware = Hardware::default();

        let mut stream = try_connect();
        println!("Connected to the server!");

        let mut data = String::new();
        stream.read_to_string(&mut data).unwrap();
        let base_hardware_list = serde_json::from_str::<Vec<BaseHardware>>(&data).unwrap();

        dbg!(&base_hardware_list);

        hardware
    }
}

fn try_connect() -> TcpStream {
    for port in DEFAULT_PORT..65535 {
        match TcpStream::connect((IP, port)) {
            Ok(stream) => return stream,
            Err(_) => continue,
        }
    }
    panic!("can't find connection")
}
