use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Machine {
    pub a: (i64, i64),
    pub b: (i64, i64),
    pub target: (i64, i64),
}

impl Display for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "(A:+{:<3},+{:<3}; B:+{:<3},+{:<3}; target:{:6},{:6})",
            self.a.0,
            self.a.1, self.b.0, self.b.1, self.target.0, self.target.1
        )
    }
}
