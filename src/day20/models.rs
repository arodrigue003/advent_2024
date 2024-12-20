use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Floor,
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Floor => write!(f, "."),
            Tile::Wall => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Map {
    pub grid: Vec<Vec<Tile>>,
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub width: usize,
    pub height: usize,
    pub save_target: usize,
}

impl Map {
    pub fn new(grid: Vec<Vec<Tile>>, start: (usize, usize), end: (usize, usize)) -> Self {
        let height = grid.len();
        let width = grid[0].len();
        let save_target = if height < 100 { 10 } else { 100 };

        Self {
            grid,
            start,
            end,
            width,
            height,
            save_target,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (y, line) in self.grid.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if self.start.0 == x && self.start.1 == y {
                    write!(f, "S")?;
                } else if self.end.0 == x && self.end.1 == y {
                    write!(f, "E")?;
                } else {
                    write!(f, "{tile}")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
