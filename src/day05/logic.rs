use crate::day05::models::ManualUpdates;
use hashbrown::{HashMap, HashSet};
use petgraph::algo::all_simple_paths;
// use petgraph::dot::{Config, Dot};
use petgraph::Graph;

/// Return true if the order between two pages is valid or if it is not defined
fn is_order_valid(left: i32, right: i32, rules: &HashMap<i32, HashSet<i32>>) -> bool {
    if let Some(successors) = rules.get(&right) {
        if successors.contains(&left) {
            // The update is invalid
            return false;
        }
    }

    true
}

fn is_update_valid(update: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> bool {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            // Check if a rule prevent the right side to be before the left side.
            // To do that we check if a rule requires the left side to be after the right side.
            if !is_order_valid(update[i], update[j], rules) {
                return false;
            }
        }
    }

    true
}

pub fn solve_part_one(manual_updates: &ManualUpdates) -> i32 {
    let mut score = 0;

    for update in &manual_updates.updates {
        if is_update_valid(update, &manual_updates.rules) {
            score += update[update.len() / 2];
        }
    }

    score
}

fn fix_order(
    update: &Vec<i32>,
    rules: &HashMap<i32, HashSet<i32>>,
    reverse_rules: &HashMap<i32, HashSet<i32>>
) -> i32 {
    // Find the last element, this is always the only element that does not have a page of the
    // manual after it
    // 1. To do that, transform the update to a set
    let update_set: HashSet<i32> = update.iter().map(|val| *val).collect();
    // 2. For every element, find the one for which the update_set intersection with its follower
    // rules is empty
    let mut end = 0;
    for elt in update {
        if let Some(followers) = rules.get(elt) {
            if followers.intersection(&update_set).count() == 0 {
                end = *elt;
                break;
            }
        } else {
            end = *elt;
            break;
        }
    }

    // Find the first element, this always the only element that does not have a page of the manual
    // before it
    let mut start = 0;
    for elt in update {
        if let Some(followers) = reverse_rules.get(elt) {
            if followers.intersection(&update_set).count() == 0 {
                start = *elt;
                break;
            }
        } else {
            start = *elt;
            break;
        }
    }

    // Build a graph with these nodes
    let mut nodes = HashMap::new();
    let mut graph = Graph::new();
    // Add nodes from the update
    for page in update {
        nodes.entry(*page).or_insert_with(|| graph.add_node(*page));
    }
    // Add rules from the rule set
    for first in update {
        if let Some(successors) = rules.get(first) {
            for successor in successors {
                if update_set.contains(successor) {
                    graph.add_edge(nodes[first], nodes[successor], 1);
                }
            }
        }
    }

    // Find the path with update.len() - 2 inner nodes
    let ways: Vec<_> = all_simple_paths::<Vec<_>, _>(
        &graph,
        nodes[&start],
        nodes[&end],
        update.len() - 2,
        Some(update.len() - 2),
    )
    .collect();

    graph[ways[0][update.len() / 2]]
}

pub fn solve_part_two(manual_updates: &ManualUpdates) -> i32 {
    // Build reverse rules that for each page indicates the list of pages that can be put before.
    let mut reverse_rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (before, afters) in &manual_updates.rules {
        for after in afters {
            reverse_rules.entry(*after).or_default().insert(*before);
        }
    }

    let mut score = 0;

    for update in &manual_updates.updates {
        if !is_update_valid(update, &manual_updates.rules) {
            score += fix_order(update, &manual_updates.rules, &reverse_rules);
        }
    }

    score
}
