use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operation {
    And,
    Xor,
    Or,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Self::And,
            "XOR" => Self::Xor,
            "OR" => Self::Or,
            _ => unreachable!(),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::And => write!(f, "AND"),
            Operation::Xor => write!(f, "XOR"),
            Operation::Or => write!(f, "OR"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Gate {
    pub left: String,
    pub operation: Operation,
    pub right: String,
    pub result: String,
}

impl Display for Gate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} -> {}", self.left, self.operation, self.right, self.result)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]

pub struct Wire {
    pub name: String,
    pub value: bool,
}

impl Display for Wire {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]

pub struct System {
    pub wires: Vec<Wire>,
    pub gates: Vec<Gate>,
}

impl Display for System {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for wire in &self.wires {
            writeln!(f, "{wire}")?;
        }
        writeln!(f)?;
        for gate in &self.gates {
            writeln!(f, "{gate}")?;
        }
        Ok(())
    }
}
