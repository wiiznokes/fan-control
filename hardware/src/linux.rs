use lm_sensors::{feature, value, ChipRef, FeatureRef, LMSensors, SubFeatureRef};
use ouroboros::self_referencing;

use crate::{ControlH, FanH, Hardware, HardwareBridge, HardwareError, HardwareType, TempH, Value};

#[self_referencing]
pub struct LinuxBridge {
    lib: LMSensors,
    #[borrows(lib)]
    #[not_covariant]
    sensors: Vec<Sensor<'this>>,
}

#[derive(Debug, Clone)]
struct Sensor<'a> {
    hardware_type: HardwareType,
    sub_feature_ref: SubFeatureRef<'a>,
    name: String,
    info: String,
    id: String,
}

impl Drop for Sensor<'_> {
    fn drop(&mut self) {
        if self.hardware_type == HardwareType::Control {
            // TODO: set to auto
        }
    }
}

impl HardwareBridge for LinuxBridge {
    fn new() -> impl HardwareBridge {
        LinuxBridgeBuilder {
            lib: lm_sensors::Initializer::default().initialize().unwrap(),
            sensors_builder: |lib: &LMSensors| generate_sub_feature_refs(lib),
        }
        .build()
    }

    fn hardware(&self) -> Hardware {
        let mut hardware = Hardware::default();

        self.with_sensors(|sensors| {
            for (i, sensor) in sensors.iter().enumerate() {
                match sensor.hardware_type {
                    HardwareType::Control => hardware.controls.push(ControlH {
                        name: sensor.name.clone(),
                        hardware_id: sensor.id.clone(),
                        info: sensor.info.clone(),
                        internal_index: i,
                    }),
                    HardwareType::Fan => hardware.fans.push(FanH {
                        name: sensor.name.clone(),
                        hardware_id: sensor.id.clone(),
                        info: sensor.info.clone(),
                        internal_index: i,
                    }),
                    HardwareType::Temp => hardware.temps.push(TempH {
                        name: sensor.name.clone(),
                        hardware_id: sensor.id.clone(),
                        info: sensor.info.clone(),
                        internal_index: i,
                    }),
                }
            }
        });

        // TODO: remove this!
        hardware.controls.push(ControlH {
            name: "control1".into(),
            hardware_id: "control1".into(),
            info: "control1".into(),
            internal_index: 0,
        });

        hardware
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

fn generate_sub_feature_refs(lib: &LMSensors) -> Vec<Sensor<'_>> {
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
                                hardware_type: HardwareType::Fan,
                                sub_feature_ref,
                                name,
                                info,
                                id,
                            };
                            sensors.push(sensor);
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
                                id,
                            };
                            sensors.push(sensor);
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
