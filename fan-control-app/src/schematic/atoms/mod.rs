mod bounds;
mod cirarc;
mod device;
mod lineseg;
mod net_label;
mod nets;
mod port;

pub use bounds::Bounds;
pub use bounds::RcRBounds;

pub use cirarc::CirArc;
pub use cirarc::RcRCirArc;

pub use lineseg::LineSeg;
pub use lineseg::RcRLineSeg;

pub use device::devicetype;
pub use device::devicetype::DeviceClass;
pub use device::RcRDevice;

pub use net_label::RcRLabel;

pub use nets::{NetEdge, NetVertex};

pub use port::Port;
pub use port::RcRPort;

pub use nets::PathWeight;

use crate::transforms::VSBox;
use crate::transforms::VSPoint;

use crate::Drawable;

use enum_dispatch::enum_dispatch;

use std::hash::Hash;

#[enum_dispatch]
pub trait SchematicAtom: Hash + Eq + Drawable + Clone {
    /// returns true if self contains ssp
    fn contains_vsp(&self, vsp: VSPoint) -> bool;
    fn bounding_box(&self) -> VSBox;
}
