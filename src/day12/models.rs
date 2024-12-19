use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MapWithBorder {
    pub grid: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl MapWithBorder {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let width = grid[0].len();
        let height = grid.len();

        // Add a border to simplify further computation
        let grid: Vec<Vec<_>> = std::iter::once(vec!['.'; width + 2])
            .chain(
                grid.into_iter()
                    .map(|line| std::iter::once('.').chain(line).chain(std::iter::once('.')).collect()),
            )
            .chain(std::iter::once(vec!['.'; width + 2]))
            .collect();

        Self {
            grid,
            width: width + 2,
            height: height + 2,
        }
    }
}

impl Display for MapWithBorder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in &self.grid {
            for block in line {
                write!(f, "{}", block)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Region {
    area: usize,
    perimeter: usize,
}

impl Region {
    pub fn new(area: usize, perimeter: usize) -> Self {
        Self { area, perimeter }
    }

    pub fn cost(&self) -> usize {
        self.area * self.perimeter
    }
}

impl Add for Region {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            area: self.area + rhs.area,
            perimeter: self.perimeter + rhs.perimeter,
        }
    }
}

impl AddAssign for Region {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            area: self.area + rhs.area,
            perimeter: self.perimeter + rhs.perimeter,
        }
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(area: {}, perimeter: {})", self.area, self.perimeter)
    }
}
