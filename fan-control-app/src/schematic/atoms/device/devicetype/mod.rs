//! device type. Resistors are a distinct type from capacitors, etc.

use crate::schematic::atoms::Port;
use crate::transforms::{Point, SSBox, VCTransform, VSPoint};
use crate::{schematic::atoms::CirArc, Drawable};
use iced::{
    widget::canvas::{path::Builder, stroke, Frame, LineCap, LineDash, Stroke},
    Color, Size,
};

pub mod gnd;
pub mod i;
pub mod v;

pub mod c;
pub mod l;
pub mod r;

pub mod d;
pub mod nmos;
// pub mod npn;
pub mod pmos;
// pub mod pnp;

const STROKE_WIDTH: f32 = 1.0;

/// graphical representation for devices
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Graphics {
    /// line is traced from point to point for each inner vector.
    pts: Vec<Vec<VSPoint>>,
    /// arbitrary number of circles (center, radius) to be drawn
    cirarcs: Vec<CirArc>,
    /// arbitrary number of device ports
    ports: Vec<Port>,
    /// device bounds
    bounds: SSBox,
}
impl Graphics {
    pub fn bounds(&self) -> &SSBox {
        &self.bounds
    }
    pub fn ports(&self) -> &[Port] {
        &self.ports
    }
    pub fn stroke_bounds(&self, vct_composite: VCTransform, frame: &mut Frame, stroke: Stroke) {
        let mut path_builder = Builder::new();
        let vsb = self.bounds.cast().cast_unit();
        let csb = vct_composite.outer_transformed_box(&vsb);
        let size = Size::new(csb.width(), csb.height());
        path_builder.rectangle(Point::from(csb.min).into(), size);
        frame.stroke(&path_builder.build(), stroke);
    }
    pub fn stroke_symbol(
        &self,
        vct_composite: VCTransform,
        vcscale: f32,
        frame: &mut Frame,
        stroke: Stroke,
    ) {
        // let mut path_builder = Builder::new();
        for v1 in &self.pts {
            // there's a bug where dashed stroke can draw a solid line across a move
            // path_builder.move_to(Point::from(vct_composite.transform_point(v1[0])).into());
            let mut path_builder = Builder::new();
            for v0 in v1 {
                path_builder.line_to(Point::from(vct_composite.transform_point(*v0)).into());
            }
            frame.stroke(&path_builder.build(), stroke.clone());
        }
        let mut path_builder = Builder::new();
        for ca in &self.cirarcs {
            ca.build_path(vct_composite, vcscale, &mut path_builder);
        }
        frame.stroke(&path_builder.build(), stroke.clone());
    }
}
impl Drawable for Graphics {
    fn draw_persistent(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        let stroke = Stroke {
            width: STROKE_WIDTH,
            style: stroke::Style::Solid(Color::from_rgb(0.0, 0.8, 0.0)),
            line_cap: LineCap::Square,
            ..Stroke::default()
        };
        self.stroke_symbol(vct, vcscale, frame, stroke.clone());
        for p in &self.ports {
            p.draw_persistent(vct, vcscale, frame)
        }
    }
    fn draw_selected(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        let stroke = Stroke {
            width: STROKE_WIDTH,
            style: stroke::Style::Solid(Color::from_rgb(1.0, 0.8, 0.0)),
            line_cap: LineCap::Round,
            ..Stroke::default()
        };
        self.stroke_bounds(vct, frame, stroke.clone());
        self.stroke_symbol(vct, vcscale, frame, stroke.clone());
        for p in &self.ports {
            p.draw_selected(vct, vcscale, frame)
        }
    }
    fn draw_preview(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        let stroke = Stroke {
            width: STROKE_WIDTH,
            style: stroke::Style::Solid(Color::from_rgb(1.0, 1.0, 0.5)),
            line_cap: LineCap::Butt,
            line_dash: LineDash {
                segments: &[4.0],
                offset: 0,
            },
            ..Stroke::default()
        };
        self.stroke_bounds(vct, frame, stroke.clone());
        self.stroke_symbol(vct, vcscale, frame, stroke.clone());
        for p in &self.ports {
            p.draw_preview(vct, vcscale, frame)
        }
    }
}

pub trait DeviceType {
    fn default_graphics() -> Graphics;
}

/// DeviceClass enumerates the various classes of devices. E.g. ground, resistor, voltage source... etc
#[derive(Debug, Clone)]
pub enum DeviceClass {
    Pm(pmos::M),
    Nm(nmos::M),
    Gnd(gnd::Gnd),
    R(r::R),
    L(l::L),
    C(c::C),
    V(v::V),
    I(i::I),
    D(d::D),
}
impl DeviceClass {
    /// sets the raw parameter of the device
    pub fn set_raw_param(&mut self, new: String) {
        match self {
            DeviceClass::Pm(x) => match &mut x.params {
                pmos::Param::Raw(y) => y.set(new),
            },
            DeviceClass::Nm(x) => match &mut x.params {
                nmos::Param::Raw(y) => y.set(new),
            },
            DeviceClass::R(x) => match &mut x.params {
                r::Param::Raw(y) => y.set(new),
            },
            DeviceClass::L(x) => match &mut x.params {
                l::Param::Raw(y) => y.set(new),
            },
            DeviceClass::C(x) => match &mut x.params {
                c::ParamC::Raw(y) => y.set(new),
            },
            DeviceClass::Gnd(_) => {}
            DeviceClass::V(x) => match &mut x.params {
                v::Param::Raw(y) => y.set(new),
            },
            DeviceClass::I(x) => match &mut x.params {
                i::Param::Raw(y) => y.set(new),
            },
            DeviceClass::D(x) => match &mut x.params {
                d::Param::Raw(y) => y.set(new),
            },
        }
    }
    /// returns a reference to the device graphics
    pub fn graphics(&self) -> &'static Graphics {
        match self {
            DeviceClass::Pm(x) => x.graphics,
            DeviceClass::Nm(x) => x.graphics,
            DeviceClass::Gnd(x) => x.graphics,
            DeviceClass::R(x) => x.graphics,
            DeviceClass::L(x) => x.graphics,
            DeviceClass::C(x) => x.graphics,
            DeviceClass::V(x) => x.graphics,
            DeviceClass::I(x) => x.graphics,
            DeviceClass::D(x) => x.graphics,
        }
    }
    /// returns a summary of the device parameter for display on canvas
    pub fn param_summary(&self) -> String {
        match self {
            DeviceClass::Pm(x) => x.params.summary(),
            DeviceClass::Nm(x) => x.params.summary(),
            DeviceClass::Gnd(x) => x.params.summary(),
            DeviceClass::R(x) => x.params.summary(),
            DeviceClass::L(x) => x.params.summary(),
            DeviceClass::C(x) => x.params.summary(),
            DeviceClass::V(x) => x.params.summary(),
            DeviceClass::I(x) => x.params.summary(),
            DeviceClass::D(x) => x.params.summary(),
        }
    }
    /// returns the id prefix of the device class
    pub fn id_prefix(&self) -> &'static str {
        match self {
            DeviceClass::Pm(_) => pmos::ID_PREFIX,
            DeviceClass::Nm(_) => nmos::ID_PREFIX,
            DeviceClass::Gnd(_) => gnd::ID_PREFIX,
            DeviceClass::R(_) => r::ID_PREFIX,
            DeviceClass::L(_) => l::ID_PREFIX,
            DeviceClass::C(_) => c::ID_PREFIX,
            DeviceClass::V(_) => v::ID_PREFIX,
            DeviceClass::I(_) => i::ID_PREFIX,
            DeviceClass::D(_) => d::ID_PREFIX,
        }
    }
}
