use std::fmt::Debug;

use lm_sensors::{feature, value, ChipRef, FeatureRef, LMSensors, SubFeatureRef};
use thiserror::Error;

use crate::{
    ControlH, FanH, Hardware, HardwareBridge, HardwareBridgeT, HardwareError, Mode, TempH, Value,
};
use ouroboros::self_referencing;

// https://www.kernel.org/doc/Documentation/hwmon/sysfs-interface
// https://www.kernel.org/doc/html/next/hwmon/nct6775.html

static DEFAULT_PWM_ENABLE: f64 = 5.0;
static MANUAL_MODE: f64 = 1.0;

#[self_referencing]
pub struct LinuxBridge {
    lib: LMSensors,
    #[borrows(lib)]
    #[not_covariant]
    sensors: Vec<InternalSubFeatureRef<'this>>,
}

#[derive(Error, Debug)]
pub enum LinuxError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("No connection was found")]
    NoConnectionFound,
    #[error("{0}: {1}")]
    LmSensors(String, lm_sensors::errors::Error),
    #[error(transparent)]
    LmSensorsRaw(#[from] lm_sensors::errors::Error),
    #[error("Wrong hardware type: {0} for the operation {1}")]
    WrongHardware(String, String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
}
type Result<T> = std::result::Result<T, LinuxError>;

struct PwmRefs<'a> {
    io: SubFeatureRef<'a>,
    enable: SubFeatureRef<'a>,
    default_enable_cached: f64,
}
struct SensorRefs<'a> {
    io: SubFeatureRef<'a>,
}

enum InternalSubFeatureRef<'a> {
    Pwm(PwmRefs<'a>),
    Sensor(SensorRefs<'a>),
}

fn generate_hardware<'a>(
    lib: &'a LMSensors,
    hardware: &mut Hardware,
) -> Vec<InternalSubFeatureRef<'a>> {
    struct HardwareMetadata {
        id: String,
        name: String,
        info: String,
    }

    impl HardwareMetadata {
        fn new(
            chip_ref: &ChipRef,
            feature_ref: &FeatureRef,
            sub_feature_ref: &SubFeatureRef,
        ) -> Result<Self> {
            let Some(chip_path) = chip_ref.path() else {
                return Err(LinuxError::InvalidData("chip path is none".to_owned()));
            };

            let bus = chip_ref.bus();

            let label = feature_ref.label()?;
            let chip_name = chip_ref.name()?;

            let sub_feature_name = match sub_feature_ref.name() {
                Some(sub_feature_name) => sub_feature_name?,
                None => {
                    return Err(LinuxError::InvalidData(
                        "sub feature name is none".to_owned(),
                    ));
                }
            };

            Ok(Self {
                id: format!("{}-{}-{}", label, chip_name, sub_feature_name),
                name: format!("{} {}", label, chip_name),
                info: format!(
                    "chip path: {}\nchip name: {}\nbus: {}\nlabel: {}\nfeature: {}",
                    chip_path.display(),
                    chip_name,
                    bus,
                    label,
                    sub_feature_name
                ),
            })
        }
    }

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

                        match HardwareMetadata::new(&chip_ref, &feature_ref, &sub_feature_ref) {
                            Ok(metadata) => {
                                let sensor = InternalSubFeatureRef::Sensor(SensorRefs {
                                    io: sub_feature_ref,
                                });
                                sensors.push(sensor);

                                let fan_h = FanH {
                                    name: metadata.name,
                                    hardware_id: metadata.id,
                                    info: metadata.info,
                                    internal_index: sensors.len() - 1,
                                };
                                hardware.fans.push(fan_h.into());
                            }
                            Err(e) => {
                                error!("can't generate hardware metadata for fan: {}", e);
                            }
                        }
                    }
                    feature::Kind::Temperature => {
                        let Ok(sub_feature_ref) =
                            feature_ref.sub_feature_by_kind(value::Kind::TemperatureInput)
                        else {
                            continue;
                        };

                        match HardwareMetadata::new(&chip_ref, &feature_ref, &sub_feature_ref) {
                            Ok(metadata) => {
                                let sensor = InternalSubFeatureRef::Sensor(SensorRefs {
                                    io: sub_feature_ref,
                                });
                                sensors.push(sensor);

                                let temp_h = TempH {
                                    name: metadata.name,
                                    hardware_id: metadata.id,
                                    info: metadata.info,
                                    internal_index: sensors.len() - 1,
                                };
                                hardware.temps.push(temp_h.into());
                            }
                            Err(e) => {
                                error!("can't generate hardware metadata for temp: {}", e);
                            }
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

                        let enable_cached = match sub_feature_ref_enable.raw_value() {
                            Ok(value) => {
                                if value == MANUAL_MODE {
                                    DEFAULT_PWM_ENABLE
                                } else {
                                    value
                                }
                            }
                            Err(e) => {
                                error!("can't read value of pwm {}", e);
                                continue;
                            }
                        };

                        match HardwareMetadata::new(&chip_ref, &feature_ref, &sub_feature_ref_io) {
                            Ok(metadata) => {
                                let sensor = InternalSubFeatureRef::Pwm(PwmRefs {
                                    io: sub_feature_ref_io,
                                    enable: sub_feature_ref_enable,
                                    default_enable_cached: enable_cached,
                                });
                                sensors.push(sensor);

                                let control_h = ControlH {
                                    name: metadata.name,
                                    hardware_id: metadata.id,
                                    info: metadata.info,
                                    internal_index: sensors.len() - 1,
                                };
                                hardware.controls.push(control_h.into());
                            }
                            Err(e) => {
                                error!("can't generate hardware metadata for control: {}", e);
                            }
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

impl HardwareBridge for LinuxBridge {
    fn generate_hardware() -> crate::Result<(Hardware, HardwareBridgeT)> {
        let mut hardware = Hardware::default();

        let bridge = LinuxBridgeBuilder {
            lib: lm_sensors::Initializer::default().initialize().unwrap(),
            sensors_builder: |lib: &LMSensors| generate_hardware(lib, &mut hardware),
        }
        .build();

        Ok((hardware, Box::new(bridge)))
    }

    fn get_value(&mut self, internal_index: &usize) -> crate::Result<Value> {
        self.with_sensors(|sensors| match sensors.get(*internal_index) {
            Some(sensor) => match sensor {
                InternalSubFeatureRef::Pwm(pwm_refs) => match pwm_refs.io.raw_value() {
                    Ok(value) => Ok((value / 2.55) as i32),
                    Err(e) => Err(HardwareError::Linux(LinuxError::LmSensors(
                        "pwm".to_owned(),
                        e,
                    ))),
                },
                InternalSubFeatureRef::Sensor(sensor_refs) => match sensor_refs.io.raw_value() {
                    Ok(value) => Ok(value as i32),
                    Err(e) => Err(HardwareError::Linux(LinuxError::LmSensors(
                        "sensor".to_owned(),
                        e,
                    ))),
                },
            },
            None => Err(HardwareError::InternalIndexNotFound),
        })
    }

    fn set_value(&mut self, internal_index: &usize, value: Value) -> crate::Result<()> {
        self.with_sensors(|sensors| match sensors.get(*internal_index) {
            Some(sensor) => match sensor {
                InternalSubFeatureRef::Pwm(pwm_refs) => {
                    let value = value as f64 * 2.55;
                    if let Err(e) = pwm_refs.io.set_raw_value(value) {
                        let explication = format!("can't set value {} to a pwm", value);
                        let e = LinuxError::LmSensors(explication, e);
                        return Err(HardwareError::Linux(e));
                    }
                    Ok(())
                }
                InternalSubFeatureRef::Sensor(_) => {
                    let e = LinuxError::WrongHardware("sensor".to_owned(), "set value".to_owned());
                    Err(HardwareError::Linux(e))
                }
            },
            None => Err(HardwareError::InternalIndexNotFound),
        })
    }

    fn set_mode(&mut self, internal_index: &usize, mode: &Mode) -> crate::Result<()> {
        self.with_sensors(|sensors| match sensors.get(*internal_index) {
            Some(sensor) => match sensor {
                InternalSubFeatureRef::Pwm(pwm_refs) => {
                    let value = match mode {
                        Mode::Auto => pwm_refs.default_enable_cached,
                        Mode::Manual => MANUAL_MODE,
                        Mode::Specific(value) => value.to_owned().into(),
                    };

                    if let Err(e) = pwm_refs.enable.set_raw_value(value) {
                        let explication = format!("can't set mode {} to a pwm", value);
                        let e = LinuxError::LmSensors(explication, e);
                        return Err(HardwareError::Linux(e));
                    }
                    Ok(())
                }
                InternalSubFeatureRef::Sensor(_) => {
                    let e = LinuxError::WrongHardware("sensor".to_owned(), "set mode".to_owned());
                    Err(HardwareError::Linux(e))
                }
            },
            None => Err(HardwareError::InternalIndexNotFound),
        })
    }
}

impl Drop for PwmRefs<'_> {
    fn drop(&mut self) {
        if let Err(e) = self.enable.set_raw_value(self.default_enable_cached) {
            error!("can't set auto to a pwn in his drop function: {}", e)
        }
    }
}
