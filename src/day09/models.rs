#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Block {
    Empty,
    File(usize),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct File {
    pub id: usize,
    pub position: usize,
    pub size: usize
}