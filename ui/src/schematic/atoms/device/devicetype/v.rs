//! device definition for independent voltage source (VXXXX)

// VXXXXXXX N + N - <<DC > DC / TRAN VALUE > < AC < ACMAG < ACPHASE > > >
// + < DISTOF1 < F1MAG < F1PHASE > > > < DISTOF2 < F2MAG < F2PHASE > > >

// VCC 10 0 DC 6 AC 1 PULSE(-1 1 2 NS 2 NS 2 NS 50 NS 100 NS 5)
// uses transient time zero value for DC if DC value not spec'd

use crate::schematic::atoms::CirArc;
use crate::schematic::atoms::Port;
use crate::schematic::interactable::Interactable;
use crate::transforms::{SSBox, SSPoint, VSPoint};

use super::super::params;
use super::Graphics;
use lazy_static::lazy_static;

pub const ID_PREFIX: &str = "V";

lazy_static! {
    static ref DEFAULT_GRAPHICS: Graphics = Graphics {
        pts: vec![
            vec![VSPoint::new(-0.25, 0.50), VSPoint::new(0.25, 0.50),],
            vec![VSPoint::new(0.00, 3.00), VSPoint::new(0.00, 1.00),],
            vec![VSPoint::new(0.00, 0.75), VSPoint::new(0.00, 0.25),],
            vec![VSPoint::new(0.00, -1.00), VSPoint::new(0.00, -3.00),],
            vec![VSPoint::new(-0.25, -0.50), VSPoint::new(0.25, -0.50),],
        ],
        cirarcs: vec![CirArc::from_triplet(
            VSPoint::new(0.00, 0.00),
            VSPoint::new(1.00, 0.00),
            VSPoint::new(1.00, 0.00)
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

// lazy_static! {
//     static ref DEFAULT_GRAPHICS: Graphics =
//         serde_json::from_slice(&std::fs::read("src/schematic/devices/devicetype/v.json").unwrap())
//             .unwrap();
// }

// DC 3.3
// AC 1
// ACPHASE 0
// transient {
//      None
//      PULSE
//      SINE
//      PWL
//      ..
// }

/// shared definition for independent voltage and current sources
#[allow(unused)]
struct VIDef {
    // implment trait for definitions
    dc: f32,
    ac: f32,
    acphase: f32,
    tran: VITran,
}
#[allow(unused)]
enum VITran {
    None,
    Pulse(VITranPulse),
    Sine(VITranSine),
    Pwl(VITranPwl),
}

/// ngspice manual 4.1.1 Voltage/Current Sources - independent - Pulse
#[allow(unused)]
struct VITranPulse {
    v1: f32,  // off/initial value
    v2: f32,  // on value
    td: f32,  // delay
    tr: f32,  // rise time, ngspice defaults to transient simulation step size
    tf: f32,  // fall time, ngspice defaults to transient simulation step size
    pw: f32,  // pulse width, ngspice defaults to transient simulation stop time
    per: f32, // period, ngspice defaults to transient simulation stop time
    np: Option<usize>, // number of pulses, ngspice defaults to unlimited
              // how to leave parameter unspecified for ngspice while specifying a parameter after?
}

/// ngspice manual 4.1.2 Voltage/Current Sources - indenpendent - Sinusoidal
#[allow(unused)]
struct VITranSine {
    vo: f32,    // offset volt/amp
    va: f32,    // amplitude volt/amp
    freq: f32,  // frequency hz, ngspice defaults to 1/simulation stop time
    td: f32,    // delay time s, ngspice defaults to 0
    theta: f32, // damping factor 1/s, ngspice defaults to 0
    phase: f32, // phase deg, ngspice defaults to 0
}

/// ngspice manual 4.1.4 Voltage/Current Sources - independent - piecewise linear
#[allow(unused)]
struct VITranPwl {
    vec: Vec<(f32, f32)>, // time, value tuple
                          // r: usize - available only with voltage source for now (ngspice)
                          // td: f32 - available only with voltage source for now (ngspice)
}

#[derive(Debug, Clone)]
pub enum Param {
    Raw(params::Raw),
}
impl Default for Param {
    fn default() -> Self {
        // ParamV::Raw(params::Raw::new(String::from("3.3")))
        Param::Raw(params::Raw::new(String::from("AC 1 SIN(3.3 1 2k 0 0)")))
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
pub struct V {
    pub params: Param,
    pub graphics: &'static Graphics,
}
impl Default for V {
    fn default() -> Self {
        Self {
            params: Param::default(),
            graphics: &DEFAULT_GRAPHICS,
        }
    }
}
