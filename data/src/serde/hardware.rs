use std::fmt::Debug;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hardware {
    #[serde(default, rename = "Control")]
    pub controls: Vec<Control>,
    #[serde(default, rename = "Fan")]
    pub fans: Vec<Fan>,
    #[serde(default, rename = "Temp")]
    pub temps: Vec<Temp>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Control {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fan {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Temp {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: String,
}