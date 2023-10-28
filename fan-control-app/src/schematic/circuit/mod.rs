//! Circuit
//! Concrete types for schematic content

use crate::schematic::atoms::NetVertex;
use crate::schematic::atoms::RcRDevice;
use crate::schematic::atoms::RcRLabel;
use crate::schematic::layers::Devices;
use crate::schematic::layers::NetLabels;
use crate::schematic::layers::Nets;
use crate::schematic::models::NgModels;
use crate::schematic::{self, interactable::Interactive, SchematicMsg};
use crate::transforms::VSPoint;
use crate::transforms::{SSPoint, VCTransform, VSBox, VVTransform};
use crate::Drawable;
use iced::keyboard::Modifiers;
use iced::widget::canvas::{event::Event, Frame};
use paprika::PkVecvaluesall;
use send_wrapper::SendWrapper;
use std::cell::RefCell;
use std::rc::Rc;

use std::{collections::HashSet, fs};

mod gui;
pub use gui::CircuitPageMsg;
pub use gui::CircuitSchematicPage;

mod atoms;
pub use atoms::CircuitAtom;

use super::layers::DevicesLayer;
use super::layers::DijkstraSt;
use super::layers::NetLabelsLayer;
use super::layers::NetsLayer;
use super::layers::SchematicLayerEnum;

#[derive(Debug, Clone)]
pub enum Msg {
    CanvasEvent(Event),
    Wire,
    NetList,
    DcOp(PkVecvaluesall),
    Ac(PkVecvaluesall),
}

impl schematic::ContentMsg for Msg {
    fn canvas_event_msg(event: Event) -> Self {
        Msg::CanvasEvent(event)
    }
}

#[derive(Clone, Default)]
pub enum CircuitSt {
    #[default]
    Idle,
    Wiring(Option<(Box<Nets>, DijkstraSt)>),
}

/// struct holding schematic state (nets, devices, and their locations)
#[derive(Clone)]
pub struct Circuit {
    pub infobarstr: Option<String>,

    state: CircuitSt,

    // nets: Nets,
    // devices: Devices,
    // labels: NetLabels,
    layers: Box<[SchematicLayerEnum]>,

    curpos_ssp: SSPoint,

    device_models: NgModels,
}

impl Default for Circuit {
    fn default() -> Self {
        Self {
            infobarstr: Default::default(),
            state: Default::default(),
            // nets: Default::default(),
            // devices: Default::default(),
            // labels: Default::default(),
            layers: Box::new([
                SchematicLayerEnum::NetsLayer(NetsLayer::default()),
                SchematicLayerEnum::DevicesLayer(DevicesLayer::default()),
                SchematicLayerEnum::NetLabelsLayer(NetLabelsLayer::default()),
            ]),
            curpos_ssp: Default::default(),
            device_models: Default::default(),
        }
    }
}

impl Circuit {
    fn nets_layer_mut(&mut self) -> &mut Nets {
        if let SchematicLayerEnum::NetsLayer(nets) = &mut self.layers[0] {
            &mut *nets
        } else {
            panic!("nets layer should be in index 0");
        }
    }
    fn devices_layer_mut(&mut self) -> &mut Devices {
        if let SchematicLayerEnum::DevicesLayer(devices) = &mut self.layers[1] {
            &mut *devices
        } else {
            panic!("devices layer should be in index 1");
        }
    }
    fn labels_layer_mut(&mut self) -> &mut NetLabels {
        if let SchematicLayerEnum::NetLabelsLayer(labels) = &mut self.layers[2] {
            &mut *labels
        } else {
            panic!("labels layer should be in index 2");
        }
    }
    fn nets_layer(&self) -> &Nets {
        if let SchematicLayerEnum::NetsLayer(nets) = &self.layers[0] {
            nets
        } else {
            panic!("nets layer should be in index 0");
        }
    }
    fn devices_layer(&self) -> &Devices {
        if let SchematicLayerEnum::DevicesLayer(devices) = &self.layers[1] {
            devices
        } else {
            panic!("devices layer should be in index 1");
        }
    }
    fn labels_layer(&self) -> &NetLabels {
        if let SchematicLayerEnum::NetLabelsLayer(labels) = &self.layers[2] {
            labels
        } else {
            panic!("labels layer should be in index 2");
        }
    }
    pub fn curpos_ssp(&self) -> SSPoint {
        self.curpos_ssp
    }
    fn update_cursor_vsp(&mut self, curpos_vsp: VSPoint) {
        self.curpos_ssp = curpos_vsp.round().cast().cast_unit();
        self.infobarstr = self.nets_layer().net_name_at(self.curpos_ssp);
        let mut ns = self.state.clone();
        match &self.state {
            CircuitSt::Wiring(Some((_nets_og, dijkstrast))) => {
                let mut nets = Nets::new();
                let mut dijkstrast_new = dijkstrast.clone();
                nets.route(
                    &mut dijkstrast_new,
                    &|prev, this, next| {
                        // do not go over ports at any cost
                        // do not go over NetVertex at any cost
                        // do not go over NetLabel at any cost
                        // do not make turn over NetEdge at any cost
                        // going straight cost 1
                        // going over symbol cost 10
                        // making turn cost 30
                        if self.devices_layer().any_port_occupy_ssp(next) {
                            // do not go over ports at any cost
                            return f32::INFINITY;
                        }
                        if self.nets_layer().any_vertex_occupy_ssp(next) {
                            // do not go over NetVertex at any cost
                            return f32::INFINITY;
                        }
                        if self.labels_layer().any_occupy_ssp(next) {
                            // do not go over NetLabel at any cost
                            return f32::INFINITY;
                        }
                        let is_turn = (prev.x != next.x) && (prev.y != next.y);
                        if is_turn && self.nets_layer().occupies_ssp(this) {
                            // next point is electrically occupied - do not use
                            return f32::INFINITY;
                        }
                        let mut ret = 1.0;
                        if self.devices_layer().occupies_ssp(next) {
                            // going through a device's graphical symbol
                            ret += 10.0;
                        }
                        if is_turn {
                            ret += 30.0;
                        }
                        ret
                    },
                    self.curpos_ssp,
                );
                ns = CircuitSt::Wiring(Some((Box::new(nets), dijkstrast_new)));
            }
            CircuitSt::Idle => {}
            _ => {}
        }
        self.state = ns;
    }

    // returns true if the coordinate is electrically significant
    fn electrically_occupies_ssp(&self, ssp: SSPoint) -> bool {
        self.nets_layer().occupies_ssp(ssp) || self.devices_layer().any_port_occupy_ssp(ssp)
    }
}

impl Drawable for Circuit {
    fn draw_persistent(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        self.nets_layer().draw_persistent(vct, vcscale, frame);
        self.devices_layer().draw_persistent(vct, vcscale, frame);
        self.labels_layer().draw_persistent(vct, vcscale, frame);
    }

    fn draw_selected(&self, _vct: VCTransform, _vcscale: f32, _frame: &mut Frame) {
        panic!("not intended for use");
    }

    fn draw_preview(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        match &self.state {
            CircuitSt::Wiring(Some((nets, _dijkst))) => {
                nets.draw_preview(vct, vcscale, frame);
            }
            CircuitSt::Idle => {}
            _ => {}
        }
    }
}

impl schematic::Content<CircuitAtom, Msg> for Circuit {
    fn curpos_update(&mut self, vsp: VSPoint) {
        self.update_cursor_vsp(vsp);
    }
    fn curpos_vsp(&self) -> VSPoint {
        self.curpos_ssp.cast().cast_unit()
    }
    fn bounds(&self) -> VSBox {
        let bbn = self.nets_layer().bounding_box();
        let bbi = self.devices_layer().bounding_box();
        let bbl = self.labels_layer().bounding_box();
        bbn.union(&bbi).union(&bbl)
    }
    fn intersects_vsb(&mut self, vsb: VSBox) -> HashSet<CircuitAtom> {
        let mut ret = HashSet::new();
        for seg in self.nets_layer_mut().intersects_vsbox(&vsb) {
            ret.insert(CircuitAtom::NetEdge(seg));
        }
        for rcrd in self.devices_layer().intersects_vsb(&vsb) {
            ret.insert(CircuitAtom::RcRDevice(rcrd));
        }
        for rcrl in self.labels_layer().intersects_vsb(&vsb) {
            ret.insert(CircuitAtom::RcRLabel(rcrl));
        }
        ret
    }
    fn contained_by(&mut self, vsb: VSBox) -> HashSet<CircuitAtom> {
        let mut ret = HashSet::new();
        for seg in self.nets_layer_mut().contained_by(&vsb) {
            ret.insert(CircuitAtom::NetEdge(seg));
        }
        for rcrd in self.devices_layer().contained_by(&vsb) {
            ret.insert(CircuitAtom::RcRDevice(rcrd));
        }
        for rcrl in self.labels_layer().contained_by(&vsb) {
            ret.insert(CircuitAtom::RcRLabel(rcrl));
        }
        ret
    }

    /// returns the first CircuitElement after skip which intersects with curpos_ssp, if any.
    /// count is updated to track the number of elements skipped over
    fn selectable(&mut self, vsp: VSPoint, skip: usize, count: &mut usize) -> Option<CircuitAtom> {
        if let Some(l) = self.labels_layer_mut().selectable(vsp, skip, count) {
            return Some(CircuitAtom::RcRLabel(l));
        }
        if let Some(e) = self.nets_layer().selectable(vsp, skip, count) {
            return Some(CircuitAtom::NetEdge(e));
        }
        if let Some(d) = self.devices_layer_mut().selectable(vsp, skip, count) {
            return Some(CircuitAtom::RcRDevice(d));
        }
        None
    }

    fn update(&mut self, msg: Msg) -> SchematicMsg<CircuitAtom> {
        let ret_msg = match msg {
            Msg::CanvasEvent(event) => {
                let mut state = self.state.clone();
                let mut ret_msg_tmp = SchematicMsg::None;
                const NO_MODIFIER: Modifiers = Modifiers::empty();
                match (&mut state, event) {
                    // wiring
                    (
                        CircuitSt::Idle,
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::W,
                            modifiers: NO_MODIFIER,
                        }),
                    ) => {
                        state = CircuitSt::Wiring(None);
                    }
                    (
                        CircuitSt::Wiring(opt_ws),
                        Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),
                    ) => {
                        let ssp = self.curpos_ssp();
                        let new_ws;
                        if let Some((g, dijkst)) = opt_ws {
                            // subsequent click
                            if ssp == dijkst.start() {
                                new_ws = None;
                            } else if self.electrically_occupies_ssp(ssp) {
                                let extra_vertices = self.devices_layer().ports_ssp();
                                self.nets_layer_mut().merge(g.as_ref(), &extra_vertices);
                                new_ws = None;
                            } else {
                                let extra_vertices = self.devices_layer().ports_ssp();
                                self.nets_layer_mut().merge(g.as_ref(), &extra_vertices);
                                new_ws = Some((Box::new(Nets::new()), DijkstraSt::new(ssp)));
                            }
                            ret_msg_tmp = SchematicMsg::ClearPassive;
                        } else {
                            // first click
                            new_ws = Some((Box::new(Nets::new()), DijkstraSt::new(ssp)));
                        }
                        state = CircuitSt::Wiring(new_ws);
                    }
                    // label
                    (
                        CircuitSt::Idle,
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::L,
                            modifiers: Modifiers::SHIFT,
                        }),
                    ) => {
                        let l = NetLabels::new_label();
                        ret_msg_tmp =
                            SchematicMsg::NewElement(SendWrapper::new(CircuitAtom::RcRLabel(l)));
                    }
                    // device placement
                    (
                        CircuitSt::Idle,
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::C,
                            modifiers: NO_MODIFIER,
                        }),
                    ) => {
                        let d = self.devices_layer_mut().new_cap();
                        ret_msg_tmp =
                            SchematicMsg::NewElement(SendWrapper::new(CircuitAtom::RcRDevice(d)));
                    }
                    (
                        CircuitSt::Idle,
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::L,
                            modifiers: NO_MODIFIER,
                        }),
                    ) => {
                        let d = self.devices_layer_mut().new_ind();
                        ret_msg_tmp =
                            SchematicMsg::NewElement(SendWrapper::new(CircuitAtom::RcRDevice(d)));
                    }
                    (
                        CircuitSt::Idle,
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::P,
                            modifiers: NO_MODIFIER,
                        }),
                    ) => {
                        let d = self.devices_layer_mut().new_pmos();
                        ret_msg_tmp =
                            SchematicMsg::NewElement(SendWrapper::new(CircuitAtom::RcRDevice(d)));
                    }
                    (
                        CircuitSt::Idle,
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::N,
                            modifiers: NO_MODIFIER,
                        }),
                    ) => {
                        let d = self.devices_layer_mut().new_nmos();
                        ret_msg_tmp =
                            SchematicMsg::NewElement(SendWrapper::new(CircuitAtom::RcRDevice(d)));
                    }
                    (
                        CircuitSt::Idle,
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::R,
                            modifiers: NO_MODIFIER,
                        }),
                    ) => {
                        let d = self.devices_layer_mut().new_res();
                        ret_msg_tmp =
                            SchematicMsg::NewElement(SendWrapper::new(CircuitAtom::RcRDevice(d)));
                    }
                    (
                        CircuitSt::Idle,
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::G,
                            modifiers: NO_MODIFIER,
                        }),
                    ) => {
                        let d = self.devices_layer_mut().new_gnd();
                        ret_msg_tmp =
                            SchematicMsg::NewElement(SendWrapper::new(CircuitAtom::RcRDevice(d)));
                    }
                    (
                        CircuitSt::Idle,
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::V,
                            modifiers: NO_MODIFIER,
                        }),
                    ) => {
                        let d = self.devices_layer_mut().new_vs();
                        ret_msg_tmp =
                            SchematicMsg::NewElement(SendWrapper::new(CircuitAtom::RcRDevice(d)));
                    }
                    (
                        CircuitSt::Idle,
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::I,
                            modifiers: NO_MODIFIER,
                        }),
                    ) => {
                        let d = self.devices_layer_mut().new_is();
                        ret_msg_tmp =
                            SchematicMsg::NewElement(SendWrapper::new(CircuitAtom::RcRDevice(d)));
                    }
                    (
                        CircuitSt::Idle,
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::D,
                            modifiers: NO_MODIFIER,
                        }),
                    ) => {
                        let d = self.devices_layer_mut().new_diode();
                        ret_msg_tmp =
                            SchematicMsg::NewElement(SendWrapper::new(CircuitAtom::RcRDevice(d)));
                    }
                    // state reset
                    (
                        _,
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::Escape,
                            modifiers: NO_MODIFIER,
                        }),
                    ) => {
                        state = CircuitSt::Idle;
                    }
                    _ => {}
                }
                self.state = state;
                ret_msg_tmp
            }
            Msg::NetList => {
                self.netlist();
                SchematicMsg::None
            }
            Msg::Wire => {
                self.state = CircuitSt::Wiring(None);
                SchematicMsg::None
            }
            Msg::DcOp(pkvecvaluesall) => {
                self.devices_layer_mut().op(&pkvecvaluesall);
                SchematicMsg::ClearPassive
            }
            Msg::Ac(pkvecvaluesall) => {
                self.devices_layer_mut().op(&pkvecvaluesall);
                SchematicMsg::ClearPassive
            }
        };
        ret_msg
    }

    fn move_elements(&mut self, elements: &mut HashSet<CircuitAtom>, sst: &VVTransform) {
        let mut nets = Vec::with_capacity(elements.len());
        for e in &*elements {
            match e {
                CircuitAtom::NetEdge(seg) => {
                    nets.push(seg.clone());
                }
                CircuitAtom::RcRDevice(d) => {
                    d.0.borrow_mut().transform(*sst);
                    // if moving an existing device, does nothing
                    // inserts the device if placing a new device
                    self.devices_layer_mut().insert(d.clone());
                }
                CircuitAtom::RcRLabel(l) => {
                    l.0.borrow_mut().transform(*sst);
                    // if moving an existing label, does nothing
                    // inserts the label if placing a new label
                    self.labels_layer_mut().insert(l.clone());
                }
            }
        }
        for n in nets {
            // remove netedge
            elements.remove(&CircuitAtom::NetEdge(n.clone()));
            self.nets_layer_mut()
                .graph
                .remove_edge(NetVertex(n.src), NetVertex(n.dst));

            // transform netedge and add
            let mut n1 = n.clone();
            n1.transform(*sst);
            elements.insert(CircuitAtom::NetEdge(n1.clone()));
            self.nets_layer_mut()
                .graph
                .add_edge(NetVertex(n1.src), NetVertex(n1.dst), n1);
        }
        self.prune();
    }

    fn copy_elements(&mut self, elements: &mut HashSet<CircuitAtom>, sst: &VVTransform) {
        let vec_ce = elements.clone().into_iter().collect::<Vec<_>>();
        elements.clear(); // clear the original elements
        for ce in vec_ce {
            match ce {
                CircuitAtom::NetEdge(seg) => {
                    let mut seg1 = seg.clone();
                    seg1.transform(*sst);
                    self.nets_layer_mut().graph.add_edge(
                        NetVertex(seg1.src),
                        NetVertex(seg1.dst),
                        seg1.clone(),
                    );
                    elements.insert(CircuitAtom::NetEdge(seg1));
                }
                CircuitAtom::RcRDevice(rcr) => {
                    //unwrap refcell
                    let mut device = (*rcr.0.borrow()).clone();
                    device.transform(*sst);

                    //build BaseElement
                    let d_refcell = RefCell::new(device);
                    let d_rc = Rc::new(d_refcell);
                    let rcr_device = RcRDevice(d_rc);
                    self.devices_layer_mut().insert(rcr_device.clone());
                    elements.insert(CircuitAtom::RcRDevice(rcr_device));
                }
                CircuitAtom::RcRLabel(rcl) => {
                    //unwrap refcell
                    let mut label = (*rcl.0.borrow()).clone();
                    label.transform(*sst);

                    //build BaseElement
                    let l_refcell = RefCell::new(label);
                    let l_rc = Rc::new(l_refcell);
                    let rcr_label = RcRLabel(l_rc);
                    self.labels_layer_mut().insert(rcr_label.clone());
                    elements.insert(CircuitAtom::RcRLabel(rcr_label));
                }
            }
        }
    }

    fn delete_elements(&mut self, elements: &HashSet<CircuitAtom>) {
        for e in elements {
            match e {
                CircuitAtom::NetEdge(e) => {
                    self.nets_layer_mut().delete_edge(e);
                }
                CircuitAtom::RcRDevice(d) => {
                    self.devices_layer_mut().delete_item(d);
                }
                CircuitAtom::RcRLabel(l) => {
                    self.labels_layer_mut().delete_item(l);
                }
            }
        }
        self.prune();
    }

    fn is_idle(&self) -> bool {
        matches!(self.state, CircuitSt::Idle)
    }
}

impl Circuit {
    /// create netlist for the current schematic and save it.
    pub fn netlist(&mut self) {
        let mut netlist = String::from("Netlist Created by Circe\n");
        netlist.push_str(&self.device_models.model_definitions());
        if self.devices_layer().get_set().is_empty() {
            // empty netlist
            netlist.push_str("V_0 0 n1 0"); // give it something so spice doesnt hang
            return;
        }

        self.prune();
        for d in self.devices_layer().get_set() {
            netlist.push_str(&d.0.borrow_mut().spice_line(self.nets_layer()));
        }
        netlist.push('\n');
        fs::write("netlist.cir", netlist.as_bytes()).expect("Unable to write file");
    }
    /// clear up nets graph: merging segments, cleaning up segment net names, etc.
    fn prune(&mut self) {
        let extra_vertices = self.devices_layer().ports_ssp();
        self.nets_layer_mut().prune(&extra_vertices);
    }
}
