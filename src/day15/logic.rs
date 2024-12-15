use crate::day15::models::big_warehouse::BigWarehouse;
use crate::day15::models::warehouse::Warehouse;

pub fn solve_part_one(warehouse: &Warehouse) -> usize {
    let mut mut_warehouse = warehouse.clone();

    for instruction in &warehouse.instructions {
        mut_warehouse.move_robot(instruction);
    }

    mut_warehouse.boxes_score()
}

pub fn solve_part_two(warehouse: &Warehouse) -> usize {
    let mut big_warehouse = BigWarehouse::from(warehouse);

    for instruction in &warehouse.instructions {
        big_warehouse.move_robot(instruction);
    }

    big_warehouse.boxes_score()
}
