use std::{fmt::Debug, rc::Rc};

use lm_sensors::{ChipRef, FeatureRef, LMSensors, SubFeatureRef, feature, value};
use thiserror::Error;

use crate::{HControl, HSensor, Hardware, HardwareBridge, HardwareError, Mode, Value};
use ouroboros::self_referencing;

// https://www.kernel.org/doc/Documentation/hwmon/sysfs-interface
// https://www.kernel.org/doc/html/next/hwmon/nct6775.html

static DEFAULT_PWM_ENABLE: f64 = 5.0;
static MANUAL_MODE: f64 = 1.0;

pub struct LinuxBridge {
    lm_sensor: LinuxBridgeSelfRef,
    hardware: Hardware,
}

#[derive(Error, Debug)]
pub enum LinuxError {
    #[error("{0}: {1}")]
    LmSensors(String, lm_sensors::errors::Error),
}

#[self_referencing]
struct LinuxBridgeSelfRef {
    lib: LMSensors,

    // ouroboros doesn't provide any documentation on the droping order
    // https://github.com/someguynamedjosh/ouroboros/issues/82
    // but this structure that store references should be dropped first
    #[borrows(lib)]
    #[not_covariant]
    sensors: Vec<InternalSubFeatureRef<'this>>,
}

impl Drop for PwmRefs<'_> {
    fn drop(&mut self) {
        if let Err(e) = self.enable.set_raw_value(self.default_enable_cached) {
            error!("can't set auto to a pwm sensor when quitting: {e}")
        }
    }
}

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
    struct HInfo {
        name: String,
        hardware_id: String,
        info: String,
    }

    #[derive(Error, Debug)]
    pub enum GetInfoError {
        #[error(transparent)]
        LmSensorsRaw(#[from] lm_sensors::errors::Error),
        #[error("Invalid data: {0}")]
        InvalidData(String),
    }

    fn get_infos_from_refs(
        chip_ref: &ChipRef,
        feature_ref: &FeatureRef,
        sub_feature_ref: &SubFeatureRef,
    ) -> std::result::Result<HInfo, GetInfoError> {
        let Some(chip_path) = chip_ref.path() else {
            return Err(GetInfoError::InvalidData("chip path is none".to_owned()));
        };

        let bus = chip_ref.bus();

        let label = feature_ref.label()?;
        let chip_name = chip_ref.name()?;

        let sub_feature_name = match sub_feature_ref.name() {
            Some(sub_feature_name) => sub_feature_name?,
            None => {
                return Err(GetInfoError::InvalidData(
                    "sub feature name is none".to_owned(),
                ));
            }
        };

        Ok(HInfo {
            name: format!("{label} {chip_name}"),
            hardware_id: format!("{label}-{chip_name}-{sub_feature_name}"),
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

                        match get_infos_from_refs(&chip_ref, &feature_ref, &sub_feature_ref) {
                            Ok(h_info) => {
                                let sensor = SensorRefs {
                                    io: sub_feature_ref,
                                };
                                sensors.push(InternalSubFeatureRef::Sensor(sensor));
                                hardware.fans.push(Rc::new(HSensor {
                                    name: h_info.name,
                                    hardware_id: h_info.hardware_id,
                                    info: h_info.info,
                                    internal_index: next_internal_index,
                                }));
                            }
                            Err(e) => {
                                error!("can't generate hardware metadata for fan: {e}");
                            }
                        }
                    }
                    feature::Kind::Temperature => {
                        let Ok(sub_feature_ref) =
                            feature_ref.sub_feature_by_kind(value::Kind::TemperatureInput)
                        else {
                            continue;
                        };

                        match get_infos_from_refs(&chip_ref, &feature_ref, &sub_feature_ref) {
                            Ok(h_info) => {
                                let sensor = SensorRefs {
                                    io: sub_feature_ref,
                                };
                                sensors.push(InternalSubFeatureRef::Sensor(sensor));
                                hardware.temps.push(Rc::new(HSensor {
                                    name: h_info.name,
                                    hardware_id: h_info.hardware_id,
                                    info: h_info.info,
                                    internal_index: next_internal_index,
                                }));
                            }
                            Err(e) => {
                                error!("can't generate hardware metadata for temp: {e}");
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
                                error!("can't read value of pwm {e}");
                                continue;
                            }
                        };

                        match get_infos_from_refs(&chip_ref, &feature_ref, &sub_feature_ref_io) {
                            Ok(h_info) => {
                                let sensor = InternalSubFeatureRef::Pwm(PwmRefs {
                                    io: sub_feature_ref_io,
                                    enable: sub_feature_ref_enable,
                                    default_enable_cached: enable_cached,
                                });
                                sensors.push(sensor);
                                hardware.controls.push(Rc::new(HControl {
                                    name: h_info.name,
                                    hardware_id: h_info.hardware_id,
                                    info: h_info.info,
                                    internal_index: next_internal_index,
                                }));
                            }
                            Err(e) => {
                                error!("can't generate hardware metadata for control: {e}");
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
    fn new() -> crate::Result<Self> {
        let mut hardware = Hardware::default();

        let lib = match lm_sensors::Initializer::default().initialize() {
            Ok(lib) => lib,
            Err(e) => {
                return Err(HardwareError::Linux(LinuxError::LmSensors(
                    "failed to init libsensor".into(),
                    e,
                )));
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
    fn hardware(&self) -> &Hardware {
        &self.hardware
    }

    fn get_sensor_value(&mut self, sensor: &HSensor) -> crate::Result<Value> {
        self.lm_sensor.with_sensors(|sensors| {
            match sensors.get(sensor.internal_index).expect("no sensor found") {
                InternalSubFeatureRef::Sensor(sensor_refs) => match sensor_refs.io.raw_value() {
                    Ok(value) => Ok(value as i32),
                    Err(e) => Err(HardwareError::Linux(LinuxError::LmSensors(
                        "sensor".to_owned(),
                        e,
                    ))),
                },
                _ => unreachable!(),
            }
        })
    }
    fn get_control_value(&mut self, control: &HControl) -> crate::Result<Value> {
        self.lm_sensor.with_sensors(|sensors| {
            match sensors
                .get(control.internal_index)
                .expect("no sensor found")
            {
                InternalSubFeatureRef::Pwm(pwm_refs) => match pwm_refs.io.raw_value() {
                    Ok(value) => Ok((value / 2.55) as i32),
                    Err(e) => Err(HardwareError::Linux(LinuxError::LmSensors(
                        "pwm".to_owned(),
                        e,
                    ))),
                },
                _ => unreachable!(),
            }
        })
    }

    fn set_value(&mut self, control: &HControl, value: Value) -> crate::Result<()> {
        self.lm_sensor.with_sensors(|sensors| {
            match sensors
                .get(control.internal_index)
                .expect("no sensor found")
            {
                InternalSubFeatureRef::Pwm(pwm_refs) => {
                    let value = value as f64 * 2.55;
                    if let Err(e) = pwm_refs.io.set_raw_value(value) {
                        let explication = format!("can't set value {value} to a pwm");
                        let e = LinuxError::LmSensors(explication, e);
                        return Err(HardwareError::Linux(e));
                    }
                    Ok(())
                }
                _ => unreachable!(),
            }
        })
    }

    fn set_mode(&mut self, control: &HControl, mode: &Mode) -> crate::Result<()> {
        self.lm_sensor.with_sensors(|sensors| {
            match sensors
                .get(control.internal_index)
                .expect("no sensor found")
            {
                InternalSubFeatureRef::Pwm(pwm_refs) => {
                    let value = match mode {
                        Mode::Auto => pwm_refs.default_enable_cached,
                        Mode::Manual => MANUAL_MODE,
                        Mode::Specific(value) => value.to_owned().into(),
                    };

                    if let Err(e) = pwm_refs.enable.set_raw_value(value) {
                        let explication = format!("can't set mode {value} to a pwm");
                        let e = LinuxError::LmSensors(explication, e);
                        return Err(HardwareError::Linux(e));
                    }
                    Ok(())
                }
                _ => unreachable!(),
            }
        })
    }
}
