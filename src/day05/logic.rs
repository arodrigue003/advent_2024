use crate::day05::models::ManualUpdates;
use hashbrown::{HashMap, HashSet};

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
                return false
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

pub fn solve_part_two(manual_updates: &ManualUpdates) -> u32 {
    0
}
