use lm_sensors::{feature, value, ChipRef, FeatureRef, LMSensors, SubFeatureRef};
use ouroboros::self_referencing;

use crate::{
    ControlH, FanH, Hardware, HardwareBridge, HardwareError, InternalControlIndex, TempH, Value,
};

#[self_referencing]
pub struct LinuxBridge {
    lib: LMSensors,
    #[borrows(lib)]
    #[not_covariant]
    sensors: Vec<Sensor<'this>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SubFeatureType {
    PwmIo,
    PwmEnable,
    Fan,
    Temp,
}

#[derive(Debug, Clone)]
struct Sensor<'a> {
    sub_feature_type: SubFeatureType,
    sub_feature_ref: SubFeatureRef<'a>,
}

impl Drop for Sensor<'_> {
    fn drop(&mut self) {
        if self.sub_feature_type == SubFeatureType::PwmEnable {
            println!("pwm sould be set to auto");
            // TODO: set to auto
        }
    }
}

impl HardwareBridge for LinuxBridge {
    fn new() -> (impl HardwareBridge, Hardware) {
        let mut hardware = Hardware::default();

        let bridge = LinuxBridgeBuilder {
            lib: lm_sensors::Initializer::default().initialize().unwrap(),
            sensors_builder: |lib: &LMSensors| generate_sub_feature_refs(lib, &mut hardware),
        }
        .build();

        (bridge, hardware)
    }

    fn value(&self, internal_index: &usize) -> Result<Value, crate::HardwareError> {
        self.with_sensors(|sensors| match sensors.get(*internal_index) {
            Some(sensor) => match sensor.sub_feature_ref.raw_value() {
                Ok(value) => Ok(value as i32),
                Err(e) => {
                    eprintln!("{}", e);
                    Err(HardwareError::LmSensors)
                }
            },
            None => Err(HardwareError::IdNotFound),
        })
    }

    fn set_value(&self, internal_index: &usize, value: Value) -> Result<(), crate::HardwareError> {
        println!("set value {} to {}", value, internal_index);
        Ok(())
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

fn generate_sub_feature_refs<'a>(lib: &'a LMSensors, hardware: &mut Hardware) -> Vec<Sensor<'a>> {
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
                            let sensor = Sensor {
                                sub_feature_type: SubFeatureType::Fan,
                                sub_feature_ref,
                            };
                            sensors.push(sensor);

                            let fan_h = FanH {
                                name,
                                hardware_id: id,
                                info,
                                internal_index: sensors.len() - 1,
                            };
                            hardware.fans.push(fan_h);
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
                                sub_feature_type: SubFeatureType::Temp,
                                sub_feature_ref,
                            };
                            sensors.push(sensor);

                            let temp_h = TempH {
                                name,
                                hardware_id: id,
                                info,
                                internal_index: sensors.len() - 1,
                            };
                            hardware.temps.push(temp_h);
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
                            let sensor_io = Sensor {
                                sub_feature_type: SubFeatureType::PwmIo,
                                sub_feature_ref: sub_feature_ref_io,
                            };
                            sensors.push(sensor_io);
                            let sensor_enable = Sensor {
                                sub_feature_type: SubFeatureType::PwmEnable,
                                sub_feature_ref: sub_feature_ref_io,
                            };
                            sensors.push(sensor_enable);

                            let control_h = ControlH {
                                name,
                                hardware_id: id,
                                info,
                                internal_index: InternalControlIndex {
                                    io: sensors.len() - 2,
                                    enable: sensors.len() - 1,
                                },
                            };
                            hardware.controls.push(control_h);
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
