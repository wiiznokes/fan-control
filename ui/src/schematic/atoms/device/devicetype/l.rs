//! device definition for inductors (LXXXX)

// LYYYYYYY n + n - < value > < mname > < nt = val > <m = val >
// + < scale = val > < temp = val > < dtemp = val > < tc1 = val >
// + < tc2 = val > < ic = init_condition >

use super::super::params;
use super::{Graphics, Port};
use crate::schematic::atoms::CirArc;
use crate::schematic::interactable::Interactable;
use crate::transforms::{SSBox, SSPoint, VSPoint};
use lazy_static::lazy_static;

pub const ID_PREFIX: &str = "L";

lazy_static! {
    static ref DEFAULT_GRAPHICS: Graphics = Graphics {
        pts: vec![
            vec![VSPoint::new(0.25, 1.00), VSPoint::new(0.00, 1.00),],
            vec![VSPoint::new(0.25, -2.00), VSPoint::new(0.00, -2.00),],
            vec![VSPoint::new(0.25, 0.00), VSPoint::new(0.00, 0.00),],
            vec![VSPoint::new(0.25, 2.00), VSPoint::new(0.00, 2.00),],
            vec![VSPoint::new(0.25, -1.00), VSPoint::new(0.00, -1.00),],
            vec![VSPoint::new(0.00, 3.00), VSPoint::new(0.00, 2.00),],
            vec![VSPoint::new(0.00, -2.00), VSPoint::new(0.00, -3.00),],
        ],
        cirarcs: vec![
            CirArc::from_triplet(
                VSPoint::new(0.25, 1.50),
                VSPoint::new(0.25, 1.00),
                VSPoint::new(0.25, 2.00)
            ),
            CirArc::from_triplet(
                VSPoint::new(0.25, 0.50),
                VSPoint::new(0.25, 0.00),
                VSPoint::new(0.25, 1.00)
            ),
            CirArc::from_triplet(
                VSPoint::new(0.25, -0.50),
                VSPoint::new(0.25, -1.00),
                VSPoint::new(0.25, 0.00)
            ),
            CirArc::from_triplet(
                VSPoint::new(0.25, -1.50),
                VSPoint::new(0.25, -2.00),
                VSPoint::new(0.25, -1.00)
            ),
        ],
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
        Param::Raw(params::Raw::new(String::from("1m")))
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
pub struct L {
    pub params: Param,
    pub graphics: &'static Graphics,
}
impl Default for L {
    fn default() -> Self {
        Self {
            params: Param::default(),
            graphics: &DEFAULT_GRAPHICS,
        }
    }
}
