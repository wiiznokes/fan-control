use crate::schematic::atoms::RcRPort;

use crate::schematic::SchematicAtom;
use crate::transforms::VSBox;
use crate::transforms::VSPoint;

use enum_dispatch::enum_dispatch;

use crate::schematic::atoms::{RcRBounds, RcRCirArc, RcRLineSeg};

/// an enum to unify different types in schematic (lines and ellipses)
#[enum_dispatch(SchematicAtom, Drawable)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DesignerElement {
    RcRLineSeg,
    RcRPort,
    RcRCirArc,
    RcRBounds,
}
