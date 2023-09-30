#![allow(unused_variables)]
#![allow(unreachable_code)]

use std::marker::PhantomData;

use lm_sensors::{feature::Kind, prelude::SharedChip, LMSensors, SubFeatureRef, ChipRef, FeatureRef};

use crate::conf::hardware::Temp;

use super::hardware::Generator;


pub struct LinuxGenerator {
    sensors: LMSensors,

}




impl Generator for LinuxGenerator {

    fn new() -> impl Generator {
        // Initialize LM sensors library.
        let sensors = lm_sensors::Initializer::default().initialize().unwrap();

        Self { sensors }
    }

 

    fn temps<'a>(&'a self) -> Vec<Box<Temp<'a>>> {
        let mut temps = Vec::new();

        self.sensors

        for chip_ref in self.sensors.chip_iter(None) {
            if let Some(path) = chip_ref.path() {
                println!("chip: {} at {} ({})", chip_ref, chip_ref.bus(), path.display());
            } else {
                println!("chip: {} at {}", chip_ref, chip_ref.bus());
            }
            

            for feature_ref in chip_ref.feature_iter() {
                if feature_ref.kind() != Some(Kind::Temperature) {
                    continue;
                }

                let Some(Ok(name)) = feature_ref.name() else {
                    continue;
                };

                let Ok(sub_feature_ref) =
                    feature_ref.sub_feature_by_kind(lm_sensors::value::Kind::FanInput)
                else {
                    continue;
                };
                    

                let linux_temp = LinuxTemp {
                    sub_feature_ref: sub_feature_ref,
                };
                
                let temp: Temp = Temp {
                    name: name.to_string(),
                    hardware_temp: Some(linux_temp)
                    
                };
                temps.push(Box::new(temp))
            }
        }
        temps
    }

  
}



#[derive(Debug, Clone)]
pub struct LinuxTemp<'a> {

    pub sub_feature_ref: SubFeatureRef<'a>,

}