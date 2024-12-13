use crate::day13::models::{DiophantineSolution, Machine};
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
    println!("a:{a}, b:{b}");

    // We can now find two values `x0` and `y0` that satisfy this equation if `c` is a multiple of
    // `gcd(a, b)`. To do that we use the extended gcd algorithm and we get two coeff verifying:
    // `left_coeff*a + right_coeff*b = gcd(a, b)`.
    let (left_coef, right_coef, gcd) = extended_gcd(a, b);

    // This equation admit a solution if and only if c is a multiple of gcd(a, b)
    if c % gcd != 0 {
        println!("no solution because of the gcd");
        return None;
    }

    // We now can compute `x0` and `y0` to have a solution for `ax + by =c ` by the operation `c / gcd`
    // This work because `c` is a multiple of the `gcd`, as well as `a` and `b`.
    // We know have `x0 = left_coef * c / gcd` and `y0 = right_coef * c / gcd` satisfying
    // `ax + by = c`
    let x0 = left_coef * c / gcd;
    let y0 = right_coef * c / gcd;
    println!("xo:{x0}, y0:{y0}, {} = {c}", a*x0 + b*y0);

    // We now want to find every solution (x, y) where x and y are positives.
    // For x, this means that `x0 - n * b / gcd(a, b) > 0`, if b and gcd are both positive or both
    // negative, this mean that the maximal value of n for which x is positive is `x0 * gcd(a, b) / b`
    // For y, this mean that `y0 + n * a / gcd(a, b) > 0`, if b and gcd are both positive or both
    // negative, this mean that the minimal value of n for which y is positive is `- y0 * gcd(a, b) / a`
    // Otherwise, min_n and max_n are interchanged
    let mut max_n = (x0 * gcd).div_euclid(b);
    let mut min_n = -(y0 * gcd).div_euclid(a);
    if (b > 0 && gcd < 0) || (b < 0 && gcd > 0) {
        println!("------------------");
        (max_n, min_n) = (min_n, max_n);
    }
    println!("{min_n}=>{max_n}:{}", max_n-min_n);

    // We offset x0 and y0 to have only positive values of n, x and y.
    let x1 = (x0 - min_n * b / gcd);
    let y1 = (y0 + min_n * a / gcd);
    let max_n = max_n - min_n;

    // Check for positivity of x and y at extreme values

    // println!("{}", x1 - min_n * b / gcd);
    // println!("{}", x1 - min_n * b / gcd);

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

pub fn solve_part_two(machines: &[Machine]) -> i64 {
    let a0 = 26;
    let b0 = 67;
    let t0 = 12748 + 10_000_000_000_000;
    // let t = 12748;

    let a1 = 66;
    let b1 = 21;
    let t1 = 12176 + 10000000000000;

    let res0 = solve_diophantine(a0, b0, t0).unwrap();
    println!("{:#?}", &res0);
    // for n in (res0.max_n-10..=res0.max_n).rev() {
    //     println!("{}", res0.x0 - n * res0.a0);
    // }

    let res1 = solve_diophantine(a1, b1, t1).unwrap();
    println!("{:#?}", &res1);
    // for n in (res1.max_n-10..=res1.max_n).rev() {
    //     println!("{}", res1.x0 - n * res1.a0);
    // }

    let res2 = solve_diophantine(res0.a0, -res1.a0, res0.x0 - res1.x0).unwrap();
    println!("{:#?}", &res2);

    println!("{}", res0.x0 - res2.x0 * res0.a0);
    println!("{}", res1.x0 - res2.y0 * res1.a0);
    println!("{}", res0.y0 + res2.x0 * res0.b0);
    println!("{}", res1.y0 + res2.y0 * res1.b0);


    return 0;

    let mut total_cost = 0;

    for machine in machines {
        let target_x = machine.target.0 + 10000000000000;
        let target_y = machine.target.1 + 10000000000000;

        // First, we compute the highest possible value for B that still respect the winning
        // conditions. Since A is more expensive than B to press, we try to maximise B in order
        // to get the cheapest combination possible.
        let max_b = min(machine.target.0 / target_x, machine.target.1 / target_y);

        // We now check for every value of B going from max_b to 0 if a value of a is compatible
        // with this value
        for b in (0..=max_b).rev() {
            let cur_x = machine.b.0 * b;
            let cur_y = machine.b.1 * b;

            // Check if A is compatible with this value of B
            let a = (target_x - cur_x) / machine.a.0;

            if cur_x + a * machine.a.0 == target_x && cur_y + a * machine.a.1 == target_y && a > 0 {
                total_cost += b + 3 * a;
                break;
            }
        }
    }

    total_cost
}
