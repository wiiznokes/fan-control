use lm_sensors::{feature, value, ChipRef, FeatureRef, LMSensors, SubFeatureRef};

use crate::{ControlH, FanH, Hardware, HardwareBridge, HardwareError, HardwareItem, TempH, Value};

pub struct LinuxBridge {}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SubFeatureType {
    PwmIo,
    PwmEnable,
    Fan,
    Temp,
}

#[derive(Debug)]
struct InternalSubFeatureRef {
    sub_feature_type: SubFeatureType,
    sub_feature_ref: SubFeatureRef<'static>,
}

#[derive(Debug)]
struct InternalSensor {
    sensor: InternalSubFeatureRef,
}

#[derive(Debug)]
struct InternalControl {
    io: InternalSubFeatureRef,
    enable: InternalSubFeatureRef,
}

impl Drop for InternalControl {
    fn drop(&mut self) {
        println!("pwm sould be set to auto");
        // TODO: set to auto
    }
}

impl HardwareBridge for LinuxBridge {
    fn generate_hardware() -> Hardware {
        let mut hardware = Hardware::default();

        let lib = lm_sensors::Initializer::default().initialize().unwrap();
        let boxed = Box::new(lib);
        // yes we leak like never here but it's not that bad in fact
        // is even safe Rust. The kernel will be in charge to release
        // memory when the process terminate.
        let leaked: &'static mut LMSensors = Box::leak(boxed);

        generate_hardware(leaked, &mut hardware);

        hardware
    }
}

fn generate_id_name_info(
    chip_ref: &ChipRef,
    feature_ref: &FeatureRef,
    sub_feature_ref: &SubFeatureRef,
) -> Option<(String, String, String)> {
    let Some(chip_path) = chip_ref.path() else {
        return None;
    };

    let bus = chip_ref.bus();

    let Ok(label) = feature_ref.label() else {
        return None;
    };

    let Ok(chip_name) = chip_ref.name() else {
        return None;
    };

    let Some(Ok(sub_feature_name)) = sub_feature_ref.name() else {
        return None;
    };

    let id = format!("{}-{}", chip_name, sub_feature_name);
    let name: String = format!("{} {}", label, chip_name);
    let info = format!(
        "chip path: {}\nchip name: {}\nbus: {}\nlabel: {}\nfeature: {}",
        chip_path.display(),
        chip_name,
        bus,
        label,
        sub_feature_name
    );

    Some((id, name, info))
}

fn generate_hardware(lib: &'static LMSensors, hardware: &mut Hardware) {
    for chip_ref in lib.chip_iter(None) {
        for feature_ref in chip_ref.feature_iter() {
            match feature_ref.kind() {
                Some(feature_kind) => match feature_kind {
                    feature::Kind::Fan => {
                        let Ok(sub_feature_ref) =
                            feature_ref.sub_feature_by_kind(value::Kind::FanInput)
                        else {
                            continue;
                        };

                        if let Some((id, name, info)) =
                            generate_id_name_info(&chip_ref, &feature_ref, &sub_feature_ref)
                        {
                            let sensor = InternalSubFeatureRef {
                                sub_feature_type: SubFeatureType::Fan,
                                sub_feature_ref,
                            };

                            let fan_h = FanH {
                                name,
                                hardware_id: id,
                                info,
                                bridge: Box::new(InternalSensor { sensor }),
                            };
                            hardware.fans.push(fan_h.into());
                        }
                    }
                    feature::Kind::Temperature => {
                        let Ok(sub_feature_ref) =
                            feature_ref.sub_feature_by_kind(value::Kind::TemperatureInput)
                        else {
                            continue;
                        };

                        if let Some((id, name, info)) =
                            generate_id_name_info(&chip_ref, &feature_ref, &sub_feature_ref)
                        {
                            let sensor = InternalSubFeatureRef {
                                sub_feature_type: SubFeatureType::Temp,
                                sub_feature_ref,
                            };

                            let temp_h = TempH {
                                name,
                                hardware_id: id,
                                info,
                                bridge: Box::new(InternalSensor { sensor }),
                            };
                            hardware.temps.push(temp_h.into());
                        }
                    }
                    feature::Kind::Pwm => {
                        let Ok(sub_feature_ref_io) =
                            feature_ref.sub_feature_by_kind(value::Kind::PwmIo)
                        else {
                            continue;
                        };
                        let Ok(sub_feature_ref_enable) =
                            feature_ref.sub_feature_by_kind(value::Kind::PwmEnable)
                        else {
                            continue;
                        };

                        if let Some((id, name, info)) =
                            generate_id_name_info(&chip_ref, &feature_ref, &sub_feature_ref_io)
                        {
                            let io = InternalSubFeatureRef {
                                sub_feature_type: SubFeatureType::PwmIo,
                                sub_feature_ref: sub_feature_ref_io,
                            };

                            let enable = InternalSubFeatureRef {
                                sub_feature_type: SubFeatureType::PwmEnable,
                                sub_feature_ref: sub_feature_ref_enable,
                            };

                            let control = InternalControl { io, enable };

                            let control_h = ControlH {
                                name,
                                hardware_id: id,
                                info,
                                bridge: Box::new(control),
                            };
                            hardware.controls.push(control_h.into());
                        }
                    }
                    _ => continue,
                },
                None => continue,
            };
        }
    }
}

impl HardwareItem for InternalSensor {
    fn value(&self) -> Result<Value, crate::HardwareError> {
        match self.sensor.sub_feature_ref.raw_value() {
            Ok(value) => Ok(value as i32),
            Err(e) => {
                eprintln!("{}", e);
                Err(HardwareError::LmSensors)
            }
        }
    }

    fn set_value(&self, value: Value) -> Result<(), crate::HardwareError> {
        panic!("can't set the value of a sensor");
    }
}

impl HardwareItem for InternalControl {
    fn value(&self) -> Result<Value, crate::HardwareError> {
        match self.io.sub_feature_ref.raw_value() {
            Ok(value) => Ok(value as i32),
            Err(e) => {
                eprintln!("{}", e);
                Err(HardwareError::LmSensors)
            }
        }
    }

    fn set_value(&self, value: Value) -> Result<(), crate::HardwareError> {
        println!("set value {} to a control", value);
        Ok(())
    }
}
