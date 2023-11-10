use std::fmt::Debug;

use rand::Rng;

use crate::{ControlH, Hardware, HardwareBridge, HardwareError, HardwareItem, TempH, Value};

pub struct TestBridge {}

#[derive(Debug)]
struct InternalSensor {}

#[derive(Debug)]
struct InternalControl {}

impl HardwareBridge for TestBridge {
    fn generate_hardware() -> Hardware {
        let mut hardware = Hardware::default();

        let temp1 = TempH {
            name: "temp1".into(),
            hardware_id: "temp1".into(),
            info: String::new(),
            bridge: Box::new(InternalSensor {}),
        };
        hardware.temps.push(temp1.into());

        let temp2 = TempH {
            name: "temp2".into(),
            hardware_id: "temp2".into(),
            info: String::new(),
            bridge: Box::new(InternalSensor {}),
        };
        hardware.temps.push(temp2.into());

        let control1 = ControlH {
            name: "control1".into(),
            hardware_id: "control1".into(),
            info: String::new(),
            bridge: Box::new(InternalControl {}),
        };
        hardware.controls.push(control1.into());

        let control2 = ControlH {
            name: "control2".into(),
            hardware_id: "control2".into(),
            info: String::new(),
            bridge: Box::new(InternalControl {}),
        };
        hardware.controls.push(control2.into());

        hardware
    }
}

impl HardwareItem for InternalSensor {
    fn get_value(&self) -> Result<Value, crate::HardwareError> {
        let nb = rand::thread_rng().gen_range(30..80);
        Ok(nb)
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
        println!("set value {} to a control", value);
        Ok(())
    }

    fn set_mode(&self, value: Value) -> Result<(), HardwareError> {
        println!("set mode {} to a control", value);
        Ok(())
    }
}
