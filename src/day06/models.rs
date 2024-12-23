use std::fmt::{Display, Formatter};

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Right => write!(f, "Right"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
        }
    }
}

impl Direction {
    pub fn next(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn get_lookup_index(&self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

#[derive(Hash, Default, Debug, Clone, Eq, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{})", self.x, self.y)
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn next(&self, direction: Direction) -> Self {
        // direction x and y value are always > 0 because of how we are using them and the
        // fact that we added a border to the map
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Floor,
    Wall,
    Outside,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Floor => write!(f, "."),
            Tile::Wall => write!(f, "#"),
            Tile::Outside => write!(f, "x"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LabWithABorder {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Tile>>,
    pub start_position: Position,
}

impl LabWithABorder {
    pub fn new(grid: Vec<Vec<Tile>>, start_position: Position) -> Self {
        let height = grid.len();
        let width = grid[0].len();

        // Add a border to simplify further computation
        let grid: Vec<Vec<_>> = std::iter::once(vec![Tile::Outside; width + 2])
            .chain(grid.into_iter().map(|line| {
                std::iter::once(Tile::Outside)
                    .chain(line)
                    .chain(std::iter::once(Tile::Outside))
                    .collect()
            }))
            .chain(std::iter::once(vec![Tile::Outside; width + 2]))
            .collect();

        Self {
            width: width + 2,
            height: height + 2,
            grid,
            start_position: Position::new(start_position.x + 1, start_position.y + 1),
        }
    }
}

impl Display for LabWithABorder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (y, line) in self.grid.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if self.start_position.x == x && self.start_position.y == y {
                    write!(f, "^")?;
                } else {
                    write!(f, "{tile}")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
