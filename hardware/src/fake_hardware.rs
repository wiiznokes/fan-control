use rand::Rng;

use crate::{HControl, HSensor, Hardware, HardwareBridge, Mode, Value};

pub struct FakeHardwareBridge {
    hardware: Hardware,
}

static TEMP_INTERNAL_INDEX: usize = 0;
static FAN_INTERNAL_INDEX: usize = 1;
static CONTROL_INTERNAL_INDEX: usize = 2;

impl HardwareBridge for FakeHardwareBridge {
    fn new() -> crate::Result<Self> {
        let mut hardware = Hardware::default();

        let temp1 = HSensor {
            name: "temp1".into(),
            hardware_id: "temp1".into(),
            info: String::new(),
            internal_index: TEMP_INTERNAL_INDEX,
        };
        hardware.temps.push(temp1.into());

        let temp2 = HSensor {
            name: "temp2".into(),
            hardware_id: "temp2".into(),
            info: String::new(),
            internal_index: TEMP_INTERNAL_INDEX,
        };
        hardware.temps.push(temp2.into());

        let fan1 = HSensor {
            name: "fan1".into(),
            hardware_id: "fan1".into(),
            info: String::new(),
            internal_index: FAN_INTERNAL_INDEX,
        };
        hardware.fans.push(fan1.into());

        let control1 = HControl {
            name: "control1".into(),
            hardware_id: "control1".into(),
            info: String::new(),
            internal_index: CONTROL_INTERNAL_INDEX,
        };
        hardware.controls.push(control1.into());

        let control2 = HControl {
            name: "control2".into(),
            hardware_id: "control2".into(),
            info: String::new(),
            internal_index: CONTROL_INTERNAL_INDEX,
        };
        hardware.controls.push(control2.into());

        Ok(Self { hardware })
    }
    fn hardware(&self) -> &Hardware {
        &self.hardware
    }

    fn get_sensor_value(&mut self, _sensor: &HSensor) -> crate::Result<Value> {
        let nb = rand::thread_rng().gen_range(30..80);
        Ok(nb)
    }

    fn get_control_value(&mut self, _control: &HControl) -> crate::Result<Value> {
        let nb = rand::thread_rng().gen_range(30..80);
        Ok(nb)
    }

    fn set_value(&mut self, _control: &HControl, value: Value) -> crate::Result<()> {
        debug!("set value {}", value);
        Ok(())
    }

    fn set_mode(&mut self, _control: &HControl, mode: &Mode) -> crate::Result<()> {
        debug!("set mode {}", mode);
        Ok(())
    }
}
