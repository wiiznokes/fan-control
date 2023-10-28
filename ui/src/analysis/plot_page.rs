//! Schematic GUI page
//! includes paramter editor, toolbar, and the canvas itself

use crate::analysis::plot::{ChartElement, Msg as PlotMsg, Plot};
use crate::analysis::viewport::Content;
use crate::analysis::viewport::VCTransformFreeAspect;
use crate::analysis::{plot, viewport};
use crate::transforms::VSPoint;

use crate::IcedStruct;
use iced::widget::row;
use iced::Element;

#[derive(Debug, Clone)]
pub enum PlotPageMsg {
    ViewportEvt(viewport::CompositeMsg<plot::Msg>),
    Traces(Vec<Vec<VSPoint>>),
}

/// schematic
pub struct PlotPage {
    /// viewport
    viewport: viewport::Viewport<Plot<ChartElement>, plot::Msg>,
}
impl Default for PlotPage {
    fn default() -> Self {
        let vct = VCTransformFreeAspect::identity()
            .pre_flip_y()
            .then_scale(10.0, 10.0);
        PlotPage {
            viewport: viewport::Viewport::new(1.0, f32::EPSILON, f32::MAX, vct),
        }
    }
}

impl IcedStruct<PlotPageMsg> for PlotPage {
    fn update(&mut self, msg: PlotPageMsg) {
        match msg {
            PlotPageMsg::ViewportEvt(msgs) => {
                self.viewport.update(msgs);
            }
            PlotPageMsg::Traces(traces) => {
                let content_msg = PlotMsg::Traces(traces);
                self.viewport.content.update(content_msg);
            }
        }
    }

    fn view(&self) -> Element<PlotPageMsg> {
        let str_ssp = format!(
            "curpos: x: {:.2e}; y: {:.2e}",
            self.viewport.curpos_vsp().x,
            self.viewport.curpos_vsp().y
        );
        let str_xyscales = format!(
            "scale: x: {:.2e}; y: {:.2e}",
            self.viewport.vct().x_scale(),
            self.viewport.vct().y_scale(),
        );

        let canvas = self.viewport.view().map(PlotPageMsg::ViewportEvt);
        let infobar = row![
            iced::widget::text(str_ssp)
                .size(16)
                .height(16)
                .vertical_alignment(iced::alignment::Vertical::Center),
            iced::widget::text(str_xyscales)
                .size(16)
                .height(16)
                .vertical_alignment(iced::alignment::Vertical::Center),
        ]
        .spacing(10);

        let schematic = iced::widget::column![canvas, infobar,];

        schematic.into()
    }
}
