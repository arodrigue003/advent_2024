use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};

#[derive(Default, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Coordinates {
    pub x: i64,
    pub y: i64,
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{})", self.x, self.y)
    }
}

impl Coordinates {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl PartialEq<(i64, i64)> for Coordinates {
    fn eq(&self, other: &(i64, i64)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

impl PartialEq<(usize, usize)> for Coordinates {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.x == other.0 as i64 && self.y == other.1 as i64
    }
}

impl Add<&Self> for Coordinates {
    type Output = Coordinates;

    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<&Self> for Coordinates {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}