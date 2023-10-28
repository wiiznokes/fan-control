//#![allow(unused_variables)]
//#![allow(unreachable_code)]


use std::collections::HashMap;

use data::hardware::HardwareType;
use lm_sensors::{feature::Kind, LMSensors, SubFeatureRef};

use crate::HardwareGenerator;






pub struct LinuxGenerator {
    lib_sensors: LMSensors,
    
    sensors: HashMap<&str, Sensor>
}


#[derive(Debug, Clone)]
struct Sensor<'a> {
    hardware_type: HardwareType,
    sub_feature_ref: SubFeatureRef<'a>,
}

impl HardwareGenerator for LinuxGenerator {

    fn new() -> impl Generator {
        let lib_sensors = lm_sensors::Initializer::default().initialize().unwrap();

        Self { 
            lib_sensors,
            sensors: HashMap::new()
        }
    }

 

    fn temps<'a>(&'a self) -> Vec<Temp> {
        let mut temps = Vec::new();

        for chip_ref in self.sensors.chip_iter(None) {
            /*
            if let Some(path) = chip_ref.path() {
                println!("chip: {} at {} ({})", chip_ref, chip_ref.bus(), path.display());
            } else {
                println!("chip: {} at {}", chip_ref, chip_ref.bus());
            }
            */
            
            for feature_ref in chip_ref.feature_iter() {
                if feature_ref.kind() != Some(Kind::Temperature) {
                    continue;
                }

                let Some(Ok(name)) = feature_ref.name() else {
                    continue;
                };

                let Ok(sub_feature_ref) =
                    feature_ref.sub_feature_by_kind(lm_sensors::value::Kind::TemperatureInput)
                else {
                    continue;
                };
                    

                let linux_temp = LinuxTemp {
                    sub_feature_ref,
                };
                
                let temp = Temp::new(
                    name.to_string(),
                    Some(linux_temp)
                );

              
                temps.push(temp)
            }
        }
        temps
    }

  
}



#[derive(Debug, Clone)]
pub struct LinuxTemp<'a> {

    pub sub_feature_ref: SubFeatureRef<'a>,

}


impl <'a>LinuxTemp<'a> {
    
    pub fn value(&self) -> Option<i32> {
        
        match self.sub_feature_ref.raw_value() {
            Ok(value) => {
                Some(value as i32)
            },
            Err(e) => {
                eprintln!("{}", e);
                None
            },
        }
    }
}