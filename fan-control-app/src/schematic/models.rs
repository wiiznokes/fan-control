//! structs for storing ngspice model definitions such as for nmos, pmos, or diode models.
//!
//!

#[derive(Debug, Clone)]
pub struct NgModels {
    models: Vec<NgModel>,
}

impl Default for NgModels {
    fn default() -> Self {
        // basic elementary models for major semiconductor devices
        Self {
            models: vec![
                NgModel {
                    name: String::from("MOSN"),
                    definition: String::from("NMOS level=1"),
                },
                NgModel {
                    name: String::from("MOSP"),
                    definition: String::from("PMOS level=1"),
                },
                NgModel {
                    name: String::from("DMOD"),
                    definition: String::from("D"),
                },
                NgModel {
                    name: String::from("BJTP"),
                    definition: String::from("PNP"),
                },
                NgModel {
                    name: String::from("BJTN"),
                    definition: String::from("NPN"),
                },
            ],
        }
    }
}

impl NgModels {
    pub fn model_definitions(&self) -> String {
        let mut ret = String::new();
        for m in &self.models {
            ret.push_str(&m.model_line())
        }
        ret
    }
}

#[derive(Debug, Clone)]
struct NgModel {
    name: String,
    definition: String,
}

impl NgModel {
    fn model_line(&self) -> String {
        format!(".model {} {}\n", self.name, self.definition)
    }
}
