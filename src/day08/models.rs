use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub antennas: HashMap<char, Vec<Antenna>>,
}

impl Map {
    pub fn is_inside(&self, coordinates: &(i32, i32)) -> bool {
        coordinates.0 >= 0 && coordinates.0 < self.width && coordinates.1 >= 0 && coordinates.1 < self.height
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Antenna {
    pub frequency: char,
    pub x: i32,
    pub y: i32,
}
