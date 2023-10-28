//! the viewport implements common canvas functions - e.g. panning, zooming
//! CanvasSpace <-> ViewportSpace <-> SchematicSpace
//! CanvasSpace is the UI canvas coordinate
//! ViewportSpace is the schematic coordinate in f32
//! SchematicSpace is the schematic coordinate in i16
//! separated from schematic controls - wouldn't want panning or zooming to cancel placing a device, etc.

use std::ops::Mul;

use crate::transforms::{
    CSBox, CSPoint, CSVec, CVTransform, Point, VCTransform, VSBox, VSPoint, VSVec,
};
use crate::IcedStruct;
use iced::keyboard::Modifiers;
use iced::widget::canvas::path::Arc;
use iced::Renderer;
use iced::{
    mouse,
    widget::canvas::{
        self, event, path::Builder, stroke, Cache, Event, Frame, Geometry, LineCap, LineDash, Path,
        Stroke, Text,
    },
    Color, Length, Rectangle, Size, Theme,
};

/// viewport to canvas space transform with locked aspect ratio
#[derive(Debug, Clone, Copy)]
pub struct VCTransformLockedAspect(VCTransform);
impl VCTransformLockedAspect {
    /// returns the identity transform of this type
    pub fn identity() -> Self {
        Self(VCTransform::identity())
    }
    /// pre flip transform along y-axis
    pub fn pre_flip_y(&self) -> Self {
        Self(self.0.pre_scale(1.0, -1.0))
    }
    /// get the scale factor of the transform
    pub fn scale(&self) -> f32 {
        self.0.m11.abs()
    }
    /// pre_translate
    pub fn pre_translate(&self, v: VSVec) -> Self {
        Self(self.0.pre_translate(v))
    }
    /// then_translate
    pub fn then_translate(&self, v: CSVec) -> Self {
        Self(self.0.then_translate(v))
    }
    /// then scale
    pub fn then_scale(&self, scale: f32) -> Self {
        Self(self.0.then_scale(scale, scale))
    }
    pub fn transform_point(&self, vsp: VSPoint) -> CSPoint {
        self.0.transform_point(vsp)
    }
    /// returns transform and scale such that VSBox (viewport/schematic bounds) fit inside CSBox (canvas bounds)
    pub fn fit_bounds(csb: CSBox, vsb: VSBox, min_zoom: f32, max_zoom: f32) -> Self {
        let mut vct = VCTransform::identity();

        let s = (csb.height() / vsb.height())
            .min(csb.width() / vsb.width())
            .mul(0.9)
            .clamp(min_zoom, max_zoom);
        vct = vct.then_scale(s, -s);
        // vector from vsb center to csb center
        let v = csb.center() - vct.transform_point(vsb.center());
        vct = vct.then_translate(v);

        Self(vct)
    }
    /// return the underlying transform
    pub fn transform(&self) -> VCTransform {
        self.0
    }
    /// return the inverse of the underlying transform
    pub fn inverse_transform(&self) -> CVTransform {
        self.0.inverse().unwrap()
    }
}

/// viewport state
#[derive(Clone, Debug, Default)]
pub enum State {
    /// default viewport state
    #[default]
    Idle,
    /// viewport panning
    Panning(CSPoint),
    /// viewport newview (right click-drag fit view to area) - first point, second point of area selection
    NewView(VSPoint, VSPoint),
}

/// viewport message
#[derive(Clone, Copy, Debug)]
pub enum Msg {
    /// do nothing
    None,
    /// change viewport-canvas transform
    NewView(VCTransformLockedAspect, CSPoint),
    /// cursor moved
    CursorMoved(CSPoint),
}

/// message type that is the union of content and viewport messages - allows content and viewport to process events simultaneously
#[derive(Clone, Copy, Debug)]
pub struct CompositeMsg<M>
where
    M: ContentMsg,
{
    /// content msg
    pub content_msg: M,
    /// viewport message processed from canvas event
    pub viewport_msg: Msg,
}

pub trait Content<Msg>: Default {
    /// returns the mouse interaction to display on canvas based on content state
    fn mouse_interaction(&self) -> mouse::Interaction;
    /// mutate self based on ContentMsg. Returns whether to clear passive cache
    fn update(&mut self, msg: Msg) -> bool;
    /// draw geometry onto active frame
    fn draw_active(&self, vct: VCTransform, scale: f32, frame: &mut Frame);
    /// draw geometry onto passive frame
    fn draw_passive(&self, vct: VCTransform, scale: f32, frame: &mut Frame);
    /// returns the bounding box of all elements in content
    fn bounds(&self) -> VSBox;
}

/// trait for message type of viewport content
pub trait ContentMsg {
    /// function to generate message to handle iced canvas event
    fn canvas_event_msg(event: Event, curpos_vsp: Option<VSPoint>) -> Self;
}

/// The viewport handles canvas to content transforms, zooming, panning, etc.
pub struct Viewport<C, M>
where
    C: Content<M>,
    M: ContentMsg,
{
    /// Contents displayed through this viewport
    pub content: C,
    /// phantom data to mark ContentMsg type
    content_msg: std::marker::PhantomData<M>,
    /// iced canvas graphical cache, cleared every frame
    pub active_cache: Cache,
    /// iced canvas graphical cache, cleared following some schematic actions
    pub passive_cache: Cache,
    /// iced canvas graphical cache, almost never cleared
    pub background_cache: Cache,

    /// viewport to canvas transform
    vct: VCTransformLockedAspect,

    /// the cursor positions in the different spaces
    curpos: (CSPoint, VSPoint),

    /// zoom in limit
    max_zoom: f32,
    /// zoom out limit
    min_zoom: f32,
}

impl<C, M> canvas::Program<CompositeMsg<M>> for Viewport<C, M>
where
    C: Content<M>,
    M: ContentMsg,
{
    type State = State;

    fn update(
        &self,
        state: &mut State,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (event::Status, Option<CompositeMsg<M>>) {
        let opt_curpos: Option<CSPoint> = cursor.position_in(bounds).map(|p| Point::from(p).into());
        let bounds_csb = CSBox::from_points([
            CSPoint::new(bounds.x, bounds.y),
            CSPoint::new(bounds.width, bounds.height),
        ]);

        self.active_cache.clear();

        if opt_curpos.is_some() {
            let msgs = Some(self.events_handler(state, event, bounds_csb, opt_curpos));
            (event::Status::Captured, msgs)
        } else {
            (event::Status::Ignored, None)
        }
    }

    fn draw(
        &self,
        state: &State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let active = self.active_cache.draw(renderer, bounds.size(), |frame| {
            self.content
                .draw_active(self.vc_transform(), self.vc_scale(), frame);

            if let State::NewView(vsp0, vsp1) = state {
                let csp0 = self.vc_transform().transform_point(*vsp0);
                let csp1 = self.vc_transform().transform_point(*vsp1);
                let selsize = Size {
                    width: csp1.x - csp0.x,
                    height: csp1.y - csp0.y,
                };
                let f = canvas::Fill {
                    style: canvas::Style::Solid(if selsize.height > 0. {
                        Color::from_rgba(1., 0., 0., 0.1)
                    } else {
                        Color::from_rgba(0., 0., 1., 0.1)
                    }),
                    ..canvas::Fill::default()
                };
                frame.fill_rectangle(Point::from(csp0).into(), selsize, f);
            }
        });

        let passive = self.passive_cache.draw(renderer, bounds.size(), |frame| {
            self.draw_grid(
                frame,
                CSBox::new(
                    CSPoint::origin(),
                    CSPoint::from([bounds.width, bounds.height]),
                ),
            );
            self.draw_origin_marker(frame);
            self.content
                .draw_passive(self.vc_transform(), self.vc_scale(), frame);
        });

        let background = self
            .background_cache
            .draw(renderer, bounds.size(), |frame| {
                let f = canvas::Fill {
                    style: canvas::Style::Solid(Color::from_rgb(0.2, 0.2, 0.2)),
                    ..canvas::Fill::default()
                };
                frame.fill_rectangle(iced::Point::ORIGIN, bounds.size(), f);
            });

        vec![background, passive, active]
    }

    fn mouse_interaction(
        &self,
        viewport_st: &State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(bounds) {
            match &viewport_st {
                State::Panning(_) => mouse::Interaction::Grabbing,
                State::Idle => self.content.mouse_interaction(),
                _ => mouse::Interaction::default(),
            }
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<C, M> IcedStruct<CompositeMsg<M>> for Viewport<C, M>
where
    C: Content<M>,
    M: ContentMsg,
{
    fn update(&mut self, msgs: CompositeMsg<M>) {
        match msgs.viewport_msg {
            Msg::NewView(vct, curpos_csp) => {
                self.vct = vct;
                // update cursor position, otherwise it is displayed according to old vct until cursor is moved again
                self.curpos_update(curpos_csp);
                self.passive_cache.clear();
            }
            Msg::CursorMoved(curpos_csp) => {
                self.curpos_update(curpos_csp);
            }
            Msg::None => {}
        }
        if self.content.update(msgs.content_msg) {
            self.passive_cache.clear();
        }
    }

    fn view(&self) -> iced::Element<CompositeMsg<M>> {
        iced::widget::canvas(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl<C, M> Viewport<C, M>
where
    C: Content<M>,
    M: ContentMsg,
{
    pub fn new(min_zoom: f32, max_zoom: f32, vct: VCTransformLockedAspect) -> Self {
        Viewport {
            min_zoom,
            max_zoom,
            vct,
            content: C::default(),
            active_cache: Default::default(),
            passive_cache: Default::default(),
            background_cache: Default::default(),
            curpos: Default::default(),
            content_msg: std::marker::PhantomData,
        }
    }

    /// generate message based on canvas event
    pub fn events_handler(
        &self,
        state: &mut State,
        event: iced::widget::canvas::Event,
        bounds_csb: CSBox,
        opt_curpos_csp: Option<CSPoint>,
    ) -> CompositeMsg<M> {
        let mut viewport_msg = Msg::None;
        let mut stcp = state.clone();
        const NO_MODIFIER: Modifiers = Modifiers::empty();
        if let Some(curpos_csp) = opt_curpos_csp {
            match (&mut stcp, event) {
                // cursor move
                (State::Idle, Event::Mouse(iced::mouse::Event::CursorMoved { .. })) => {
                    viewport_msg = Msg::CursorMoved(curpos_csp);
                }
                // zooming
                (_, Event::Mouse(iced::mouse::Event::WheelScrolled { delta })) => match delta {
                    iced::mouse::ScrollDelta::Lines { y, .. }
                    | iced::mouse::ScrollDelta::Pixels { y, .. } => {
                        let zoom_scale = 1.0 + y.clamp(-5.0, 5.0) / 5.;
                        viewport_msg = self.zoom(zoom_scale, curpos_csp);
                    }
                },
                // panning
                (
                    State::Idle,
                    Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Middle)),
                ) => {
                    stcp = State::Panning(curpos_csp);
                }
                (
                    State::Panning(csp_prev),
                    Event::Mouse(iced::mouse::Event::CursorMoved { .. }),
                ) => {
                    viewport_msg = self.pan(curpos_csp, *csp_prev);
                    *csp_prev = curpos_csp;
                }
                (
                    State::Panning(_),
                    Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Middle)),
                ) => {
                    stcp = State::Idle;
                }
                // newview
                (
                    State::Idle,
                    Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Right)),
                ) => {
                    let vsp = self.cv_transform().transform_point(curpos_csp);
                    stcp = State::NewView(vsp, vsp);
                }
                (
                    State::NewView(vsp0, vsp1),
                    Event::Mouse(iced::mouse::Event::CursorMoved { .. }),
                ) => {
                    let vsp_now = self.cv_transform().transform_point(curpos_csp);
                    if (vsp_now - *vsp0).length() > 10. {
                        *vsp1 = vsp_now;
                    } else {
                        *vsp1 = *vsp0;
                    }
                    viewport_msg = Msg::CursorMoved(curpos_csp);
                }
                (
                    State::NewView(_vsp0, _vsp1),
                    Event::Keyboard(iced::keyboard::Event::KeyPressed {
                        key_code,
                        modifiers,
                    }),
                ) => {
                    if let (iced::keyboard::KeyCode::Escape, 0) = (key_code, modifiers.bits()) {
                        stcp = State::Idle;
                    }
                }
                (
                    State::NewView(vsp0, vsp1),
                    Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Right)),
                ) => {
                    if vsp1 != vsp0 {
                        viewport_msg = self.display_bounds(
                            bounds_csb,
                            VSBox::from_points([vsp0, vsp1]),
                            curpos_csp,
                        );
                    }
                    stcp = State::Idle;
                }
                // fit view to content
                (
                    State::Idle,
                    Event::Keyboard(iced::keyboard::Event::KeyPressed {
                        key_code: iced::keyboard::KeyCode::F,
                        modifiers: NO_MODIFIER,
                    }),
                ) => {
                    let vsb = self.content.bounds();
                    let csp = self.curpos_csp();
                    viewport_msg = self.display_bounds(bounds_csb, vsb, csp);
                }
                _ => {}
            }
            *state = stcp;
        }

        let content_msg = M::canvas_event_msg(
            event,
            opt_curpos_csp.map(|csp| self.cv_transform().transform_point(csp)),
        );
        CompositeMsg {
            content_msg,
            viewport_msg,
        }
    }

    /// returns the cursor position in canvas space
    pub fn curpos_csp(&self) -> CSPoint {
        self.curpos.0
    }

    /// returns the cursor position in viewport space
    #[allow(dead_code)]
    pub fn curpos_vsp(&self) -> VSPoint {
        self.curpos.1
    }

    /// change transform such that VSBox (viewport/schematic bounds) fit inside CSBox (canvas bounds)
    pub fn display_bounds(&self, csb: CSBox, vsb: VSBox, csp: CSPoint) -> Msg {
        let vct = VCTransformLockedAspect::fit_bounds(csb, vsb, self.min_zoom, self.max_zoom);
        Msg::NewView(vct, csp)
    }

    /// pan by vector v
    pub fn pan(&self, csp_now: CSPoint, csp_prev: CSPoint) -> Msg {
        let v = self.cv_transform().transform_vector(csp_now - csp_prev);
        let vct = self.vct.pre_translate(v);
        Msg::NewView(vct, csp_now)
    }

    /// return the canvas to viewport space transform
    pub fn cv_transform(&self) -> CVTransform {
        self.vct.inverse_transform()
    }

    /// return the viewport to canvas space transform
    pub fn vc_transform(&self) -> VCTransform {
        self.vct.transform()
    }

    /// returns the scale factor in the viewwport to canvas transform
    /// this value is stored to avoid calling sqrt() each time
    pub fn vc_scale(&self) -> f32 {
        self.vct.scale()
    }

    /// returns the scale factor in the viewwport to canvas transform
    /// this value is stored to avoid calling sqrt() each time
    #[allow(dead_code)]
    pub fn cv_scale(&self) -> f32 {
        1. / self.vct.scale()
    }

    /// update the cursor position
    pub fn curpos_update(&mut self, csp1: CSPoint) {
        let vsp1 = self.cv_transform().transform_point(csp1);
        self.curpos = (csp1, vsp1);
    }

    /// change the viewport zoom by scale
    pub fn zoom(&self, zoom_scale: f32, curpos_csp: CSPoint) -> Msg {
        let (csp, vsp) = self.curpos;
        let scaled_transform = self.vct.then_scale(zoom_scale);

        let mut new_transform; // transform with applied scale and translated to maintain p_viewport position
        let new_scale = scaled_transform.scale();
        if new_scale < self.min_zoom {
            // minimum scale
            let clamped_scale = self.min_zoom / self.vc_scale();
            new_transform = self.vct.then_scale(clamped_scale);
        } else if new_scale <= self.max_zoom {
            // adjust scale
            new_transform = scaled_transform;
        } else {
            // maximum scale
            let clamped_scale = self.max_zoom / self.vc_scale();
            new_transform = self.vct.then_scale(clamped_scale);
        }
        let csp1 = new_transform.transform_point(vsp); // translate based on cursor location
        let translation = csp - csp1;
        new_transform = new_transform.then_translate(translation);

        Msg::NewView(new_transform, curpos_csp)
    }

    /// draw the origin marker
    pub fn draw_origin_marker(&self, frame: &mut Frame) {
        let a = Text {
            content: String::from("origin"),
            position: Point::from(self.vc_transform().transform_point(VSPoint::origin())).into(),
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.25),
            size: self.vc_scale(),
            ..Default::default()
        };
        frame.fill_text(a);
        let ref_stroke = Stroke {
            width: (0.1 * self.vc_scale()).clamp(0.1, 3.0),
            style: stroke::Style::Solid(Color::from_rgba(1.0, 1.0, 1.0, 0.25)),
            line_cap: LineCap::Round,
            ..Stroke::default()
        };
        let mut path_builder = Builder::new();
        path_builder.move_to(
            Point::from(self.vc_transform().transform_point(VSPoint::new(0.0, 1.0))).into(),
        );
        path_builder.line_to(
            Point::from(self.vc_transform().transform_point(VSPoint::new(0.0, -1.0))).into(),
        );
        path_builder.move_to(
            Point::from(self.vc_transform().transform_point(VSPoint::new(1.0, 0.0))).into(),
        );
        path_builder.line_to(
            Point::from(self.vc_transform().transform_point(VSPoint::new(-1.0, 0.0))).into(),
        );
        let p = self.vc_transform().transform_point(VSPoint::origin());
        let r = self.vc_scale() * 0.5;
        path_builder.circle(Point::from(p).into(), r);

        path_builder.arc(Arc {
            center: Point::from(p).into(),
            radius: r * 2.0,
            start_angle: 1.0,
            end_angle: 3.0,
        });
        path_builder.ellipse(canvas::path::arc::Elliptical {
            center: Point::from(p).into(),
            radii: iced::Vector::new(r * 3.0, r * 1.0),
            rotation: 1.0,
            start_angle: 6.3,
            end_angle: 0.0,
        });
        frame.stroke(&path_builder.build(), ref_stroke);
    }

    /// draw the schematic grid onto canvas
    pub fn draw_grid(&self, frame: &mut Frame, bb_canvas: CSBox) {
        fn draw_grid_w_spacing(
            spacing: f32,
            bb_canvas: CSBox,
            vct: VCTransform,
            cvt: CVTransform,
            frame: &mut Frame,
            stroke: Stroke,
        ) {
            let bb_viewport = cvt.outer_transformed_box(&bb_canvas);
            let v = ((bb_viewport.min / spacing).ceil() * spacing) - bb_viewport.min;
            let bb_viewport = bb_viewport.translate(v);

            let v = bb_viewport.max - bb_viewport.min;
            for col in 0..=(v.x / spacing).ceil() as u32 {
                let csp0 = bb_viewport.min + VSVec::from([col as f32 * spacing, 0.0]);
                let csp1 = bb_viewport.min + VSVec::from([col as f32 * spacing, v.y.ceil()]);
                let c = Path::line(
                    Point::from(vct.transform_point(csp0)).into(),
                    Point::from(vct.transform_point(csp1)).into(),
                );
                frame.stroke(&c, stroke.clone());
            }
        }
        let coarse_grid_threshold: f32 = 2.0;
        let fine_grid_threshold: f32 = 6.0;
        if self.vc_scale() > coarse_grid_threshold {
            // draw coarse grid
            let spacing = 16.0;

            let grid_stroke = Stroke {
                width: (0.5 * self.vc_scale()).clamp(0.5, 3.0),
                style: stroke::Style::Solid(Color::from_rgba(1.0, 1.0, 1.0, 0.5)),
                line_cap: LineCap::Round,
                line_dash: LineDash {
                    segments: &[0.0, spacing * self.vc_scale()],
                    offset: 0,
                },
                ..Stroke::default()
            };
            draw_grid_w_spacing(
                spacing,
                bb_canvas,
                self.vc_transform(),
                self.cv_transform(),
                frame,
                grid_stroke,
            );

            if self.vc_scale() > fine_grid_threshold {
                // draw fine grid if sufficiently zoomed in
                let spacing = 2.0;

                let grid_stroke = Stroke {
                    width: 1.0,
                    style: stroke::Style::Solid(Color::from_rgba(1.0, 1.0, 1.0, 0.5)),
                    line_cap: LineCap::Round,
                    line_dash: LineDash {
                        segments: &[0.0, spacing * self.vc_scale()],
                        offset: 0,
                    },
                    ..Stroke::default()
                };

                draw_grid_w_spacing(
                    spacing,
                    bb_canvas,
                    self.vc_transform(),
                    self.cv_transform(),
                    frame,
                    grid_stroke,
                );
            }
        }
    }
}
