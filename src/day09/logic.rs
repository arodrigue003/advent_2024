use std::collections::BTreeSet;

use crate::day09::models::{Block, EmptySpace, File};

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
    let mut min_pos = (0, usize::MAX);
    for (bracket, item) in bitmap.iter().enumerate().take(10).skip(len) {
        if let Some(position) = item.first() {
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
    let mut disk: Vec<_> = disk.into_iter().flatten().collect();

    // Create the bitmap
    // 1. filter out empty elements
    let empty_spaces: Vec<_> = empty_spaces.into_iter().flatten().collect();
    // 2. create the bitmap
    let mut bitmap: Vec<_> = vec![BTreeSet::new(); 10];
    // 3. fill it
    for empty_space in empty_spaces {
        bitmap[empty_space.size].insert(empty_space.position);
    }

    // Try to compact the disk from the end
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

            // We don't need to mark the file used space as free since we treat them from the back

            //  Move the file
            file.position = position;
        }
    }

    // Compute the checksum
    disk.iter()
        .map(|file| (0..file.size).map(|idx| (file.position + idx) * file.id).sum::<usize>())
        .sum()
}
