use crate::day15::models::Warehouse;

pub fn solve_part_one(warehouse: &Warehouse) -> usize {
    let mut mut_warehouse = warehouse.clone();

    for instruction in &warehouse.instructions {
        mut_warehouse.move_robot(instruction);
    }

    mut_warehouse.boxes_score()
}

pub fn solve_part_two(warehouse: &Warehouse) -> u32 {
    0
}
