use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use crate::day15::models::coordinates::Coordinates;
use crate::day15::models::instruction::Instruction;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MapTile {
    Wall,
    Box,
    Empty,
}

impl Display for MapTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MapTile::Wall => write!(f, "#"),
            MapTile::Box => write!(f, "O"),
            MapTile::Empty => write!(f, "."),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid(pub Vec<Vec<MapTile>>);

impl Index<&Coordinates> for Grid {
    type Output = MapTile;

    fn index(&self, index: &Coordinates) -> &Self::Output {
        &self.0[index.y as usize][index.x as usize]
    }
}

impl IndexMut<&Coordinates> for Grid {
    fn index_mut(&mut self, index: &Coordinates) -> &mut Self::Output {
        &mut self.0[index.y as usize][index.x as usize]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Warehouse {
    pub grid: Grid,
    pub robot_positions: Coordinates,
    pub instructions: Vec<Instruction>,
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (y, line) in self.grid.0.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if self.robot_positions == (x, y) {
                    write!(f, "@")?;
                } else {
                    write!(f, "{tile}")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Warehouse {
    /// Check if the robot can move in the desired direction
    pub fn move_robot(&mut self, instruction: &Instruction) {
        let mut check = self.robot_positions.clone();
        let mut boxed = false; // true if we found a box on the way
        let movement_vector = instruction.get_movement_vec();

        // The robot can move if an empty tile is located in the moving direction
        // and if no wall is present before.
        // Since the map contains wall all around, we do not need to check for boundaries and
        // we now that our loop will always end
        loop {
            // move the robot
            check += &movement_vector;

            // Check the new position
            match &self.grid[&check] {
                MapTile::Wall => {
                    // Nothing to do return
                    return;
                }
                MapTile::Box => boxed = true,
                MapTile::Empty => {
                    // Move the robot
                    self.robot_positions += &movement_vector;


                    // Move the line of boxes if needed
                    if boxed {
                        // Move the line of boxes
                        // 1. from the start
                        self.grid[&self.robot_positions] = MapTile::Empty;
                        // 2. to the end
                        self.grid[&check] = MapTile::Box;
                    }

                    return;
                }
            }
        }
    }

    pub fn boxes_score(&self) -> usize {
        let mut score = 0;
        for (y, line) in self.grid.0.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if tile == &MapTile::Box {
                    score += 100*y+x;
                }
            }
        }
        score
    }
}