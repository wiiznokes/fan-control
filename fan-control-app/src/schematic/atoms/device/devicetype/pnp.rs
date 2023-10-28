//! device definition for pnp BJTs (QXXXX)
// port order: c b e (vbe <-> vgs for reference)
// followed by model name

// QXXXXXXX nc nb ne <ns > <tj > mname < area = val > < areac = val >
// + < areab = val > <m= val > <off > < ic = vbe , vce > < temp = val >
// + < dtemp = val >

use super::super::params;
use super::{Graphics, Port};
use crate::schematic::interactable::Interactable;
use crate::transforms::{SSBox, SSPoint, VSPoint};
use lazy_static::lazy_static;

pub const ID_PREFIX: &str = "QP";

lazy_static! {
    static ref DEFAULT_GRAPHICS: Graphics = Graphics {
        pts: vec![
            vec![VSPoint::new(0.00, 0.75), VSPoint::new(2.00, 1.50),],
            vec![VSPoint::new(0.00, 1.50), VSPoint::new(0.00, -1.50),],
            vec![VSPoint::new(2.00, 1.50), VSPoint::new(2.00, 3.00),],
            vec![VSPoint::new(0.00, 0.00), VSPoint::new(-2.00, 0.00),],
            vec![VSPoint::new(1.00, -1.00), VSPoint::new(1.75, -0.75),],
            vec![VSPoint::new(2.00, -1.50), VSPoint::new(2.00, -3.00),],
            vec![VSPoint::new(1.00, -1.00), VSPoint::new(1.25, -1.75),],
            vec![VSPoint::new(0.00, -0.75), VSPoint::new(2.00, -1.50),],
        ],
        cirarcs: vec![],
        ports: vec![
            Port {
                name: "0".to_string(),
                offset: SSPoint::new(2, 3),
                interactable: Interactable::default()
            },
            Port {
                name: "1".to_string(),
                offset: SSPoint::new(-2, 0),
                interactable: Interactable::default()
            },
            Port {
                name: "2".to_string(),
                offset: SSPoint::new(2, -3),
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
        Param::Raw(params::Raw::new(String::from("bjtp")))
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
pub struct M {
    pub params: Param,
    pub graphics: &'static Graphics,
}
impl Default for M {
    fn default() -> Self {
        Self {
            params: Param::default(),
            graphics: &DEFAULT_GRAPHICS,
        }
    }
}
