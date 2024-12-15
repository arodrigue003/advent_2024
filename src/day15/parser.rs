use crate::day15::models::instruction::Instruction;
use crate::day15::models::coordinates::Coordinates;
use crate::day15::models::warehouse::{Grid, MapTile, Warehouse};

pub fn parse_input(input: String) -> Warehouse {
    let mut grid: Vec<Vec<MapTile>> = vec![];
    let mut robot_positions = Coordinates::default();
    let mut instructions: Vec<Instruction> = vec![];
    let mut grid_completed = false;

    for (y, line) in input.lines().enumerate() {
        // If line is empty, go the instructions parsing part
        if line.is_empty() {
            grid_completed = true
        } else if !grid_completed {
            // Parsing the line
            grid.push(
                line.chars()
                    .enumerate()
                    .map(|(x, tile)| match tile {
                        '#' => MapTile::Wall,
                        '.' => MapTile::Empty,
                        'O' => MapTile::Box,
                        '@' => {
                            robot_positions = Coordinates::new(x as i64, y as i64);
                            MapTile::Empty
                        }
                        _ => unreachable!(),
                    })
                    .collect(),
            )
        } else {
            instructions.append(&mut line.chars().map(From::from).collect());
        }
    }

    Warehouse {
        grid: Grid(grid),
        robot_positions,
        instructions,
    }
}
