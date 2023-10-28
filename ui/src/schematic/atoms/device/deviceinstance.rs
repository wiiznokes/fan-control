//! device instance. Every instance of a device in the schematic is a distinct device instance.

use std::hash::Hasher;

use super::devicetype::DeviceClass;

use iced::{
    widget::canvas::{Frame, Text},
    Color,
};

use crate::{
    schematic::{
        interactable::{Interactable, Interactive},
        layers::Nets,
    },
    transforms::{
        sst_to_vvt, vvt_to_sst, Point, SSPoint, SSTransform, VCTransform, VSPoint, VVTransform,
    },
    Drawable,
};
use std::hash::Hash;

/// device identifier
#[derive(Debug, Clone)]
pub struct Identifier {
    /// prefix which determines device type in NgSpice - a few characters at most
    id_prefix: &'static str,
    /// watermark to efficiently generate unique identifiers
    wm: usize,
    /// if some, is set by the user - must use this as is for id - if multiple instances have same, both should be highlighted
    custom: Option<String>,
}
/*
id collision check:
    create hashset, for every identifier insert. if duplicate, save in second hashset
    every key in second hashset has duplicates
    iterate through devices and highlight every device with id which matches a key in second hashset

immutable identifier:
    abuse rwlock? references take read lock
    if mutation is desired, must acquire write lock - e.g. no read locks.
 */
impl Identifier {
    /// returns a string denoting the device which starts a device line in netlist. E.g. V1, R0
    pub fn ng_id(&self) -> String {
        let mut ret = String::new();
        ret.push_str(self.id_prefix);
        if let Some(s) = &self.custom {
            ret.push_str(s);
        } else {
            ret.push_str(&format!("{}", self.wm));
        }
        ret
    }
    /// creates a new identifier with a prefix and watermark
    pub fn new_with_prefix_ord(id_prefix: &'static str, wm: usize) -> Self {
        Identifier {
            id_prefix,
            wm,
            custom: None,
        }
    }
}
impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.ng_id().eq(&other.ng_id())
    }
}
impl Hash for Identifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ng_id().hash(state);
    }
}

/// A device - e.g. a resistor, bjt, voltage source, ground
#[derive(Debug, Clone)]
pub struct Device {
    /// id which uniquely identifies the device in netlist
    id: Identifier,
    /// device interactable
    pub interactable: Interactable,
    /// device transform - determines the posisiton and orientation of the device in schematic space
    transform: SSTransform,
    /// the class of the device - is the device a resistor, ground, voltage source... ?
    class: DeviceClass,

    /// vector of the connected net names in order of device ports
    connected_nets: Vec<String>,
    /// vector of the connect net voltages in order of device ports
    op: Vec<num::Complex<f32>>,
}
impl Device {
    /// sets the device identifier watermark
    pub fn set_wm(&mut self, wm: usize) {
        self.id.wm = wm;
    }
    /// returns a reference to the device class
    pub fn class(&self) -> &DeviceClass {
        &self.class
    }
    /// returns a mut reference to the device class
    pub fn class_mut(&mut self) -> &mut DeviceClass {
        &mut self.class
    }
    /// creates a new device with watermark and class
    pub fn new_with_ord_class(wm: usize, class: DeviceClass) -> Self {
        Device {
            id: Identifier::new_with_prefix_ord(class.id_prefix(), wm),
            interactable: Interactable::new(class.graphics().bounds().cast().cast_unit()),
            transform: SSTransform::identity(),
            class,
            connected_nets: vec![],
            op: vec![],
        }
    }
    /// returns the schematic coordiantes of the devices ports in order
    pub fn ports_ssp(&self) -> Vec<SSPoint> {
        self.class
            .graphics()
            .ports()
            .iter()
            .map(|p| self.transform.transform_point(p.offset))
            .collect()
    }
    /// returns true if any port occupies ssp
    pub fn ports_occupy_ssp(&self, ssp: SSPoint) -> bool {
        for p in self.class.graphics().ports() {
            if self.transform.transform_point(p.offset) == ssp {
                return true;
            }
        }
        false
    }
    /// returns the composite of the device's transform and the given vct
    fn compose_transform(&self, vct: VCTransform) -> VCTransform {
        sst_to_vvt(self.transform).then(&vct)
    }
    /// sets the position of the device
    pub fn set_position(&mut self, ssp: SSPoint) {
        self.transform.m31 = ssp.x;
        self.transform.m32 = ssp.y;
        self.interactable.bounds = self
            .transform
            .outer_transformed_box(self.class.graphics().bounds())
            .cast()
            .cast_unit();
    }
    /// returns the device's spice netlist line
    pub fn spice_line(&mut self, nets: &Nets) -> String {
        self.connected_nets.clear();
        let mut sline = self.id.ng_id();
        sline.push(' ');
        for p in self.class.graphics().ports() {
            let pt = self.transform.transform_point(p.offset);
            let net_name;
            if let Some(nn) = nets.net_name_at(pt) {
                net_name = nn;
            } else {
                panic!("no net found at port location");
            }
            sline.push_str(&net_name);
            sline.push(' ');
            self.connected_nets.push(net_name);
        }
        sline.push_str(&self.class.param_summary());
        sline.push('\n');
        sline
    }
    /// fill in the operating point for the device
    pub fn op(&mut self, pkvecvaluesall: &paprika::PkVecvaluesall) {
        self.op.clear();
        for n in &self.connected_nets {
            for v in &pkvecvaluesall.vecsa {
                if &v.name == n {
                    let pkval = (*v.as_ref()).clone();
                    self.op.push(pkval.into());
                    break;
                }
            }
        }
    }
}

impl Drawable for Device {
    fn draw_persistent(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        let vct_c = self.compose_transform(vct);
        self.class.graphics().draw_persistent(vct_c, vcscale, frame);

        let a = Text {
            content: self.id.ng_id(),
            position: Point::from(vct_c.transform_point(VSPoint::new(1.0, 1.0))).into(),
            color: Color::from_rgba(1.0, 0.5, 1.0, 1.0),
            size: vcscale,
            ..Default::default()
        };
        frame.fill_text(a);

        let b = Text {
            content: self.class.param_summary(),
            position: Point::from(vct_c.transform_point(VSPoint::new(1.0, 0.0))).into(),
            color: Color::from_rgba(0.5, 1.0, 1.0, 1.0),
            size: vcscale,
            ..Default::default()
        };
        frame.fill_text(b);

        let ports = self.class.graphics().ports();
        for (i, v) in self.op.iter().enumerate() {
            let b = Text {
                content: if v.im != 0.0 {
                    format! {"{:+.2e}", v}
                } else {
                    format! {"{:+.2e}", v.re}
                },
                position: Point::from(vct_c.transform_point(ports[i].offset.cast().cast_unit()))
                    .into(),
                color: Color::from_rgba(1.0, 1.0, 1.0, 1.0),
                size: vcscale,
                ..Default::default()
            };
            frame.fill_text(b);
        }
    }
    fn draw_selected(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        let vct_c = self.compose_transform(vct);
        self.class.graphics().draw_selected(vct_c, vcscale, frame);
    }
    fn draw_preview(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        let vct_c = self.compose_transform(vct);
        self.class.graphics().draw_preview(vct_c, vcscale, frame);
    }
}

impl Interactive for Device {
    fn transform(&mut self, vvt: VVTransform) {
        self.transform = self.transform.then(&vvt_to_sst(vvt));
        self.interactable.bounds = self
            .transform
            .outer_transformed_box(self.class.graphics().bounds())
            .cast()
            .cast_unit();
    }
}
