use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

use hashbrown::HashSet;
use once_cell::sync::Lazy;

use crate::day15::models::coordinates::Coordinates;
use crate::day15::models::instruction::Instruction;
use crate::day15::models::warehouse::{MapTile, Warehouse};

static LEFT: Lazy<Coordinates> = Lazy::new(|| Coordinates::new(-1, 0));
static RIGHT: Lazy<Coordinates> = Lazy::new(|| Coordinates::new(1, 0));

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BigMapTile {
    Wall,
    LeftBox,
    RightBox,
    Empty,
}

impl Display for BigMapTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BigMapTile::Wall => write!(f, "#"),
            BigMapTile::LeftBox => write!(f, "["),
            BigMapTile::RightBox => write!(f, "]"),
            BigMapTile::Empty => write!(f, "."),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BigGrid(pub Vec<Vec<BigMapTile>>);

impl Index<&Coordinates> for BigGrid {
    type Output = BigMapTile;

    fn index(&self, index: &Coordinates) -> &Self::Output {
        &self.0[index.y as usize][index.x as usize]
    }
}

impl IndexMut<&Coordinates> for BigGrid {
    fn index_mut(&mut self, index: &Coordinates) -> &mut Self::Output {
        &mut self.0[index.y as usize][index.x as usize]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BigWarehouse {
    pub grid: BigGrid,
    pub robot_positions: Coordinates,
    pub instructions: Vec<Instruction>,
}

impl From<&Warehouse> for BigWarehouse {
    fn from(value: &Warehouse) -> Self {
        // Transform the warehouse map to build a big warehouse map
        let grid = BigGrid(
            value
                .grid
                .0
                .iter()
                .map(|line| {
                    line.iter()
                        .flat_map(|tile| match tile {
                            MapTile::Wall => [BigMapTile::Wall, BigMapTile::Wall],
                            MapTile::Box => [BigMapTile::LeftBox, BigMapTile::RightBox],
                            MapTile::Empty => [BigMapTile::Empty, BigMapTile::Empty],
                        })
                        .collect()
                })
                .collect(),
        );

        // Move the robot coordinates
        let robot_positions = Coordinates::new(value.robot_positions.x * 2, value.robot_positions.y);

        BigWarehouse {
            grid,
            robot_positions,
            instructions: value.instructions.clone(),
        }
    }
}

impl Display for BigWarehouse {
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

impl BigWarehouse {
    pub fn move_robot(&mut self, instruction: &Instruction) {
        // Get the movement vector
        let movement_vector = instruction.get_movement_vec();

        let mut to_check = VecDeque::from([self.robot_positions.clone()]);
        let mut checked = HashSet::new();

        // The robot can move if an empty tile is located in the moving direction
        // and if no wall is present before.
        // Since the map contains wall all around, we do not need to check for boundaries and
        // we now that our loop will always end
        while let Some(mut current) = to_check.pop_front() {
            // Move it
            current += &movement_vector;

            // If the position is a wall, nothing can move, return
            if self.grid[&current] == BigMapTile::Wall {
                return;
            }

            // Check if the current position contains a box
            let big_box = if self.grid[&current] == BigMapTile::LeftBox {
                Some(current.clone())
            } else if self.grid[&current] == BigMapTile::RightBox {
                Some(current.clone() + &LEFT)
            } else {
                None
            };

            // If so, check if we already handled it, if not add its coordinates to the checklist
            if let Some(big_box) = big_box {
                if !checked.contains(&big_box) {
                    to_check.push_back(big_box.clone());
                    to_check.push_back(big_box.clone() + &RIGHT);
                    checked.insert(big_box);
                }
            }
        }

        // Here we should have a list of box to move
        // 1. Remove old boxes
        for big_box in &checked {
            self.grid[big_box] = BigMapTile::Empty;
            self.grid[&(big_box.clone() + &RIGHT)] = BigMapTile::Empty;
        }
        // 2. Add new ones
        for mut big_box in checked {
            big_box += &movement_vector;
            self.grid[&big_box] = BigMapTile::LeftBox;
            big_box += &RIGHT;
            self.grid[&big_box] = BigMapTile::RightBox;
        }
        // 3. Move the robot
        self.robot_positions += &movement_vector;
    }

    pub fn boxes_score(&self) -> usize {
        let mut score = 0;
        for (y, line) in self.grid.0.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if tile == &BigMapTile::LeftBox {
                    score += 100 * y + x;
                }
            }
        }
        score
    }
}
