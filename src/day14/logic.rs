use hashbrown::{HashMap, HashSet};

use crate::day14::models::{Bathroom, Robot};

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
        } else if x > bathroom.width / 2 && y > bathroom.height / 2 {
            q4 += count;
        }
    }

    // Return the resul
    q1 * q2 * q3 * q4
}

fn step(bathroom: &mut Bathroom) {
    for robot in &mut bathroom.robots {
        robot.x = (robot.x + robot.vx + bathroom.width) % bathroom.width;
        robot.y = (robot.y + robot.vy + bathroom.height) % bathroom.height;
    }
}

fn check_if_might_work(bathroom: &Bathroom) -> bool {
    // Check for the alignment of at least 10 robots to form a Christmas tree

    // First put robots coordinates in a set
    let robots: HashSet<_> = bathroom.robots.iter().map(|robot| (robot.x, robot.y)).collect();

    // Iterate over robots and check if we can find 10 friends at his bottom
    for robot in &bathroom.robots {
        if (1..10).all(|offset| robots.contains(&(robot.x, robot.y + offset))) {
            return true;
        }
    }

    false
}

pub fn solve_part_two(bathroom: &Bathroom) -> usize {
    // Part two does not work for the example, return 0 for it
    if bathroom.width == 11 {
        return 0;
    }

    let mut bathroom = bathroom.clone();
    let mut current_step = 0;

    loop {
        current_step += 1;
        step(&mut bathroom);

        if check_if_might_work(&bathroom) {
            // println!("{}", &bathroom);
            break;
        }
    }

    current_step
}
