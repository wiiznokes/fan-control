//! Designer Schematic GUI page
//! includes paramter editor, toolbar, and the canvas itself
//! for now, intended only as a dev tool for adding new devices

use crate::schematic::symbols::{Designer, DesignerElement, Msg};
use crate::schematic::viewport::VCTransformLockedAspect;
use crate::schematic::{self, Content};

use crate::schematic::viewport;
use crate::schematic::viewport::Viewport;
use crate::schematic::Schematic;
use crate::IcedStruct;
use iced::widget::row;
use iced::Element;

#[derive(Debug, Clone)]
pub enum DevicePageMsg {
    ViewportEvt(viewport::CompositeMsg<schematic::Msg<Msg, DesignerElement>>),
}

/// schematic
pub struct SymbolDesignerPage {
    /// viewport
    viewport:
        Viewport<Schematic<Designer, DesignerElement, Msg>, schematic::Msg<Msg, DesignerElement>>,
}

impl Default for SymbolDesignerPage {
    fn default() -> Self {
        let vct = VCTransformLockedAspect::identity()
            .pre_flip_y()
            .then_scale(10.0);
        SymbolDesignerPage {
            viewport: viewport::Viewport::new(1.0, 100.0, vct),
        }
    }
}

impl IcedStruct<DevicePageMsg> for SymbolDesignerPage {
    fn update(&mut self, msg: DevicePageMsg) {
        match msg {
            DevicePageMsg::ViewportEvt(msgs) => {
                self.viewport.update(msgs);
            }
        }
    }

    fn view(&self) -> Element<DevicePageMsg> {
        let str_ssp = format!(
            "x: {:02.2}; y: {:02.2}",
            self.viewport.content.content.curpos_vsp().x,
            self.viewport.content.content.curpos_vsp().y
        );
        let canvas = self.viewport.view().map(DevicePageMsg::ViewportEvt);
        let infobar = row![
            iced::widget::text(str_ssp)
                .size(16)
                .height(16)
                .vertical_alignment(iced::alignment::Vertical::Center),
            iced::widget::text(&format!("{:04.1}", self.viewport.vc_scale()))
                .size(16)
                .height(16)
                .vertical_alignment(iced::alignment::Vertical::Center),
        ]
        .spacing(10);

        let schematic = iced::widget::column![canvas, infobar];

        schematic.into()
    }
}
