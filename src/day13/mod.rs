mod logic;
mod models;
mod parser;

#[cfg(test)]
mod tests;

use crate::day13::logic::{solve_part_one, solve_part_two};
use crate::day13::models::Machine;
use crate::day13::parser::parse_input;
use crate::models::AdventSolution;

#[derive(Default)]
pub struct Day13 {
    parsed_data: Option<Vec<Machine>>,
}

impl AdventSolution for Day13 {
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
