use crate::day15::models::coordinates::Coordinates;

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum Instruction {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => unreachable!(),
        }
    }
}

impl Instruction {
    pub fn get_movement_vec(&self) -> Coordinates {
        match self {
            Instruction::Up => Coordinates::new(0, -1),
            Instruction::Right => Coordinates::new(1, 0),
            Instruction::Down => Coordinates::new(0, 1),
            Instruction::Left => Coordinates::new(-1, 0),
        }
    }
}