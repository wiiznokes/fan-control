#![allow(unused_variables)]
#![allow(unreachable_code)]

use lm_sensors::{feature::Kind, prelude::SharedChip, LMSensors, SubFeatureRef};

use crate::conf::hardware::Temp;

use super::hardware::{FetchHardware, HardwareGenerator, TempH};

pub struct LmSensorsGenerator {
    sensors: LMSensors,
}

impl LmSensorsGenerator {}

impl<'a> HardwareGenerator<'a> for LmSensorsGenerator {

    type Output = LmSensor<'a>;
    fn new() -> impl HardwareGenerator<'a> {
        // Initialize LM sensors library.
        let sensors = lm_sensors::Initializer::default().initialize().unwrap();

        Self { sensors }
    }

 

    fn temps(&self) -> Vec<Box<TempH<'a, Self::Output>>> {
        let mut temps:Vec<Box<TempH<Self::Output>>> = Vec::new();

        for chip in self.sensors.chip_iter(None) {
            if let Some(path) = chip.path() {
                println!("chip: {} at {} ({})", chip, chip.bus(), path.display());
            } else {
                println!("chip: {} at {}", chip, chip.bus());
            }

            for feature in chip.feature_iter() {
                if feature.kind() != Some(Kind::Temperature) {
                    continue;
                }

                let Some(Ok(name)) = feature.name() else {
                    continue;
                };

                let Ok(sub_feature) =
                    feature.sub_feature_by_kind(lm_sensors::value::Kind::FanInput)
                else {
                    continue;
                };
                    
                let temp_h = TempH {
                    temp: Temp {
                        name: name.to_string()
                    },
                    sensor: &LmSensor { sub_feature }
                };
                temps.push(Box::new(temp_h))
            }
        }
        temps
    }

  
}

#[derive(Debug, Clone)]
pub struct LmSensor<'a> {
    sub_feature: SubFeatureRef<'a>,
}

impl<'a> FetchHardware for LmSensor<'a> {
    fn get_value(&self) -> i32 {
        self.get_value()
    }

    fn new(name: String) -> Self {
        let a = todo!();
        Self { sub_feature: a }
    }
}
