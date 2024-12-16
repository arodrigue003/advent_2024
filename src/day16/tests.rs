use crate::common::{test_part_one_common, test_part_two_common};
use crate::day16::Day16;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day16");
static INPUT_EXAMPLE_2: &str = include_str!("../../input_examples/day16_2");

#[test]
fn test_part_one() {
    test_part_one_common(Day16::default(), INPUT_EXAMPLE, 7036);
    test_part_one_common(Day16::default(), INPUT_EXAMPLE_2, 11048);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day16::default(), INPUT_EXAMPLE, 0);
}
