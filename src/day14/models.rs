use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Robot {
    pub x: i64,
    pub y: i64,
    pub vx: i64,
    pub vy: i64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Bathroom {
    pub robots: Vec<Robot>,

    pub width: i64,
    pub height: i64,
}

impl Bathroom {
    pub fn create_grid(&self) -> Vec<Vec<usize>> {
        let mut grid = vec![vec![0; self.width as usize]; self.height as usize];

        for robot in &self.robots {
            grid[robot.y as usize][robot.x as usize] += 1;
        }

        grid
    }
}

impl Display for Bathroom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let grid = self.create_grid();

        for line in grid {
            for block in line {
                if block == 0 {
                    write!(f, ".")?
                } else {
                    write!(f, "#")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
