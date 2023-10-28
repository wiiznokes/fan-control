//! device definition for diodes (DXXXX)
// . model DMOD D

// DXXXXXXX n + n - mname < area = val > <m = val > < pj = val > <off >
// + < ic = vd > < temp = val > < dtemp = val >
// + < lm = val > < wm = val > < lp = val > < wp = val >

use crate::schematic::atoms::Port;
use crate::schematic::interactable::Interactable;
use crate::transforms::{SSBox, SSPoint, VSPoint};

use super::super::params;
use super::Graphics;
use lazy_static::lazy_static;

pub const ID_PREFIX: &str = "D";

lazy_static! {
    static ref DEFAULT_GRAPHICS: Graphics = Graphics {
        pts: vec![
            vec![VSPoint::new(-1.00, -0.50), VSPoint::new(1.00, -0.50),],
            vec![VSPoint::new(0.00, -0.50), VSPoint::new(0.00, -3.00),],
            vec![VSPoint::new(-1.00, 1.00), VSPoint::new(1.00, 1.00),],
            vec![VSPoint::new(0.00, 1.00), VSPoint::new(0.00, 3.00),],
            vec![VSPoint::new(0.00, -0.50), VSPoint::new(1.00, 1.00),],
            vec![VSPoint::new(0.00, -0.50), VSPoint::new(-1.00, 1.00),],
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

#[derive(Debug, Clone)]
pub enum Param {
    Raw(params::Raw),
}
impl Default for Param {
    fn default() -> Self {
        Param::Raw(params::Raw::new(String::from("dmod")))
    }
}
impl Param {
    pub fn summary(&self) -> String {
        match self {
            Param::Raw(s) => s.raw.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct D {
    pub params: Param,
    pub graphics: &'static Graphics,
}
impl Default for D {
    fn default() -> Self {
        Self {
            params: Param::default(),
            graphics: &DEFAULT_GRAPHICS,
        }
    }
}
