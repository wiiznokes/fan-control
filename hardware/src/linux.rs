use std::collections::HashMap;

use data::{
    node::HardwareType,
    serde::hardware::{Fan, Temp},
};
use lm_sensors::{feature, value, ChipRef, FeatureRef, LMSensors, SubFeatureRef};

use crate::HardwareGenerator;

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

    fn temps(&self) -> Vec<Temp> {
        Vec::new()
    }

    fn validate(
        &self,
        _hardware_type: &HardwareType,
        _hardware_id: &String,
    ) -> Result<(), crate::HardwareError> {
        todo!()
    }

    fn controls(&self) -> Vec<data::serde::hardware::Control> {
        todo!()
    }

    fn fans(&self) -> Vec<Fan> {
        todo!()
    }

    fn value(_hardware_id: &String) -> Result<Option<i32>, crate::HardwareError> {
        todo!()
    }

    fn set_value(_hardware_id: &String, _value: i32) -> Result<(), crate::HardwareError> {
        todo!()
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
