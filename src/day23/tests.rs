use crate::common::{test_part_one_common, test_part_two_common};
use crate::day23::Day23;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day23");

#[test]
fn test_part_one() {
    test_part_one_common(Day23::default(), INPUT_EXAMPLE, 7);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day23::default(), INPUT_EXAMPLE, 0);
}
