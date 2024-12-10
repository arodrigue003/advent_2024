use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MapWithBorder {
    pub grid: Vec<Vec<u32>>,
    pub width: usize,
    pub height: usize,
}

impl MapWithBorder {
    pub fn new(grid: Vec<Vec<u32>>) -> Self {
        let width = grid[0].len();
        let height = grid.len();

        // Add a border to simplify further computation
        let grid: Vec<Vec<_>> = std::iter::once(vec![0; width + 2])
            .chain(
                grid.into_iter()
                    .map(|line| std::iter::once(0).chain(line).chain(std::iter::once(0)).collect()),
            )
            .chain(std::iter::once(vec![0; width + 2]))
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