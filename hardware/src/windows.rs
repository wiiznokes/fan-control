use std::{net::{TcpStream}, io::{Read, Write}};

use crate::{HardwareBridge, Hardware};

pub struct WindowsBridge {}

const IP: &str = "127.0.0.1";
const DEFAULT_PORT: u16  = 55555;


impl HardwareBridge for WindowsBridge {
    fn generate_hardware() -> Hardware {
        
        let hardware = Hardware::default();

        let mut stream = try_connect();

        println!("Connected to the server!");

        let message = "Hello, from Rust";
        stream.write_all(message.as_bytes()).unwrap();


        let mut response = String::new();
        let mut buf = [0, 255u8];

        let mut data = vec![0; 1024];

        stream.read(&mut data).unwrap();
        let data_string = String::from_utf8_lossy(&data);

        println!("Server response: {}", data_string);
    
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