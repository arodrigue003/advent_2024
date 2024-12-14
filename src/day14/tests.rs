use crate::common::{test_part_one_common, test_part_two_common};
use crate::day14::Day14;

static INPUT_EXAMPLE: &str = "";

#[test]
fn test_part_one() {
    test_part_one_common(Day14::default(), INPUT_EXAMPLE, 12);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day14::default(), INPUT_EXAMPLE, 0);
}
