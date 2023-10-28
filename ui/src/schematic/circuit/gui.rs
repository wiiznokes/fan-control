//! Circuit Schematic GUI page
//! includes paramter editor, toolbar, and the canvas itself
//! waiting on multiple windows support for new device instance menu

use crate::schematic;
use crate::schematic::circuit::{Circuit, CircuitAtom, Msg};
use crate::schematic::viewport::CompositeMsg;
use crate::schematic::viewport::VCTransformLockedAspect;
use crate::transforms::VSPoint;

use crate::schematic::viewport;
use crate::schematic::viewport::Viewport;
use crate::schematic::Schematic;
use crate::IcedStruct;
use iced::keyboard::Modifiers;
use iced::widget::canvas::Event;
use iced::widget::{row, text, text_input};
use iced::{Element, Length};
use std::sync::{Arc, Mutex};

use colored::Colorize;
use paprika::*;

/// Spice Manager to facillitate interaction with NgSpice
#[derive(Debug, Default)]
struct SpManager {
    vecvals: Mutex<Vec<PkVecvaluesall>>,
    vecinfo: Option<PkVecinfoall>,
}

impl SpManager {
    fn new() -> Self {
        SpManager::default()
    }
}

#[allow(unused_variables)]
impl paprika::PkSpiceManager for SpManager {
    fn cb_send_char(&mut self, msg: String, id: i32) {
        let opt = msg.split_once(' ');
        let (token, msgs) = match opt {
            Some(tup) => (tup.0, tup.1),
            None => (msg.as_str(), msg.as_str()),
        };
        let msgc = match token {
            "stdout" => msgs.green(),
            "stderr" => msgs.red(),
            _ => msg.magenta().strikethrough(),
        };
        println!("{}", msgc);
    }
    fn cb_send_stat(&mut self, msg: String, id: i32) {
        println!("{}", msg.blue());
    }
    fn cb_ctrldexit(&mut self, status: i32, is_immediate: bool, is_quit: bool, id: i32) {}
    fn cb_send_init(&mut self, pkvecinfoall: PkVecinfoall, id: i32) {
        self.vecinfo = Some(pkvecinfoall);
    }
    fn cb_send_data(&mut self, pkvecvaluesall: PkVecvaluesall, count: i32, id: i32) {
        // this is called every simulation step when running tran
        self.vecvals.try_lock().unwrap().push(pkvecvaluesall);
    }
    fn cb_bgt_state(&mut self, is_fin: bool, id: i32) {}
}

#[derive(Debug, Clone)]
pub enum CircuitPageMsg {
    ViewportEvt(viewport::CompositeMsg<schematic::Msg<Msg, CircuitAtom>>),
    ParamChanged(String),
    ParamSubmit,
    HzChanged(String),
    StepChanged(String),
    TranChanged(String),
}

/// schematic
pub struct CircuitSchematicPage {
    /// viewport
    viewport: Viewport<Schematic<Circuit, CircuitAtom, Msg>, schematic::Msg<Msg, CircuitAtom>>,

    /// tentative net name, used only for display in the infobar
    net_name: Option<String>,

    /// spice manager
    spmanager: Arc<SpManager>,
    /// ngspice library
    lib: PkSpice<SpManager>,
    /// traces from certain simulations e.g. transient
    pub traces: Option<Vec<Vec<VSPoint>>>,

    /// active device - some if only 1 device selected, otherwise is none
    active_element: Option<CircuitAtom>,
    /// parameter editor text
    param: String,
    /// ac simulation frequency (hertz)
    ac_hz: String,
    /// tran simulation step size (seconds)
    tran_step: String,
    /// tran simulation end time (seconds)
    tran_end: String,
}
impl Default for CircuitSchematicPage {
    fn default() -> Self {
        let spmanager = Arc::new(SpManager::new());
        let mut lib;
        #[cfg(target_family = "windows")]
        {
            lib = PkSpice::<SpManager>::new(std::ffi::OsStr::new("ngspice.dll")).unwrap();
        }
        #[cfg(target_os = "macos")]
        {
            // retrieve libngspice.dylib from the following possible directories
            let ret = std::process::Command::new("find")
                .args(&["/usr/lib", "/usr/local/lib"])
                .arg("-name")
                .arg("*libngspice.dylib")
                .stdout(std::process::Stdio::piped())
                .output()
                .unwrap_or_else(|_| {
                    eprintln!("Error: Could not find libngspice.dylib. Make sure it is installed.");
                    std::process::exit(1);
                });
            let path = String::from_utf8(ret.stdout).unwrap();
            lib = PkSpice::<SpManager>::new(&std::ffi::OsString::from(path.trim())).unwrap();
        }
        #[cfg(target_os = "linux")]
        {
            // dynamically retrieves libngspice from system
            let ret = std::process::Command::new("sh")
                .arg("-c")
                .arg("ldconfig -p | grep ngspice | awk '/.*libngspice.so$/{print $4}'")
                .stdout(std::process::Stdio::piped())
                .output()
                .unwrap_or_else(|_| {
                    eprintln!("Error: Could not find libngspice. Make sure it is installed.");
                    std::process::exit(1);
                });

            let path = String::from_utf8(ret.stdout).unwrap();
            lib = PkSpice::<SpManager>::new(&std::ffi::OsString::from(path.trim())).unwrap();
        }
        lib.init(Some(spmanager.clone()));
        let vct = VCTransformLockedAspect::identity()
            .pre_flip_y()
            .then_scale(10.0);
        CircuitSchematicPage {
            viewport: viewport::Viewport::new(1.0, 100.0, vct),
            net_name: Default::default(),
            active_element: Default::default(),
            param: Default::default(),
            spmanager,
            lib,
            traces: None,
            ac_hz: String::from("60"),
            tran_step: String::from("10u"),
            tran_end: String::from("1m"),
        }
    }
}

impl IcedStruct<CircuitPageMsg> for CircuitSchematicPage {
    fn update(&mut self, msg: CircuitPageMsg) {
        const NO_MODIFIER: Modifiers = Modifiers::empty();
        match msg {
            CircuitPageMsg::ParamChanged(s) => {
                self.param = s;
            }
            CircuitPageMsg::ParamSubmit => {
                if let Some(ad) = &self.active_element {
                    match ad {
                        CircuitAtom::NetEdge(_) => {}
                        CircuitAtom::RcRDevice(d) => {
                            d.0.borrow_mut()
                                .class_mut()
                                .set_raw_param(self.param.clone());
                        }
                        CircuitAtom::RcRLabel(l) => {
                            l.0.borrow_mut().set_name(self.param.clone());
                        }
                    }
                    self.viewport.passive_cache.clear();
                }
            }
            CircuitPageMsg::ViewportEvt(msgs) => {
                match msgs.content_msg {
                    schematic::Msg::Event(
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::Space,
                            modifiers: NO_MODIFIER,
                        }),
                        _,
                    ) => {
                        self.viewport.update(CompositeMsg {
                            content_msg: schematic::Msg::ContentMsg(Msg::NetList),
                            viewport_msg: viewport::Msg::None,
                        });
                        self.lib.command("source netlist.cir"); // results pointer array starts at same address
                        self.lib.command("op"); // ngspice recommends sending in control statements separately, not as part of netlist
                        if let Some(pkvecvaluesall) =
                            self.spmanager.vecvals.try_lock().unwrap().pop()
                        {
                            self.viewport.update(CompositeMsg {
                                content_msg: schematic::Msg::ContentMsg(Msg::DcOp(
                                    pkvecvaluesall.clone(),
                                )),
                                viewport_msg: viewport::Msg::None,
                            });
                        }
                    }
                    schematic::Msg::Event(
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::Space,
                            modifiers: Modifiers::CTRL,
                        }),
                        _,
                    ) => {
                        self.viewport.update(CompositeMsg {
                            content_msg: schematic::Msg::ContentMsg(Msg::NetList),
                            viewport_msg: viewport::Msg::None,
                        });
                        self.lib.command("source netlist.cir"); // results pointer array starts at same address
                        self.lib
                            .command(&format!("ac lin 0 {} {}", self.ac_hz, self.ac_hz)); // ngspice recommends sending in control statements separately, not as part of netlist
                        if let Some(pkvecvaluesall) =
                            self.spmanager.vecvals.try_lock().unwrap().pop()
                        {
                            self.viewport.update(CompositeMsg {
                                content_msg: schematic::Msg::ContentMsg(Msg::Ac(
                                    pkvecvaluesall.clone(),
                                )),
                                viewport_msg: viewport::Msg::None,
                            });
                        }
                    }
                    schematic::Msg::Event(
                        Event::Keyboard(iced::keyboard::Event::KeyPressed {
                            key_code: iced::keyboard::KeyCode::T,
                            modifiers: iced::keyboard::Modifiers::SHIFT,
                        }),
                        _,
                    ) => {
                        self.viewport.update(CompositeMsg {
                            content_msg: schematic::Msg::ContentMsg(Msg::NetList),
                            viewport_msg: viewport::Msg::None,
                        });
                        self.lib.command("source netlist.cir"); // results pointer array starts at same address
                        self.spmanager.vecvals.try_lock().unwrap().clear();
                        self.lib
                            .command(&format!("tran {} {}", self.tran_step, self.tran_end)); // ngspice recommends sending in control statements separately, not as part of netlist

                        let pk_results = self.spmanager.vecvals.try_lock().unwrap();

                        let trace_count = pk_results.first().unwrap().count as usize;
                        let mut results: Vec<Vec<VSPoint>> = Vec::with_capacity(trace_count);
                        for _ in 0..trace_count {
                            results.push(Vec::with_capacity(pk_results.len()));
                        }

                        let x_i = pk_results
                            .first()
                            .unwrap()
                            .vecsa
                            .iter()
                            .position(|x| x.name == "time")
                            .unwrap();
                        for step_val in pk_results.iter() {
                            for (trace_i, trace_val) in step_val.vecsa.iter().enumerate() {
                                results[trace_i].push(VSPoint::new(
                                    step_val.vecsa[x_i].creal as f32,
                                    trace_val.creal as f32,
                                ));
                            }
                        }
                        results.remove(x_i);

                        self.traces = Some(results);
                    }
                    _ => {
                        self.viewport.update(msgs);
                    }
                }

                match &self.viewport.content.active_element {
                    Some(ae) => {
                        self.active_element = Some(ae.clone());
                        match ae {
                            CircuitAtom::NetEdge(_) => {}
                            CircuitAtom::RcRDevice(d) => {
                                self.param = d.0.borrow().class().param_summary();
                            }
                            CircuitAtom::RcRLabel(l) => {
                                self.param = l.0.borrow().read().to_string();
                            }
                        }
                    }
                    None => self.param = String::from(""),
                }

                self.net_name = self.viewport.content.content.infobarstr.take();
            }
            CircuitPageMsg::HzChanged(s) => self.ac_hz = s,
            CircuitPageMsg::StepChanged(s) => self.tran_step = s,
            CircuitPageMsg::TranChanged(s) => self.tran_end = s,
        }
    }

    fn view(&self) -> Element<CircuitPageMsg> {
        let str_ssp = format!(
            "x: {}; y: {}",
            self.viewport.content.content.curpos_ssp().x,
            self.viewport.content.content.curpos_ssp().y
        );
        let canvas = self.viewport.view().map(CircuitPageMsg::ViewportEvt);
        let infobar = row![
            iced::widget::text(str_ssp)
                .size(16)
                .height(16)
                .vertical_alignment(iced::alignment::Vertical::Center),
            iced::widget::text(&format!("{:04.1}", self.viewport.vc_scale()))
                .size(16)
                .height(16)
                .vertical_alignment(iced::alignment::Vertical::Center),
            iced::widget::text(self.net_name.as_deref().unwrap_or_default())
                .size(16)
                .height(16)
                .vertical_alignment(iced::alignment::Vertical::Center),
        ]
        .spacing(10);
        let toolbar = row![
            // button("wire").on_press(CircuitPageMsg::ViewportEvt(viewport::CompositeMsg {
            //     content_msg: schematic::Msg::ContentMsg(Msg::Wire),
            //     viewport_msg: viewport::Msg::None,
            // })),
            text("ac freq (Hz): "),
            text_input("", &self.ac_hz)
                .width(50)
                .on_input(CircuitPageMsg::HzChanged),
            text("tran step (S): "),
            text_input("", &self.tran_step)
                .width(50)
                .on_input(CircuitPageMsg::StepChanged),
            text("tran end (S): "),
            text_input("", &self.tran_end)
                .width(50)
                .on_input(CircuitPageMsg::TranChanged),
            text("Param: "),
            text_input("", &self.param)
                .width(iced::Length::Fill)
                .on_input(CircuitPageMsg::ParamChanged)
                .on_submit(CircuitPageMsg::ParamSubmit),
        ]
        .width(Length::Fill);

        let schematic = iced::widget::column![canvas, infobar, toolbar];

        schematic.into()
    }
}
