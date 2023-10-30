use lm_sensors::{value, LMSensors, SubFeatureRef};
use ouroboros::self_referencing;

#[self_referencing]
pub struct X {
    lib: LMSensors,
    #[borrows(lib)]
    #[not_covariant]
    v: Vec<SubFeatureRef<'this>>,
}

impl X {
    pub fn contruct() -> Self {
        XBuilder {
            lib: lm_sensors::Initializer::default().initialize().unwrap(),
            v_builder: |lib_from_struct: &LMSensors| {
                let mut v = Vec::new();

                for chip_ref in lib_from_struct.chip_iter(None) {
                    for feature_ref in chip_ref.feature_iter() {
                        if let Ok(sub_feature_ref) =
                            feature_ref.sub_feature_by_kind(value::Kind::FanInput)
                        {
                            v.push(sub_feature_ref)
                        }
                    }
                }
                v
            },
        }
        .build()
    }
}

fn main() {
    let _x = X::contruct();
}
