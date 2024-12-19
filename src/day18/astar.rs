use std::cmp::Ordering;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;

use hashbrown::HashSet;

use crate::day18::models::Corruption;

fn get_neighbors(node: (usize, usize), size: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..4).filter_map(move |i| match i {
        0 if node.0 > 0 => Some((node.0 - 1, node.1)),        // Left
        1 if node.1 > 0 => Some((node.0, node.1 - 1)),        // Up
        2 if node.0 + 1 < size => Some((node.0 + 1, node.1)), // Right
        3 if node.1 + 1 < size => Some((node.0, node.1 + 1)), // Bot
        _ => None,
    })
}

/// An astar implementation that work for corruption maps
pub fn astar(corruption_map: &Corruption, to_simulate: usize) -> Option<usize> {
    // Put target corruption in a hashset to improve search speed
    let damage_map: HashSet<_> = corruption_map.bytes.iter().take(to_simulate).collect();

    // Set start and end
    let start = (0, 0);
    let target = (corruption_map.size - 1, corruption_map.size - 1);

    let mut visit_next = BinaryHeap::new();
    let mut scores = HashMap::new(); // g-values, cost to reach the node

    let zero_score = 0;
    scores.insert(start, zero_score);
    visit_next.push(MinScored(zero_score, start));

    while let Some(MinScored(_, node)) = visit_next.pop() {
        if node == target {
            return Some(scores[&node]);
        }

        // This lookup can be unwrapped without fear of panic since the node was necessarily scored
        // before adding it to `visit_next`.
        let node_score = scores[&node];

        for next in get_neighbors(node, corruption_map.size) {
            // Don't go here if the bit is damaged
            if damage_map.contains(&next) {
                continue;
            }

            let next_score = node_score + 1;

            match scores.entry(next) {
                Occupied(mut entry) => {
                    // No need to add neighbors that we have already reached through a shorter path
                    // than now.
                    if *entry.get() <= next_score {
                        continue;
                    }
                    entry.insert(next_score);
                }
                Vacant(entry) => {
                    entry.insert(next_score);
                }
            }
            visit_next.push(MinScored(next_score, next));
        }
    }

    None
}

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
