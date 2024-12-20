use crate::day20::models::{Map, Tile};

pub fn parse_input(input: String) -> Map {
    // We won't need to add a border here since the map already has one.
    let mut grid: Vec<Vec<Tile>> = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in input.lines().enumerate() {
        grid.push(
            line.chars()
                .enumerate()
                .map(|(x, tile)| match tile {
                    '#' => Tile::Wall,
                    '.' => Tile::Floor,
                    'S' => {
                        start = (x,y);
                        Tile::Floor
                    }
                    'E' => {
                        end = (x, y);
                        Tile::Floor
                    }
                    _ => unreachable!(),
                })
                .collect(),
        )
    }

    Map::new(grid, start, end)
}
