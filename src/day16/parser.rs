use crate::day16::models::{Map, Tile};

pub fn parse_input(input: String) -> Map {
    // We won't need to add a border here since the map already has one.
    let mut grid: Vec<Vec<Tile>> = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, line) in input.lines().enumerate() {
        grid.push(
            line.chars()
                .enumerate()
                .map(|(j, tile)| match tile {
                    '#' => Tile::Wall,
                    '.' => Tile::Floor,
                    'S' => {
                        start = (i, j);
                        Tile::Floor
                    }
                    'E' => {
                        end = (i, j);
                        Tile::Floor
                    }
                    _ => unreachable!(),
                })
                .collect(),
        )
    }

    Map { grid, start, end }
}
