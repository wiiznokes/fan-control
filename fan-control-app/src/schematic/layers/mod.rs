mod devices;
pub use devices::Devices;
mod net_labels;
use enum_dispatch::enum_dispatch;
pub use net_labels::NetLabels;
mod nets;
pub use nets::Nets;

use crate::transforms::VCTransform;
use crate::transforms::VSBox;
use crate::transforms::VSPoint;
use iced::widget::canvas::Frame;

pub use self::devices::DevicesLayer;
pub use self::net_labels::NetLabelsLayer;
pub use self::nets::DijkstraSt;
pub use self::nets::NetsLayer;

use super::atoms::SchematicAtom;

// draw persistent (draw tentative/selected leave to schematic)
// get bounds (for fit view)
// selectable/intersects/contained by (for selection purposes)
// Box<[Layer1, Layer2, ...]> - CircuitLayer Enum type with enum dispatch
// move/copy/del

// render ports on a separate layer so wires can be drawn under ports but above device symbols

#[enum_dispatch]
#[derive(Clone)]
pub enum SchematicLayerEnum {
    NetsLayer,
    // Ports,
    DevicesLayer,
    NetLabelsLayer,
    // LineSegs,
    // CirArcs,
    // Bounds,
}

// #[enum_dispatch]
pub trait SchematicLayerTrait<Atom>
where
    Atom: SchematicAtom,
{
    /// draws self's contents on frame
    fn draw_persistent(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame);
    /// returns bounding box containing all atoms in layer
    fn bounds(&self) -> VSBox;
    /// increments count for every atom over vsp, returns Some(atom) once count == skip
    fn selectable(&self, vsp: VSPoint, skip: usize, count: &mut usize) -> Option<Atom>;
    /// returns slice of all atoms in layer which intersect with closed area defined by vsb
    fn intersect(&self, vsb: &VSBox) -> Box<[Atom]>;
    /// returns slice of all atoms in layer which fit in open area defined by vsb
    fn contained(&self, vsb: &VSBox) -> Box<[Atom]>;
    /// place the device in layer - replace existing if atom equates to existing, or adds new if not
    fn place(&mut self, atom: Atom);
    /// delete the specified atom if it exists
    fn delete(&mut self, atom: &Atom);
}
