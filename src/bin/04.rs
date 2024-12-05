advent_of_code::solution!(4);

type Size = usize;

fn get_input_bytes_len(input: &str) -> (&[u8], usize) {
    let len = input.lines().next().unwrap().len() + 1; // Newline chars ignored as doesn't add major overhead
    (input.as_bytes(), len)
}

fn scan_horz(input: &[u8]) -> Size {
    input
        .windows(4)
        .map(|s| matches!(s, b"XMAS" | b"SAMX"))
        .filter(|f| *f)
        .count()
}

fn scan_skew(input: &[u8], stop: usize, offset: usize) -> usize {
    (0..(input.len() - stop))
        .map(|idx| {
            matches!(
                &[
                    input[idx],
                    input[idx + offset],
                    input[idx + 2 * offset],
                    input[idx + 3 * offset],
                ],
                b"XMAS" | b"SAMX"
            )
        })
        .filter(|f| *f)
        .count()
}

pub fn part_one(input: &str) -> Option<Size> {
    let (input, line_len) = get_input_bytes_len(input);

    let horz = scan_horz(input);
    let diag_l = scan_skew(input, 3 * line_len + 3, line_len + 1);

    // May be an opt combining iter over these given only diff is offset
    let vert = scan_skew(input, 3 * line_len, line_len);
    let diag_r = scan_skew(input, 3 * line_len, line_len - 1);

    Some(horz + vert + diag_l + diag_r)
}

fn check_x_mas(input: &[u8], line_len: usize) -> usize {
    (0..(input.len() - (3 * line_len + 3)))
        .map(|idx| {
            let ld = matches!(
                &[
                    input[idx],
                    input[idx + line_len + 1],
                    input[idx + 2 * (line_len + 1)],
                ],
                b"MAS" | b"SAM"
            );
            let rd = matches!(
                &[
                    input[idx + 2],
                    input[idx + line_len + 1],
                    input[idx + 2 * line_len],
                ],
                b"MAS" | b"SAM"
            );

            ld && rd
        })
        .filter(|f| *f)
        .count()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (input, line_len) = get_input_bytes_len(input);

    Some(check_x_mas(input, line_len))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
