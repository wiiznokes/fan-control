//! Circe
//! Schematic Capture for EDA with ngspice integration

use std::fmt::Debug;

mod analysis;
mod schematic;
mod transforms;

use analysis::plot_page::{PlotPage, PlotPageMsg};
use enum_dispatch::enum_dispatch;
use schematic::circuit::CircuitSchematicPage;
use schematic::symbols::SymbolDesignerPage;

use iced::{executor, Application, Command, Element, Settings, Theme};

use iced_aw::{TabLabel, Tabs};

use schematic::circuit::CircuitAtom;
use schematic::symbols::DesignerElement;

pub fn main() -> iced::Result {
    Circe::run(Settings {
        window: iced::window::Settings {
            size: (800, 500),
            ..iced::window::Settings::default()
        },
        antialiasing: true,
        ..Settings::default()
    })
}

/// main program
pub struct Circe {
    /// active tab index
    active_tab: usize,

    /// unstable - early development - for viewing plots of simulation results
    plot_view: PlotPage,
    /// circuits schematic for schematic capture
    circuit_schematic: CircuitSchematicPage,
    /// dev use only - for drawing custom devices or new device graphics
    symbol_designer: SymbolDesignerPage,
}

#[derive(Debug, Clone)]
pub enum Msg {
    DesignerMsg(schematic::symbols::DevicePageMsg),
    SchematicMsg(schematic::circuit::CircuitPageMsg),
    PlotViewMsg(analysis::plot_page::PlotPageMsg),
    TabSel(usize),
}

impl Application for Circe {
    type Executor = executor::Default;
    type Message = Msg;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Msg>) {
        (
            Circe {
                circuit_schematic: CircuitSchematicPage::default(),
                symbol_designer: SymbolDesignerPage::default(),
                plot_view: PlotPage::default(),
                active_tab: 1,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Circe")
    }

    fn update(&mut self, message: Msg) -> Command<Msg> {
        match message {
            Msg::TabSel(i) => {
                self.active_tab = i;

                // transfer simulation results from circuit_schematic to plot
                if let Some(traces) = self.circuit_schematic.traces.take() {
                    let msg = PlotPageMsg::Traces(traces);
                    self.plot_view.update(msg);
                }
            }
            Msg::DesignerMsg(device_designer_msg) => {
                self.symbol_designer.update(device_designer_msg);
            }
            Msg::PlotViewMsg(plot_msg) => {
                self.plot_view.update(plot_msg);
            }
            Msg::SchematicMsg(schematic_msg) => {
                self.circuit_schematic.update(schematic_msg);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Msg> {
        let schematic = self.circuit_schematic.view().map(Msg::SchematicMsg);
        let plot = self.plot_view.view().map(Msg::PlotViewMsg);
        let devices = self.symbol_designer.view().map(Msg::DesignerMsg);

        let tabs = Tabs::with_tabs(
            vec![
                (0, TabLabel::Text("Graphs".to_string()), plot),
                (1, TabLabel::Text("Schematic".to_string()), schematic),
                (2, TabLabel::Text("Device Designer".to_string()), devices),
            ],
            Msg::TabSel,
        );

        tabs.set_active_tab(&self.active_tab).into()
    }
}

trait IcedStruct<T> {
    fn update(&mut self, msg: T);
    fn view(&self) -> Element<T>;
}
use crate::transforms::VCTransform;
use iced::widget::canvas::Frame;
/// trait for element which can be drawn on canvas
#[enum_dispatch]
pub trait Drawable {
    fn draw_persistent(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame);
    fn draw_selected(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame);
    fn draw_preview(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame);
}
