use hashbrown::{HashMap, HashSet};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ManualUpdates {
    pub rules: HashMap<i32, HashSet<i32>>,
    pub updates: Vec<Vec<i32>>
}
