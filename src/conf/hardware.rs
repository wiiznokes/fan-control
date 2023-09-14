use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hardware<'a, S: FetchHardware> {
    #[serde(default, rename = "Control")]
    pub controls: Vec<Control>,
    #[serde(default, rename = "Temp")]
    pub temps: Vec<Box<Temp<'a, S>>>,
    #[serde(default, rename = "Fan")]
    pub fans: Vec<Box<Fan<'a, S>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Control {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Temp<'a, S: FetchHardware> {
    pub name: String,

    #[serde(skip)]
    pub sensor: Option<&'a S>,
}

impl<'a, S: FetchHardware> Temp<'a, S> {
    pub fn new(name: String) -> Box<Temp<'a, S>> {
        Box::new(Self { name, sensor: None })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fan<'a, S: FetchHardware> {
    pub name: String,

    #[serde(skip)]
    pub sensor: Option<&'a S>,
}

impl<'a, S: FetchHardware> Fan<'a, S> {
    pub fn new(name: String) -> Box<Fan<'a, S>> {
        Box::new(Self { name, sensor: None })
    }
}

pub trait FetchHardware
where
    Self: Debug + Clone,
{
    fn get_value(&self) -> i32;

    fn new(name: String) -> impl FetchHardware
    where
        Self: Sized;
}

pub trait SetHardware {
    fn set_value(value: i32);
}

pub trait HardwareGenerator<'a> {
    type Output: FetchHardware;

    fn new() -> impl HardwareGenerator<'a>;

    fn generate_controls(&self) -> Vec<Control>;
    fn generate_temps(&self) -> Vec<Box<Temp<'a, Self::Output>>>;
    fn generate_fans(&self) -> Vec<Box<Fan<'a, Self::Output>>>;
}
