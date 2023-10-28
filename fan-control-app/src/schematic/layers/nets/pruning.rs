//! logic to simplify nets through merging overlapped edges, etc.
//!
//!

use std::rc::Rc;

use petgraph::algo::tarjan_scc;

use crate::{
    schematic::atoms::{NetEdge, NetVertex},
    transforms::SSPoint,
};

use super::Nets;

/// bisect all edges in nets at all coordinate in coords
/// also merges overlapping edges
fn bisect_merge<I>(nets: &mut Nets, coords: I)
where
    I: IntoIterator<Item = SSPoint>,
{
    for this in coords {
        let mut colliding_edges = vec![];
        // colliding edges detection excludes endpoints
        for e in nets.graph.all_edges() {
            if e.2.intersects_ssp(this.cast().cast_unit()) {
                colliding_edges.push((e.0, e.1, e.2.label.clone()));
            }
        }
        for e in colliding_edges {
            nets.graph.remove_edge(e.0, e.1);
            nets.graph.add_edge(
                e.0,
                NetVertex(this),
                NetEdge {
                    src: e.0 .0,
                    dst: this,
                    label: e.2.clone(),
                    interactable: NetEdge::interactable(e.0 .0, this),
                },
            );
            nets.graph.add_edge(
                e.1,
                NetVertex(this),
                NetEdge {
                    src: e.1 .0,
                    dst: this,
                    label: e.2,
                    interactable: NetEdge::interactable(e.1 .0, this),
                },
            );
        }
    }
}

/// removes net vertices at coord in coords if redundant
/// designed to be called after `bisect_merge`
fn cull_redundant_vertices<I>(nets: &mut Nets, coords: I)
where
    I: IntoIterator<Item = SSPoint>,
{
    for this in coords {
        let this_vertex = NetVertex(this);
        let neighbor_vertices: Box<[NetVertex]> = nets
            .graph
            .neighbors(this_vertex)
            .filter(|&neighbor| neighbor != this_vertex) // ignore connections to self
            .collect();

        match neighbor_vertices.len() {
            0 => {
                // isolated vertex with no neighbor
                nets.graph.remove_node(NetVertex(this));
            }
            2 => {
                // exactly two neighbors
                let delta = neighbor_vertices[1].0 - neighbor_vertices[0].0;
                let del = this - neighbor_vertices[0].0;
                if delta.cross(del) != 0 {
                    // this is a corner vertex
                    continue;
                }
                // replace two connecting edges with single edge
                let label = nets
                    .graph
                    .edges(NetVertex(this))
                    .next()
                    .unwrap()
                    .2
                    .label
                    .clone();
                let src = neighbor_vertices[0];
                let dst = neighbor_vertices[1];
                let ew = NetEdge {
                    src: src.0,
                    dst: dst.0,
                    label,
                    interactable: NetEdge::interactable(src.0, dst.0),
                };
                if ew.intersects_ssp(this) {
                    nets.graph.remove_node(NetVertex(this));
                    nets.graph.add_edge(src, dst, ew);
                }
            }
            _ => {}
        }
    }
}

/// finds an appropriate net name and assigns it to all edge in edges.
fn unify_labels(
    nets: &mut Nets,
    edges: Vec<(NetVertex, NetVertex)>,
    taken_net_names: &[Rc<String>],
) -> Rc<String> {
    let mut label = None;
    // get smallest untaken of existing labels, if any
    for tup in &edges {
        if let Some(ew) = nets.graph.edge_weight(tup.0, tup.1) {
            if let Some(label1) = &ew.label {
                if taken_net_names.contains(label1) {
                    continue;
                }
                if label.is_none() || label1 < label.as_ref().unwrap() {
                    label = Some(label1.clone());
                }
            }
        }
    }
    // if no edge is labeled, create a new label
    if label.is_none() {
        label = Some(nets.label_manager.new_label());
    }
    // assign label to all edges
    for tup in edges {
        if let Some(ew) = nets.graph.edge_weight_mut(tup.0, tup.1) {
            ew.label = label.clone();
        }
    }
    label.unwrap()
}

/// this function is called whenever schematic is changed. Ensures all connected nets have the same net name, overlapping segments are merged, etc.
/// extra_vertices are coordinates where net segments should be bisected (device ports)
pub fn prune(nets: &mut Nets, port_coords: &[SSPoint]) {
    let net_vertices: Box<[SSPoint]> = nets.graph.nodes().map(|nv| nv.0).collect();

    // bisect/merge edges
    bisect_merge(nets, net_vertices.iter().chain(port_coords.iter()).copied());

    // remove non-corner vertices, leave vertices on ports alone
    cull_redundant_vertices(nets, net_vertices.iter().copied());

    // add vertices at port coords if not already in graph - need to add edge to itself so netlisting connects overlapping ports
    for p in port_coords {
        nets.graph
            .add_edge(NetVertex(*p), NetVertex(*p), NetEdge::new_from_pts(*p, *p));
    }

    // assign net names
    // for each subnet
    // unify labels - give vector of taken labels
    let subgraph_vertices = tarjan_scc(&*nets.graph); // this finds the subnets
    let mut taken_net_names = vec![];
    for vertices in subgraph_vertices {
        let edges = nets.nodes_to_edge_nodes(vertices);
        taken_net_names.push(unify_labels(nets, edges, &taken_net_names));
    }
}
