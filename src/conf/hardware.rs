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
pub struct Temp {
    pub name: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fan {
    pub name: String,
}

