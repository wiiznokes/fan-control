//! device strokes in device designer
//!

use std::{cell::RefCell, hash::Hasher, rc::Rc};

use by_address::ByAddress;
use iced::{
    widget::canvas::{path::Builder, stroke, Frame, LineCap, LineDash, Stroke},
    Color,
};

use crate::{
    schematic::interactable::{Interactable, Interactive},
    transforms::{Point, SSBox, SSPoint, VCTransform},
};
use crate::{transforms::VSPoint, Drawable};

use super::SchematicAtom;

/// width of the stroke
const STROKE_WIDTH: f32 = 0.1;

/// newtype wrapper for `Rc<RefCell<Bounds>>`
#[derive(Debug, Clone)]
pub struct RcRBounds(pub Rc<RefCell<Bounds>>);

impl RcRBounds {
    pub fn new(b: Bounds) -> Self {
        Self(Rc::new(RefCell::new(b)))
    }
}

impl PartialEq for RcRBounds {
    fn eq(&self, other: &Self) -> bool {
        ByAddress(self.0.clone()) == ByAddress(other.0.clone())
    }
}
impl Eq for RcRBounds {}
impl std::hash::Hash for RcRBounds {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ByAddress(self.0.clone()).hash(state);
    }
}

impl Drawable for RcRBounds {
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
impl SchematicAtom for RcRBounds {
    fn contains_vsp(&self, vsp: VSPoint) -> bool {
        self.0.borrow().interactable.contains_vsp(vsp)
    }
    fn bounding_box(&self) -> crate::transforms::VSBox {
        self.0.borrow().interactable.bounds
    }
}

#[derive(Debug, Clone)]
pub struct Bounds {
    ssb: SSBox,
    pub interactable: Interactable,
}

impl Bounds {
    pub fn new(ssb: SSBox) -> Self {
        Bounds {
            ssb,
            interactable: Interactable::new(ssb.cast().cast_unit()),
        }
    }
    pub fn pts(&self) -> (SSPoint, SSPoint) {
        (self.ssb.min, self.ssb.max)
    }
}

impl Drawable for Bounds {
    fn draw_persistent(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        let stroke = Stroke {
            width: (STROKE_WIDTH * vcscale).max(STROKE_WIDTH * 2.0),
            style: stroke::Style::Solid(Color::from_rgba(0.8, 0.8, 0.8, 0.2)),
            line_cap: LineCap::Square,
            ..Stroke::default()
        };
        let mut path_builder = Builder::new();
        let cbox = vct.outer_transformed_box(&self.ssb.cast().cast_unit());
        let csize = cbox.max - cbox.min;
        let iced_size = iced::Size::from([csize.x, csize.y]);
        path_builder.rectangle(Point::from(cbox.min).into(), iced_size);
        frame.stroke(&path_builder.build(), stroke);
    }
    fn draw_selected(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        let stroke = Stroke {
            width: (STROKE_WIDTH * vcscale).max(STROKE_WIDTH * 2.0) / 2.0,
            style: stroke::Style::Solid(Color::from_rgba(1.0, 1.0, 1.0, 0.8)),
            line_cap: LineCap::Round,
            ..Stroke::default()
        };
        let mut path_builder = Builder::new();
        let cbox = vct.outer_transformed_box(&self.ssb.cast().cast_unit());
        let csize = cbox.max - cbox.min;
        let iced_size = iced::Size::from([csize.x, csize.y]);
        path_builder.rectangle(Point::from(cbox.min).into(), iced_size);
        frame.stroke(&path_builder.build(), stroke);
    }
    fn draw_preview(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        let stroke = Stroke {
            width: (STROKE_WIDTH * vcscale).max(STROKE_WIDTH * 2.0) / 2.0,
            style: stroke::Style::Solid(Color::from_rgba(1.0, 1.0, 1.0, 0.5)),
            line_cap: LineCap::Butt,
            line_dash: LineDash {
                segments: &[3. * (STROKE_WIDTH * vcscale).max(STROKE_WIDTH * 2.0)],
                offset: 0,
            },
            ..Stroke::default()
        };
        let mut path_builder = Builder::new();
        let cbox = vct.outer_transformed_box(&self.ssb.cast().cast_unit());
        let csize = cbox.max - cbox.min;
        let iced_size = iced::Size::from([csize.x, csize.y]);
        path_builder.rectangle(Point::from(cbox.min).into(), iced_size);
        frame.stroke(&path_builder.build(), stroke);
    }
}

impl Interactive for Bounds {
    fn transform(&mut self, vvt: crate::transforms::VVTransform) {
        self.ssb = vvt
            .outer_transformed_box(&self.ssb.cast().cast_unit())
            .round()
            .cast()
            .cast_unit();
        self.interactable = Interactable {
            bounds: self.ssb.cast().cast_unit(),
        }
    }
}
