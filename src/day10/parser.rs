use crate::day10::models::MapWithBorder;

pub fn parse_input(input: String) -> MapWithBorder {
    MapWithBorder::new(
        input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect(),
    )
}
