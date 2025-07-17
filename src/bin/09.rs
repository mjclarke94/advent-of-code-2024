use itertools::Itertools;

advent_of_code::solution!(9);

type S = usize;

fn parse_input(input: &str) -> Vec<u8> {
    input.trim().bytes().map(|b| b - b'0').collect()
}

struct Buffer {
    idx: S,
    size: u8,
}

pub fn part_one(input: &str) -> Option<S> {
    let v = parse_input(input);

    let blocks: Vec<u8> = v.iter().step_by(2).cloned().collect();
    let n_blocks = blocks.len() as S;
    let mut block_iter = blocks.into_iter().peekable();

    let gaps: Vec<u8> = v.into_iter().skip(1).step_by(2).collect();
    let mut gap_iter = gaps.into_iter();

    let mut idx: S = 0;
    let mut buff: Buffer = Buffer {
        idx: n_blocks - 1,
        size: block_iter.next_back().unwrap(),
    };

    let mut disk: Vec<S> = Vec::new();

    while block_iter.peek().is_some() {
        disk.extend((0..block_iter.next().unwrap()).map(|_| idx));
        idx += 1;

        for _ in 0..gap_iter.next().unwrap() {
            if buff.size != 0 {
                disk.push(buff.idx);
                buff.size -= 1;
            } else if let Some(n) = block_iter.next_back() {
                buff.size = n - 1;
                buff.idx -= 1;
                disk.push(buff.idx);
            }
        }
    }

    while buff.size != 0 {
        disk.push(buff.idx);
        buff.size -= 1;
    }

    Some(disk.into_iter().enumerate().map(|(i, v)| i as S * v).sum())
}

pub fn part_two(input: &str) -> Option<S> {
    let v = parse_input(input);
    
    // Separate blocks and gaps
    let blocks: Vec<u8> = v.iter().step_by(2).cloned().collect();
    let gaps: Vec<u8> = v.iter().skip(1).step_by(2).cloned().collect();
    
    // Build initial disk representation with file positions
    let mut disk: Vec<Option<S>> = Vec::new();
    let mut file_positions: Vec<(S, S, u8)> = Vec::new(); // (file_id, start_pos, length)
    
    for (file_id, either_or_both) in blocks.iter().zip_longest(gaps.iter()).enumerate() {
        let start_pos = disk.len();
        
        match either_or_both {
            itertools::EitherOrBoth::Both(&block_size, &gap_size) => {
                // Add file blocks
                for _ in 0..block_size {
                    disk.push(Some(file_id));
                }
                
                if block_size > 0 {
                    file_positions.push((file_id, start_pos, block_size));
                }
                
                // Add gap blocks
                for _ in 0..gap_size {
                    disk.push(None);
                }
            }
            itertools::EitherOrBoth::Left(&block_size) => {
                // Only file blocks, no gap after
                for _ in 0..block_size {
                    disk.push(Some(file_id));
                }
                
                if block_size > 0 {
                    file_positions.push((file_id, start_pos, block_size));
                }
            }
            itertools::EitherOrBoth::Right(_) => {
                // This shouldn't happen in valid input
            }
        }
    }
    
    // Process files in decreasing file ID order
    for &(file_id, _original_pos, file_length) in file_positions.iter().rev() {
        // Find current position of file (it may have moved)
        let mut current_pos = None;
        for i in 0..disk.len() {
            if disk[i] == Some(file_id) {
                current_pos = Some(i);
                break;
            }
        }
        
        if let Some(pos) = current_pos {
            // Look for leftmost suitable free space
            let mut free_start = None;
            let mut consecutive_free = 0;
            
            for i in 0..pos {
                if disk[i].is_none() {
                    if consecutive_free == 0 {
                        free_start = Some(i);
                    }
                    consecutive_free += 1;
                    
                    if consecutive_free >= file_length {
                        // Found suitable space, move the file
                        if let Some(start) = free_start {
                            for j in 0..file_length as usize {
                                disk[start + j] = Some(file_id);
                                disk[pos + j] = None;
                            }
                            break;
                        }
                    }
                } else {
                    consecutive_free = 0;
                    free_start = None;
                }
            }
        }
    }
    
    // Calculate checksum
    Some(
        disk.iter()
            .enumerate()
            .filter_map(|(i, &file_id)| file_id.map(|id| i * id))
            .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
