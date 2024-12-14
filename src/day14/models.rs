#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Robot {
    pub x: i64,
    pub y: i64,
    pub vx: i64,
    pub vy: i64
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Bathroom {
    pub robots: Vec<Robot>,

    pub width: i64,
    pub height: i64
}