//! types and constants facillitating geometry and transforms

use euclid::Point2D;
use iced::Point as IcedPoint;
use serde::{Deserialize, Serialize};

/// PhantomData tag used to denote the patch of screen being drawn on (f32)
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize,
)]
pub struct CanvasSpace;

/// PhantomData tag used to denote the f32 space on which the schematic is drawn
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize,
)]
pub struct ViewportSpace;

/// PhantomData tag used to denote the i16 space in which the schematic exists
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize,
)]
pub struct SchematicSpace;

/// CanvasSpace Point
pub type CSPoint = euclid::Point2D<f32, CanvasSpace>;
/// ViewportSpace Point
pub type VSPoint = euclid::Point2D<f32, ViewportSpace>;
/// SchematicSpace Point
pub type SSPoint = euclid::Point2D<i16, SchematicSpace>;

/// CanvasSpace Box
pub type CSBox = euclid::Box2D<f32, CanvasSpace>;
/// ViewportSpace Box
pub type VSBox = euclid::Box2D<f32, ViewportSpace>;
/// SchematicSpace Box
pub type SSBox = euclid::Box2D<i16, SchematicSpace>;

pub trait VSBoxExt {
    fn inclusive_contains(&self, p: VSPoint) -> bool;
}

impl VSBoxExt for VSBox {
    #[inline]
    fn inclusive_contains(&self, p: VSPoint) -> bool {
        self.min.x <= p.x && p.x <= self.max.x && self.min.y <= p.y && p.y <= self.max.y
    }
}

/// CanvasSpace Vector
pub type CSVec = euclid::Vector2D<f32, CanvasSpace>;
/// ViewportSpace Vector
pub type VSVec = euclid::Vector2D<f32, ViewportSpace>;
/// SchematicSpace Vector
#[allow(unused)]
pub type SSVec = euclid::Vector2D<i16, SchematicSpace>;

/// viewport to canvas space transform
pub type VCTransform = euclid::Transform2D<f32, ViewportSpace, CanvasSpace>;
/// canvas to viewport space transform
pub type CVTransform = euclid::Transform2D<f32, CanvasSpace, ViewportSpace>;
/// schematic space transform
pub type SSTransform = euclid::Transform2D<i16, SchematicSpace, SchematicSpace>;
/// viewport space transform
pub type VVTransform = euclid::Transform2D<f32, ViewportSpace, ViewportSpace>;

/// 90 deg clockwise rotation transform
pub const SST_CWR: SSTransform = SSTransform::new(0, -1, 1, 0, 0, 0);

/// 90 deg counter clockwise rotation transform
pub const SST_CCWR: SSTransform = SSTransform::new(0, 1, -1, 0, 0, 0);

/// mirror along X axis
pub const SST_XMIR: SSTransform = SSTransform::new(-1, 0, 0, 1, 0, 0);

/// mirror along Y axis
pub const SST_YMIR: SSTransform = SSTransform::new(1, 0, 0, -1, 0, 0);

/// converts SSTransform to SSTransform so that it can be composited with VCTransform
pub fn sst_to_vvt(sst: SSTransform) -> VVTransform {
    sst.cast().with_destination().with_source()
}

/// converts SSTransform to SSTransform so that it can be composited with VCTransform
pub fn vvt_to_sst(vvt: VVTransform) -> SSTransform {
    SSTransform {
        m11: vvt.m11.round() as i16,
        m12: vvt.m12.round() as i16,
        m21: vvt.m21.round() as i16,
        m22: vvt.m22.round() as i16,
        m31: vvt.m31.round() as i16,
        m32: vvt.m32.round() as i16,
        _unit: std::marker::PhantomData,
    }
}

/// Newtype for working with iced::Point and euclid::Point2D s
#[derive(Debug, Copy, Clone)]
pub struct Point(CSPoint);

impl From<IcedPoint> for Point {
    fn from(src: IcedPoint) -> Self {
        Point(Point2D::new(src.x, src.y))
    }
}

impl From<Point> for IcedPoint {
    fn from(src: Point) -> Self {
        IcedPoint::new(src.0.x, src.0.y)
    }
}

impl From<Point> for CSPoint {
    fn from(src: Point) -> Self {
        src.0
    }
}

impl From<CSPoint> for Point {
    fn from(src: CSPoint) -> Self {
        Self(src)
    }
}
