//! plot
//! Space in which math values form plots

use crate::analysis::viewport;
use crate::transforms::CSVec;
use crate::transforms::{CSPoint, Point, VCTransform, VSBox, VSPoint};
use crate::Drawable;
use iced::widget::canvas::{stroke, Path};
use iced::{
    mouse,
    widget::canvas::{self, event::Event, path::Builder, Frame, LineCap, Stroke},
    Color, Size,
};
use std::collections::HashSet;
use std::default::Default;
use std::hash::Hash;

pub trait PlotElement: Hash + Eq + Drawable + Clone {
    fn bounding_box(&self) -> VSBox;
}

pub type PlotTrace = Vec<VSPoint>;

/// an enum to unify different types in schematic (nets and devices)
#[derive(Debug, Clone)]
pub enum ChartElement {
    PlotTrace(PlotTrace),
}

impl Default for ChartElement {
    fn default() -> Self {
        ChartElement::PlotTrace(Vec::from([VSPoint::origin(), VSPoint::new(1.0, 1.0)]))
    }
}

impl PartialEq for ChartElement {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::PlotTrace(l0), Self::PlotTrace(r0)) => {
                by_address::ByAddress(l0) == by_address::ByAddress(r0)
            }
        }
    }
}

impl Eq for ChartElement {}

impl std::hash::Hash for ChartElement {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            ChartElement::PlotTrace(d) => by_address::ByAddress(d).hash(state),
        }
    }
}

impl Drawable for ChartElement {
    fn draw_persistent(&self, vct: VCTransform, _vcscale: f32, frame: &mut Frame) {
        match self {
            ChartElement::PlotTrace(trace) => {
                let stroke = Stroke {
                    width: 1.0,
                    style: stroke::Style::Solid(Color::from_rgb(0.8, 0.8, 0.8)),
                    line_cap: LineCap::Square,
                    ..Stroke::default()
                };
                let mut path_builder = Builder::new();
                for vsp in trace {
                    path_builder.line_to(Point::from(vct.transform_point(*vsp)).into());
                }
                frame.stroke(&path_builder.build(), stroke.clone());
            }
        }
    }

    fn draw_selected(&self, vct: VCTransform, _vcscale: f32, frame: &mut Frame) {
        match self {
            ChartElement::PlotTrace(trace) => {
                let stroke = Stroke {
                    width: 1.0,
                    style: stroke::Style::Solid(Color::from_rgb(0.9, 0.9, 0.9)),
                    line_cap: LineCap::Square,
                    ..Stroke::default()
                };
                let mut path_builder = Builder::new();
                for vsp in trace {
                    path_builder.line_to(Point::from(vct.transform_point(*vsp)).into());
                }
                frame.stroke(&path_builder.build(), stroke.clone());
            }
        }
    }

    fn draw_preview(&self, vct: VCTransform, _vcscale: f32, frame: &mut Frame) {
        match self {
            ChartElement::PlotTrace(trace) => {
                let stroke = Stroke {
                    width: 1.0,
                    style: stroke::Style::Solid(Color::from_rgb(1.0, 1.0, 1.0)),
                    line_cap: LineCap::Square,
                    ..Stroke::default()
                };
                let mut path_builder = Builder::new();
                for vsp in trace {
                    path_builder.line_to(Point::from(vct.transform_point(*vsp)).into());
                }
                frame.stroke(&path_builder.build(), stroke.clone());
            }
        }
    }
}

impl PlotElement for ChartElement {
    fn bounding_box(&self) -> VSBox {
        match self {
            ChartElement::PlotTrace(trace) => VSBox::from_points(trace),
        }
    }
}

/// Trait for message type of schematic content
pub trait ContentMsg {
    /// Create message to have schematic content process canvas event
    fn canvas_event_msg(event: Event, curpos_vsp: VSPoint) -> Self;
}

/// Message type which is a composite of canvas Event, SchematicMsg, and ContentMsg
/// This structure allows schematic and its content to process events in parallel
#[derive(Debug, Clone)]
pub enum Msg {
    /// do nothing
    None,
    /// new trace data
    Traces(Vec<Vec<VSPoint>>),
    /// iced canvas event, along with cursor position inside canvas bounds
    Event(Event, VSPoint),
}

/// implements Msg to be ContentMsg of viewport
impl viewport::ContentMsg for Msg {
    // create event to handle iced canvas event
    fn canvas_event_msg(event: Event, curpos_vsp: Option<VSPoint>) -> Self {
        if let Some(vsp) = curpos_vsp {
            Msg::Event(event, vsp)
        } else {
            Msg::None
        }
    }
}

/// Schematic States
#[derive(Debug, Clone, Copy, Default)]
pub enum PlotSt {
    /// idle state
    #[default]
    Idle,
    /// left click-drag area selection
    AreaSelect(VSBox),
}

/// struct holding schematic state (nets, devices, and their locations)
#[derive(Debug, Clone, Default)]
pub struct Plot<E>
where
    E: PlotElement,
{
    /// chart contents
    content: HashSet<E>,
    /// active element
    pub active_element: Option<E>,
    /// schematic state
    state: PlotSt,
    /// single selection cycling watermark
    selskip: usize,
    /// Hashset of selected elements
    selected: HashSet<E>,
    /// Hashset of tentative elements (mouse hovering over, inside area selection)
    tentatives: HashSet<E>,
    /// cursor position in schematic space
    curpos_vsp: VSPoint,
}

/// implement Schematic as viewport content
impl viewport::Content<Msg> for Plot<ChartElement> {
    /// change cursor graphic based on schematic state
    fn mouse_interaction(&self) -> mouse::Interaction {
        match self.state {
            PlotSt::Idle => mouse::Interaction::default(),
            PlotSt::AreaSelect(_) => mouse::Interaction::Crosshair,
        }
    }

    /// draw onto active cache
    fn draw_active(&self, vct: VCTransform, frame: &mut Frame) {
        match &self.state {
            PlotSt::Idle => {}
            PlotSt::AreaSelect(vsb) => {
                // draw the selection area
                let color = if vsb.height() > 0.0 {
                    Color::from_rgba(1., 1., 0., 0.1)
                } else {
                    Color::from_rgba(0., 1., 1., 0.1)
                };
                let f = canvas::Fill {
                    style: canvas::Style::Solid(color),
                    ..canvas::Fill::default()
                };
                let csb = vct.outer_transformed_box(&vsb.cast().cast_unit());
                let size = Size::new(csb.width(), csb.height());
                frame.fill_rectangle(Point::from(csb.min).into(), size, f);

                let mut path_builder = Builder::new();
                path_builder.line_to(Point::from(csb.min).into());
                path_builder.line_to(Point::from(CSPoint::new(csb.min.x, csb.max.y)).into());
                path_builder.line_to(Point::from(csb.max).into());
                path_builder.line_to(Point::from(CSPoint::new(csb.max.x, csb.min.y)).into());
                path_builder.line_to(Point::from(csb.min).into());
                let stroke = Stroke {
                    width: 0.5,
                    style: canvas::stroke::Style::Solid(color),
                    line_cap: LineCap::Square,
                    ..Stroke::default()
                };
                frame.stroke(&path_builder.build(), stroke);
            }
            _ => {}
        }

        // draw preview for tentatives
        let _: Vec<_> = self
            .tentatives
            .iter()
            .map(|e| e.draw_preview(vct, 1.0, frame))
            .collect();

        /// draw the cursor onto canvas
        pub fn draw_cursor(vct: VCTransform, frame: &mut Frame, curpos_vsp: VSPoint) {
            let cursor_stroke = || -> Stroke {
                Stroke {
                    width: 1.0,
                    style: stroke::Style::Solid(Color::from_rgb(1.0, 0.9, 0.0)),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };
            let curdim = 5.0;
            let csp = vct.transform_point(curpos_vsp);
            let csp_topleft = csp - CSVec::from([curdim / 2.; 2]);
            let s = iced::Size::from([curdim, curdim]);
            let c = Path::rectangle(iced::Point::from([csp_topleft.x, csp_topleft.y]), s);
            frame.stroke(&c, cursor_stroke());
        }
        draw_cursor(vct, frame, self.curpos_vsp);
    }
    /// draw onto passive cache
    fn draw_passive(&self, vct: VCTransform, frame: &mut Frame) {
        let _: Vec<_> = self
            .selected
            .iter()
            .map(|e| e.draw_selected(vct, 1.0, frame))
            .collect();
        let _: Vec<_> = self
            .content
            .iter()
            .map(|e| e.draw_selected(vct, 1.0, frame))
            .collect();
    }

    /// returns the bouding box of schematic content
    fn bounds(&self) -> VSBox {
        if !self.content.is_empty() {
            let vec_vsp: Vec<_> = self
                .content
                .iter()
                .flat_map(|f| [f.bounding_box().min, f.bounding_box().max])
                .collect();
            VSBox::from_points(vec_vsp)
        } else {
            VSBox::default()
        }
    }
    /// mutate state based on message and cursor position
    fn update(&mut self, msg: Msg) -> bool {
        let mut clear_passive = false;

        match msg {
            Msg::Event(event, curpos_vsp) => {
                if let Event::Mouse(iced::mouse::Event::CursorMoved { .. }) = event {
                    self.update_cursor_vsp(curpos_vsp);
                }
            }
            Msg::None => {}
            Msg::Traces(traces) => {
                self.selected.clear();
                self.tentatives.clear();
                self.content.clear();

                for trace in traces {
                    self.content.insert(ChartElement::PlotTrace(trace));
                }

                clear_passive = true;
            }
        }
        clear_passive
    }
}

impl<E> Plot<E>
where
    E: PlotElement,
{
    fn update_cursor_vsp(&mut self, curpos_vsp: VSPoint) {
        self.curpos_vsp = curpos_vsp;
        self.tentative_by_vspoint(curpos_vsp, &mut self.selskip.clone());

        let mut stcp = self.state;
        match &mut stcp {
            PlotSt::AreaSelect(vsb) => {
                vsb.max = curpos_vsp;
                self.tentatives_by_vsbox(vsb);
            }
            _ => {}
        }
        self.state = stcp;
    }
    /// set tentative flags by intersection with ssb
    pub fn tentatives_by_vsbox(&mut self, vsb: &VSBox) {
        let _ssb_p = VSBox::from_points([vsb.min, vsb.max]);
        // self.tentatives = self.content.intersects_vsb(ssb_p)
    }
    /// set 1 tentative flag by ssp, skipping skip elements which contains ssp. Returns netname if tentative is a net segment
    pub fn tentative_by_vspoint(&mut self, vsp: VSPoint, skip: &mut usize) {
        self.tentatives.clear();
        if let Some(e) = self.selectable(vsp, skip) {
            self.tentatives.insert(e);
        }
    }
    /// set 1 tentative flag by ssp, sets flag on next qualifying element. Returns netname i tentative is a net segment
    pub fn tentative_next_by_vsp(&mut self, vsp: VSPoint) {
        let mut skip = self.selskip.wrapping_add(1);
        self.tentative_by_vspoint(vsp, &mut skip);
        self.selskip = skip;
    }
    /// put every element with tentative flag set into selected vector
    fn tentatives_to_selected(&mut self) {
        self.selected = self.tentatives.clone();
        if self.tentatives.len() == 1 {
            let mut v: Vec<_> = self.tentatives.iter().collect();
            self.active_element = v.pop().cloned();
        }
        self.tentatives.clear();
    }
    /// set 1 tentative flag based on ssp and skip number. Returns the flagged element, if any.
    fn selectable(&mut self, _vsp: VSPoint, skip: &mut usize) -> Option<E> {
        loop {
            let count = 0; // tracks the number of skipped elements
                           // if let Some(e) = self.content.selectable(vsp, *skip, &mut count) {
                           //     return Some(e);
                           // }
            if count == 0 {
                *skip = 0;
                return None;
            }
            *skip -= count;
        }
    }
}
