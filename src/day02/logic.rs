use itertools::Itertools;

fn is_safe(report: &[i64]) -> bool {
    report.iter().tuple_windows().all(|(left, right)| {
        let diff = left.abs_diff(*right);
        (1..=3).contains(&diff)
    }) && report
        .iter()
        .tuple_windows()
        .map(|(left, right)| if right > left { 1i64 } else { -1i64 })
        .sum::<i64>()
        .abs()
        == report.len() as i64 - 1
}

pub fn solve_part_one(reports: &[Vec<i64>]) -> usize {
    reports.iter().filter(|report| is_safe(report)).count()
}

pub fn solve_part_two(reports: &[Vec<i64>]) -> usize {
    reports.iter().filter(|report| {
        if is_safe(report) {
            true
        } else {
            (0..report.len()).any(|i| {
                let mut temp_report = (*report).clone();
                temp_report.remove(i);
                is_safe(&temp_report)
            })
        }
    }).count()
}
