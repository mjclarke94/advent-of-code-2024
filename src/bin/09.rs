use itertools::Itertools;

advent_of_code::solution!(9);

type S = usize;

fn parse_input(input: &str) -> Vec<u8> {
    let input = input.as_bytes();
    let len = input.len();
    input[..len - 1].iter().map(|f| f - 48).collect()
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

pub fn part_two(input: &str) -> Option<u32> {
    let v = parse_input(input);

    let blocks: Vec<u8> = v.iter().step_by(2).cloned().collect();
    let n_blocks = blocks.len() as S;
    let mut block_iter = blocks.into_iter().peekable();

    let gaps: Vec<u8> = v.into_iter().skip(1).step_by(2).collect();
    let mut gap_iter = gaps.into_iter();
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
        assert_eq!(result, None);
    }
}
