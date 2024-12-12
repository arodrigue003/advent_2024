use crate::day12::models::{MapWithBorder, Region};

/// Visit a neighbor
fn visit_neighbor(
    map: &MapWithBorder,
    visited: &mut Vec<Vec<bool>>,
    plant_type: char,
    line: usize,
    column: usize,
) -> Region {
    let neighbor = map.grid[line][column];
    if neighbor != plant_type {
        // The region is terminated here, this mean we have a new border, add it to the score
        return Region::new(0, 1);
    };

    // Check up if we didn't do it yet
    if visited[line][column] {
        Region::new(0, 0)
    } else {
        // Visit the rest of the region
        visit_region_rec(map, visited, plant_type, line, column)
    }
}

/// Compute the size of a region.
///
/// Returns
/// * The tuple (area, perimeter_length)
fn visit_region_rec(
    map: &MapWithBorder,
    visited: &mut Vec<Vec<bool>>,
    plant_type: char,
    line: usize,
    column: usize,
) -> Region {
    // The stopping condition is on visits of neighbours.
    // This ensures that the tile we are on is always a plant_type one.
    if visited[line][column] {
        return Region::new(0, 0);
    }

    // Mark the current tile as visited
    visited[line][column] = true;

    // Score accumulator, we start with an area of one for the tile we are standing on.
    let mut result = Region::new(1, 0);

    result += visit_neighbor(map, visited, plant_type, line - 1, column); // up
    result += visit_neighbor(map, visited, plant_type, line, column + 1); // right
    result += visit_neighbor(map, visited, plant_type, line + 1, column); // down
    result += visit_neighbor(map, visited, plant_type, line, column - 1); // left

    result
}

pub fn solve_part_one(map: &MapWithBorder) -> usize {
    let mut visited = vec![vec![false; map.width]; map.height];
    let mut total_cost = 0;
    for line in 1..map.height - 1 {
        for column in 1..map.width - 1 {
            let res = visit_region_rec(map, &mut visited, map.grid[line][column], line, column);
            // println!("{}:{line}:{column} => {res}", map.grid[line][column]);
            total_cost += res.cost();
        }
    }

    total_cost
}

/// identify a neighbor
fn identify_neighbor(
    map: &MapWithBorder,
    visited: &mut Vec<Vec<bool>>,
    regions: &mut Vec<Vec<usize>>,
    plant_type: char,
    region_id: usize,
    line: usize,
    column: usize,
) {
    let neighbor = map.grid[line][column];
    if neighbor != plant_type {
        // We are not in our region anymore
        return;
    };

    // Check up if we didn't do it yet
    if visited[line][column] {
    } else {
        // Visit the rest of the region
        identify_regions_rec(map, visited, regions, plant_type, region_id, line, column);
    }
}

fn identify_regions_rec(
    map: &MapWithBorder,
    visited: &mut Vec<Vec<bool>>,
    regions: &mut Vec<Vec<usize>>,
    plant_type: char,
    region_id: usize,
    line: usize,
    column: usize,
) {
    // mark the current tile as visited and add it to the region
    visited[line][column] = true;
    regions[line][column] = region_id;

    // Visit neighbors.
    identify_neighbor(map, visited, regions, plant_type, region_id, line - 1, column);
    identify_neighbor(map, visited, regions, plant_type, region_id, line, column + 1);
    identify_neighbor(map, visited, regions, plant_type, region_id, line + 1, column);
    identify_neighbor(map, visited, regions, plant_type, region_id, line, column - 1);
}

fn identify_regions(map: &MapWithBorder) -> (usize, Vec<Vec<usize>>) {
    let mut visited = vec![vec![false; map.width]; map.height];
    let mut regions = vec![vec![0; map.width]; map.height];
    let mut region_id: usize = 1;

    for line in 1..map.height - 1 {
        for column in 1..map.width - 1 {
            if visited[line][column] {
                // Dont visit regions we already visited
                continue;
            }

            identify_regions_rec(
                map,
                &mut visited,
                &mut regions,
                map.grid[line][column],
                region_id,
                line,
                column,
            );
            region_id += 1;
        }
    }

    (region_id, regions)
}

fn update_border_count(regions_border: &mut [usize], last_tuple: &(usize, usize), new_tuple: &(usize, usize)) {
    // A A
    // A A
    // No border
    if last_tuple == new_tuple {
        return;
    }

    // A B or A B
    // A B    C B
    // No horizontal border. Vertical border is counted separately
    if new_tuple.0 == new_tuple.1 {
        return;
    }

    // A A
    // B B
    // We are only continuing a running border
    if last_tuple.0 == new_tuple.0 && last_tuple.1 == new_tuple.1 {
        return;
    }

    // A A or B A
    // B A    A A
    // No new border, we only terminated the last one
    if (last_tuple.0 == new_tuple.0 && last_tuple.0 == new_tuple.1)
        || (last_tuple.1 == new_tuple.0 && last_tuple.1 == new_tuple.1)
    {
        return;
    }

    // We only have new border to borders that changed
    // A B or A A
    // C C    A B
    if new_tuple.0 != last_tuple.0 || new_tuple.0 == last_tuple.1 {
        regions_border[new_tuple.0] += 1;
    }

    // C C or A B
    // A B    A A
    if new_tuple.1 != last_tuple.1 || new_tuple.1 == last_tuple.0 {
        regions_border[new_tuple.1] += 1;
    }
}

fn compute_border_count(map: &MapWithBorder, region_count: usize, regions: &[Vec<usize>]) -> Vec<usize> {
    // Now compute the number of border of each region
    let mut region_borders = vec![0; region_count];

    // horizontal traversal. Two lines at a time
    for line in 0..map.height - 1 {
        let mut last_tuple = (regions[line][0], regions[line + 1][0]);
        for column in 1..map.width - 1 {
            let new_tuple = (regions[line][column], regions[line + 1][column]);

            // Update border count
            update_border_count(&mut region_borders, &last_tuple, &new_tuple);

            // Update last_tuple
            last_tuple = new_tuple;
        }
    }

    // Vertical traversal. Two columns at a time
    for column in 0..map.width - 1 {
        let mut last_tuple = (regions[0][column], regions[0][column + 1]);
        for line in regions.iter().take(map.height - 1).skip(1) {
            let new_tuple = (line[column], line[column + 1]);

            // Update border count
            update_border_count(&mut region_borders, &last_tuple, &new_tuple);

            // Update last_tuple
            last_tuple = new_tuple;
        }
    }

    region_borders
}

fn compute_regions_sizes(region_count: usize, regions: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut regions_sizes = vec![0; region_count];

    for line in regions {
        for tile in line {
            regions_sizes[*tile] += 1;
        }
    }

    regions_sizes
}

pub fn solve_part_two(map: &MapWithBorder) -> usize {
    // First, assign a unique id to every region
    let (region_count, regions) = identify_regions(map);

    let region_borders = compute_border_count(map, region_count, &regions);
    let region_sizes = compute_regions_sizes(region_count, &regions);

    region_borders
        .iter()
        .zip(&region_sizes)
        .skip(1)
        .map(|(border, area)| *border * *area)
        .sum()
}
