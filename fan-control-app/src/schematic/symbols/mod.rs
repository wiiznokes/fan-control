//! Symbols Designer
//! Concrete types for schematic content for designing device appearances
//! intended to eventually allow users to define hierarchical devices
//! for now, intended only to allow devs to quickly draw up basic device symbols

use crate::schematic::atoms::{Port, RcRPort};
use crate::schematic::interactable::Interactive;
use crate::schematic::{self, SchematicAtom, SchematicMsg};
use crate::transforms::{Point, SSBox, SSPoint, VSPoint};
use crate::transforms::{VCTransform, VSBox, VVTransform};
use crate::Drawable;
use iced::keyboard::Modifiers;
use iced::widget::canvas::path::Builder;
use iced::widget::canvas::{event::Event, Frame};
use iced::widget::canvas::{stroke, LineCap, Stroke};
use iced::Color;
use send_wrapper::SendWrapper;

use crate::schematic::atoms::{Bounds, CirArc, LineSeg, RcRBounds, RcRCirArc, RcRLineSeg};
use std::collections::HashSet;
use std::fs;

mod gui;
pub use gui::DevicePageMsg;
pub use gui::SymbolDesignerPage;

mod atoms;
pub use atoms::DesignerElement;

#[derive(Debug, Clone)]
pub enum Msg {
    CanvasEvent(Event),
    Line,
}

impl schematic::ContentMsg for Msg {
    fn canvas_event_msg(event: Event) -> Self {
        Msg::CanvasEvent(event)
    }
}

#[derive(Debug, Clone, Default)]
pub enum DesignerSt {
    #[default]
    Idle,
    Line(Option<(VSPoint, VSPoint)>),
    CirArc((Option<(VSPoint, VSPoint, VSPoint)>, u8)),
    Bounds(Option<(SSPoint, SSPoint)>),
}

/// struct holding schematic state (lines and ellipses)
#[derive(Debug, Clone)]
pub struct Designer {
    pub infobarstr: Option<String>,

    state: DesignerSt,

    content: HashSet<DesignerElement>,

    rounding_interval: f32,
    curpos_vsp: VSPoint,
}

impl Default for Designer {
    fn default() -> Self {
        Self {
            infobarstr: Default::default(),
            state: Default::default(),
            content: Default::default(),
            rounding_interval: 0.25,
            curpos_vsp: Default::default(),
        }
    }
}

impl Designer {
    fn update_cursor_vsp(&mut self, curpos_vsp: VSPoint) {
        self.curpos_vsp = (curpos_vsp / self.rounding_interval).round() * self.rounding_interval;
        match &mut self.state {
            DesignerSt::Bounds(opt_vsps) => {
                if let Some((_ssp0, ssp1)) = opt_vsps {
                    *ssp1 = self.curpos_vsp.round().cast().cast_unit();
                }
            }
            DesignerSt::CirArc((opt_vsps, wmi)) => {
                if let Some((_vsp_center, vsp0, vsp1)) = opt_vsps {
                    match wmi {
                        0 => {
                            *vsp0 = self.curpos_vsp;
                            *vsp1 = self.curpos_vsp;
                        }
                        1 => {
                            *vsp1 = self.curpos_vsp;
                        }
                        _ => {}
                    }
                }
            }
            DesignerSt::Line(opt_vsps) => {
                if let Some((_vsp0, vsp1)) = opt_vsps {
                    *vsp1 = self.curpos_vsp;
                }
            }
            DesignerSt::Idle => {}
        }
    }
    fn occupies_vsp(&self, _vsp: VSPoint) -> bool {
        false
    }
    /// create graphics for the current designer and save it.
    fn graphics(&mut self) {
        let mut graphics = String::from(
            "lazy_static! {\n    static ref DEFAULT_GRAPHICS: Graphics = Graphics {\n",
        );
        let pts: Vec<_> = self
            .content
            .iter()
            .filter_map(|x| {
                if let DesignerElement::RcRLineSeg(l) = x {
                    Some(l)
                } else {
                    None
                }
            })
            .collect();
        let cirarcs: Vec<_> = self
            .content
            .iter()
            .filter_map(|x| {
                if let DesignerElement::RcRCirArc(l) = x {
                    Some(l)
                } else {
                    None
                }
            })
            .collect();
        let ports: Vec<_> = self
            .content
            .iter()
            .filter_map(|x| {
                if let DesignerElement::RcRPort(p) = x {
                    Some(p)
                } else {
                    None
                }
            })
            .collect();
        let bounds: Vec<_> = self
            .content
            .iter()
            .filter_map(|x| {
                if let DesignerElement::RcRBounds(b) = x {
                    Some(b)
                } else {
                    None
                }
            })
            .collect();
        graphics.push_str("        pts: vec![\n");
        // lines
        for line in pts {
            let pt01 = line.0.borrow().pts();
            graphics.push_str(&format!(
                "            vec![VSPoint::new({:0.2}, {:0.2}), VSPoint::new({:0.2}, {:0.2}),],\n",
                pt01.0.x, pt01.0.y, pt01.1.x, pt01.1.y
            ))
        }
        graphics.push_str("        ],\n");
        graphics.push_str("        cirarcs: vec![\n");
        for &c in cirarcs.iter() {
            //         cirarcs: vec![
            //             CirArc::from_triplet(VSPoint::origin(), VSPoint::new(0.0, 1.5), VSPoint::new(0.0, 1.5))
            //             ],
            //         for c in circles
            let pts = c.0.borrow().pts();
            graphics.push_str(&format!("             CirArc::from_triplet(VSPoint::new({:0.2}, {:0.2}), VSPoint::new({:0.2}, {:0.2}), VSPoint::new({:0.2}, {:0.2})),\n", pts.0.x, pts.0.y, pts.1.x, pts.1.y, pts.2.x, pts.2.y));
        }
        graphics.push_str("        ],\n");

        // ports
        graphics.push_str("        ports: vec![\n");
        for (i, &port) in ports.iter().enumerate() {
            //             Port {
            //                 name: "+".to_string(),
            //                 offset: SSPoint::new(0, 3),
            //                 interactable: Interactable::default(),
            //             },
            graphics.push_str("            Port {\n");
            graphics.push_str(&format!("                name: \"{}\".to_string(),\n", i));
            graphics.push_str(&format!(
                "                offset: SSPoint::new({}, {}),\n",
                port.0.borrow().offset.x,
                port.0.borrow().offset.y
            ));
            graphics.push_str("                interactable: Interactable::default()\n");
            graphics.push_str("            },\n");
        }
        graphics.push_str("        ],\n");
        // bounds
        for bound in bounds {
            //         bounds: SSBox::new(SSPoint::new(-2, 3), SSPoint::new(2, -3)),
            let pt01 = bound.0.borrow().pts();
            graphics.push_str(&format!(
                "        bounds: SSBox::new(SSPoint::new({}, {}), SSPoint::new({}, {})),\n",
                pt01.0.x, pt01.0.y, pt01.1.x, pt01.1.y
            ))
        }
        graphics.push_str("    };\n");
        graphics.push_str(" }\n");

        fs::write("graphics.txt", graphics.as_bytes()).expect("Unable to write file");
    }
}

impl Drawable for Designer {
    fn draw_persistent(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        for e in &self.content {
            e.draw_persistent(vct, vcscale, frame);
        }
    }

    fn draw_selected(&self, _vct: VCTransform, _vcscale: f32, _frame: &mut Frame) {
        panic!("not intended for use");
    }

    fn draw_preview(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        fn draw_snap_marker(vsp: VSPoint, vct: VCTransform, _vcscale: f32, frame: &mut Frame) {
            let cursor_stroke = || -> Stroke {
                Stroke {
                    width: 1.0,
                    style: stroke::Style::Solid(Color::from_rgb(1.0, 0.9, 0.0)),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };
            let dim = 0.25;
            let x0 = vsp.x - dim;
            let x1 = vsp.x + dim;
            let y0 = vsp.y - dim;
            let y1 = vsp.y + dim;
            let mut path_builder = Builder::new();
            path_builder.move_to(Point::from(vct.transform_point(VSPoint::new(x0, vsp.y))).into());
            path_builder.line_to(Point::from(vct.transform_point(VSPoint::new(x1, vsp.y))).into());
            path_builder.move_to(Point::from(vct.transform_point(VSPoint::new(vsp.x, y0))).into());
            path_builder.line_to(Point::from(vct.transform_point(VSPoint::new(vsp.x, y1))).into());
            frame.stroke(&path_builder.build(), cursor_stroke());
        }
        match &self.state {
            DesignerSt::Line(opt_vsps) => {
                if let Some((vsp0, vsp1)) = opt_vsps {
                    LineSeg::new(*vsp0, *vsp1).draw_preview(vct, vcscale, frame);
                }
            }
            DesignerSt::CirArc(opt_vsps) => {
                if let (Some((vsp_center, vsp0, vsp1)), _) = opt_vsps {
                    CirArc::from_triplet(*vsp_center, *vsp0, *vsp1)
                        .draw_preview(vct, vcscale, frame);
                }
            }
            DesignerSt::Bounds(opt_vsps) => {
                draw_snap_marker(self.curpos_vsp.round(), vct, vcscale, frame);
                if let Some((ssp0, ssp1)) = opt_vsps {
                    Bounds::new(SSBox::from_points([ssp0, ssp1])).draw_preview(vct, vcscale, frame);
                }
            }
            DesignerSt::Idle => {}
        }
    }
}

impl schematic::Content<DesignerElement, Msg> for Designer {
    fn curpos_update(&mut self, vsp: VSPoint) {
        self.update_cursor_vsp(vsp);
    }
    fn curpos_vsp(&self) -> VSPoint {
        self.curpos_vsp
    }
    fn bounds(&self) -> VSBox {
        if !self.content.is_empty() {
            let v_pts: Vec<_> = self
                .content
                .iter()
                .flat_map(|f| [f.bounding_box().min, f.bounding_box().max])
                .collect();
            VSBox::from_points(v_pts)
        } else {
            VSBox::from_points([VSPoint::new(-1.0, -1.0), VSPoint::new(1.0, 1.0)])
        }
    }
    fn intersects_vsb(&mut self, vsb: VSBox) -> HashSet<DesignerElement> {
        let mut ret = HashSet::new();
        for d in &self.content {
            match d {
                DesignerElement::RcRLineSeg(l) => {
                    if l.0.borrow_mut().interactable.intersects_vsb(&vsb) {
                        ret.insert(DesignerElement::RcRLineSeg(l.clone()));
                    }
                }
                DesignerElement::RcRCirArc(l) => {
                    if l.0.borrow_mut().interactable.intersects_vsb(&vsb) {
                        ret.insert(DesignerElement::RcRCirArc(l.clone()));
                    }
                }
                DesignerElement::RcRPort(l) => {
                    if l.0.borrow_mut().interactable.intersects_vsb(&vsb) {
                        ret.insert(DesignerElement::RcRPort(l.clone()));
                    }
                }
                DesignerElement::RcRBounds(l) => {
                    if l.0.borrow_mut().interactable.intersects_vsb(&vsb) {
                        ret.insert(DesignerElement::RcRBounds(l.clone()));
                    }
                }
            }
        }
        ret
    }
    fn contained_by(&mut self, vsb: VSBox) -> HashSet<DesignerElement> {
        let mut ret = HashSet::new();
        for d in &self.content {
            match d {
                DesignerElement::RcRLineSeg(l) => {
                    if l.0.borrow_mut().interactable.contained_by(&vsb) {
                        ret.insert(DesignerElement::RcRLineSeg(l.clone()));
                    }
                }
                DesignerElement::RcRCirArc(l) => {
                    if l.0.borrow_mut().interactable.contained_by(&vsb) {
                        ret.insert(DesignerElement::RcRCirArc(l.clone()));
                    }
                }
                DesignerElement::RcRPort(l) => {
                    if l.0.borrow_mut().interactable.contained_by(&vsb) {
                        ret.insert(DesignerElement::RcRPort(l.clone()));
                    }
                }
                DesignerElement::RcRBounds(l) => {
                    if l.0.borrow_mut().interactable.contained_by(&vsb) {
                        ret.insert(DesignerElement::RcRBounds(l.clone()));
                    }
                }
            }
        }
        ret
    }
    /// returns the first CircuitElement after skip which intersects with curpos_ssp, if any.
    /// count is updated to track the number of elements skipped over
    fn selectable(
        &mut self,
        vsp: VSPoint,
        skip: usize,
        count: &mut usize,
    ) -> Option<DesignerElement> {
        for d in &self.content {
            match d {
                DesignerElement::RcRLineSeg(l) => {
                    if l.0.borrow_mut().interactable.contains_vsp(vsp) {
                        if *count == skip {
                            // skipped just enough
                            return Some(d.clone());
                        } else {
                            *count += 1;
                        }
                    }
                }
                DesignerElement::RcRCirArc(l) => {
                    if l.0.borrow_mut().interactable.contains_vsp(vsp) {
                        if *count == skip {
                            // skipped just enough
                            return Some(d.clone());
                        } else {
                            *count += 1;
                        }
                    }
                }
                DesignerElement::RcRPort(l) => {
                    if l.0.borrow_mut().interactable.contains_vsp(vsp) {
                        if *count == skip {
                            // skipped just enough
                            return Some(d.clone());
                        } else {
                            *count += 1;
                        }
                    }
                }
                DesignerElement::RcRBounds(b) => {
                    if b.0.borrow_mut().interactable.contains_vsp(vsp) {
                        if *count == skip {
                            // skipped just enough
                            return Some(d.clone());
                        } else {
                            *count += 1;
                        }
                    }
                }
            }
        }
        None
    }

    fn update(&mut self, msg: Msg) -> SchematicMsg<DesignerElement> {
        match msg {
            Msg::CanvasEvent(event) => {
                let mut state = self.state.clone();
                let mut ret_msg_tmp = SchematicMsg::None;
                const NO_MODIFIER: Modifiers = Modifiers::empty();
                match (&mut state, event) {
                    // draw an arc
                    (
                        DesignerSt::Idle,
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::A,
                            modifiers: NO_MODIFIER,
                        }),
                    ) => {
                        state = DesignerSt::CirArc((None, 0));
                    }
                    (
                        DesignerSt::CirArc((cirarc_st, wmi)),
                        Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),
                    ) => {
                        if let Some((vsp_center, vsp0, vsp1)) = cirarc_st {
                            match wmi {
                                0 => {
                                    *wmi += 1;
                                }
                                1 => {
                                    self.content.insert(DesignerElement::RcRCirArc(
                                        RcRCirArc::new(CirArc::from_triplet(
                                            *vsp_center,
                                            *vsp0,
                                            *vsp1,
                                        )),
                                    ));
                                    state = DesignerSt::Idle;
                                }
                                _ => {
                                    state = DesignerSt::Idle;
                                }
                            }
                        } else {
                            state = DesignerSt::CirArc((
                                Some((self.curpos_vsp, self.curpos_vsp, self.curpos_vsp)),
                                0,
                            ))
                        }
                        ret_msg_tmp = SchematicMsg::ClearPassive;
                    }
                    // output graphics definition
                    (
                        DesignerSt::Idle,
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::Space,
                            modifiers: NO_MODIFIER,
                        }),
                    ) => {
                        self.graphics();
                    }
                    // draw bounds
                    (
                        DesignerSt::Idle,
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::B,
                            modifiers: NO_MODIFIER,
                        }),
                    ) => {
                        state = DesignerSt::Bounds(None);
                    }
                    (
                        DesignerSt::Bounds(opt_ws),
                        Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),
                    ) => {
                        let new_st;
                        if let Some((ssp0, ssp1)) = opt_ws {
                            // subsequent click
                            if self.curpos_vsp.round().cast().cast_unit() == *ssp0 {
                                new_st = DesignerSt::Idle; // zero size bounds: do not make
                            } else {
                                self.content
                                    .insert(DesignerElement::RcRBounds(RcRBounds::new(
                                        Bounds::new(SSBox::from_points([ssp0, ssp1])),
                                    )));
                                new_st = DesignerSt::Idle; // created a valid bound: return to idle state
                            }
                            ret_msg_tmp = SchematicMsg::ClearPassive;
                        } else {
                            // first click
                            let ssp = self.curpos_vsp.round().cast().cast_unit();
                            new_st = DesignerSt::Bounds(Some((ssp, ssp)));
                        }
                        state = new_st;
                    }
                    // port placement
                    (
                        DesignerSt::Idle,
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::P,
                            modifiers: NO_MODIFIER,
                        }),
                    ) => {
                        ret_msg_tmp = SchematicMsg::NewElement(SendWrapper::new(
                            DesignerElement::RcRPort(RcRPort::new(Port::default())),
                        ));
                    }
                    // line placement
                    (
                        DesignerSt::Idle,
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::W,
                            modifiers: NO_MODIFIER,
                        }),
                    ) => {
                        state = DesignerSt::Line(None);
                    }
                    (
                        DesignerSt::Line(opt_ws),
                        Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),
                    ) => {
                        let vsp = self.curpos_vsp;
                        let new_ws;
                        if let Some((ssp0, _ssp1)) = opt_ws {
                            // subsequent click
                            if vsp == *ssp0 {
                                new_ws = None;
                            } else if self.occupies_vsp(vsp) {
                                self.content
                                    .insert(DesignerElement::RcRLineSeg(RcRLineSeg::new(
                                        LineSeg::new(*ssp0, vsp),
                                    )));
                                new_ws = None;
                            } else {
                                self.content
                                    .insert(DesignerElement::RcRLineSeg(RcRLineSeg::new(
                                        LineSeg::new(*ssp0, vsp),
                                    )));
                                new_ws = Some((vsp, vsp));
                            }
                            ret_msg_tmp = SchematicMsg::ClearPassive;
                        } else {
                            // first click
                            new_ws = Some((vsp, vsp));
                        }
                        state = DesignerSt::Line(new_ws);
                    }
                    // state reset
                    (
                        _,
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::Escape,
                            modifiers: NO_MODIFIER,
                        }),
                    ) => {
                        state = DesignerSt::Idle;
                    }
                    _ => {}
                }
                self.state = state;
                ret_msg_tmp
            }
            Msg::Line => {
                self.state = DesignerSt::Line(None);
                SchematicMsg::None
            }
        }
    }

    fn move_elements(&mut self, elements: &mut HashSet<DesignerElement>, sst: &VVTransform) {
        for e in &*elements {
            match e {
                DesignerElement::RcRLineSeg(l) => {
                    l.0.borrow_mut().transform(*sst);
                    // if moving an existing line, does nothing
                    // inserts the line if placing a new line
                    self.content.insert(DesignerElement::RcRLineSeg(l.clone()));
                }
                DesignerElement::RcRCirArc(l) => {
                    l.0.borrow_mut().transform(*sst);
                    // if moving an existing line, does nothing
                    // inserts the line if placing a new line
                    self.content.insert(DesignerElement::RcRCirArc(l.clone()));
                }
                DesignerElement::RcRPort(l) => {
                    l.0.borrow_mut().transform(*sst);
                    // if moving an existing line, does nothing
                    // inserts the line if placing a new line
                    self.content.insert(DesignerElement::RcRPort(l.clone()));
                }
                DesignerElement::RcRBounds(l) => {
                    l.0.borrow_mut().transform(*sst);
                    // if moving an existing line, does nothing
                    // inserts the line if placing a new line
                    self.content.insert(DesignerElement::RcRBounds(l.clone()));
                }
            }
        }
    }

    fn copy_elements(&mut self, elements: &mut HashSet<DesignerElement>, sst: &VVTransform) {
        for e in &*elements {
            match e {
                DesignerElement::RcRLineSeg(rcl) => {
                    //unwrap refcell
                    let refcell_d = rcl.0.borrow();
                    let mut line = (*refcell_d).clone();
                    line.transform(*sst);

                    //build BaseElement
                    self.content
                        .insert(DesignerElement::RcRLineSeg(RcRLineSeg::new(line)));
                }
                DesignerElement::RcRCirArc(rcl) => {
                    //unwrap refcell
                    let refcell_d = rcl.0.borrow();
                    let mut cirarc = (*refcell_d).clone();
                    cirarc.transform(*sst);

                    //build BaseElement
                    self.content
                        .insert(DesignerElement::RcRCirArc(RcRCirArc::new(cirarc)));
                }
                DesignerElement::RcRPort(rcl) => {
                    //unwrap refcell
                    let refcell_d = rcl.0.borrow();
                    let mut port = (*refcell_d).clone();
                    port.transform(*sst);

                    //build BaseElement
                    self.content
                        .insert(DesignerElement::RcRPort(RcRPort::new(port)));
                }
                DesignerElement::RcRBounds(rcl) => {
                    //unwrap refcell
                    let refcell_d = rcl.0.borrow();
                    let mut bounds = (*refcell_d).clone();
                    bounds.transform(*sst);

                    //build BaseElement
                    self.content
                        .insert(DesignerElement::RcRBounds(RcRBounds::new(bounds)));
                }
            }
        }
    }

    fn delete_elements(&mut self, elements: &HashSet<DesignerElement>) {
        for e in elements {
            self.content.remove(e);
        }
    }

    fn is_idle(&self) -> bool {
        matches!(self.state, DesignerSt::Idle)
    }
}
