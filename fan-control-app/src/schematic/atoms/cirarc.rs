//! device strokes in device designer
//!

use std::{cell::RefCell, hash::Hasher, rc::Rc};

use by_address::ByAddress;
use euclid::approxeq::ApproxEq;
use iced::{
    widget::canvas::{
        path::{Arc, Builder},
        stroke, Frame, LineCap, Stroke,
    },
    Color,
};

use crate::Drawable;
use crate::{
    schematic::interactable::{Interactable, Interactive},
    transforms::{Point, VCTransform, VSBox, VSPoint},
};

use super::SchematicAtom;

/// width of the stroke
const STROKE_WIDTH: f32 = 0.1;

/// newtype wrapper for `Rc<RefCell<Bounds>>`
#[derive(Debug, Clone)]
pub struct RcRCirArc(pub Rc<RefCell<CirArc>>);

impl RcRCirArc {
    pub fn new(b: CirArc) -> Self {
        Self(Rc::new(RefCell::new(b)))
    }
}
impl PartialEq for RcRCirArc {
    fn eq(&self, other: &Self) -> bool {
        ByAddress(self.0.clone()) == ByAddress(other.0.clone())
    }
}
impl Eq for RcRCirArc {}
impl std::hash::Hash for RcRCirArc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ByAddress(self.0.clone()).hash(state);
    }
}

impl Drawable for RcRCirArc {
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

impl SchematicAtom for RcRCirArc {
    fn contains_vsp(&self, vsp: VSPoint) -> bool {
        self.0.borrow().interactable.contains_vsp(vsp)
    }
    fn bounding_box(&self) -> crate::transforms::VSBox {
        self.0.borrow().interactable.bounds
    }
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CirArc {
    vsp0: VSPoint,
    vsp1: VSPoint,
    center: VSPoint,
    radius: f32,
    // start_angle: f32,
    // end_angle: f32,
    pub interactable: Interactable,
}

impl CirArc {
    pub fn from_triplet(vsp_center: VSPoint, vsp0: VSPoint, vsp1: VSPoint) -> Self {
        let radius = (vsp0 - vsp_center).length();
        let p0 = VSPoint::new(vsp_center.x - radius, vsp_center.y - radius);
        let p1 = VSPoint::new(vsp_center.x + radius, vsp_center.y + radius);
        CirArc {
            center: vsp_center,
            vsp0,
            vsp1,
            radius,
            interactable: Interactable::new(VSBox::from_points([p0, p1])),
        }
    }
    pub fn pts(&self) -> (VSPoint, VSPoint, VSPoint) {
        (self.center, self.vsp0, self.vsp1)
    }
    pub fn build_path(&self, vct: VCTransform, vcscale: f32, path_builder: &mut Builder) {
        if self.vsp0.approx_eq(&self.vsp1) {
            // render as circle
            path_builder.circle(
                Point::from(vct.transform_point(self.center)).into(),
                self.radius * vcscale,
            )
        } else {
            let start_angle_raw = vct
                .transform_vector(self.vsp0 - self.center)
                .angle_from_x_axis()
                .radians;
            let end_angle_raw = vct
                .transform_vector(self.vsp1 - self.center)
                .angle_from_x_axis()
                .radians;
            let start_angle = if start_angle_raw.is_finite() {
                start_angle_raw
            } else {
                0.0
            };
            let end_angle = if end_angle_raw.is_finite() {
                end_angle_raw
            } else {
                start_angle
            };
            path_builder.arc(Arc {
                center: Point::from(vct.transform_point(self.center)).into(),
                radius: self.radius * vcscale,
                start_angle,
                end_angle,
            })
        }
    }
}

impl Drawable for CirArc {
    fn draw_persistent(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        let stroke = Stroke {
            width: (STROKE_WIDTH * vcscale).max(STROKE_WIDTH * 2.0),
            style: stroke::Style::Solid(Color::from_rgb(0.0, 0.8, 0.0)),
            line_cap: LineCap::Square,
            ..Stroke::default()
        };
        let mut path_builder = Builder::new();
        self.build_path(vct, vcscale, &mut path_builder);
        frame.stroke(&path_builder.build(), stroke);
    }
    fn draw_selected(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        let stroke = Stroke {
            width: (STROKE_WIDTH * vcscale).max(STROKE_WIDTH * 2.) / 2.0,
            style: stroke::Style::Solid(Color::from_rgb(1.0, 0.8, 0.0)),
            line_cap: LineCap::Round,
            ..Stroke::default()
        };
        let mut path_builder = Builder::new();
        self.build_path(vct, vcscale, &mut path_builder);
        frame.stroke(&path_builder.build(), stroke);
    }
    fn draw_preview(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        let stroke = Stroke {
            width: (STROKE_WIDTH * vcscale).max(STROKE_WIDTH * 1.) / 2.0,
            style: stroke::Style::Solid(Color::from_rgba(1.0, 1.0, 0.5, 0.2)),
            line_cap: LineCap::Butt,
            ..Stroke::default()
        };
        let mut path_builder = Builder::new();
        self.build_path(vct, vcscale, &mut path_builder);
        frame.stroke(&path_builder.build(), stroke);
    }
}

impl Interactive for CirArc {
    fn transform(&mut self, vvt: crate::transforms::VVTransform) {
        self.center = vvt.transform_point(self.center);
        self.vsp0 = vvt.transform_point(self.vsp0);
        self.vsp1 = vvt.transform_point(self.vsp1);
        let p0 = VSPoint::new(self.center.x - self.radius, self.center.y - self.radius);
        let p1 = VSPoint::new(self.center.x + self.radius, self.center.y + self.radius);
        self.interactable = Interactable {
            bounds: VSBox::from_points([p0, p1]),
        }
    }
}
