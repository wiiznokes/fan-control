use std::fmt::Debug;

use rand::Rng;

use crate::{
    ControlH, FanH, Hardware, HardwareBridge, HardwareBridgeT, HardwareError, TempH, Value,
};

pub struct FakeHardwareBridge {}

#[derive(Debug)]
struct InternalSensor {}

#[derive(Debug)]
struct InternalControl {}


static TEMP_INTERNAL_INDEX: usize = 0;
static FAN_INTERNAL_INDEX: usize = 1;
static CONTROL_INTERNAL_INDEX: usize = 2;

impl HardwareBridge for FakeHardwareBridge {
    fn generate_hardware() -> (Hardware, HardwareBridgeT) {
        let mut hardware = Hardware::default();

        let temp1 = TempH {
            name: "temp1".into(),
            hardware_id: "temp1".into(),
            info: String::new(),
            internal_index: TEMP_INTERNAL_INDEX,
        };
        hardware.temps.push(temp1.into());

        let temp2 = TempH {
            name: "temp2".into(),
            hardware_id: "temp2".into(),
            info: String::new(),
            internal_index: TEMP_INTERNAL_INDEX,
        };
        hardware.temps.push(temp2.into());

        let fan1 = FanH {
            name: "fan1".into(),
            hardware_id: "fan1".into(),
            info: String::new(),
            internal_index: FAN_INTERNAL_INDEX,
        };
        hardware.fans.push(fan1.into());

        let control1 = ControlH {
            name: "control1".into(),
            hardware_id: "control1".into(),
            info: String::new(),
            internal_index: CONTROL_INTERNAL_INDEX,
        };
        hardware.controls.push(control1.into());

        let control2 = ControlH {
            name: "control2".into(),
            hardware_id: "control2".into(),
            info: String::new(),
            internal_index: CONTROL_INTERNAL_INDEX,
        };
        hardware.controls.push(control2.into());

        (hardware, Box::new(Self {}))
    }

    fn get_value(&mut self, internal_index: &usize) -> Result<Value, HardwareError> {
        let nb = rand::thread_rng().gen_range(30..80);
        Ok(nb)
    }

    fn set_value(&mut self, internal_index: &usize, value: Value) -> Result<(), HardwareError> {
        if internal_index != &CONTROL_INTERNAL_INDEX {
            panic!("set value to hardware != Control")
        }
        debug!("set value {}", value);
        return Ok(());
    }

    fn set_mode(&mut self, internal_index: &usize, value: Value) -> Result<(), HardwareError> {
        if internal_index != &CONTROL_INTERNAL_INDEX {
            panic!("set mode to hardware != Control")
        }
        debug!("set mode {}", value);
        return Ok(());
    }
}
