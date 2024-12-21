use crate::common::{test_part_one_common, test_part_two_common};
use crate::day19::Day19;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day21");

#[test]
fn test_part_one() {
    test_part_one_common(Day19::default(), INPUT_EXAMPLE, 126384);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day19::default(), INPUT_EXAMPLE, 0);
}
