//! Circuit
//! Concrete types for schematic content

use crate::schematic::atoms::NetEdge;
use crate::schematic::atoms::RcRDevice;
use crate::schematic::atoms::RcRLabel;
use crate::transforms::VSBox;

use enum_dispatch::enum_dispatch;

use crate::schematic::atoms::SchematicAtom;
use crate::transforms::VSPoint;
use std::hash::Hash;

/// an enum to unify different types in schematic (nets and devices)
#[enum_dispatch(SchematicAtom, Drawable)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CircuitAtom {
    NetEdge,
    RcRDevice,
    RcRLabel,
}
