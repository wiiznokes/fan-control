use std::fmt::Debug;

use rand::Rng;

use crate::{HItem, Hardware, HardwareBridge, Mode, Value};

pub struct FakeHardwareBridge {
    hardware: Hardware,
}

#[derive(Debug)]
struct InternalSensor {}

#[derive(Debug)]
struct InternalControl {}

static TEMP_INTERNAL_INDEX: usize = 0;
static FAN_INTERNAL_INDEX: usize = 1;
static CONTROL_INTERNAL_INDEX: usize = 2;

impl FakeHardwareBridge {
    pub fn new() -> crate::Result<Self> {
        let mut hardware = Hardware::default();

        let temp1 = HItem {
            name: "temp1".into(),
            hardware_id: "temp1".into(),
            info: String::new(),
            internal_index: TEMP_INTERNAL_INDEX,
        };
        hardware.temps.push(temp1.into());

        let temp2 = HItem {
            name: "temp2".into(),
            hardware_id: "temp2".into(),
            info: String::new(),
            internal_index: TEMP_INTERNAL_INDEX,
        };
        hardware.temps.push(temp2.into());

        let fan1 = HItem {
            name: "fan1".into(),
            hardware_id: "fan1".into(),
            info: String::new(),
            internal_index: FAN_INTERNAL_INDEX,
        };
        hardware.fans.push(fan1.into());

        let control1 = HItem {
            name: "control1".into(),
            hardware_id: "control1".into(),
            info: String::new(),
            internal_index: CONTROL_INTERNAL_INDEX,
        };
        hardware.controls.push(control1.into());

        let control2 = HItem {
            name: "control2".into(),
            hardware_id: "control2".into(),
            info: String::new(),
            internal_index: CONTROL_INTERNAL_INDEX,
        };
        hardware.controls.push(control2.into());

        Ok(Self { hardware })
    }
}

impl HardwareBridge for FakeHardwareBridge {
    fn hardware(&self) -> &Hardware {
        &self.hardware
    }

    fn get_value(&mut self, _internal_index: &usize) -> crate::Result<Value> {
        let nb = rand::thread_rng().gen_range(30..80);
        Ok(nb)
    }

    fn set_value(&mut self, internal_index: &usize, value: Value) -> crate::Result<()> {
        if internal_index != &CONTROL_INTERNAL_INDEX {
            panic!("set value to hardware != Control")
        }
        debug!("set value {}", value);
        Ok(())
    }

    fn set_mode(&mut self, internal_index: &usize, mode: &Mode) -> crate::Result<()> {
        if internal_index != &CONTROL_INTERNAL_INDEX {
            panic!("set mode to hardware != Control")
        }
        debug!("set mode {}", mode);
        Ok(())
    }
}
