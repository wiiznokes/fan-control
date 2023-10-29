use std::collections::HashMap;

use data::{
    config::Hardware,
    items::{Control, Fan, Temp},
    node::HardwareType,
};
use lm_sensors::{feature, value, ChipRef, FeatureRef, LMSensors, SubFeatureRef};

use crate::{HardwareError, HardwareGenerator};

pub struct LinuxGenerator<'a> {
    lib: LMSensors,
    sensors: HashMap<String, Sensor<'a>>,
}

#[derive(Debug, Clone)]
struct Sensor<'a> {
    hardware_type: HardwareType,
    sub_feature_ref: SubFeatureRef<'a>,
    name: String,
    info: String,
    id: String,
}

impl<'a> HardwareGenerator<'a> for LinuxGenerator<'a> {
    fn new() -> impl HardwareGenerator<'a> {
        let lib = lm_sensors::Initializer::default().initialize().unwrap();

        LinuxGenerator {
            lib,
            sensors: HashMap::new(),
        }
    }

    fn init<'b: 'a>(&'b mut self) {
        for chip_ref in self.lib.chip_iter(None) {
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
                                let sensor = Sensor {
                                    hardware_type: HardwareType::Fan,
                                    sub_feature_ref,
                                    name,
                                    info,
                                    id: id.clone(),
                                };
                                self.sensors.insert(id, sensor);
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
                                let sensor = Sensor {
                                    hardware_type: HardwareType::Temp,
                                    sub_feature_ref,
                                    name,
                                    info,
                                    id: id.clone(),
                                };
                                self.sensors.insert(id, sensor);
                            }
                        }
                        _ => continue,
                    },
                    None => continue,
                };
            }
        }
    }

    fn validate(
        &self,
        hardware_type: &HardwareType,
        hardware_id: &str,
    ) -> Result<(), crate::HardwareError> {
        match self.sensors.get(hardware_id) {
            Some(sensor) => {
                if sensor.hardware_type == *hardware_type {
                    Ok(())
                } else {
                    Err(HardwareError::WrongType)
                }
            }
            None => Err(HardwareError::IdNotFound),
        }
    }

    fn hardware(&self) -> Hardware {
        let mut hardware = Hardware::default();

        for sensor in self.sensors.values() {
            match sensor.hardware_type {
                HardwareType::Control => hardware.controls.push(Control {
                    name: sensor.name.clone(),
                    hardware_id: sensor.id.clone(),
                }),
                HardwareType::Fan => hardware.fans.push(Fan {
                    name: sensor.name.clone(),
                    hardware_id: sensor.id.clone(),
                }),
                HardwareType::Temp => hardware.temps.push(Temp {
                    name: sensor.name.clone(),
                    hardware_id: sensor.id.clone(),
                }),
            }
        }

        hardware
    }

    fn value(&self, hardware_id: &str) -> Result<Option<i32>, crate::HardwareError> {
        match self.sensors.get(hardware_id) {
            Some(sensor) => match sensor.sub_feature_ref.raw_value() {
                Ok(value) => Ok(Some(value as i32)),
                Err(e) => {
                    eprintln!("{}", e);
                    Err(HardwareError::LmSensors)
                }
            },
            None => Err(HardwareError::IdNotFound),
        }
    }

    fn set_value(&self, hardware_id: &str, value: i32) -> Result<(), crate::HardwareError> {
        todo!()
    }

    fn info(&self, hardware_id: &str) -> Result<String, crate::HardwareError> {
        match self.sensors.get(hardware_id) {
            Some(sensor) => Ok(sensor.info.clone()),
            None => Err(HardwareError::IdNotFound),
        }
    }
}

fn generate_id_name_info(
    chip_ref: &ChipRef,
    feature_ref: &FeatureRef,
    _sub_feature_ref: &SubFeatureRef,
) -> Option<(String, String, String)> {
    let Ok(sub_feature_ref) = feature_ref.sub_feature_by_kind(value::Kind::FanInput) else {
        return None;
    };

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
    let name = format!("{} {} {}", label, chip_name, sub_feature_name);
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
