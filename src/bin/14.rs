advent_of_code::solution!(14);

#[derive(Debug, Clone, Copy)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    #[inline]
    fn parse(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let pos = parts.next().unwrap();
        let vel = parts.next().unwrap();

        let pos_str = &pos[2..];
        let comma_pos = pos_str.find(',').unwrap();
        let px = pos_str[..comma_pos].parse().unwrap();
        let py = pos_str[comma_pos + 1..].parse().unwrap();

        let vel_str = &vel[2..];
        let comma_pos = vel_str.find(',').unwrap();
        let vx = vel_str[..comma_pos].parse().unwrap();
        let vy = vel_str[comma_pos + 1..].parse().unwrap();

        Robot { px, py, vx, vy }
    }

    #[inline]
    fn position_after(&self, seconds: i32, width: i32, height: i32) -> (i32, i32) {
        let final_x = (self.px + self.vx * seconds).rem_euclid(width);
        let final_y = (self.py + self.vy * seconds).rem_euclid(height);
        (final_x, final_y)
    }
}

fn parse_robots(input: &str) -> Vec<Robot> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Robot::parse)
        .collect()
}

fn calculate_safety_factor(robots: &[Robot], seconds: i32, width: i32, height: i32) -> u32 {
    let mid_x = width / 2;
    let mid_y = height / 2;

    let mut quadrants = [0u32; 4];

    for robot in robots {
        let (x, y) = robot.position_after(seconds, width, height);

        if x == mid_x || y == mid_y {
            continue;
        }

        let quadrant = match (x < mid_x, y < mid_y) {
            (true, true) => 0,   // top-left
            (false, true) => 1,  // top-right
            (true, false) => 2,  // bottom-left
            (false, false) => 3, // bottom-right
        };

        quadrants[quadrant] += 1;
    }

    quadrants.iter().product()
}

pub fn part_one(input: &str) -> Option<u32> {
    let robots = parse_robots(input);

    let (width, height) = if robots.len() == 12 {
        (11, 7)
    } else {
        (101, 103)
    };

    Some(calculate_safety_factor(&robots, 100, width, height))
}

pub fn part_two(input: &str) -> Option<u32> {
    let robots = parse_robots(input);
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;

    let mut min_x_variance = i64::MAX;
    let mut min_y_variance = i64::MAX;
    let mut best_x_time = 0;
    let mut best_y_time = 0;

    for seconds in 0..WIDTH.max(HEIGHT) {
        let mut sum_x = 0i64;
        let mut sum_y = 0i64;
        let mut sum_x2 = 0i64;
        let mut sum_y2 = 0i64;

        for robot in &robots {
            let x = (robot.px + robot.vx * seconds).rem_euclid(WIDTH);
            let y = (robot.py + robot.vy * seconds).rem_euclid(HEIGHT);
            sum_x += x as i64;
            sum_y += y as i64;
            sum_x2 += (x as i64) * (x as i64);
            sum_y2 += (y as i64) * (y as i64);
        }

        let n = robots.len() as i64;
        let x_variance = sum_x2 - sum_x * sum_x / n;
        let y_variance = sum_y2 - sum_y * sum_y / n;

        if x_variance < min_x_variance {
            min_x_variance = x_variance;
            best_x_time = seconds;
        }
        if y_variance < min_y_variance {
            min_y_variance = y_variance;
            best_y_time = seconds;
        }
    }

    let mut t = best_x_time;
    while t % HEIGHT != best_y_time {
        t += WIDTH;
    }

    Some(t as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert!(result.is_some());
    }
}
