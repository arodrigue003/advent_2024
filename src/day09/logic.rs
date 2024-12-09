use crate::day09::models::{Block, EmptySpace, File};
use std::collections::BTreeSet;

static BITMAP_SIZE: usize = 100000;

pub fn compact_disk(mut disk: Vec<Block>) -> Vec<Block> {
    // Compact it
    let mut first_empty: usize = 0;
    while first_empty < disk.len() {
        // Look for a valid first_empty position
        while let Block::File(_) = &disk[first_empty] {
            first_empty += 1;
            if first_empty >= disk.len() {
                return disk;
            }
        }

        // Get the last file from the disk
        #[allow(unused_assignments)]
        let mut last_elem = usize::MAX;
        loop {
            let last = disk.pop().unwrap();
            match last {
                Block::Empty => {
                    // check if we emptied the free spaces, in this case we are done
                    if first_empty >= disk.len() {
                        return disk;
                    }
                }
                Block::File(id) => {
                    last_elem = id;
                    break;
                }
            }
        }

        // Put the last elem at the first free space
        disk[first_empty] = Block::File(last_elem);
    }

    disk
}

/// Compute the checksum of a disk by ignoring empty spaces
pub fn compute_checksum(disk: &[Block]) -> usize {
    disk.iter()
        .enumerate()
        .map(|(pos, elem)| match elem {
            Block::Empty => 0,
            Block::File(id) => id * pos,
        })
        .sum()
}

pub fn solve_part_one(data: &[usize]) -> usize {
    // Build the disk
    let disk: Vec<_> = data
        .iter()
        .enumerate()
        .flat_map(|(idx, elt)| {
            if idx % 2 == 0 {
                // We are on a file node
                vec![Block::File(idx / 2); *elt]
            } else {
                // We are on an empty node
                vec![Block::Empty; *elt]
            }
        })
        .collect();

    // Compact the disk
    let disk = compact_disk(disk);

    // Compute checksum
    compute_checksum(&disk)
}

/// Look for the first empty space that can contains the file data if it exists
/// First element is the bracket, second one is the position.
fn compute_first_empty_space(bitmap: &[BTreeSet<usize>], len: usize) -> Option<(usize, usize)> {
    // Look for empty spaces in the tree
    let mut min_pos = (0,usize::MAX);
    for bracket in len..BITMAP_SIZE {
        if let Some(position) = bitmap[bracket].first() {
            if *position < min_pos.1 {
                min_pos = (bracket, *position);
            }
        }
    }

    if min_pos.1 != usize::MAX {
        Some(min_pos)
    } else {
        None
    }
}

pub fn solve_part_two(data: &[usize]) -> usize {
    // Create the list of files
    let mut current_position: usize = 0;
    let (disk, empty_spaces): (Vec<_>, Vec<_>) = data
        .iter()
        .enumerate()
        .map(|(idx, elt)| {
            let file = if idx % 2 == 0 {
                // We are on a file node
                (
                    Some(File {
                        id: idx / 2,
                        position: current_position,
                        size: *elt,
                    }),
                    None,
                )
            } else {
                (
                    None,
                    Some(EmptySpace {
                        position: current_position,
                        size: *elt,
                    }),
                )
            };

            // Increase the position
            current_position += *elt;

            // Return the file if it was created
            file
        })
        .collect();

    // Get the list of files
    let mut disk: Vec<_> = disk.into_iter().filter_map(|elt| elt).collect();

    // Create the bitmap
    // 1. filter out empty elements
    let empty_spaces: Vec<_> = empty_spaces.into_iter().filter_map(|elt| elt).collect();
    // 2. create the bitmap
    let mut bitmap: Vec<_> = vec![BTreeSet::new(); BITMAP_SIZE];
    // 3. fill it
    for empty_space in empty_spaces {
        bitmap[empty_space.size].insert(empty_space.position);
    }

    // Try to compact the disk from the end
    // 1. get the highest pos
    let mut latest_pos = disk[disk.len()-1].position;
    // 2. Do the compacting
    for file in disk.iter_mut().rev() {
        if let Some((bracket, position)) = compute_first_empty_space(&bitmap, file.size) {
            // Check if the new position is an improvement
            if position >= file.position {
                continue;
            }

            // Remove the position in the bracket
            bitmap[bracket].remove(&position);

            // If needed, add the remaining free space to the according bracket
            if bracket > file.size {
                bitmap[bracket - file.size].insert(position + file.size);
            }

            // Add the fill freed space to the bitmap
            // 1. look for the free space before the file
            let mut start = file.position;
            for bracket in 0..BITMAP_SIZE {
                if file.position >= bracket && bitmap[bracket].get(&(file.position - bracket)).is_some() {
                    // We found the free space before the file, remove it to add it to the pool
                    bitmap[bracket].remove(&(file.position - bracket));
                    start = file.position - bracket;
                    break
                }
            }
            // 2. look for the free space after the file
            let mut end = file.position + file.size;
            for bracket in 0..BITMAP_SIZE {
                if bitmap[bracket].get(&(file.position + file.size)).is_some() {
                    // We found the free space before the file, remove it to add it to the pool
                    bitmap[bracket].remove(&(file.position + file.size));
                    end = file.position + file.size + bracket;
                    break
                }
            }
            // 3. add the new space to the bracket if we are not at the end
            bitmap[end-start].insert(start);

            //  Move the file
            file.position = position;

            // println!("{:#?}", &bitmap);
        }
    }

    // Compute the checksum
    disk.iter()
        .map(|file| (0..file.size).map(|idx| (file.position + idx) * file.id).sum::<usize>())
        .sum()
}
