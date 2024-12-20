use std::cmp::Ordering;
use std::collections::{BinaryHeap};
use std::fmt::Debug;


use crate::day20::models::{Map, Tile};

fn get_neighbors(node: (usize, usize), height: usize, width: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..4).filter_map(move |i| match i {
        0 if node.0 > 0 => Some((node.0 - 1, node.1)),          // Left
        1 if node.1 > 0 => Some((node.0, node.1 - 1)),          // Up
        2 if node.0 + 1 < width => Some((node.0 + 1, node.1)),  // Right
        3 if node.1 + 1 < height => Some((node.0, node.1 + 1)), // Bot
        _ => None,
    })
}

/// An astar implementation that work for corruption maps that resturn the score map
pub fn astar(map: &Map) -> Option<Vec<Vec<usize>>> {
    let mut visit_next = BinaryHeap::new();
    // We put scores on a vec for later reasons
    let mut scores = vec![vec![usize::MAX; map.width]; map.height];

    // Set starting conditions
    let zero_score = 0;
    scores[map.start.1][map.start.0] = zero_score;
    visit_next.push(MinScored(zero_score, map.start));

    while let Some(MinScored(_, node)) = visit_next.pop() {
        if node == map.end {
            // We can do this here because there is a single path
            return Some(scores);
        }

        let node_score = scores[node.1][node.0];

        for next in get_neighbors(node, map.height, map.height) {
            // Don't go here if the bit is damaged
            if map.grid[next.1][next.0] == Tile::Wall {
                continue;
            }

            let next_score = node_score + 1;

            let cached_score = scores[next.1][next.0];
            if cached_score <= next_score {
                continue;
            } else {
                scores[next.1][next.0] = next_score;
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
