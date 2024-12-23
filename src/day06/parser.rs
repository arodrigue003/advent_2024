use crate::day06::models::{LabWithABorder, Position, Tile};

pub fn parse_input(input: String) -> LabWithABorder {
    // We won't need to add a border here since the map already has one.
    let mut grid: Vec<Vec<Tile>> = vec![];
    let mut start_position = Position::default();

    for (y, line) in input.lines().enumerate() {
        grid.push(
            line.chars()
                .enumerate()
                .map(|(x, tile)| match tile {
                    '#' => Tile::Wall,
                    '.' => Tile::Floor,
                    '^' => {
                        start_position = Position::new(x, y);
                        Tile::Floor
                    }
                    _ => unreachable!(),
                })
                .collect(),
        )
    }

    LabWithABorder::new(grid, start_position)
}
