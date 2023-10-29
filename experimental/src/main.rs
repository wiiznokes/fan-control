use lm_sensors::{value, LMSensors, SubFeatureRef};

pub struct X {
    lib: &'static LMSensors,
    v: Vec<SubFeatureRef<'static>>,
}

impl Drop for X {
    fn drop(&mut self) {
        let boxed = Box::new(self.lib);
        let ptr = Box::into_raw(boxed);
        unsafe {
            let _raw = Box::from_raw(ptr);
        }
    }
}

impl Default for X {
    fn default() -> Self {
        Self::new()
    }
}

impl X {
    pub fn new() -> Self {
        let lib = lm_sensors::Initializer::default().initialize().unwrap();
        let boxed = Box::new(lib);
        let leaked: &'static mut LMSensors = Box::leak(boxed);

        let mut v = Vec::new();

        for chip_ref in leaked.chip_iter(None) {
            for feature_ref in chip_ref.feature_iter() {
                if let Ok(sub_feature_ref) = feature_ref.sub_feature_by_kind(value::Kind::FanInput)
                {
                    v.push(sub_feature_ref)
                }
            }
        }

        X { lib: leaked, v }
    }
}

fn main() {
    let _x = X::new();
}
