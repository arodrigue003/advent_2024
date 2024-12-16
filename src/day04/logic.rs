use std::cmp::min;

static PATTERN: [char; 4] = ['X', 'M', 'A', 'S'];
static ANTI_PATTERN: [char; 4] = ['S', 'A', 'M', 'X'];

/// We assume the grid is a square
#[allow(clippy::needless_range_loop)]
pub fn solve_part_one(grid: &[Vec<char>]) -> u32 {
    let width = grid[0].len();
    let height = grid.len();

    // Count
    let mut count = 0;

    // Init walkers
    let mut pattern_pos;
    let mut anti_pattern_pos;

    // Look horizontally
    for line in grid {
        pattern_pos = 0;
        anti_pattern_pos = 0;
        for item in line {
            search_pattern(&PATTERN, *item, &mut pattern_pos, &mut count);
            search_pattern(&ANTI_PATTERN, *item, &mut anti_pattern_pos, &mut count);
        }
    }

    // Look vertically
    for column in 0..grid[0].len() {
        pattern_pos = 0;
        anti_pattern_pos = 0;
        for line in 0..grid.len() {
            search_pattern(&PATTERN, grid[line][column], &mut pattern_pos, &mut count);
            search_pattern(&ANTI_PATTERN, grid[line][column], &mut anti_pattern_pos, &mut count);
        }
    }

    // Look on the positive diagonal first part
    for i in 0..height {
        pattern_pos = 0;
        anti_pattern_pos = 0;
        for j in 0..min(i+1, width) {
            let item = grid[i - j][j];
            search_pattern(&PATTERN, item, &mut pattern_pos, &mut count);
            search_pattern(&ANTI_PATTERN, item, &mut anti_pattern_pos, &mut count);
        }
    }

    // Look on the positive diagonal second part
    for i in 1..width {
        pattern_pos = 0;
        anti_pattern_pos = 0;
        for j in 0..min(width-i, height) {
            let item = grid[height - j - 1][i + j];
            search_pattern(&PATTERN, item, &mut pattern_pos, &mut count);
            search_pattern(&ANTI_PATTERN, item, &mut anti_pattern_pos, &mut count);
        }
    }

    // Look on the negative diagonal first part
    for i in 0..height {
        pattern_pos = 0;
        anti_pattern_pos = 0;
        for j in 0..min(i+1, width) {
            let item = grid[height + j - i - 1][j];
            search_pattern(&PATTERN, item, &mut pattern_pos, &mut count);
            search_pattern(&ANTI_PATTERN, item, &mut anti_pattern_pos, &mut count);
        }
    }

    // Look on the negative diagonal second part
    for i in 1..width {
        pattern_pos = 0;
        anti_pattern_pos = 0;
        for j in 0..min(width-i, height) {
            let item = grid[j][i+j];
            search_pattern(&PATTERN, item, &mut pattern_pos, &mut count);
            search_pattern(&ANTI_PATTERN, item, &mut anti_pattern_pos, &mut count);
        }
    }

    count
}

fn search_pattern(pattern: &'static [char; 4], item: char, pattern_pos: &mut usize, count: &mut u32) {
    if item == pattern[*pattern_pos] {
        *pattern_pos += 1;
        if pattern_pos == &pattern.len() {
            *count += 1;
            *pattern_pos = 0;
        }
    } else if item == pattern[0] {
        *pattern_pos = 1;
    } else {
        *pattern_pos = 0;
    }
}

pub fn solve_part_two(grid: &[Vec<char>]) -> u32 {
    let width = grid[0].len();
    let height = grid.len();

    // walkers
    let mut m_count;
    let mut s_count;

    // Count
    let mut count = 0;

    for i in 1..height-1 {
        for j in 1..width-1 {
            if grid[i][j] == 'A' {
                m_count = 0;
                s_count = 0;
                match grid[i-1][j-1] {
                    'M' => m_count += 1,
                    'S' => s_count += 1,
                    _ => {}
                }
                match grid[i+1][j+1] {
                    'M' => m_count += 1,
                    'S' => s_count += 1,
                    _ => {}
                }
                // Prevent MAM and SAS words
                if m_count ==2 || s_count == 2 {
                    continue
                }
                match grid[i-1][j+1] {
                    'M' => m_count += 1,
                    'S' => s_count += 1,
                    _ => {}
                }
                match grid[i+1][j-1] {
                    'M' => m_count += 1,
                    'S' => s_count += 1,
                    _ => {}
                }
                if m_count == 2 && s_count == 2 {
                    count += 1;
                }
            }
        }
    }

    count
}
