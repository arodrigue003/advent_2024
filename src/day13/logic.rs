use std::cmp::{max, min};

use crate::day13::models::{DiophantineSolution, Machine};

pub fn solve_part_one(machines: &[Machine]) -> i64 {
    let mut total_cost = 0;

    for machine in machines {
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
                break;
            }
        }
    }

    total_cost
}

/// Implementation of the extended Euclidean algorithm
/// This is taken from https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
/// return the tuple (left_coef, right_coef, gcd)
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let mut old_r = a;
    let mut r = b;
    let mut old_s = 1;
    let mut s = 0;
    let mut old_t = 0;
    let mut t = 1;

    loop {
        if r == 0 {
            break;
        }
        let quotient = old_r / r;

        let prov = r;
        r = old_r - quotient * r;
        old_r = prov;

        let prov = s;
        s = old_s - quotient * s;
        old_s = prov;

        let prov = t;
        t = old_t - quotient * t;
        old_t = prov
    }

    if old_r < 0 {
        (-old_s, -old_t, -old_r)
    } else {
        (old_s, old_t, old_r)
    }
}

/// Find every positive solution of the equation ax + by = c
fn solve_diophantine(a: i64, b: i64, c: i64) -> Option<DiophantineSolution> {
    // We can now find two values `x0` and `y0` that satisfy this equation if `c` is a multiple of
    // `gcd(a, b)`. To do that we use the extended gcd algorithm and we get two coeff verifying:
    // `left_coeff*a + right_coeff*b = gcd(a, b)`.
    let (left_coef, right_coef, gcd) = extended_gcd(a, b);

    // This equation admit a solution if and only if c is a multiple of gcd(a, b)
    if c % gcd != 0 {
        // println!("no solution because of the gcd");
        return None;
    }

    // We now can compute `x0` and `y0` to have a solution for `ax + by =c ` by the operation `c / gcd`
    // This work because `c` is a multiple of the `gcd`, as well as `a` and `b`.
    // We know have `x0 = left_coef * c / gcd` and `y0 = right_coef * c / gcd` satisfying
    // `ax + by = c`
    let x0 = left_coef * c / gcd;
    let y0 = right_coef * c / gcd;

    // We now want to find every solution (x, y) where x and y are positives.
    // For x, this means that `x0 - n * b / gcd(a, b) > 0`, if b and gcd are both positive or both
    // negative, this mean that the maximal value of n for which x is positive is `x0 * gcd(a, b) / b`
    // For y, this mean that `y0 + n * a / gcd(a, b) > 0`, if b and gcd are both positive or both
    // negative, this mean that the minimal value of n for which y is positive is `- y0 * gcd(a, b) / a`
    // Otherwise, min_n and max_n are interchanged
    let mut min_n = i64::MIN;
    let mut max_n = i64::MAX;
    if (b > 0 && gcd > 0) || (b < 0 && gcd < 0) {
        max_n = min(max_n, (x0 * gcd).div_euclid(b));
    } else {
        min_n = max(min_n, (x0 * gcd).div_euclid(b));
    }
    if (a > 0 && gcd > 0) || (a < 0 && gcd < 0) {
        min_n = max(min_n, -(y0 * gcd).div_euclid(a));
    } else {
        max_n = min(max_n, -(y0 * gcd).div_euclid(a));
    }

    // If we can, offset x0 and yo to only have positive values of n.
    // println!("before:{min_n}:{max_n}");
    if min_n == i64::MIN {
        unimplemented!("Houston we have an unhandled case");
    }

    // We offset x0 and y0 to have only positive values of n, x and y.
    let x1 = x0 - min_n * b / gcd;
    let y1 = y0 + min_n * a / gcd;

    // We update min_n and max_n values
    if max_n != i64::MAX {
        max_n -= min_n;
    }

    // The result is a tuple (x1, a1, y1, b1, max_n) where `x1 - n * a1` and `y1 + n * b1` for
    // `0 <= n <= max_n` are every positive solution of the equation ax + by = c
    Some(DiophantineSolution {
        x0: x1,
        a0: b / gcd,
        y0: y1,
        b0: a / gcd,
        max_n,
    })
}

/// THIS IS THE WORST POSSIBLE SOLUTION TO A SIMPLE PROBLEM
pub fn solve_part_two(machines: &[Machine]) -> i64 {
    let mut total_cost = 0;

    for (i, machine) in machines.iter().enumerate() {
        let a0 = machine.a.0;
        let b0 = machine.b.0;
        let t0 = machine.target.0 + 10_000_000_000_000;

        let a1 = machine.a.1;
        let b1 = machine.b.1;
        let t1 = machine.target.1 + 10_000_000_000_000;

        // Solve on x
        let res0 = if let Some(res) = solve_diophantine(a0, b0, t0) {
            res
        } else {
            continue;
        };

        // Solve on y
        let res1 = if let Some(res) = solve_diophantine(a1, b1, t1) {
            res
        } else {
            continue;
        };

        // get solutions for which the number of A press is equal for x and y
        let res2 = if let Some(res) = solve_diophantine(res0.a0, -res1.a0, res0.x0 - res1.x0) {
            res
        } else {
            continue;
        };

        let new_x0 = res0.x0 - res0.a0 * res2.x0;
        let new_a0 = -res0.a0 * res2.a0;
        let new_y0 = res0.y0 + res0.b0 * res2.x0;
        let new_b0 = -res0.b0 * res2.a0;

        let new_x1 = res1.x0 - res1.a0 * res2.y0;
        let new_a1 = res1.a0 * res2.b0;
        let new_y1 = res1.y0 + res1.b0 * res2.y0;
        let new_b1 = res1.b0 * res2.b0;

        // Update the solution to take this modification into account
        let res0 = DiophantineSolution {
            x0: new_x0,
            a0: new_a0,
            y0: new_y0,
            b0: new_b0,
            max_n: i64::MAX,
        };
        let res1 = DiophantineSolution {
            x0: new_x1,
            a0: new_a1,
            y0: new_y1,
            b0: new_b1,
            max_n: i64::MAX,
        };

        // We now need to solve res0.y0 + n*res0.b0 = res1.y0 + n*res1.b0
        if res0.b0 == res1.b0 {
            unimplemented!("Machine {} has an infinite number of solution", i);
        }
        let quot = res1.y0 - res0.y0;
        let num = res0.b0 - res1.b0;
        if quot % num == 0 {
            let n = quot / num;

            // Compute res_a and res_b
            let res_a = res0.x0 - n * res0.a0;
            let res_b = res0.y0 + n * res0.b0;

            // Add it to the cost
            total_cost += res_a * 3 + res_b;
        }
    }

    total_cost
}
