//! device definition for resistors (RXXXX)

use super::super::params;
use super::Graphics;
use crate::schematic::atoms::Port;
use crate::schematic::interactable::Interactable;
use crate::transforms::{SSBox, SSPoint, VSPoint};
use lazy_static::lazy_static;

pub const ID_PREFIX: &str = "R";

lazy_static! {
    static ref DEFAULT_GRAPHICS: Graphics = Graphics {
        pts: vec![
            vec![VSPoint::new(1.00, -0.25), VSPoint::new(-1.00, -0.75),],
            vec![VSPoint::new(-1.00, -0.75), VSPoint::new(1.00, -1.25),],
            vec![VSPoint::new(1.00, -1.25), VSPoint::new(-1.00, -1.75),],
            vec![VSPoint::new(0.00, -2.00), VSPoint::new(0.00, -3.00),],
            vec![VSPoint::new(-1.00, -1.75), VSPoint::new(0.00, -2.00),],
            vec![VSPoint::new(1.00, 1.75), VSPoint::new(-1.00, 1.25),],
            vec![VSPoint::new(1.00, 0.75), VSPoint::new(-1.00, 0.25),],
            vec![VSPoint::new(-1.00, 1.25), VSPoint::new(1.00, 0.75),],
            vec![VSPoint::new(0.00, 3.00), VSPoint::new(0.00, 2.00),],
            vec![VSPoint::new(0.00, 2.00), VSPoint::new(1.00, 1.75),],
            vec![VSPoint::new(-1.00, 0.25), VSPoint::new(1.00, -0.25),],
        ],
        cirarcs: vec![],
        ports: vec![
            Port {
                name: "0".to_string(),
                offset: SSPoint::new(0, 3),
                interactable: Interactable::default()
            },
            Port {
                name: "1".to_string(),
                offset: SSPoint::new(0, -3),
                interactable: Interactable::default()
            },
        ],
        bounds: SSBox::new(SSPoint::new(-2, -3), SSPoint::new(2, 3)),
    };
}

/// Enumerates the different ways to specifify parameters for a resistor
#[derive(Debug, Clone)]
pub enum Param {
    /// specify the spice line directly (after id and port connections)
    Raw(params::Raw),
}
impl Default for Param {
    fn default() -> Self {
        Param::Raw(params::Raw::new(String::from("1k")))
    }
}
impl Param {
    pub fn summary(&self) -> String {
        match self {
            Param::Raw(s) => s.raw.clone(),
        }
    }
}

/// resistor device class
#[derive(Debug, Clone)]
pub struct R {
    /// parameters of the resistor
    pub params: Param,
    /// graphic representation of the resistor
    pub graphics: &'static Graphics,
}
impl Default for R {
    fn default() -> Self {
        Self {
            params: Param::default(),
            graphics: &DEFAULT_GRAPHICS,
        }
    }
}
