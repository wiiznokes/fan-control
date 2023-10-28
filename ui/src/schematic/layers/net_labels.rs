//! net name
//!
//!
//!

// every strongly separated net graph should be assigned one NetLabel on checking ... ?
// or: every net seg holds a Rc<NetLabel>, the same underlying NetLabel if connected
// how to handle multiple user defined within a connected graph? should highlight segments

// maybe do what cadence does and do check only when user asks

use std::collections::HashSet;

use crate::transforms::{SSPoint, VCTransform, VSBox, VSPoint};
use crate::Drawable;
use iced::widget::canvas::Frame;

use crate::schematic::atoms::RcRLabel;

use super::SchematicLayerTrait;

pub type NetLabelsLayer = Box<NetLabels>;

impl SchematicLayerTrait<RcRLabel> for NetLabelsLayer {
    #[doc = " draws self\\'s contents on frame"]
    fn draw_persistent(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        for d in &self.set {
            d.0.borrow().draw_persistent(vct, vcscale, frame);
        }
    }

    #[doc = " returns bounding box containing all atoms in layer"]
    fn bounds(&self) -> VSBox {
        let pts = self.set.iter().flat_map(|l| {
            [
                l.0.borrow().interactable.bounds.min,
                l.0.borrow().interactable.bounds.max,
            ]
            .into_iter()
        });
        VSBox::from_points(pts).cast().cast_unit()
    }

    #[doc = " increments count for every atom over vsp, returns Some(atom) once count == skip"]
    fn selectable(&self, vsp: VSPoint, skip: usize, count: &mut usize) -> Option<RcRLabel> {
        for l in &self.set {
            if l.0.borrow_mut().interactable.contains_vsp(vsp) {
                if *count == skip {
                    // skipped just enough
                    return Some(l.clone());
                } else {
                    *count += 1;
                }
            }
        }
        None
    }

    #[doc = " returns slice of all atoms in layer which intersect with closed area defined by vsb"]
    fn intersect(&self, vsb: &VSBox) -> Box<[RcRLabel]> {
        self.set
            .iter()
            .filter_map(|l| {
                if l.0.borrow_mut().interactable.intersects_vsb(vsb) {
                    Some(l.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    #[doc = " returns slice of all atoms in layer which fit in open area defined by vsb"]
    fn contained(&self, vsb: &VSBox) -> Box<[RcRLabel]> {
        self.set
            .iter()
            .filter_map(|l| {
                if l.0.borrow_mut().interactable.contained_by(vsb) {
                    Some(l.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    #[doc = " place the device in layer - replace existing if atom equates to existing, or adds new if not"]
    fn place(&mut self, atom: RcRLabel) {
        self.set.insert(atom);
    }

    #[doc = " delete the specified atom if it exists"]
    fn delete(&mut self, atom: &RcRLabel) {
        self.set.remove(atom);
    }
}

/// struct containing all devices in schematic
#[derive(Debug, Default, Clone)]
pub struct NetLabels {
    /// set of all devices
    set: HashSet<RcRLabel>,
}

impl NetLabels {
    /// returns the first Label after skip which intersects with curpos_ssp in a BaseElement, if any.
    /// count is updated to track the number of elements skipped over
    pub fn selectable(
        &mut self,
        curpos_vsp: VSPoint,
        skip: usize,
        count: &mut usize,
    ) -> Option<RcRLabel> {
        for l in &self.set {
            if l.0.borrow_mut().interactable.contains_vsp(curpos_vsp) {
                if *count == skip {
                    // skipped just enough
                    return Some(l.clone());
                } else {
                    *count += 1;
                }
            }
        }
        None
    }
    /// returns the bounding box of all devices
    pub fn bounding_box(&self) -> VSBox {
        let pts = self.set.iter().flat_map(|l| {
            [
                l.0.borrow().interactable.bounds.min,
                l.0.borrow().interactable.bounds.max,
            ]
            .into_iter()
        });
        VSBox::from_points(pts).cast().cast_unit()
    }
    /// inserts label l into self.
    pub fn insert(&mut self, l: RcRLabel) {
        self.set.insert(l);
    }
    /// return vector of RcRDevice which intersects vsb
    pub fn intersects_vsb(&self, vsb: &VSBox) -> Vec<RcRLabel> {
        let ret: Vec<_> = self
            .set
            .iter()
            .filter_map(|l| {
                if l.0.borrow_mut().interactable.intersects_vsb(vsb) {
                    Some(l.clone())
                } else {
                    None
                }
            })
            .collect();
        ret
    }
    /// return vector of RcRDevice which are contained by vsb
    pub fn contained_by(&self, vsb: &VSBox) -> Vec<RcRLabel> {
        let ret: Vec<_> = self
            .set
            .iter()
            .filter_map(|l| {
                if l.0.borrow_mut().interactable.contained_by(vsb) {
                    Some(l.clone())
                } else {
                    None
                }
            })
            .collect();
        ret
    }
    pub fn delete_item(&mut self, d: &RcRLabel) {
        self.set.remove(d);
    }
    pub fn new_label() -> RcRLabel {
        RcRLabel::default()
    }
    /// returns true if any label is on ssp
    pub fn any_occupy_ssp(&self, ssp: SSPoint) -> bool {
        self.set.iter().any(|label| label.0.borrow().pos() == ssp)
    }
}

impl Drawable for NetLabels {
    fn draw_persistent(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        for d in &self.set {
            d.0.borrow().draw_persistent(vct, vcscale, frame);
        }
    }
    fn draw_selected(&self, _vct: VCTransform, _vcscale: f32, _frame: &mut Frame) {
        panic!("not intended for use");
    }
    fn draw_preview(&self, _vct: VCTransform, _vcscale: f32, _frame: &mut Frame) {
        panic!("not intended for use");
    }
}
