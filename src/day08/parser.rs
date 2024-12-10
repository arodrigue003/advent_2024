use std::collections::HashMap;

use crate::day08::models::{Antenna, Map};

pub fn parse_input(input: String) -> Map {
    // Parse the map
    let map: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    // Retrieve antennas
    let mut antennas: HashMap<char, Vec<Antenna>> = HashMap::new();
    for (y, line) in map.iter().enumerate() {
        for (x, frequency) in line.iter().enumerate() {
            if *frequency != '.' {
                antennas.entry(*frequency).or_default().push(Antenna {
                    frequency: *frequency,
                    x: x as i32,
                    y: y as i32,
                })
            }
        }
    }

    Map {
        width: map[0].len() as i32,
        height: map.len() as i32,
        antennas,
    }
}
