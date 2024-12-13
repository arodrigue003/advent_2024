use crate::day13::models::Machine;
use std::cmp::min;

pub fn solve_part_one(machines: &[Machine]) -> i64 {
    let mut total_cost = 0;

    for (i, machine) in machines.iter().enumerate() {
        // First, we compute the highest possible value for B that still respect the winning
        // conditions. Since A is more expensive than B to press, we try to maximise B in order
        // to get the cheapest combination possible.
        let max_b = min(min(machine.target.0 / machine.b.0, machine.target.1 / machine.b.1), 100);

        // We now check for every value of B going from max_b to 0 if a value of a is compatible
        // with this value
        for b in (0..=max_b).rev() {
            let cur_x = machine.b.0 * b;
            let cur_y = machine.b.1 * b;

            // Check if A is compatible with this value of B
            let a = (machine.target.0 - cur_x) / machine.a.0;
            if cur_x + a * machine.a.0 == machine.target.0
                && cur_y + a * machine.a.1 == machine.target.1
                && a > 0
                && a <= 100
            {
                total_cost += b + 3 * a;
                break
            }
        }

    }

    total_cost
}

pub fn solve_part_two(machines: &[Machine]) -> i64 {
    0
}
