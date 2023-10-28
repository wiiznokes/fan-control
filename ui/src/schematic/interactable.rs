//! common functionality for interactive schematic elements
use crate::transforms::{SSPoint, VSBox, VSBoxExt, VSPoint, VVTransform};

/// trait to facillitates and unify implementation of interactive logic
pub trait Interactive {
    fn transform(&mut self, sst: VVTransform);
}

/// struct to facillitates and unify implementation of interactive logic through composition
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Interactable {
    /// the bounds of the interactable. e.g. mouse hover over this area should highlight the interactable.
    pub bounds: VSBox,
}

impl Interactable {
    pub fn new(vsb: VSBox) -> Self {
        Interactable { bounds: vsb }
    }
    /// returns true if Schematic Space Point intersects with bounds.
    pub fn contains_ssp(&self, ssp: SSPoint) -> bool {
        let vsp = ssp.cast().cast_unit();
        self.bounds.inclusive_contains(vsp)
    }
    /// returns true if Viewport Space Point intersects with bounds.
    pub fn contains_vsp(&self, vsp: VSPoint) -> bool {
        self.bounds.inclusive_contains(vsp)
    }
    /// returns true if bounds intersects with argument.
    pub fn intersects_vsb(&self, vsb: &VSBox) -> bool {
        self.bounds.intersects(vsb)
    }
    /// returns true if bound is contained by argument.
    pub fn contained_by(&self, vsb: &VSBox) -> bool {
        vsb.contains_box(&self.bounds)
    }
}
