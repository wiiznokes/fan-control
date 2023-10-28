//! device definition for ground, implemented as a 0-volt voltage source

use super::Graphics;
use crate::schematic::atoms::Port;
use crate::{
    schematic::interactable::Interactable,
    transforms::{SSBox, SSPoint, VSPoint},
};
use lazy_static::lazy_static;

pub const ID_PREFIX: &str = "VGND";

lazy_static! {
    static ref DEFAULT_GRAPHICS: Graphics = Graphics {
        pts: vec![
            vec![VSPoint::new(0., 2.), VSPoint::new(0., -1.)],
            vec![
                VSPoint::new(0., -2.),
                VSPoint::new(1., -1.),
                VSPoint::new(-1., -1.),
                VSPoint::new(0., -2.),
            ],
        ],
        cirarcs: vec![],
        ports: vec![Port {
            name: "gnd".to_string(),
            offset: SSPoint::new(0, 2),
            interactable: Interactable::default(),
        }],
        bounds: SSBox::new(SSPoint::new(-1, 2), SSPoint::new(1, -2)),
    };
}

#[derive(Debug, Default, Clone)]
pub enum Param {
    #[default]
    None,
}
impl Param {
    pub fn summary(&self) -> String {
        String::from("0 0")
    }
}

#[derive(Debug, Clone)]
pub struct Gnd {
    pub params: Param,
    pub graphics: &'static Graphics,
}
impl Default for Gnd {
    fn default() -> Self {
        Self {
            params: Param::default(),
            graphics: &DEFAULT_GRAPHICS,
        }
    }
}
