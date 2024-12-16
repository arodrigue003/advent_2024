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
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, line) in self.grid.iter().enumerate() {
            for (j, tile) in line.iter().enumerate() {
                if self.start.0 == i && self.start.1 == j {
                    write!(f, "S")?;
                } else if self.end.0 == i && self.end.1 == j {
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