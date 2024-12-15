use itertools::Itertools;
use nom::Parser;

pub fn solve_part_one(reports: &[Vec<i64>]) -> usize {
    reports
        .iter()
        .filter(|report| {
            report.iter().tuple_windows().all(|(left, right)| {
                let diff = left.abs_diff(*right);
                diff >= 1 && diff <= 3
            }) && report
                .iter()
                .tuple_windows()
                .map(|(left, right)| if right > left { 1i64 } else { -1i64 })
                .sum::<i64>()
                .abs()
                == report.len() as i64 - 1
        })
        .count()
}

pub fn solve_part_two(reports: &[Vec<i64>]) -> u32 {
    for report in reports {
        let diff_safe = report.iter().tuple_windows().filter(|(left, right)| {
            let diff = left.abs_diff(**right);
            diff >= 1 && diff <= 3
        }).count() >= report.len() - 1;

        println!("{diff_safe}")
    };

    0
}
