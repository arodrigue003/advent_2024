use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Machine {
    pub a: (i64, i64),
    pub b: (i64, i64),
    pub target: (i64, i64),
}

impl Display for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(A:+{:<3},+{:<3}; B:+{:<3},+{:<3}; target:{:6},{:6})",
            self.a.0, self.a.1, self.b.0, self.b.1, self.target.0, self.target.1
        )
    }
}

/// Contains the solution of the equation ax + by = c given by the solve_diophantine function where
/// `x0 - n * a0` and `y0 + n * b0` for `0 <= n <= max_n` are every positive solution of the
/// equation ax + by = c
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiophantineSolution {
    pub x0: i64,
    pub a0: i64,
    pub y0: i64,
    pub b0: i64,
    pub max_n: i64,
}
