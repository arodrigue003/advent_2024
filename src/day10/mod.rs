mod logic;
mod models;
mod parser;

#[cfg(test)]
mod tests;

use crate::day10::logic::{solve_part_one, solve_part_two};
use crate::day10::models::MapWithBorder;
use crate::day10::parser::parse_input;
use crate::models::AdventSolution;

#[derive(Default)]
pub struct Day10 {
    parsed_data: Option<MapWithBorder>,
}

impl AdventSolution for Day10 {
    fn parse(&mut self, _data: String) {
        self.parsed_data = Some(parse_input(_data));
    }

    fn solve_part_one(&self) -> i128 {
        solve_part_one(self.parsed_data.as_ref().unwrap()) as i128
    }

    fn solve_part_two(&self) -> i128 {
        solve_part_two(self.parsed_data.as_ref().unwrap()) as i128
    }
}
