use crate::day09::models::{Block, File};

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
        .map(|(idx, elt)| {
            if idx % 2 == 0 {
                // We are on a file node
                vec![Block::File(idx / 2); *elt]
            } else {
                // We are on an empty node
                vec![Block::Empty; *elt]
            }
        })
        .flatten()
        .collect();

    // Compact the disk
    let disk = compact_disk(disk);

    // Compute checksum
    compute_checksum(&disk)
}

/// Look for the first empty space that can contains the file data if it exists
fn compute_first_empty_space(bitmap: &[u8], len: usize) -> Option<usize> {
    let mut start: usize = usize::MAX;
    for (idx, elt) in bitmap.iter().enumerate() {
        // If the element is occupied, set start and skip this position as it is not valid
        if *elt == 1 {
            start = usize::MAX;
            continue;
        }

        // if start equals usize::MAX, it means last position was not valid, check if this one is.
        if start == usize::MAX && *elt == 0 {
            start = idx
        }

        // If start is defined, try to fit the shape. We know that we are on a free space
        if start != usize::MAX {
            if idx - start + 1 >= len {
                return Some(start);
            }
        }
    }

    None
}

pub fn solve_part_two(data: &[usize]) -> usize {
    // Create the list of files
    let mut current_position: usize = 0;
    let mut disk: Vec<_> = data
        .iter()
        .enumerate()
        .filter_map(|(idx, elt)| {
            let file = if idx % 2 == 0 {
                // We are on a file node
                Some(File {
                    id: idx / 2,
                    position: current_position,
                    size: *elt,
                })
            } else {
                None
            };

            // Increase the position
            current_position += *elt;

            // Return the file if it was created
            file
        })
        .collect();

    // Create a disk bitmap
    let mut bitmap: Vec<u8> = vec![0; disk[disk.len() - 1].position + disk[disk.len() - 1].size];
    for file in &disk {
        for i in file.position..file.position + file.size {
            bitmap[i] = 1;
        }
    }

    // Try to compact the disk from the end
    for file in disk.iter_mut().rev() {
        if let Some(position) = compute_first_empty_space(&bitmap, file.size) {
            // Check if the new position is an improvement
            if position >= file.position {
                continue;
            }

            // Update the bitmap to remove the file
            for i in file.position..file.position + file.size {
                bitmap[i] = 0;
            }

            // Update the bitmap to add the file
            for i in position..position + file.size {
                bitmap[i] = 1;
            }

            //  Move the file
            file.position = position;
        }
    }

    // Compute the checksum
    disk.iter()
        .map(|file| (0..file.size).map(|idx| (file.position + idx) * file.id).sum::<usize>())
        .sum()
}
