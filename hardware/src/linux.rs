use std::{fmt::Debug, rc::Rc};

use lm_sensors::{feature, value, ChipRef, FeatureRef, LMSensors, SubFeatureRef};
use thiserror::Error;

use crate::{HItem, Hardware, HardwareBridgeT, HardwareError, Mode, Value};
use ouroboros::self_referencing;

// https://www.kernel.org/doc/Documentation/hwmon/sysfs-interface
// https://www.kernel.org/doc/html/next/hwmon/nct6775.html

static DEFAULT_PWM_ENABLE: f64 = 5.0;
static MANUAL_MODE: f64 = 1.0;

#[self_referencing]
pub struct LinuxBridgeSelfRef {
    lib: LMSensors,
    #[borrows(lib)]
    #[not_covariant]
    sensors: Vec<InternalSubFeatureRef<'this>>,
}

pub struct LinuxBridge {
    lm_sensor: LinuxBridgeSelfRef,
    hardware: Hardware,
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
    fn h_item_from_refs(
        chip_ref: &ChipRef,
        feature_ref: &FeatureRef,
        sub_feature_ref: &SubFeatureRef,
        internal_index: usize,
    ) -> Result<Rc<HItem>> {
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

        let h_item = HItem {
            name: format!("{} {}", label, chip_name),
            hardware_id: format!("{}-{}-{}", label, chip_name, sub_feature_name),
            info: format!(
                "chip path: {}\nchip name: {}\nbus: {}\nlabel: {}\nfeature: {}",
                chip_path.display(),
                chip_name,
                bus,
                label,
                sub_feature_name
            ),
            internal_index,
        };

        Ok(Rc::new(h_item))
    }

    let mut sensors = Vec::new();

    for chip_ref in lib.chip_iter(None) {
        for feature_ref in chip_ref.feature_iter() {
            let next_internal_index = sensors.len();

            match feature_ref.kind() {
                Some(feature_kind) => match feature_kind {
                    feature::Kind::Fan => {
                        let Ok(sub_feature_ref) =
                            feature_ref.sub_feature_by_kind(value::Kind::FanInput)
                        else {
                            continue;
                        };

                        match h_item_from_refs(
                            &chip_ref,
                            &feature_ref,
                            &sub_feature_ref,
                            next_internal_index,
                        ) {
                            Ok(h_item) => {
                                let sensor = SensorRefs {
                                    io: sub_feature_ref,
                                };
                                sensors.push(InternalSubFeatureRef::Sensor(sensor));
                                hardware.fans.push(h_item);
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

                        match h_item_from_refs(
                            &chip_ref,
                            &feature_ref,
                            &sub_feature_ref,
                            next_internal_index,
                        ) {
                            Ok(h_item) => {
                                let sensor = SensorRefs {
                                    io: sub_feature_ref,
                                };
                                sensors.push(InternalSubFeatureRef::Sensor(sensor));
                                hardware.temps.push(h_item);
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

                        match h_item_from_refs(
                            &chip_ref,
                            &feature_ref,
                            &sub_feature_ref_io,
                            next_internal_index,
                        ) {
                            Ok(h_item) => {
                                let sensor = InternalSubFeatureRef::Pwm(PwmRefs {
                                    io: sub_feature_ref_io,
                                    enable: sub_feature_ref_enable,
                                    default_enable_cached: enable_cached,
                                });
                                sensors.push(sensor);
                                hardware.controls.push(h_item);
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

impl LinuxBridge {
    pub fn new() -> crate::Result<Self> {
        let mut hardware = Hardware::default();

        let lib = match lm_sensors::Initializer::default().initialize() {
            Ok(lib) => lib,
            Err(e) => {
                return Err(HardwareError::Linux(LinuxError::LmSensors(
                    "failed to init libsensor".into(),
                    e,
                )))
            }
        };
        let bridge = LinuxBridgeSelfRefBuilder {
            lib,
            sensors_builder: |lib: &LMSensors| generate_hardware(lib, &mut hardware),
        }
        .build();

        Ok(Self {
            lm_sensor: bridge,
            hardware,
        })
    }
}

impl HardwareBridgeT for LinuxBridge {
    fn hardware(&self) -> &Hardware {
        &self.hardware
    }

    fn get_value(&mut self, internal_index: &usize) -> crate::Result<Value> {
        self.lm_sensor
            .with_sensors(|sensors| match sensors.get(*internal_index) {
                Some(sensor) => match sensor {
                    InternalSubFeatureRef::Pwm(pwm_refs) => match pwm_refs.io.raw_value() {
                        Ok(value) => Ok((value / 2.55) as i32),
                        Err(e) => Err(HardwareError::Linux(LinuxError::LmSensors(
                            "pwm".to_owned(),
                            e,
                        ))),
                    },
                    InternalSubFeatureRef::Sensor(sensor_refs) => {
                        match sensor_refs.io.raw_value() {
                            Ok(value) => Ok(value as i32),
                            Err(e) => Err(HardwareError::Linux(LinuxError::LmSensors(
                                "sensor".to_owned(),
                                e,
                            ))),
                        }
                    }
                },
                None => Err(HardwareError::InternalIndexNotFound),
            })
    }

    fn set_value(&mut self, internal_index: &usize, value: Value) -> crate::Result<()> {
        self.lm_sensor
            .with_sensors(|sensors| match sensors.get(*internal_index) {
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
                        let e =
                            LinuxError::WrongHardware("sensor".to_owned(), "set value".to_owned());
                        Err(HardwareError::Linux(e))
                    }
                },
                None => Err(HardwareError::InternalIndexNotFound),
            })
    }

    fn set_mode(&mut self, internal_index: &usize, mode: &Mode) -> crate::Result<()> {
        self.lm_sensor
            .with_sensors(|sensors| match sensors.get(*internal_index) {
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
                        let e =
                            LinuxError::WrongHardware("sensor".to_owned(), "set mode".to_owned());
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
