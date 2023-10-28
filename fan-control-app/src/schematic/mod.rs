//! Schematic
//! Space in which devices and nets live in

mod atoms;
pub mod circuit;
mod interactable;
mod layers;
mod models;
pub mod symbols;
mod viewport;

use crate::transforms::{self, CSPoint, Point, SSTransform, VCTransform, VSBox, VSPoint};
use crate::transforms::{CSVec, VVTransform};
use crate::Drawable;

use iced::keyboard::Modifiers;
use iced::widget::canvas::{stroke, Path};
use iced::{
    mouse,
    widget::canvas::{self, event::Event, path::Builder, Frame, LineCap, Stroke},
    Color, Size,
};
use send_wrapper::SendWrapper;
use std::collections::HashSet;

use atoms::SchematicAtom;

/// Internal Schematic Message
#[derive(Debug, Clone)]
pub enum SchematicMsg<E>
where
    E: SchematicAtom,
{
    /// do nothing
    None,
    /// clear passive cache
    ClearPassive,
    /// place new element E
    NewElement(SendWrapper<E>),
}

/// Trait for message type of schematic content
pub trait ContentMsg {
    /// Create message to have schematic content process canvas event
    fn canvas_event_msg(event: Event) -> Self;
}

/// Message type which is a composite of canvas Event, SchematicMsg, and ContentMsg
/// This structure allows schematic and its content to process events in parallel
#[derive(Debug, Clone)]
pub enum Msg<M, E>
where
    M: ContentMsg,
    E: SchematicAtom,
{
    /// iced canvas event, along with cursor position inside canvas bounds
    Event(Event, Option<VSPoint>),
    /// Schematic Message
    SchematicMsg(SchematicMsg<E>),
    /// Content Message
    ContentMsg(M),
}

/// implements Msg to be ContentMsg of viewport
impl<M, E> viewport::ContentMsg for Msg<M, E>
where
    M: ContentMsg,
    E: SchematicAtom,
{
    // create event to handle iced canvas event
    fn canvas_event_msg(event: Event, curpos_vsp: Option<VSPoint>) -> Self {
        Msg::Event(event, curpos_vsp)
    }
}

/// Schematic States
#[derive(Debug, Clone, Copy, Default)]
pub enum SchematicSt {
    /// idle state
    #[default]
    Idle,
    /// left click-drag area selection
    AreaSelect(VSBox),
    /// moving, but keep wiring connections
    Grabbing(Option<(VSPoint, VSPoint, SSTransform)>),
    /// selected elements preview follow mouse cursor - move, new device,
    Moving(Option<(VSPoint, VSPoint, SSTransform)>),
    /// identical to `Moving` state but signals content to make copy of elements instead of move
    Copying(Option<(VSPoint, VSPoint, SSTransform)>),
}

impl SchematicSt {
    /// this function returns a transform which applies sst about ssp0 and then translates to ssp1
    fn move_transform(vsp0: VSPoint, vsp1: VSPoint, sst: SSTransform) -> VVTransform {
        let vvt = transforms::sst_to_vvt(sst);
        vvt.pre_translate(VSPoint::origin() - vsp0)
            .then_translate(vsp0 - VSPoint::origin())
            .then_translate(vsp1 - vsp0)
    }
}

pub trait Content<E, M>: Drawable + Default
where
    E: SchematicAtom,
{
    /// return true if content is in its default/idle state
    fn is_idle(&self) -> bool;
    /// apply sst to elements
    fn move_elements(&mut self, elements: &mut HashSet<E>, vvt: &VVTransform);
    /// apply sst to a copy of elements
    fn copy_elements(&mut self, elements: &mut HashSet<E>, vvt: &VVTransform);
    /// delete elements
    fn delete_elements(&mut self, elements: &HashSet<E>);
    /// process message, returns whether or not to clear the passive cache
    fn update(&mut self, msg: M) -> SchematicMsg<E>;
    /// return bounds which enclose all elements
    fn bounds(&self) -> VSBox;
    /// returns a single SchematicElement over which ssp lies. Skips the first skip elements
    fn selectable(&mut self, vsp: VSPoint, skip: usize, count: &mut usize) -> Option<E>;
    /// returns hashset of elements which intersects ssb
    fn intersects_vsb(&mut self, vsb: VSBox) -> HashSet<E>;
    /// returns hashset of elements which is contained by vsb
    fn contained_by(&mut self, vsb: VSBox) -> HashSet<E>;
    /// returns the cursor position as stored by content
    fn curpos_vsp(&self) -> VSPoint;
    /// update cursor position
    fn curpos_update(&mut self, vsp: VSPoint);
}

/// struct holding schematic state (nets, devices, and their locations)
#[derive(Debug, Clone)]
pub struct Schematic<C, E, M>
where
    C: Content<E, M>,
    E: SchematicAtom,
{
    /// schematic state
    state: SchematicSt,
    /// schematic content - circuit or device designer
    pub content: C,
    /// phantom data to mark ContentMsg type
    content_msg: std::marker::PhantomData<M>,
    /// single selection cycling watermark
    selskip: usize,
    /// Hashset of selected elements
    selected: HashSet<E>,
    /// Hashset of tentative elements (mouse hovering over, inside area selection)
    tentatives: HashSet<E>,
    /// cursor position in schematic space
    curpos_vsp: VSPoint,

    /// last single selected element
    pub active_element: Option<E>,
}

impl<C, E, M> Default for Schematic<C, E, M>
where
    C: Content<E, M>,
    E: SchematicAtom,
{
    fn default() -> Self {
        Self {
            state: Default::default(),
            content: Default::default(),
            selskip: Default::default(),
            selected: Default::default(),
            tentatives: Default::default(),
            content_msg: std::marker::PhantomData,
            curpos_vsp: Default::default(),
            active_element: Default::default(),
        }
    }
}

/// implement Schematic as viewport content
impl<C, E, M> viewport::Content<Msg<M, E>> for Schematic<C, E, M>
where
    M: ContentMsg,
    C: Content<E, M>,
    E: SchematicAtom,
{
    /// change cursor graphic based on schematic state
    fn mouse_interaction(&self) -> mouse::Interaction {
        match self.state {
            SchematicSt::Idle => mouse::Interaction::default(),
            SchematicSt::AreaSelect(_) => mouse::Interaction::Crosshair,
            SchematicSt::Moving(_) => mouse::Interaction::Grabbing,
            SchematicSt::Copying(_) => mouse::Interaction::Grabbing,
            SchematicSt::Grabbing(_) => mouse::Interaction::Grabbing,
        }
    }

    /// draw onto active cache
    fn draw_active(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        match &self.state {
            SchematicSt::Idle => {}
            SchematicSt::AreaSelect(vsb) => {
                // draw the selection area
                let color = if vsb.height() > 0.0 {
                    // intended to distinguish between select by contains and select by intersects
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
                    width: (0.1 * vcscale).max(0.1 * 2.0),
                    style: canvas::stroke::Style::Solid(color),
                    line_cap: LineCap::Square,
                    ..Stroke::default()
                };
                frame.stroke(&path_builder.build(), stroke);
            }
            SchematicSt::Moving(Some((vsp0, vsp1, vvt)))
            | SchematicSt::Copying(Some((vsp0, vsp1, vvt))) => {
                // draw selected preview with transform applied
                let vvt = SchematicSt::move_transform(*vsp0, *vsp1, *vvt);

                let vct_c = vvt.then(&vct);
                for be in &self.selected {
                    be.draw_preview(vct_c, vcscale, frame);
                }
            }
            _ => {}
        }

        // draw preview for tentatives
        let _: Vec<_> = self
            .tentatives
            .iter()
            .map(|e| e.draw_preview(vct, vcscale, frame))
            .collect();
        // draw content preview
        self.content.draw_preview(vct, vcscale, frame);

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
        draw_cursor(vct, frame, self.content.curpos_vsp());
    }
    /// draw onto passive cache
    fn draw_passive(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        self.content.draw_persistent(vct, vcscale, frame);
        let _: Vec<_> = self
            .selected
            .iter()
            .map(|e| e.draw_selected(vct, vcscale, frame))
            .collect();
    }

    /// returns the bouding box of schematic content
    fn bounds(&self) -> VSBox {
        self.content.bounds()
    }
    /// mutate state based on message and cursor position
    fn update(&mut self, msg: Msg<M, E>) -> bool {
        let mut clear_passive = false;

        match msg {
            Msg::Event(event, opt_curpos_vsp) => {
                if let Some(vsp) = opt_curpos_vsp {
                    if let Event::Mouse(iced::mouse::Event::CursorMoved { .. }) = event {
                        self.content.curpos_update(vsp);
                        self.update_cursor_vsp(self.content.curpos_vsp());
                    }
                } else {
                    return false;
                }

                if self.content.is_idle() {
                    const NO_MODIFIER: Modifiers = Modifiers::empty();
                    // if content is idle, allow schematic to process event before passing onto content - otherwise pass event to content directly
                    match (&mut self.state, event) {
                        // drag/area select - todo move to viewport - content should allow viewport to discern areaselect or drag
                        (
                            SchematicSt::Idle,
                            Event::Mouse(iced::mouse::Event::ButtonPressed(
                                iced::mouse::Button::Left,
                            )),
                        ) => {
                            let mut click_selected = false;

                            for s in &self.selected {
                                if s.contains_vsp(self.curpos_vsp) {
                                    click_selected = true;
                                    break;
                                }
                            }

                            if click_selected {
                                self.state = SchematicSt::Moving(Some((
                                    self.curpos_vsp,
                                    self.curpos_vsp,
                                    SSTransform::identity(),
                                )));
                            } else {
                                self.state = SchematicSt::AreaSelect(VSBox::new(
                                    self.curpos_vsp,
                                    self.curpos_vsp,
                                ));
                            }
                        }

                        // area select
                        (
                            SchematicSt::AreaSelect(_),
                            Event::Mouse(iced::mouse::Event::ButtonReleased(
                                iced::mouse::Button::Left,
                            )),
                        ) => {
                            self.tentatives_to_selected();
                            self.state = SchematicSt::Idle;
                            clear_passive = true;
                        }
                        // moving
                        (
                            _,
                            Event::Keyboard(iced::keyboard::Event::KeyPressed {
                                key_code: iced::keyboard::KeyCode::M,
                                modifiers: NO_MODIFIER,
                            }),
                        ) => {
                            self.state = SchematicSt::Moving(None);
                        }
                        (
                            SchematicSt::Moving(Some((_ssp0, _ssp1, sst)))
                            | SchematicSt::Copying(Some((_ssp0, _ssp1, sst))),
                            Event::Keyboard(iced::keyboard::Event::KeyPressed {
                                key_code: iced::keyboard::KeyCode::R,
                                modifiers: m,
                            }),
                        ) => {
                            if m.control() {
                                *sst = sst.then(&transforms::SST_CCWR);
                            } else {
                                *sst = sst.then(&transforms::SST_CWR);
                            }
                        }
                        (
                            SchematicSt::Moving(Some((_ssp0, _ssp1, sst)))
                            | SchematicSt::Copying(Some((_ssp0, _ssp1, sst))),
                            Event::Keyboard(iced::keyboard::Event::KeyPressed {
                                key_code: iced::keyboard::KeyCode::X,
                                modifiers: NO_MODIFIER,
                            }),
                        ) => {
                            *sst = sst.then(&transforms::SST_XMIR);
                        }
                        (
                            SchematicSt::Moving(Some((_ssp0, _ssp1, sst)))
                            | SchematicSt::Copying(Some((_ssp0, _ssp1, sst))),
                            Event::Keyboard(iced::keyboard::Event::KeyPressed {
                                key_code: iced::keyboard::KeyCode::Y,
                                modifiers: NO_MODIFIER,
                            }),
                        ) => {
                            *sst = sst.then(&transforms::SST_YMIR);
                        }
                        (
                            SchematicSt::Moving(mut opt_pts),
                            Event::Mouse(iced::mouse::Event::ButtonReleased(
                                iced::mouse::Button::Left,
                            )),
                        ) => {
                            if let Some((vsp0, vsp1, vvt)) = &mut opt_pts {
                                let vvt = SchematicSt::move_transform(*vsp0, *vsp1, *vvt);
                                self.content.move_elements(&mut self.selected, &vvt);
                                clear_passive = true;
                                self.state = SchematicSt::Idle;
                            } else {
                                let sst = SSTransform::identity();
                                self.state = SchematicSt::Moving(Some((
                                    self.curpos_vsp,
                                    self.curpos_vsp,
                                    sst,
                                )));
                            }
                        }
                        // copying
                        (
                            SchematicSt::Idle,
                            Event::Keyboard(iced::keyboard::Event::KeyPressed {
                                key_code: iced::keyboard::KeyCode::C,
                                modifiers: Modifiers::CTRL,
                            }),
                        ) => {
                            self.state = SchematicSt::Copying(None);
                        }
                        (
                            SchematicSt::Copying(opt_pts),
                            Event::Mouse(iced::mouse::Event::ButtonReleased(
                                iced::mouse::Button::Left,
                            )),
                        ) => match opt_pts {
                            Some((vsp0, vsp1, vvt)) => {
                                self.content.copy_elements(
                                    &mut self.selected,
                                    &SchematicSt::move_transform(*vsp0, *vsp1, *vvt),
                                );
                                clear_passive = true;
                                self.state = SchematicSt::Idle;
                            }
                            None => {
                                self.state = SchematicSt::Copying(Some((
                                    self.curpos_vsp,
                                    self.curpos_vsp,
                                    SSTransform::identity(),
                                )));
                            }
                        },
                        // delete
                        (
                            SchematicSt::Idle,
                            Event::Keyboard(iced::keyboard::Event::KeyPressed {
                                key_code: iced::keyboard::KeyCode::Delete,
                                modifiers: NO_MODIFIER,
                            }),
                        ) => {
                            self.content.delete_elements(&self.selected);
                            self.active_element = None;
                            self.selected.clear();
                            clear_passive = true;
                        }
                        // tentative selection cycle
                        (
                            SchematicSt::Idle,
                            Event::Keyboard(iced::keyboard::Event::KeyPressed {
                                key_code: iced::keyboard::KeyCode::C,
                                modifiers: Modifiers::SHIFT,
                            }),
                        ) => {
                            self.tentative_next_by_vsp(self.curpos_vsp);
                            clear_passive = true;
                        }

                        // rst
                        (
                            st,
                            Event::Keyboard(iced::keyboard::Event::KeyPressed {
                                key_code: iced::keyboard::KeyCode::Escape,
                                modifiers: NO_MODIFIER,
                            }),
                        ) => match st {
                            SchematicSt::Idle => {
                                self.active_element = None;
                                self.selected.clear();
                                clear_passive = true;
                            }
                            _ => {
                                self.state = SchematicSt::Idle;
                            }
                        },
                        // something else - pass to content
                        _ => {
                            let m = self.content.update(M::canvas_event_msg(event));
                            clear_passive = self.update(Msg::SchematicMsg(m));
                        }
                    }
                } else {
                    // if content is not idling, pass event directly to content
                    let m = self.content.update(M::canvas_event_msg(event));
                    clear_passive = self.update(Msg::SchematicMsg(m));
                }
            }
            Msg::ContentMsg(content_msg) => {
                let m = self.content.update(content_msg);
                clear_passive = self.update(Msg::SchematicMsg(m));
            }
            Msg::SchematicMsg(schematic_msg) => {
                match schematic_msg {
                    SchematicMsg::None => {}
                    SchematicMsg::ClearPassive => {
                        clear_passive = true;
                    }
                    SchematicMsg::NewElement(e) => {
                        // place into selected
                        self.active_element = None;
                        self.selected.clear();
                        self.selected.insert(e.take());
                        self.state = SchematicSt::Moving(Some((
                            VSPoint::origin(),
                            self.curpos_vsp,
                            SSTransform::identity(),
                        )));
                    }
                }
            }
        }
        clear_passive
    }
}

impl<C, E, M> Schematic<C, E, M>
where
    C: Content<E, M>,
    E: SchematicAtom,
{
    /// update schematic cursor position
    fn update_cursor_vsp(&mut self, curpos_vsp: VSPoint) {
        self.curpos_vsp = curpos_vsp;
        self.tentative_by_vspoint(curpos_vsp, &mut self.selskip.clone());

        let mut stcp = self.state;
        match &mut stcp {
            SchematicSt::AreaSelect(vsb) => {
                vsb.max = curpos_vsp;
                self.tentatives_by_vsbox(vsb);
            }
            SchematicSt::Moving(Some((_ssp0, ssp1, _sst)))
            | SchematicSt::Copying(Some((_ssp0, ssp1, _sst))) => {
                *ssp1 = curpos_vsp;
            }
            _ => {}
        }
        self.state = stcp;
    }
    /// set tentative flags by intersection with ssb
    pub fn tentatives_by_vsbox(&mut self, vsb: &VSBox) {
        let vsb_p = VSBox::from_points([vsb.min, vsb.max]).inflate(0.5, 0.5);
        // self.tentatives = self.content.intersects_vsb(vsb_p);
        self.tentatives = self.content.contained_by(vsb_p);
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
    /// put every tentative element into selected
    fn tentatives_to_selected(&mut self) {
        self.selected = self.tentatives.clone();
        if self.tentatives.len() == 1 {
            self.active_element = self.tentatives.iter().next().cloned();
        } else {
            self.active_element = None;
        }
        self.tentatives.clear();
    }
    /// set 1 tentative flag based on ssp and skip number. Returns the flagged element, if any.
    fn selectable(&mut self, vsp: VSPoint, skip: &mut usize) -> Option<E> {
        loop {
            let mut count = 0; // tracks the number of skipped elements
            if let Some(e) = self.content.selectable(vsp, *skip, &mut count) {
                return Some(e);
            }
            if count == 0 {
                *skip = 0;
                return None;
            }
            *skip -= count;
        }
    }
}
