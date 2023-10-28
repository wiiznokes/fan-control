use std::fmt::Debug;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hardware {
    #[serde(default, rename = "Control")]
    pub controls: Vec<Control>,
    #[serde(default, rename = "Temp")]
    pub temps: Vec<Temp>,
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
pub struct Temp {
    pub name: String,
    
    pub hardware_id: String,

    #[serde(skip, default = "temp")]
    hardware_type: HardwareType,
}

impl Temp {
    
    pub fn new(name: String, hardware_id: Option<u32>) -> Temp {
        return Temp {
            name: name.clone(),
            hardware_id: name,
            hardware_type: HardwareType::Temp
        };
    }
}

fn temp() -> HardwareType {
    return HardwareType::Temp;
}

#[derive(Debug, Clone)]
pub enum HardwareType {
    Temp,
    Fan,
    Control
}


