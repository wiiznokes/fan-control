use lm_sensors::{LMSensors, prelude::SharedChip, feature::Kind};

use super::hardware::{HardwareGenerator, Temp};



struct LmSensorsGenerator {

    sensors: LMSensors
}


impl LmSensorsGenerator {
    
    fn new() -> Self {
        // Initialize LM sensors library.
        let sensors = lm_sensors::Initializer::default().initialize().unwrap();

        return Self { sensors };
    }
}

impl HardwareGenerator for LmSensorsGenerator  {
    fn generate_controls(&self) -> Vec<super::hardware::Control> {
        todo!()
    }

    fn generate_temps(&self) -> Vec<super::hardware::Temp> {

        let temps = Vec::new();

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
                
                let Ok(sub_feature) = feature.sub_feature_by_kind(lm_sensors::value::Kind::FanInput) else {
                    continue;
                };
                
                let temp = Temp {
                    name: name.to_string(),
                };


            }
        }
        temps
    }

    fn generate_fans(&self) -> Vec<super::hardware::Fan> {
        todo!()
    }
}