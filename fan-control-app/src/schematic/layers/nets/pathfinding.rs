//! Variation of Dijkstra's algo adapted for EDA wiring.
//!  
//! Allows assigning cost to making turns

use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{BinaryHeap, HashMap, HashSet};

use std::cmp::Ordering;

use crate::transforms::{SSPoint, SSVec};

/// private struct copied from petgraph
/// `MinScored<K, T>` holds a score `K` and a scored object `T` in
/// a pair for use with a `BinaryHeap`.
///
/// `MinScored` compares in reverse order by the score, so that we can
/// use `BinaryHeap` as a min-heap to extract the score-value pair with the
/// least score.
///
/// **Note:** `MinScored` implements a total order (`Ord`), so that it is
/// possible to use float types as scores.
#[derive(Copy, Clone, Debug)]
pub struct MinScored<K, T>(pub K, pub T);

impl<K: PartialOrd, T> PartialEq for MinScored<K, T> {
    #[inline]
    fn eq(&self, other: &MinScored<K, T>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<K: PartialOrd, T> Eq for MinScored<K, T> {}

impl<K: PartialOrd, T> PartialOrd for MinScored<K, T> {
    #[inline]
    fn partial_cmp(&self, other: &MinScored<K, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: PartialOrd, T> Ord for MinScored<K, T> {
    #[inline]
    fn cmp(&self, other: &MinScored<K, T>) -> Ordering {
        let a = &self.0;
        let b = &other.0;
        if a == b {
            Ordering::Equal
        } else if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else if a.ne(a) && b.ne(b) {
            // these are the NaN cases
            Ordering::Equal
        } else if a.ne(a) {
            // Order NaN less, so that it is last in the MinScore order
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

/// Modified Dijkstra specialized for schematic wiring
pub fn wiring_pathfinder<F>(goals: &[SSPoint], st: &mut DijkstraSt, edge_cost: F) -> bool
where
    // closure to get cost from parent, current, and next node
    F: Fn(SSPoint, SSPoint, SSPoint) -> f32,
{
    // first check if given st already includes path to goal
    if goals.iter().any(|n| st.cost_map.contains_key(n)) {
        return true;
    }

    let mut i: u8 = 0;
    let mut goal_flag = false;
    // start visiting frontier nodes
    while let Some(MinScored(cost, this)) = st.to_visit.pop() {
        // if this == SSPoint::new(0, 2) {
        //     // debug helper
        //     dbg!("kek");
        // }
        if st.visited.contains(&this) {
            // was already visited through a lower cost path
            continue;
        }

        if i == u8::MAX {
            // give up after set iterations
            return false;
        } else {
            i = i.saturating_add(1);
        }
        let prev = st.cost_map.get(&this).unwrap().1;
        for next in [
            this + SSVec::new(1, 0),
            this + SSVec::new(0, 1),
            this + SSVec::new(-1, 0),
            this + SSVec::new(0, -1),
        ] {
            // if next == SSPoint::new(0, 3) {
            //     // debug helper
            //     dbg!("kek");
            // }
            if st.visited.contains(&next) {
                // already found a lower cost path to this target
                continue;
            }
            let next_score = cost + edge_cost(prev, this, next);
            match st.cost_map.entry(next) {
                Occupied(value) => {
                    if next_score < value.get().0 {
                        *value.into_mut() = (next_score, this);
                        st.to_visit.push(MinScored(next_score, next));
                    }
                }
                Vacant(value) => {
                    value.insert((next_score, this));
                    st.to_visit.push(MinScored(next_score, next));
                }
            }
            if goals.iter().any(|n| n == &next) {
                // goal was reached
                goal_flag = true;
            }
        }
        st.visited.insert(this);
        if goal_flag {
            return true;
        }
    }
    false
}

/// build path to target from pathfinding state
#[allow(clippy::if_same_then_else)] // to avoid panic in case of None
pub fn path_to_goal(st: &DijkstraSt, goals: &[SSPoint]) -> Option<Box<[SSPoint]>> {
    // find cheapest goal to reach
    let max = goals
        .iter()
        .map(|n| st.cost_map.get(n).map(|tup| MinScored(tup.0, n)))
        .max();
    if max.is_none() {
        return None;
    } else if max.unwrap().is_none() {
        return None;
    }
    let goal = max.unwrap().unwrap().1;

    // build path to reach goal
    let mut ret = vec![*goal];
    let mut breadcrumbs = HashSet::new();
    breadcrumbs.insert(*goal);
    let mut this = goal;
    loop {
        if let Some((_, prev)) = st.cost_map.get(this) {
            ret.push(*prev);
            if !breadcrumbs.insert(*prev) {
                // cyclic path detected
                return None;
            }
            if *prev == st.start {
                // backtraced to start
                break;
            }
            this = prev;
        } else {
            // return None in anything goes wrong
            return None;
        }
    }
    Some(ret.into())
}

#[derive(Clone)]
pub struct DijkstraSt {
    // cost map of nodeid to cost and optimal parent
    cost_map: HashMap<SSPoint, (f32, SSPoint)>,
    visited: HashSet<SSPoint>,
    to_visit: BinaryHeap<MinScored<f32, SSPoint>>,
    start: SSPoint,
}

impl DijkstraSt {
    pub fn new(start: SSPoint) -> Self {
        let mut cost_map = HashMap::default();
        let mut to_visit = BinaryHeap::new();

        cost_map.insert(start, (0.0, start));
        to_visit.push(MinScored(0.0, start));
        Self {
            cost_map,
            visited: HashSet::default(),
            to_visit,
            start,
        }
    }
    pub fn start(&self) -> SSPoint {
        self.start
    }
}
