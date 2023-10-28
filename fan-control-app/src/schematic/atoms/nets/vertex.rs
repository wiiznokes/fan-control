//! NetVertex, also used as keys to petgraph::GraphMap

use std::cmp::Ordering;

use crate::{
    transforms::{SSPoint, VCTransform},
    Drawable,
};
use iced::{
    widget::canvas::{stroke, Frame, LineCap, Path, Stroke},
    Color,
};

/// petgraph vertices weight.
/// In GraphMap, also serve as the keys.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NetVertex(pub SSPoint);

/// two vertices are equal if their coordinates are equal
impl PartialOrd for NetVertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// ord of points based on their x - y coordinate tuple
impl Ord for NetVertex {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.0.x, self.0.y).cmp(&(other.0.x, other.0.y))
    }
}

/// helper function for drawing the netedge on the canvas
fn draw_with(ssp: SSPoint, vct: VCTransform, frame: &mut Frame, stroke: Stroke) {
    let p = vct.transform_point(ssp.cast().cast_unit());
    let p = iced::Point::from([p.x, p.y]);
    let c = Path::line(p, p);
    frame.stroke(&c, stroke);
}

/// diameter of the solder point
const SOLDER_DIAMETER: f32 = 0.25;

/// zoom level below which solder point stops becoming smaller
const ZOOM_THRESHOLD: f32 = 5.0;

impl Drawable for NetVertex {
    fn draw_persistent(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        let solder_dia = self::SOLDER_DIAMETER;
        let zoom_thshld = self::ZOOM_THRESHOLD;
        let wire_stroke = Stroke {
            width: (solder_dia * vcscale).max(solder_dia * zoom_thshld),
            style: stroke::Style::Solid(Color::from_rgb(0.0, 0.8, 1.0)),
            line_cap: LineCap::Round,
            ..Stroke::default()
        };
        draw_with(self.0, vct, frame, wire_stroke);
    }
    fn draw_selected(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        let solder_dia = self::SOLDER_DIAMETER;
        let zoom_thshld = self::ZOOM_THRESHOLD;
        let wire_stroke = Stroke {
            width: (solder_dia * vcscale).max(solder_dia * zoom_thshld),
            style: stroke::Style::Solid(Color::from_rgb(1.0, 0.8, 0.0)),
            line_cap: LineCap::Round,
            ..Stroke::default()
        };
        draw_with(self.0, vct, frame, wire_stroke);
    }
    fn draw_preview(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame) {
        let solder_dia = self::SOLDER_DIAMETER;
        let zoom_thshld = self::ZOOM_THRESHOLD;
        let wire_stroke = Stroke {
            width: (solder_dia * vcscale).max(solder_dia * zoom_thshld),
            style: stroke::Style::Solid(Color::from_rgb(1.0, 1.0, 0.5)),
            line_cap: LineCap::Round,
            ..Stroke::default()
        };
        draw_with(self.0, vct, frame, wire_stroke);
    }
}
