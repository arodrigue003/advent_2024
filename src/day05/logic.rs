use hashbrown::{HashMap, HashSet};

use crate::day05::models::ManualUpdates;

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

fn fix_order(update: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>, reverse_rules: &HashMap<i32, HashSet<i32>>) -> i32 {
    // Create a set from the update
    let update_set: HashSet<i32> = update.iter().copied().collect();

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

    // Create a subset of rules that only contains our rules
    let mut sub_rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (before, afters) in rules {
        for after in afters {
            if update_set.contains(after) && update_set.contains(before) {
                sub_rules.entry(*before).or_default().insert(*after);
            }
        }
    }
    // Clone it for read only access
    let sub_rules_ref = sub_rules.clone();

    // We note that if we have the rule A->(B|C) but if B->C then we can remove C from A->(B|C)
    for page in update {
        if let Some(successor) = sub_rules_ref.get(page) {
            for first in successor {
                for second in successor {
                    if first != second {
                        if let Some(first_succ) = sub_rules_ref.get(first) {
                            if first_succ.contains(second) {
                                // page is A, first is B, C is second. Remove A->C so page->second
                                sub_rules.get_mut(page).unwrap().remove(second);
                            }
                        }
                    }
                }
            }
        }
    }

    // Now each rule should only have one image, just iter it from the start until we find the
    // middle one
    let mut current = start;
    for _ in 0..update.len() / 2 {
        current = *sub_rules[&current].iter().next().unwrap();
    }
    current
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
