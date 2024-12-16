use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{BinaryHeap, HashMap, VecDeque};

use std::hash::Hash;

use petgraph::visit::{EdgeRef, GraphBase, IntoEdges, Visitable};

use petgraph::algo::Measure;

/// An astar implementation that track every possible path with the lowest score
pub fn multi_astar<G, F, K, IsGoal>(
    graph: G,
    start: G::NodeId,
    mut is_goal: IsGoal,
    mut edge_cost: F,
) -> Option<(K, HashSet<G::NodeId>)>
where
    G: IntoEdges + Visitable,
    IsGoal: FnMut(G::NodeId) -> bool,
    G::NodeId: Eq + Hash + Debug,
    F: FnMut(G::EdgeRef) -> K,
    K: Measure + Copy,
{
    let mut visit_next = BinaryHeap::new();
    let mut scores = HashMap::new(); // g-values, cost to reach the node
    let mut path_tracker = PathTracker::<G>::new();

    let zero_score = K::default();
    scores.insert(start, zero_score);
    visit_next.push(MinScored(zero_score, start));

    while let Some(MinScored(_, node)) = visit_next.pop() {
        if is_goal(node) {
            // let path = path_tracker.reconstruct_path_to(node);
            let cost = scores[&node];
            return Some((cost, path_tracker.reconstruct_possible_paths(node)));
        }

        // This lookup can be unwrapped without fear of panic since the node was necessarily scored
        // before adding it to `visit_next`.
        let node_score = scores[&node];

        for edge in graph.edges(node) {
            let next = edge.target();
            let next_score = node_score + edge_cost(edge);

            match scores.entry(next) {
                Occupied(mut entry) => {
                    // No need to add neighbors that we have already reached through a shorter path
                    // than now.
                    if *entry.get() < next_score {
                        continue;
                    } else if *entry.get() == next_score {
                        path_tracker.add_predecessor(next, node);
                        continue;
                    }
                    entry.insert(next_score);
                }
                Vacant(entry) => {
                    entry.insert(next_score);
                }
            }

            path_tracker.set_predecessor(next, node);
            visit_next.push(MinScored(next_score, next));
        }
    }

    None
}

use hashbrown::HashSet;
use std::cmp::Ordering;
use std::fmt::Debug;

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

#[derive(Debug)]
struct PathTracker<G>
where
    G: GraphBase,
    G::NodeId: Eq + Hash + Debug,
{
    came_from: HashMap<G::NodeId, Vec<G::NodeId>>,
}

impl<G> PathTracker<G>
where
    G: GraphBase,
    G::NodeId: Eq + Hash + Debug,
{
    fn new() -> PathTracker<G> {
        PathTracker {
            came_from: HashMap::new(),
        }
    }

    fn set_predecessor(&mut self, node: G::NodeId, previous: G::NodeId) {
        self.came_from.insert(node, vec![previous]);
    }

    fn add_predecessor(&mut self, node: G::NodeId, previous: G::NodeId) {
        self.came_from.entry(node).or_default().push(previous);
    }

    fn reconstruct_possible_paths(&self, last: G::NodeId) -> HashSet<G::NodeId> {
        let mut paths = HashSet::new();
        let mut to_visit = VecDeque::new();

        // Add the last to the list of nodes to visit
        paths.insert(last);
        to_visit.push_back(last);

        while let Some(current) = to_visit.pop_front() {
            if let Some(previous) = self.came_from.get(&current) {
                for previous in previous {
                    if !paths.contains(previous) {
                        to_visit.push_back(*previous);
                    }
                    paths.insert(*previous);
                }
            }
        }

        paths
    }
}
