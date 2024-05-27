use std::{
    io::{self, BufRead, BufReader, Read, Write},
    net::TcpStream,
    process::{self},
    rc::Rc,
    thread,
    time::Duration,
};

use serde::Deserialize;
use thiserror::Error;

use crate::{HControl, HSensor, Hardware, HardwareBridge, Mode, Value};

use self::packet::{command::Command, i32::I32, Packet};

pub struct WindowsBridge {
    process_handle: std::process::Child,
    stream: TcpStream,
    hardware: Hardware,
}

#[derive(Error, Debug)]
pub enum WindowsError {
    #[error("{0}: {1}")]
    Io(String, std::io::Error),
    #[error("Can't spawn the windows server: {0}")]
    SpawnServer(std::io::Error),
    #[error("No connection was found")]
    NoConnectionFound,
    #[error("Failed to parse hardware struct: {0}")]
    JSONConfigParseError(#[from] serde_json::Error),
}

type Result<T> = std::result::Result<T, WindowsError>;

fn spawn_windows_server() -> Result<std::process::Child> {
    let resource_path = if cfg!(test) {
        std::path::PathBuf::from("../res".to_string())
    } else {
        utils::resource_dir()
    };

    let exe_path = resource_path.join("lhmbuild/LibreHardwareMonitorWrapper");

    let mut command = process::Command::new(exe_path);

    if !log_enabled!(log::Level::Info) {
        use std::os::windows::process::CommandExt;

        // https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
        // because of this line, we loose the ability to see logs of the child process
        // with the benefit of no console poping
        command.creation_flags(0x08000000);

        info!("Output for Windows server will be discarded.");
    }

    if log_enabled!(log::Level::Debug) {
        command.arg("--log=debug");
    } else if log_enabled!(log::Level::Info) {
        command.arg("--log=info");
    }

    debug!("Command to launch Windows server: {:?}.", command);

    match command.spawn() {
        Ok(handle) => Ok(handle),
        Err(e) => Err(WindowsError::SpawnServer(e)),
    }
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

                    if let Err(e) = stream.set_read_timeout(prev_timeout) {
                        return Err(WindowsError::Io("can't set read timeout back".into(), e));
                    }

                    info!("Check passed for {}:{}.", IP, port);
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

    if let Err(e) = buf_reader.read_line(&mut data) {
        return Err(WindowsError::Io(
            "can't read hardware data from socket".into(),
            e,
        ));
    }

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
            HardwareType::Control => hardware.controls.push(Rc::new(HControl {
                name: base_hardware.name,
                hardware_id: base_hardware.id,
                info: String::new(),
                internal_index: base_hardware.index,
            })),
            HardwareType::Fan => hardware.fans.push(Rc::new(HSensor {
                name: base_hardware.name,
                hardware_id: base_hardware.id,
                info: String::new(),
                internal_index: base_hardware.index,
            })),
            HardwareType::Temp => hardware.temps.push(Rc::new(HSensor {
                name: base_hardware.name,
                hardware_id: base_hardware.id,
                info: String::new(),
                internal_index: base_hardware.index,
            })),
        }
    }

    info!("Hardware was succefully received.");
    Ok(hardware)
}

mod packet {

    pub struct Packet(pub [u8; 4]);

    pub mod command {
        use super::Packet;

        #[derive(Debug, Clone, PartialEq, Eq)]
        #[repr(i32)]
        pub enum Command {
            GetHardware = 0,
            SetAuto = 1,
            SetValue = 2,
            GetValue = 3,
            Shutdown = 4,
            Update = 5,
        }

        impl From<Command> for Packet {
            #[inline]
            fn from(command: Command) -> Self {
                let bytes = (command as i32).to_ne_bytes();
                Packet(bytes)
            }
        }
    }

    pub mod i32 {
        use super::Packet;

        pub struct I32(pub i32);

        impl From<I32> for Packet {
            #[inline]
            fn from(number: I32) -> Self {
                let bytes = number.0.to_ne_bytes();
                Packet(bytes)
            }
        }

        impl From<Packet> for I32 {
            #[inline]
            fn from(packet: Packet) -> Self {
                let number = i32::from_ne_bytes(packet.0);
                I32(number)
            }
        }

        impl From<usize> for I32 {
            fn from(value: usize) -> Self {
                let value: i32 = (value).try_into().expect("Can't convert usize to i32.");
                I32(value)
            }
        }
    }
}

impl WindowsBridge {
    fn send(&mut self, packet: impl Into<Packet>) -> Result<()> {
        let packet: Packet = packet.into();
        if let Err(e) = self.stream.write_all(&packet.0) {
            return Err(WindowsError::Io("can't send packet".into(), e));
        }

        Ok(())
    }

    fn read<T>(&mut self) -> Result<T>
    where
        T: From<Packet>,
    {
        let mut buf: Packet = Packet([0u8; 4]);
        if let Err(e) = self.stream.read_exact(&mut buf.0) {
            return Err(WindowsError::Io("can't read packet".into(), e));
        }

        Ok(buf.into())
    }

    fn close_and_wait_server(&mut self) -> Result<()> {
        self.send(Command::Shutdown)?;

        match self.process_handle.wait() {
            Ok(status) => {
                if !status.success() {
                    let io_error = io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("exit status: {:?}", status.code()),
                    );
                    return Err(WindowsError::Io(
                        "wrong Windows server exit status".into(),
                        io_error,
                    ));
                }
            }
            Err(e) => {
                return Err(WindowsError::Io(
                    "can't wait for the server to finish".into(),
                    e,
                ))
            }
        };

        Ok(())
    }
}

impl HardwareBridge for WindowsBridge {
    const TIME_TO_UPDATE: Duration = Duration::from_millis(250);

    fn new() -> crate::Result<Self> {
        let process_handle = spawn_windows_server()?;
        let stream = try_connect()?;

        let mut windows_bridge = WindowsBridge {
            process_handle,
            stream,
            hardware: Hardware::default(),
        };

        windows_bridge.send(Command::GetHardware)?;
        windows_bridge.hardware = read_hardware(&windows_bridge.stream)?;

        Ok(windows_bridge)
    }
    fn hardware(&self) -> &Hardware {
        &self.hardware
    }

    fn get_sensor_value(&mut self, sensor: &HSensor) -> crate::Result<Value> {
        self.send(Command::GetValue)?;
        self.send(I32::from(sensor.internal_index))?;

        let value = self.read::<I32>()?;
        Ok(value.0)
    }

    fn get_control_value(&mut self, control: &HControl) -> crate::Result<Value> {
        self.send(Command::GetValue)?;
        self.send(I32::from(control.internal_index))?;

        let value = self.read::<I32>()?;
        Ok(value.0)
    }

    fn set_value(&mut self, control: &HControl, value: Value) -> crate::Result<()> {
        self.send(Command::SetValue)?;
        self.send(I32::from(control.internal_index))?;
        self.send(I32(value))?;
        Ok(())
    }

    fn set_mode(&mut self, control: &HControl, mode: &Mode) -> crate::Result<()> {
        if mode == &Mode::Manual {
            debug!(
                "An attempt was made to set the mode to manual on control {}, which is not necessary under Windows.",
                control.name
            );
            return Ok(());
        }

        self.send(Command::SetAuto)?;
        self.send(I32::from(control.internal_index))?;
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
    use crate::HardwareBridge;
    use std::{
        thread::sleep,
        time::{Duration, Instant},
    };

    fn init_test_logging() {
        let _ = env_logger::builder().format_timestamp(None).try_init();
    }

    #[test]
    fn test_time() {
        init_test_logging();

        let now = Instant::now();

        let mut bridge = WindowsBridge::new().unwrap();

        info!("generation took {} millis", now.elapsed().as_millis());

        for _ in 0..5 {
            bench(
                || {
                    update(&mut bridge);
                    "all sensors".to_string()
                },
                "update",
            );
            sleep(Duration::from_millis(500))
        }
        bridge.shutdown().unwrap();
    }

    fn update(bridge: &mut WindowsBridge) {
        info!("");

        bridge.update().unwrap();
        std::thread::sleep(WindowsBridge::TIME_TO_UPDATE);

        for h in &bridge.hardware().controls.clone() {
            bench(
                || match bridge.get_control_value(h) {
                    Ok(value) => {
                        format!("{} = {}", h.name, value)
                    }
                    Err(e) => {
                        format!("error for {}: {}", h.name, e)
                    }
                },
                "get_value",
            );
        }
        for h in &bridge.hardware().temps.clone() {
            bench(
                || match bridge.get_sensor_value(h) {
                    Ok(value) => {
                        format!("{} = {}", h.name, value)
                    }
                    Err(e) => {
                        format!("error for {}: {}", h.name, e)
                    }
                },
                "get_value",
            );
        }
        for h in &bridge.hardware().fans.clone() {
            bench(
                || match bridge.get_sensor_value(h) {
                    Ok(value) => {
                        format!("{} = {}", h.name, value)
                    }
                    Err(e) => {
                        format!("error for {}: {}", h.name, e)
                    }
                },
                "get_value",
            );
        }
    }

    fn bench(f: impl FnOnce() -> String, info: &str) {
        let now = Instant::now();
        let output = f();
        info!(
            "{}: {} in {} millis",
            info,
            output,
            now.elapsed().as_millis()
        );
    }
}
