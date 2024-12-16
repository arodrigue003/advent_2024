mod logic;
mod models;
mod parser;

#[cfg(test)]
mod tests;

use crate::day16::logic::{prepare_data, solve_part_one, solve_part_two};
use crate::day16::models::{Map, PreparedData};
use crate::day16::parser::parse_input;
use crate::models::AdventSolution;

#[derive(Default)]
pub struct Day16 {
    parsed_data: Option<Map>,
    prepared_data: Option<PreparedData>
}

impl AdventSolution for Day16 {
    fn parse(&mut self, _data: String) {
        self.parsed_data = Some(parse_input(_data));
    }

    fn prepare(&mut self) {
        self.prepared_data = Some(prepare_data(self.parsed_data.as_ref().unwrap()));
    }

    fn solve_part_one(&self) -> i128 {
        solve_part_one(self.prepared_data.as_ref().unwrap()) as i128
    }

    fn solve_part_two(&self) -> i128 {
        solve_part_two(self.prepared_data.as_ref().unwrap()) as i128
    }
}
