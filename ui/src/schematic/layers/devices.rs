//! devices, e.g. resistors, voltage sources, etc.

use std::collections::HashSet;

use crate::schematic::atoms::DeviceClass;
use crate::schematic::atoms::RcRDevice;
use crate::schematic::interactable::Interactive;
use crate::transforms::{self, SSPoint, VCTransform, VSBox, VSPoint};
use crate::Drawable;

use iced::widget::canvas::Frame;

use crate::schematic::atoms::devicetype::{
    c::C, d::D, gnd::Gnd, i::I, l::L, nmos, pmos, r::R, v::V,
};

/// struct to keep track of unique IDs for all devices of a type
#[derive(Debug, Clone)]
struct ClassManager {
    // watermark keeps track of the last ID given out
    wm: usize,
}

impl ClassManager {
    pub fn new() -> Self {
        ClassManager { wm: 0 }
    }
    pub fn incr(&mut self) -> usize {
        self.wm += 1;
        self.wm
    }
}

/// struct to keep track of unique IDs for all devices of all types
#[derive(Debug, Clone)]
struct DevicesManager {
    pm: ClassManager,
    nm: ClassManager,
    gnd: ClassManager,
    r: ClassManager,
    l: ClassManager,
    c: ClassManager,
    v: ClassManager,
    i: ClassManager,
    d: ClassManager,
}

impl Default for DevicesManager {
    fn default() -> Self {
        Self {
            pm: ClassManager::new(),
            nm: ClassManager::new(),
            gnd: ClassManager::new(),
            r: ClassManager::new(),
            l: ClassManager::new(),
            c: ClassManager::new(),
            v: ClassManager::new(),
            i: ClassManager::new(),
            d: ClassManager::new(),
        }
    }
}

/// struct containing all devices in schematic
#[derive(Debug, Default, Clone)]
pub struct Devices {
    /// set of all devices
    set: HashSet<RcRDevice>,
    /// manager to facillate assignment of unique IDs to each device
    manager: DevicesManager,
}

pub type DevicesLayer = Box<Devices>;

impl super::SchematicLayerTrait<RcRDevice> for DevicesLayer {
    fn draw_persistent(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        for d in &self.set {
            d.0.borrow().draw_persistent(vct, vcscale, frame);
        }
    }

    fn bounds(&self) -> VSBox {
        let pts = self.set.iter().flat_map(|d| {
            [
                d.0.borrow().interactable.bounds.min,
                d.0.borrow().interactable.bounds.max,
            ]
            .into_iter()
        });
        VSBox::from_points(pts)
    }

    fn selectable(&self, vsp: VSPoint, skip: usize, count: &mut usize) -> Option<RcRDevice> {
        for d in &self.set {
            if d.0.borrow_mut().interactable.contains_vsp(vsp) {
                if *count == skip {
                    // skipped just enough
                    return Some(d.clone());
                } else {
                    *count += 1;
                }
            }
        }
        None
    }

    fn intersect(&self, vsb: &VSBox) -> Box<[RcRDevice]> {
        let ret = self
            .set
            .iter()
            .filter_map(|d| {
                if d.0.borrow_mut().interactable.intersects_vsb(vsb) {
                    Some(d.clone())
                } else {
                    None
                }
            })
            .collect();
        ret
    }

    fn contained(&self, vsb: &VSBox) -> Box<[RcRDevice]> {
        let ret = self
            .set
            .iter()
            .filter_map(|d| {
                if d.0.borrow_mut().interactable.contained_by(vsb) {
                    Some(d.clone())
                } else {
                    None
                }
            })
            .collect();
        ret
    }

    fn place(&mut self, atom: RcRDevice) {
        self.insert(atom);
    }

    fn delete(&mut self, atom: &RcRDevice) {
        self.delete_item(atom)
    }
}

impl Drawable for Devices {
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

impl Devices {
    pub fn occupies_ssp(&self, ssp: SSPoint) -> bool {
        for d in self.set.iter() {
            if d.0.borrow().interactable.contains_ssp(ssp) {
                return true;
            }
        }
        false
    }
    /// returns the first Device after skip which intersects with curpos_ssp in a BaseElement, if any.
    /// count is updated to track the number of elements skipped over
    pub fn selectable(
        &mut self,
        curpos_vsp: VSPoint,
        skip: usize,
        count: &mut usize,
    ) -> Option<RcRDevice> {
        for d in &self.set {
            if d.0.borrow_mut().interactable.contains_vsp(curpos_vsp) {
                if *count == skip {
                    // skipped just enough
                    return Some(d.clone());
                } else {
                    *count += 1;
                }
            }
        }
        None
    }
    /// returns the bounding box of all devices
    pub fn bounding_box(&self) -> VSBox {
        let pts = self.set.iter().flat_map(|d| {
            [
                d.0.borrow().interactable.bounds.min,
                d.0.borrow().interactable.bounds.max,
            ]
            .into_iter()
        });
        VSBox::from_points(pts)
    }
    /// process dc operating point simulation results - draws the voltage of connected nets near the connected port
    pub fn op(&mut self, pkvecvaluesall: &paprika::PkVecvaluesall) {
        for d in &self.set {
            d.0.borrow_mut().op(pkvecvaluesall);
        }
    }
    /// inserts device d into self.
    pub fn insert(&mut self, d: RcRDevice) {
        if !self.set.contains(&d) {
            let ord = match d.0.borrow().class() {
                DeviceClass::Pm(_) => self.manager.pm.incr(),
                DeviceClass::Nm(_) => self.manager.nm.incr(),
                DeviceClass::Gnd(_) => self.manager.gnd.incr(),
                DeviceClass::R(_) => self.manager.r.incr(),
                DeviceClass::L(_) => self.manager.l.incr(),
                DeviceClass::C(_) => self.manager.c.incr(),
                DeviceClass::V(_) => self.manager.v.incr(),
                DeviceClass::I(_) => self.manager.i.incr(),
                DeviceClass::D(_) => self.manager.d.incr(),
            };
            d.0.borrow_mut().set_wm(ord);
            self.set.insert(d);
        }
    }
    /// return vector of RcRDevice which intersects vsb
    pub fn intersects_vsb(&self, vsb: &VSBox) -> Vec<RcRDevice> {
        let ret: Vec<_> = self
            .set
            .iter()
            .filter_map(|d| {
                if d.0.borrow_mut().interactable.intersects_vsb(vsb) {
                    Some(d.clone())
                } else {
                    None
                }
            })
            .collect();
        ret
    }
    /// return vector of RcRDevice which is contained by vsb
    pub fn contained_by(&self, vsb: &VSBox) -> Vec<RcRDevice> {
        let ret: Vec<_> = self
            .set
            .iter()
            .filter_map(|d| {
                if d.0.borrow_mut().interactable.contained_by(vsb) {
                    Some(d.clone())
                } else {
                    None
                }
            })
            .collect();
        ret
    }
    /// create a new pmos with unique ID
    pub fn new_pmos(&mut self) -> RcRDevice {
        let d = RcRDevice::new_with_ord_class(0, DeviceClass::Pm(pmos::M::default()));
        d.0.borrow_mut()
            .transform(transforms::sst_to_vvt(transforms::SST_YMIR));
        d
    }
    /// create a new nmos with unique ID
    pub fn new_nmos(&mut self) -> RcRDevice {
        RcRDevice::new_with_ord_class(0, DeviceClass::Nm(nmos::M::default()))
    }
    /// create a new resistor with unique ID
    pub fn new_res(&mut self) -> RcRDevice {
        RcRDevice::new_with_ord_class(0, DeviceClass::R(R::default()))
    }
    /// create a new inductor with unique ID
    pub fn new_ind(&mut self) -> RcRDevice {
        RcRDevice::new_with_ord_class(0, DeviceClass::L(L::default()))
    }
    /// create a new capacitor with unique ID
    pub fn new_cap(&mut self) -> RcRDevice {
        RcRDevice::new_with_ord_class(0, DeviceClass::C(C::default()))
    }
    /// create a new gnd with unique ID
    pub fn new_gnd(&mut self) -> RcRDevice {
        RcRDevice::new_with_ord_class(0, DeviceClass::Gnd(Gnd::default()))
    }
    /// create a new voltage source with unique ID
    pub fn new_vs(&mut self) -> RcRDevice {
        RcRDevice::new_with_ord_class(0, DeviceClass::V(V::default()))
    }
    /// create a new current source with unique ID
    pub fn new_is(&mut self) -> RcRDevice {
        RcRDevice::new_with_ord_class(0, DeviceClass::I(I::default()))
    }
    /// create a new diode with unique ID
    pub fn new_diode(&mut self) -> RcRDevice {
        RcRDevice::new_with_ord_class(0, DeviceClass::D(D::default()))
    }
    /// returns a vector of SSPoints of all coordinates occupied by all ports of all devices. A coordinate is returned once for each port on that coordinate
    pub fn ports_ssp(&self) -> Box<[SSPoint]> {
        self.set
            .iter()
            .flat_map(|d| d.0.borrow().ports_ssp())
            .collect()
    }
    pub fn any_port_occupy_ssp(&self, ssp: SSPoint) -> bool {
        for d in &self.set {
            if d.0.borrow().ports_occupy_ssp(ssp) {
                return true;
            }
        }
        false
    }
    pub fn delete_item(&mut self, d: &RcRDevice) {
        self.set.remove(d);
    }
    pub fn get_set(&self) -> &HashSet<RcRDevice> {
        &self.set
    }
}

impl Drawable for RcRDevice {
    fn draw_persistent(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        self.0.borrow().draw_persistent(vct, vcscale, frame);
    }

    fn draw_selected(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        self.0.borrow().draw_selected(vct, vcscale, frame);
    }

    fn draw_preview(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        self.0.borrow().draw_preview(vct, vcscale, frame);
    }
}
