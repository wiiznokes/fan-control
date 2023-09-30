use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::sensors::lm_sensors::LinuxTemp;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hardware<'a> {
    #[serde(default, rename = "Control")]
    pub controls: Vec<Control>,
    #[serde(default, rename = "Temp")]
    pub temps: Vec<Temp<'a>>,
    #[serde(default, rename = "Fan")]
    pub fans: Vec<Fan>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Control {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fan {
    pub name: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Temp<'a> {
    pub name: String,

    
    #[cfg(target_os = "linux")]
    #[serde(skip)]
    pub hardware_temp: Option<LinuxTemp<'a>>,
}



impl <'a>Temp<'a> {
    

    fn value(&self) -> Option<i32>{

        #[cfg(target_os = "linux")]
        {
            // todo: move this part in Linux file
            match &self.hardware_temp {
                Some(hardware_temp) => match hardware_temp.sub_feature_ref.raw_value() {
                    Ok(value) => {
                        Some(value as i32)
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                        None
                    },
                },
                None => None,
            }
        }
    }

    pub fn new(name: String) -> Self {
        Temp { name, hardware_temp: None }
    }
}