use lm_sensors::{feature::Kind, prelude::SharedChip, LMSensors, SubFeatureRef};

use super::hardware::{HardwareGenerator, Temp, FetchHardware, Sensor};

pub struct LmSensorsGenerator {
    sensors: LMSensors,
}

impl LmSensorsGenerator {}

impl HardwareGenerator for LmSensorsGenerator {
    fn new() -> Self {
        // Initialize LM sensors library.
        let sensors = lm_sensors::Initializer::default().initialize().unwrap();

        Self { sensors }
    }

    fn generate_controls(&self) -> Vec<super::hardware::Control> {
        todo!()
    }

    fn generate_temps(&self) -> Vec<Box<dyn Sensor>> {
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

                let Ok(sub_feature) =
                    feature.sub_feature_by_kind(lm_sensors::value::Kind::FanInput)
                else {
                    continue;
                };

                let sensor = Temp {
                    name: name.to_string(),
                    sensor: LmSensor{ sub_feature},
                };
            }
        }
        temps
    }

    fn generate_fans(&self) -> Vec<Box<dyn Sensor>> {
        todo!()
    }

}





struct LmSensor<'a> {
    name: String,
    sub_feature: SubFeatureRef<'a>
}



impl FetchHardware for LmSensor<'_> {
    fn get_value(&self) -> i32 {
        self.get_value()
    }
}

impl Sensor for LmSensor<'_> {
  

    fn name(&self) -> String {
        self.name
    }
}