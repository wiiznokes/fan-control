//! device definition for capacitors (CXXXX)

// CXXXXXXX n + n - < value > < mname > <m = val > < scale = val > < temp = val >
// + < dtemp = val > < tc1 = val > < tc2 = val > < ic = init_condition >

use super::super::params;
use super::{Graphics, Port};
use crate::schematic::atoms::CirArc;
use crate::schematic::interactable::Interactable;
use crate::transforms::{SSBox, SSPoint, VSPoint};
use lazy_static::lazy_static;

pub const ID_PREFIX: &str = "C";

lazy_static! {
    static ref DEFAULT_GRAPHICS: Graphics = Graphics {
        pts: vec![
            vec![VSPoint::new(0.00, -0.25), VSPoint::new(0.00, -3.00),],
            vec![VSPoint::new(-1.00, 0.50), VSPoint::new(1.00, 0.50),],
            vec![VSPoint::new(0.00, 3.00), VSPoint::new(0.00, 0.50),],
        ],
        cirarcs: vec![CirArc::from_triplet(
            VSPoint::new(0.00, -2.00),
            VSPoint::new(1.00, -0.50),
            VSPoint::new(-1.00, -0.50)
        ),],
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
pub enum ParamC {
    Raw(params::Raw),
}
impl Default for ParamC {
    fn default() -> Self {
        ParamC::Raw(params::Raw::new(String::from("10p")))
    }
}
impl ParamC {
    pub fn summary(&self) -> String {
        match self {
            ParamC::Raw(s) => s.raw.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct C {
    pub params: ParamC,
    pub graphics: &'static Graphics,
}
impl Default for C {
    fn default() -> Self {
        Self {
            params: ParamC::default(),
            graphics: &DEFAULT_GRAPHICS,
        }
    }
}
