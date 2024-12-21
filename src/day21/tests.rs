use crate::common::{test_part_one_common, test_part_two_common};
use crate::day21::Day21;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day21");

#[test]
fn test_part_one() {
    test_part_one_common(Day21::default(), INPUT_EXAMPLE, 126384);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day21::default(), INPUT_EXAMPLE, 154115708116294);
}
