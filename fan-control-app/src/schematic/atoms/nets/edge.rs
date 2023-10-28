//! NetEdge, also used edge weights in petgraph::GraphMap.

use std::rc::Rc;

use crate::{
    schematic::{
        interactable::{Interactable, Interactive},
        SchematicAtom,
    },
    transforms::{vvt_to_sst, SSPoint, VCTransform, VSBox, VSPoint, VVTransform},
    Drawable,
};

use iced::{
    widget::canvas::{stroke, Frame, LineCap, LineDash, Path, Stroke},
    Color,
};

/// A NetEdge represents a segment of wiring.
/// It exists in the program as an edge weight for petgraph::Graph.
#[derive(Clone, Debug, Default)]
pub struct NetEdge {
    /// source point of edge segment
    pub src: SSPoint,
    /// destination point of edge segment
    pub dst: SSPoint,
    /// interactable associated with this edge segment
    pub interactable: Interactable,
    /// auto generated net name associated with this edge segment
    pub label: Option<Rc<String>>,
}

/// two edges are equal if their source and destination pts are equal
impl PartialEq for NetEdge {
    fn eq(&self, other: &Self) -> bool {
        self.src == other.src && self.dst == other.dst
    }
}

impl Eq for NetEdge {}

/// hash based on the source and destination points
impl std::hash::Hash for NetEdge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.src.hash(state);
        self.dst.hash(state);
    }
}

impl NetEdge {
    /// create a new net edge from src and dst
    pub fn new_from_pts(src: SSPoint, dst: SSPoint) -> Self {
        NetEdge {
            src,
            dst,
            interactable: Self::interactable(src, dst),
            label: None,
        }
    }
    /// creates an interactable based on source and destination points, with settable 'tentative' flag
    pub fn interactable(src: SSPoint, dst: SSPoint) -> Interactable {
        Interactable {
            bounds: NetEdge::bounds_from_pts(src, dst),
        }
    }
    /// creates a bound based on source and destination points - return value is guaranteed to have positive area
    pub fn bounds_from_pts(src: SSPoint, dst: SSPoint) -> VSBox {
        VSBox::from_points([src.cast().cast_unit(), dst.cast().cast_unit()]).inflate(0.1, 0.1)
    }
    // /// checks if argument VSPoint lies on the edge
    // pub fn intersects_vsp(&self, vsp: VSPoint) -> bool {
    //     self.interactable.contains_vsp(vsp)
    // }
    /// checks if argument SSPoint lies on the edge (excludes source and destination points)
    pub fn intersects_ssp(&self, ssp: SSPoint) -> bool {
        self.interactable.contains_ssp(ssp) && self.src != ssp && self.dst != ssp
    }
}

impl Interactive for NetEdge {
    /// transform the edge based on SSTransform argument
    fn transform(&mut self, vvt: VVTransform) {
        let sst = vvt_to_sst(vvt);
        (self.src, self.dst) = (sst.transform_point(self.src), sst.transform_point(self.dst));
        self.interactable.bounds = NetEdge::bounds_from_pts(self.src, self.dst);
    }
}

/// helper function for drawing the netedge on the canvas
fn draw_with(src: SSPoint, dst: SSPoint, vct: VCTransform, frame: &mut Frame, stroke: Stroke) {
    let psrcv = vct.transform_point(src.cast().cast_unit());
    let pdstv = vct.transform_point(dst.cast().cast_unit());
    let c = Path::line(
        iced::Point::from([psrcv.x, psrcv.y]),
        iced::Point::from([pdstv.x, pdstv.y]),
    );
    frame.stroke(&c, stroke);
}

/// width of the wire segment
const WIRE_WIDTH: f32 = 3.0;

impl Drawable for NetEdge {
    fn draw_persistent(&self, vct: VCTransform, _vcscale: f32, frame: &mut Frame) {
        // let wire_width = self::WIRE_WIDTH;
        // let zoom_thshld = self::ZOOM_THRESHOLD;
        let wire_stroke = Stroke {
            width: self::WIRE_WIDTH,
            style: stroke::Style::Solid(Color::from_rgb(0.0, 0.8, 1.0)),
            line_cap: LineCap::Round,
            ..Stroke::default()
        };
        draw_with(self.src, self.dst, vct, frame, wire_stroke);
    }
    fn draw_selected(&self, vct: VCTransform, _vcscale: f32, frame: &mut Frame) {
        // let wire_width = self::WIRE_WIDTH;
        // let zoom_thshld = self::ZOOM_THRESHOLD;
        let wire_stroke = Stroke {
            width: self::WIRE_WIDTH,
            style: stroke::Style::Solid(Color::from_rgb(1.0, 0.8, 0.0)),
            line_cap: LineCap::Round,
            ..Stroke::default()
        };
        draw_with(self.src, self.dst, vct, frame, wire_stroke);
    }
    fn draw_preview(&self, vct: VCTransform, _vcscale: f32, frame: &mut Frame) {
        // let wire_width = self::WIRE_WIDTH;
        // let zoom_thshld = self::ZOOM_THRESHOLD;
        let wire_stroke = Stroke {
            width: self::WIRE_WIDTH,
            style: stroke::Style::Solid(Color::from_rgb(1.0, 1.0, 0.5)),
            line_cap: LineCap::Butt,
            line_dash: LineDash {
                segments: &[4.0],
                offset: 0,
            },
            ..Stroke::default()
        };
        draw_with(self.src, self.dst, vct, frame, wire_stroke);
    }
}

impl SchematicAtom for NetEdge {
    fn contains_vsp(&self, vsp: VSPoint) -> bool {
        self.interactable.contains_vsp(vsp)
    }
    fn bounding_box(&self) -> crate::transforms::VSBox {
        self.interactable.bounds
    }
}
