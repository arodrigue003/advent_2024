mod logic;
mod models;
mod parser;

#[cfg(test)]
mod tests;

use crate::day19::logic::{count_combinations};
use crate::day19::models::Onsen;
use crate::day19::parser::parse_input;
use crate::models::AdventSolution;

#[derive(Default)]
pub struct Day19 {
    parsed_data: Option<Onsen>,
    counts: Vec<usize>,
}

impl AdventSolution for Day19 {
    fn parse(&mut self, _data: String) {
        self.parsed_data = Some(parse_input(_data));
    }

    fn prepare(&mut self) {
        let onsen = self.parsed_data.as_ref().unwrap();

        let min_size = onsen.available_towels.iter().map(|a| a.len()).min().unwrap();
        let max_size = onsen.available_towels.iter().map(|a| a.len()).max().unwrap();

        self.counts = onsen
            .target_designs
            .iter()
            .map(|design| count_combinations(&onsen.available_towels_set, design, min_size, max_size))
            .collect();
    }

    fn solve_part_one(&self) -> i128 {
        self.counts.iter().filter(|count| **count != 0).count() as i128
    }

    fn solve_part_two(&self) -> i128 {
        self.counts.iter().copied().sum::<usize>() as i128
    }
}
