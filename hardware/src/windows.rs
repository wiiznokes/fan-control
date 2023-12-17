use std::{
    io::{self, BufRead, BufReader, Read, Write},
    net::TcpStream,
    process,
    rc::Rc,
    thread,
    time::Duration,
};

use serde::Deserialize;
use thiserror::Error;

use crate::{ControlH, FanH, Hardware, HardwareBridge, HardwareBridgeT, TempH, Value};

use cargo_packager_resource_resolver as resource_resolver;

use self::packet::{Command, Packet, I32};

pub struct WindowsBridge {
    process_handle: std::process::Child,
    stream: TcpStream,
}

#[derive(Error, Debug)]
pub enum WindowsError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("No connection was found")]
    NoConnectionFound,
    #[error(transparent)]
    Resource(#[from] resource_resolver::error::Error),
    #[error("Failed to parse hardware struct: {0}")]
    JSONConfigParseError(#[from] serde_json::Error),
}

type Result<T> = std::result::Result<T, WindowsError>;

fn spawn_windows_server() -> Result<std::process::Child> {
    let exe_path = resource_resolver::resource_dir_with_suffix("resource")?
        .join("windows/build/LibreHardwareMonitorWrapper");
    let handle = process::Command::new(exe_path).spawn()?;
    Ok(handle)
}

fn try_connect() -> Result<TcpStream> {
    const IP: &str = "127.0.0.1";
    const DEFAULT_PORT: u16 = 55555;
    // need to have different values because
    // i think we can write the TcpSteam and
    // then read what we write
    const CHECK: &str = "fan-control-check";
    const CHECK_RESPONSE: &str = "fan-control-ok";

    for i in 0..10 {
        for port in DEFAULT_PORT..65535 {
            match TcpStream::connect((IP, port)) {
                Ok(mut stream) => {
                    let write_buf = CHECK.as_bytes();

                    if let Err(_e) = stream.write_all(write_buf) {
                        continue;
                    }

                    let Ok(prev_timeout) = stream.read_timeout() else {
                        continue;
                    };
                    if let Err(_e) = stream.set_read_timeout(Some(Duration::from_millis(10))) {
                        continue;
                    }

                    let mut read_buf = [0u8; CHECK_RESPONSE.len()];

                    if let Err(_e) = stream.read_exact(&mut read_buf) {
                        continue;
                    }

                    let Ok(str) = std::str::from_utf8(&read_buf) else {
                        continue;
                    };

                    if str != CHECK_RESPONSE {
                        continue;
                    }

                    stream.set_read_timeout(prev_timeout)?;

                    info!("check passed for {}:{}!", IP, port);
                    return Ok(stream);
                }
                Err(_) => continue,
            }
        }
        thread::sleep(Duration::from_millis(50 * i))
    }

    Err(WindowsError::NoConnectionFound)
}

fn read_hardware(stream: &TcpStream) -> Result<Hardware> {
    let mut hardware = Hardware::default();

    let mut data = String::new();
    let mut buf_reader = BufReader::new(stream);
    buf_reader.read_line(&mut data)?;

    #[derive(Deserialize)]
    enum HardwareType {
        Control = 1,
        Fan = 2,
        Temp = 3,
    }

    #[derive(Deserialize)]
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

    let base_hardware_list = serde_json::from_str::<Vec<BaseHardware>>(&data)?;

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

    Ok(hardware)
}

mod packet {
    #[derive(Debug, Clone)]
    #[repr(i32)]
    pub enum Command {
        SetAuto = 1,
        SetValue = 2,
        GetValue = 3,
        Shutdown = 4,
        Update = 5,
    }

    pub struct I32<T>(pub T);

    pub type Packet = [u8; 4];

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

    impl From<I32<i32>> for [u8; 4] {
        #[inline]
        fn from(number: I32<i32>) -> Self {
            if is_little_endian() {
                number.0.to_le_bytes()
            } else {
                number.0.to_be_bytes()
            }
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
}

impl WindowsBridge {
    fn send(&mut self, command: impl Into<Packet>) -> Result<()> {
        let packet: Packet = command.into();
        self.stream.write_all(&packet)?;
        Ok(())
    }

    fn read<T>(&mut self) -> Result<T>
    where
        T: From<Packet>,
    {
        let mut buf: Packet = [0u8; 4];
        self.stream.read_exact(&mut buf)?;
        Ok(buf.into())
    }

    fn close_and_wait_server(&mut self) -> Result<()> {
        self.send(Command::Shutdown)?;
        let status = self.process_handle.wait()?;
        if !status.success() {
            let io_error = io::Error::new(
                io::ErrorKind::InvalidData,
                "the windows server terminated with an error",
            );
            return Err(WindowsError::Io(io_error));
        }
        Ok(())
    }
}

impl HardwareBridge for WindowsBridge {
    fn generate_hardware() -> crate::Result<(Hardware, HardwareBridgeT)> {
        let process_handle = spawn_windows_server()?;
        let stream = try_connect()?;

        let hardware = read_hardware(&stream)?;

        let windows_bridge = WindowsBridge {
            process_handle,
            stream,
        };

        Ok((hardware, Box::new(windows_bridge)))
    }

    fn get_value(&mut self, internal_index: &usize) -> crate::Result<Value> {
        self.send(Command::GetValue)?;
        self.send(I32(*internal_index))?;

        let value = self.read::<I32<i32>>()?;
        Ok(value.0)
    }

    fn set_value(&mut self, internal_index: &usize, value: Value) -> crate::Result<()> {
        self.send(Command::SetValue)?;
        self.send(I32(*internal_index))?;
        self.send(I32(value))?;
        Ok(())
    }

    fn set_mode(&mut self, internal_index: &usize, value: Value) -> crate::Result<()> {
        if value != 0 {
            debug!("try to set {}, whitch is unecessary on Windows", value);
            return Ok(());
        }

        self.send(Command::SetAuto)?;
        self.send(I32(*internal_index))?;
        Ok(())
    }

    fn update(&mut self) -> crate::Result<()> {
        self.send(Command::Update)?;
        Ok(())
    }

    fn shutdown(&mut self) -> crate::Result<()> {
        self.close_and_wait_server()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::WindowsBridge;
    use crate::{Hardware, HardwareBridge, HardwareBridgeT};
    use std::{
        thread::sleep,
        time::{Duration, Instant},
    };

    #[test]
    fn test_time() {
        let now = Instant::now();
        let (hardware, mut bridge) = WindowsBridge::generate_hardware().unwrap();
        println!("generation took {} millis", now.elapsed().as_millis());

        for _ in 0..5 {
            bench(
                || {
                    update(&mut bridge, &hardware);
                    "all sensors".to_string()
                },
                "update",
            );
            sleep(Duration::from_millis(500))
        }
    }

    fn update(bridge: &mut HardwareBridgeT, hardware: &Hardware) {
        println!();

        bench(
            || {
                bridge.update().unwrap();
                "lhm".to_string()
            },
            "update",
        );

        for h in &hardware.controls {
            get_value(bridge, &h.internal_index, &h.name);
        }
        for h in &hardware.temps {
            get_value(bridge, &h.internal_index, &h.name);
        }
        for h in &hardware.fans {
            get_value(bridge, &h.internal_index, &h.name);
        }
    }

    fn get_value(bridge: &mut HardwareBridgeT, index: &usize, name: &str) {
        bench(
            || match bridge.get_value(index) {
                Ok(value) => {
                    format!("{} = {}", name, value)
                }
                Err(e) => {
                    format!("error for {}: {}", name, e)
                }
            },
            "get_value",
        );
    }

    fn bench(f: impl FnOnce() -> String, info: &str) {
        let now = Instant::now();
        let output = f();
        println!(
            "{}: {} in {} millis",
            info,
            output,
            now.elapsed().as_millis()
        );
    }
}
