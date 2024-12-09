use crate::common::{test_part_one_common, test_part_two_common};
use crate::day09::Day09;

static INPUT_EXAMPLE: &str = "2333133121414131402";

#[test]
fn test_part_one() {
    test_part_one_common(Day09::default(), INPUT_EXAMPLE, 1928);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day09::default(), INPUT_EXAMPLE, 2858);
}
