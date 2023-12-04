use std::fmt::Debug;

use lm_sensors::{feature, value, ChipRef, FeatureRef, LMSensors, SubFeatureRef};

use crate::{
    ControlH, FanH, Hardware, HardwareBridge, HardwareBridgeT, HardwareError, TempH, Value,
};
use ouroboros::self_referencing;

// https://www.kernel.org/doc/Documentation/hwmon/sysfs-interface
// https://www.kernel.org/doc/html/next/hwmon/nct6775.html

// todo: cache PWM_ENABLE value before setting manual mode
// so we can use it instead of this hardcored value
static DEFAULT_PWM_ENABLE: i32 = 5;

#[self_referencing]
pub struct LinuxBridge {
    lib: LMSensors,
    #[borrows(lib)]
    #[not_covariant]
    sensors: Vec<InternalSubFeatureRef<'this>>,
}

impl HardwareBridge for LinuxBridge {
    fn generate_hardware() -> (Hardware, HardwareBridgeT) {
        let mut hardware = Hardware::default();

        let bridge = LinuxBridgeBuilder {
            lib: lm_sensors::Initializer::default().initialize().unwrap(),
            sensors_builder: |lib: &LMSensors| generate_hardware(lib, &mut hardware),
        }
        .build();

        (hardware, Box::new(bridge))
    }

    fn get_value(&mut self, internal_index: &usize) -> Result<Value, HardwareError> {
        self.with_sensors(|sensors| match sensors.get(*internal_index) {
            Some(sensor) => match sensor {
                InternalSubFeatureRef::Pwm(pwm_refs) => match pwm_refs.io.raw_value() {
                    Ok(value) => Ok((value / 2.55) as i32),
                    Err(e) => Err(HardwareError::LmSensors(format!("{}", e))),
                },
                InternalSubFeatureRef::Sensor(sensor_refs) => match sensor_refs.io.raw_value() {
                    Ok(value) => Ok(value as i32),
                    Err(e) => Err(HardwareError::LmSensors(format!("{}", e))),
                },
            },
            None => Err(HardwareError::InternalIndexNotFound),
        })
    }

    fn set_value(&mut self, internal_index: &usize, value: Value) -> Result<(), HardwareError> {
        self.with_sensors(|sensors| match sensors.get(*internal_index) {
            Some(sensor) => match sensor {
                InternalSubFeatureRef::Pwm(pwm_refs) => {
                    let value = value as f64 * 2.55;
                    if let Err(e) = pwm_refs.io.set_raw_value(value) {
                        return Err(HardwareError::LmSensors(format!(
                            "can't set value {} to a pwm: {:?}",
                            value, e
                        )));
                    }
                    Ok(())
                }
                InternalSubFeatureRef::Sensor(_) => {
                    panic!("can't set the value of a sensor");
                }
            },
            None => Err(HardwareError::InternalIndexNotFound),
        })
    }

    fn set_mode(&mut self, internal_index: &usize, value: Value) -> Result<(), HardwareError> {
        let value = if value == 0 {
            DEFAULT_PWM_ENABLE
        } else {
            value
        };

        self.with_sensors(|sensors| match sensors.get(*internal_index) {
            Some(sensor) => match sensor {
                InternalSubFeatureRef::Pwm(pwm_refs) => {
                    if let Err(e) = pwm_refs.enable.set_raw_value(value.into()) {
                        return Err(HardwareError::LmSensors(format!(
                            "can't set mode {} to a pwm: {:?}",
                            value, e
                        )));
                    }
                    Ok(())
                }
                InternalSubFeatureRef::Sensor(_) => {
                    panic!("can't set mode of a sensor");
                }
            },
            None => Err(HardwareError::InternalIndexNotFound),
        })
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum SubFeatureType {
    PwmIo,
    PwmEnable,
    Fan,
    Temp,
}

enum InternalSubFeatureRef<'a> {
    Pwm(PwmRefs<'a>),
    Sensor(SensorRefs<'a>),
}

struct PwmRefs<'a> {
    io: SubFeatureRef<'a>,
    enable: SubFeatureRef<'a>,
}
struct SensorRefs<'a> {
    io: SubFeatureRef<'a>,
}

impl Drop for PwmRefs<'_> {
    fn drop(&mut self) {
        if let Err(e) = self.enable.set_raw_value(DEFAULT_PWM_ENABLE.into()) {
            error!("can't set auto to a pwn in drop function: {:?}", e)
        }
    }
}

fn generate_hardware<'a>(
    lib: &'a LMSensors,
    hardware: &mut Hardware,
) -> Vec<InternalSubFeatureRef<'a>> {
    let mut sensors = Vec::new();

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
                            let sensor = InternalSubFeatureRef::Sensor(SensorRefs {
                                io: sub_feature_ref,
                            });
                            sensors.push(sensor);

                            let fan_h = FanH {
                                name,
                                hardware_id: id,
                                info,
                                internal_index: sensors.len() - 1,
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
                            let sensor = InternalSubFeatureRef::Sensor(SensorRefs {
                                io: sub_feature_ref,
                            });
                            sensors.push(sensor);

                            let temp_h = TempH {
                                name,
                                hardware_id: id,
                                info,
                                internal_index: sensors.len() - 1,
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
                            let sensor = InternalSubFeatureRef::Pwm(PwmRefs {
                                io: sub_feature_ref_io,
                                enable: sub_feature_ref_enable,
                            });

                            sensors.push(sensor);

                            let control_h = ControlH {
                                name,
                                hardware_id: id,
                                info,
                                internal_index: sensors.len() - 1,
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
    sensors
}
