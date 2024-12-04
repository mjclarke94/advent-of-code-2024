advent_of_code::solution!(4);

use ndarray::{s, Array2, ArrayView2};
type Size = u16;

pub fn part_one(input: &str) -> Option<Size> {
    let arr = parse_input(input);
    Some(scan(arr))
}

pub fn part_two(input: &str) -> Option<usize> {
    let arr = parse_input(input);
    let x_mas = arr
        .windows((3, 3))
        .into_iter()
        .map(|f| check_diagonal_x_mas(f))
        .filter(|f| *f)
        .count();

    Some(x_mas)
}

fn parse_input(input: &str) -> Array2<char> {
    let l = input.len(); // Total number of characters
    let i = input.lines().peekable().peek().unwrap().len(); // Width
    let j = l / (i + 1); // Height

    unsafe {
        Array2::from_shape_vec_unchecked(
            (i, j),
            input.chars().filter(|f| !f.is_ascii_whitespace()).collect(),
        )
    }
}

fn scan(arr: Array2<char>) -> Size {
    let diag: Size = arr
        .windows((4, 4))
        .into_iter()
        .map(|f| check_diagonal(f))
        .sum();

    let horz: Size = arr
        .windows((1, 4))
        .into_iter()
        .map(|f| find_xmas_perm(f.iter().collect::<String>()))
        .sum();

    let vert: Size = arr
        .windows((4, 1))
        .into_iter()
        .map(|f| find_xmas_perm(f.iter().collect::<String>()))
        .sum();

    vert + horz + diag
}

fn check_diagonal(f: ArrayView2<char>) -> Size {
    let rd: String = f.diag().iter().collect();
    let ld: String = f.slice(s![.., ..;-1]).diag().iter().collect();

    find_xmas_perm(rd) + find_xmas_perm(ld)
}

fn check_diagonal_x_mas(f: ArrayView2<char>) -> bool {
    let rd: String = f.diag().iter().collect();
    let ld: String = f.slice(s![.., ..;-1]).diag().iter().collect();

    find_mas_perm(rd) && find_mas_perm(ld)
}

fn find_xmas_perm(s: String) -> Size {
    match &s[..4] {
        "XMAS" | "SAMX" => 1,
        _ => 0,
    }
}

fn find_mas_perm(s: String) -> bool {
    match &s[..3] {
        "MAS" | "SAM" => true,
        _ => false,
    }
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
