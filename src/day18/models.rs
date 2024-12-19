#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Corruption {
    pub size: usize,
    pub to_simulate: usize,
    pub bytes: Vec<(usize, usize)>,
}

impl Corruption {
    pub fn new(bytes: Vec<(usize, usize)>) -> Self {
        let (size, to_simulate) = if bytes.iter().all(|(x, y)| *x <= 6 && *y <= 6) {
            (7, 12)
        } else {
            (71, 1024)
        };

        Self { size, to_simulate, bytes }
    }
}
