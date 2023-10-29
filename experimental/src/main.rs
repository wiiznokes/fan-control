use lm_sensors::{SubFeatureRef, value, LMSensors};

pub struct X<'a> {
    lib: LMSensors,
    v: Vec<SubFeatureRef<'a>>
}

impl <'a>X<'a> {
    
    pub fn new() -> Self {
        let lib = lm_sensors::Initializer::default().initialize().unwrap();
        let mut v = Vec::new();

        for chip_ref in lib.chip_iter(None) {
            for feature_ref in chip_ref.feature_iter() {
                if let Ok(sub_feature_ref) =
                    feature_ref.sub_feature_by_kind(value::Kind::FanInput) {
                        v.push(sub_feature_ref)
                }
            }
        }

        X {
            lib,
            v
        }
    }
}

fn main() {
    let _x = X::new();
}
