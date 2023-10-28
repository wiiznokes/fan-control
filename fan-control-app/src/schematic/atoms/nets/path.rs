//! Path, also used edge weights in petgraph::GraphMap.

/// A Path to help pathfinding
#[derive(Clone, Debug, Default)]
pub struct PathWeight {
    /// the cost of traversing this edge
    pub cost: f32,
}

// /// two edges are equal if their source and destination pts are equal
// impl PartialEq for NetEdge {
//     fn eq(&self, other: &Self) -> bool {
//         self.src == other.src && self.dst == other.dst
//     }
// }

// /// hash based on the source and destination points
// impl std::hash::Hash for NetEdge {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.src.hash(state);
//         self.dst.hash(state);
//     }
// }
