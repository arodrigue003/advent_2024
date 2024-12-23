use crate::day06::models::{Direction, LabWithABorder, Position, Tile};
use hashbrown::HashSet;

type Lookup = Vec<Vec<[Option<usize>; 4]>>;
type Changes = Vec<(Position, Direction, Option<usize>)>;

pub fn prepare(lab: &LabWithABorder) -> Vec<Vec<bool>> {
    // Create a list of visited tiles
    let mut visited = vec![vec![false; lab.width]; lab.height];

    // Simulate the guard visit
    let mut guard_position = lab.start_position.clone();
    let mut guard_direction = Direction::Up;
    loop {
        // Add the guard position to the visited list
        visited[guard_position.y][guard_position.x] = true;

        // Get the next position
        let next = guard_position.next(guard_direction);

        // If we are outside, we are done.
        // If the next position is occupied, we only get the next direction.
        match lab.grid[next.y][next.x] {
            Tile::Floor => guard_position = next,
            Tile::Wall => guard_direction = guard_direction.next(),
            Tile::Outside => break,
        }
    }

    visited
}

pub fn solve_part_one(visited: &[Vec<bool>]) -> usize {
    visited.iter().map(|line| line.iter().filter(|x| **x).count()).sum()
}

pub fn solve_part_two(lab: &LabWithABorder, visited: &[Vec<bool>]) -> u32 {
    let mut lookup = build_lookup_table(lab);

    let mut possible_loops = 0;
    for (y, line) in visited.iter().enumerate() {
        for (x, visited) in line.iter().enumerate() {
            if *visited == true && (lab.start_position.x != x || lab.start_position.y != y) {
                // The tile was visited and is not the starting tile

                // Add the wall to the map
                let changes = add_wall(&Position::new(x, y), &mut lookup, lab.width, lab.height);

                // Check if the guard does a loop
                if does_loop(&lab.start_position, Direction::Up, &lookup) {
                    possible_loops += 1;
                }

                // Revert the add_wall operation
                revert_changes(changes, &mut lookup);
            }
        }
    }

    possible_loops
}

/// Add a wall at the given position
fn add_wall(position: &Position, lookup: &mut Lookup, width: usize, height: usize) -> Changes {
    // Store change to simplify reverting them after
    let mut original = vec![];

    // we start at the wall line, for every lookup at it's left that did not change from the
    // lookup that was present at the wall position, change it to reference this new lookup.
    let right_index = Direction::Right.get_lookup_index();
    let initial_lookup = lookup[position.y][position.x][right_index].clone();
    for x in (0..position.x).rev() {
        if &lookup[position.y][x][right_index] == &initial_lookup {
            // Store the initial value for later
            original.push((Position::new(x, position.y), Direction::Right, initial_lookup.clone()));

            // Change the value
            lookup[position.y][x][right_index] = Some(position.x - 1);
        } else {
            break;
        }
    }

    // we start at the wall line, for every lookup at it's right that did not change from the
    // lookup that was present at the wall position, change it to reference this new lookup.
    let left_index = Direction::Left.get_lookup_index();
    let initial_lookup = lookup[position.y][position.x][left_index].clone();
    for x in position.x..width {
        if &lookup[position.y][x][left_index] == &initial_lookup {
            // Store the initial value for later
            original.push((Position::new(x, position.y), Direction::Left, initial_lookup.clone()));

            // Change the value
            lookup[position.y][x][left_index] = Some(position.x + 1);
        } else {
            break;
        }
    }

    // we start at the wall column, for every lookup upper that did not change from the
    // lookup that was present at the wall position, change it to reference this new lookup.
    let down_index = Direction::Down.get_lookup_index();
    let initial_lookup = lookup[position.y][position.x][down_index].clone();
    for y in (0..position.y).rev() {
        if &lookup[y][position.x][down_index] == &initial_lookup {
            // Store the initial value for later
            original.push((Position::new(position.x, y), Direction::Down, initial_lookup.clone()));

            // Change the value
            lookup[y][position.x][down_index] = Some(position.y - 1);
        } else {
            break;
        }
    }

    // we start at the wall column, for every lookup lower that did not change from the
    // lookup that was present at the wall position, change it to reference this new lookup.
    let up_index = Direction::Up.get_lookup_index();
    let initial_lookup = lookup[position.y][position.x][up_index].clone();
    for y in position.y..height {
        if &lookup[y][position.x][up_index] == &initial_lookup {
            // Store the initial value for later
            original.push((Position::new(position.x, y), Direction::Up, initial_lookup.clone()));

            // Change the value
            lookup[y][position.x][up_index] = Some(position.y + 1);
        } else {
            break;
        }
    }

    original
}


/// Revert the last add_wall operation
fn revert_changes(changes: Changes, lookup: &mut Lookup) {
    for (position, direction, value) in changes {
        lookup[position.y][position.x][direction.get_lookup_index()] = value;
    }
}

fn does_loop(position: &Position, direction: Direction, lookup: &Lookup) -> bool {
    // Store visited vertical positions
    let mut visited = HashSet::new();

    // Set mutable variables for the position and the direction
    let mut position = position.clone();
    let mut direction = direction.clone();

    loop {
        // println!("{}=>{}", &position, &direction);

        // Teleport the guard to the next position
        if let Some(next_position) = lookup[position.y][position.x][direction.get_lookup_index()] {
            // Teleport the guard depending on the direction
            match direction {
                Direction::Up | Direction::Down => position.y = next_position,
                Direction::Right | Direction::Left => position.x = next_position,
            }

            // Add the new position to the list of visited position if the direction is up
            if direction == Direction::Up {
                // Stopping condition
                if visited.contains(&position) {
                    // println!("{:#?}", &visited);
                    return true;
                }

                visited.insert(position.clone());
            }

            // Rotate
            direction = direction.next();
        } else {
            // The guard is going OOB, this cannot be a loop
            // println!("{:#?}", &visited);
            return false;
        }
    }
}

fn build_lookup_table(lab: &LabWithABorder) -> Lookup {
    // Create lookup tables for every direction that allows us to tp the guard to the next
    // obstacle in constant time
    let mut lookup = vec![vec![[None, None, None, None]; lab.width]; lab.height];

    // Fill the up lookup, to do that, we iterate from top to bottom on every column
    let up_index = Direction::Up.get_lookup_index();
    for x in 0..lab.width {
        let mut last_wall = None;
        for y in 0..lab.height {
            if lab.grid[y][x] == Tile::Wall {
                // Update the last wall
                last_wall = Some(y + 1);
            } else {
                // Set the lookup tp position
                lookup[y][x][up_index] = last_wall;
            }
        }
    }

    // Fill the right lookup, to do that, we iterate from right to left on every line
    let right_index = Direction::Right.get_lookup_index();
    for y in 0..lab.height {
        let mut last_wall = None;
        for x in (0..lab.width).rev() {
            if lab.grid[y][x] == Tile::Wall {
                // Update the last wall
                last_wall = Some(x - 1);
            } else {
                // Set the lookup tp position
                lookup[y][x][right_index] = last_wall;
            }
        }
    }

    // Fill the down lookup, to do that, we iterate from bottom to top on every column
    let down_index = Direction::Down.get_lookup_index();
    for x in 0..lab.width {
        let mut last_wall = None;
        for y in (0..lab.height).rev() {
            if lab.grid[y][x] == Tile::Wall {
                // Update the last wall
                last_wall = Some(y - 1);
            } else {
                // Set the lookup tp position
                lookup[y][x][down_index] = last_wall;
            }
        }
    }

    // Fill the left lookup, to do that, we iterate from left to right on every line
    let left_index = Direction::Left.get_lookup_index();
    for (y, line) in lab.grid.iter().enumerate() {
        let mut last_wall = None;
        for (x, tile) in line.iter().enumerate() {
            if tile == &Tile::Wall {
                // Update the last wall
                last_wall = Some(x + 1);
            } else {
                // Set the lookup tp position
                lookup[y][x][left_index] = last_wall;
            }
        }
    }
    lookup
}
