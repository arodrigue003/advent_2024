use hashbrown::HashSet;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Onsen {
    pub available_towels: Vec<String>,
    pub available_towels_set: HashSet<String>,
    pub target_designs: Vec<String>,
}

impl Onsen {
    pub fn new(available_towels: Vec<String>, target_designs: Vec<String>) -> Self {
        let available_towels_set = available_towels.iter().cloned().collect();
        Self {
            available_towels,
            available_towels_set,
            target_designs,
        }
    }
}
