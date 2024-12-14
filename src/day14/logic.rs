use crate::day14::models::{Bathroom, Robot};
use std::collections::HashMap;

fn simulate_robot(robot: &Robot, width: i64, height: i64, steps: usize) -> (i64, i64) {
    let mut cur_x = robot.x;
    let mut cur_y = robot.y;
    for _ in 0..steps {
        cur_x = (cur_x + robot.vx + width) % width;
        cur_y = (cur_y + robot.vy + height) % height;
    }

    (cur_x, cur_y)
}

pub fn solve_part_one(bathroom: &Bathroom) -> usize {
    let mut robot_final_positions: HashMap<(i64, i64), usize> = HashMap::new();

    // Simulate robots
    for robot in &bathroom.robots {
        *robot_final_positions
            .entry(simulate_robot(robot, bathroom.width, bathroom.height, 100))
            .or_default() += 1;
    }

    // Count robots in each quadrant
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for ((x, y), count) in robot_final_positions {
        if x < bathroom.width / 2 && y < bathroom.height / 2 {
            q1 += count;
        } else if x > bathroom.width / 2 && y < bathroom.height / 2 {
            q2 += count;
        } else if x < bathroom.width / 2 && y > bathroom.height / 2 {
            q3 += count;
        } else if x> bathroom.width / 2 && y > bathroom.height / 2 {
            q4 += count;
        }
    }

    // Return the resul
    q1*q2*q3*q4
}

pub fn solve_part_two(bathroom: &Bathroom) -> usize {
    0
}
