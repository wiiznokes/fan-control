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
    hardware_temp: Option<LinuxTemp<'a>>,
}



impl <'a>Temp<'a> {
    

    pub fn value(&self) -> Option<i32>{

        #[cfg(target_os = "linux")]
        if let Some(hardware_temp) = &self.hardware_temp {
            hardware_temp.value()
        } else {
            None
        }
        
        #[cfg(target_os = "windows")]
        todo!()
       
    }

    #[cfg(target_os = "linux")]
    pub fn new(name: String, hardware_temp: Option<LinuxTemp<'a>>) -> Self {
        Temp { name, hardware_temp }
    }

    #[cfg(target_os = "windows")]
    pub fn new(name: String) -> Self {
        Temp { name }
    }

}