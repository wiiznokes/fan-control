//! device strokes in device designer
//!

use std::{cell::RefCell, hash::Hasher, rc::Rc};

use by_address::ByAddress;
use iced::{
    widget::canvas::{path::Builder, stroke, Frame, LineCap, LineDash, Stroke},
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

/// newtype wrapper for `Rc<RefCell<Linear>>`
#[derive(Debug, Clone)]
pub struct RcRLineSeg(pub Rc<RefCell<LineSeg>>);

impl RcRLineSeg {
    pub fn new(l: LineSeg) -> Self {
        Self(Rc::new(RefCell::new(l)))
    }
}

impl PartialEq for RcRLineSeg {
    fn eq(&self, other: &Self) -> bool {
        ByAddress(self.0.clone()) == ByAddress(other.0.clone())
    }
}
impl Eq for RcRLineSeg {}
impl std::hash::Hash for RcRLineSeg {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ByAddress(self.0.clone()).hash(state);
    }
}

impl Drawable for RcRLineSeg {
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

impl SchematicAtom for RcRLineSeg {
    fn contains_vsp(&self, vsp: VSPoint) -> bool {
        self.0.borrow().interactable.contains_vsp(vsp)
    }
    fn bounding_box(&self) -> crate::transforms::VSBox {
        self.0.borrow().interactable.bounds
    }
}

#[derive(Debug, Clone)]
pub struct LineSeg {
    pt0: VSPoint,
    pt1: VSPoint,
    pub interactable: Interactable,
}

impl LineSeg {
    pub fn new(vsp0: VSPoint, vsp1: VSPoint) -> Self {
        LineSeg {
            pt0: vsp0,
            pt1: vsp1,
            interactable: Interactable {
                bounds: VSBox::from_points([vsp0, vsp1]),
            },
        }
    }
    pub fn pts(&self) -> (VSPoint, VSPoint) {
        (self.pt0, self.pt1)
    }
}

impl Drawable for LineSeg {
    fn draw_persistent(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        let stroke = Stroke {
            width: (STROKE_WIDTH * vcscale).max(STROKE_WIDTH * 2.0),
            style: stroke::Style::Solid(Color::from_rgb(0.0, 0.8, 0.0)),
            line_cap: LineCap::Square,
            ..Stroke::default()
        };
        let mut path_builder = Builder::new();
        path_builder.line_to(Point::from(vct.transform_point(self.pt0.cast().cast_unit())).into());
        path_builder.line_to(Point::from(vct.transform_point(self.pt1.cast().cast_unit())).into());
        frame.stroke(&path_builder.build(), stroke.clone());
    }
    fn draw_selected(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        let stroke = Stroke {
            width: (STROKE_WIDTH * vcscale).max(STROKE_WIDTH * 2.) / 2.0,
            style: stroke::Style::Solid(Color::from_rgb(1.0, 0.8, 0.0)),
            line_cap: LineCap::Round,
            ..Stroke::default()
        };
        let mut path_builder = Builder::new();
        path_builder.line_to(Point::from(vct.transform_point(self.pt0.cast().cast_unit())).into());
        path_builder.line_to(Point::from(vct.transform_point(self.pt1.cast().cast_unit())).into());
        frame.stroke(&path_builder.build(), stroke.clone());
    }
    fn draw_preview(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        let stroke = Stroke {
            width: (STROKE_WIDTH * vcscale).max(STROKE_WIDTH * 1.) / 2.0,
            style: stroke::Style::Solid(Color::from_rgba(1.0, 1.0, 0.5, 0.2)),
            line_cap: LineCap::Butt,
            ..Stroke::default()
        };
        let mut path_builder = Builder::new();
        path_builder.line_to(Point::from(vct.transform_point(self.pt0)).into());
        path_builder.line_to(Point::from(vct.transform_point(self.pt1)).into());
        let built_path = path_builder.build();
        frame.stroke(&built_path, stroke);

        let stroke = Stroke {
            width: (STROKE_WIDTH * vcscale).max(STROKE_WIDTH * 1.) / 2.0,
            style: stroke::Style::Solid(Color::from_rgb(1.0, 1.0, 0.5)),
            line_cap: LineCap::Butt,
            line_dash: LineDash {
                segments: &[3. * (STROKE_WIDTH * vcscale).max(STROKE_WIDTH * 2.0)],
                offset: 0,
            },
            ..Stroke::default()
        };
        frame.stroke(&built_path, stroke);
    }
}

impl Interactive for LineSeg {
    fn transform(&mut self, vvt: crate::transforms::VVTransform) {
        self.pt0 = vvt.transform_point(self.pt0);
        self.pt1 = vvt.transform_point(self.pt1);
        self.interactable = Interactable {
            bounds: VSBox::from_points([self.pt0, self.pt1]),
        }
    }
}
