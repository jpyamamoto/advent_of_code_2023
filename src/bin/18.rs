advent_of_code::solution!(18);

enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
struct Point(isize, isize);

struct Instruction {
    dir: Direction,
    steps: u64,
}

fn parse_instruction1(input: &str) -> Instruction {
    let mut parts = input.split_ascii_whitespace();

    let dir = match parts.next().unwrap() {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _   => panic!("Invalid instruction")
    };

    let steps: u64 = parts.next().unwrap().parse().unwrap();

    Instruction { dir, steps }
}

fn parse_instruction2(input: &str) -> Instruction {
    let mut parts = input.split_ascii_whitespace();
    let color = &parts.nth(2).unwrap()[2..8];

    let dir = match color.as_bytes()[5] as char {
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' => Direction::Up,
        _   => panic!("Invalid instruction")
    };

    let steps: u64 = u64::from_str_radix(&color[0..5], 16).unwrap();

    Instruction { dir, steps }
}

fn parse(input: &str, part: usize) -> Vec<Instruction> {
    if part == 1 {
        input.lines()
             .map(|line| parse_instruction1(line))
             .collect()
    } else {
        input.lines()
             .map(|line| parse_instruction2(line))
             .collect()
    }
}

fn steps_to_points(instructions: &Vec<Instruction>) -> Vec<Point> {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut points: Vec<Point> = vec![Point(0, 0)];

    for i in instructions {
        match i.dir {
            Direction::Up    => y -= i.steps as isize,
            Direction::Down  => y += i.steps as isize,
            Direction::Left  => x -= i.steps as isize,
            Direction::Right => x += i.steps as isize,
        }

        points.push(Point(x, y));
    }

    points
}

fn polygon_area(points: &Vec<Point>) -> i64 {
    let twice_area = points.windows(2)
                           .map(|p| {
                               let Point(x1, y1) = p[0];
                               let Point(x2, y2) = p[1];
                               (x1*y2) as i64 - (y1*x2) as i64
                           })
                           .sum::<i64>()
                           .abs();

    twice_area / 2
}

fn polygon_perimeter(points: &Vec<Point>) -> i64 {
    points.windows(2)
          .map(|p| {
              let Point(x1, y1) = p[0];
              let Point(x2, y2) = p[1];
              ((x1 - x2).abs() + (y1 - y2).abs()) as i64
          })
          .sum()
}

fn compute_space(steps: &Vec<Instruction>) -> i64 {
    let points = steps_to_points(&steps);
    let exterior = polygon_perimeter(&points);
    let area = polygon_area(&points);

    let interior = area - (exterior / 2) + 1;

    exterior + interior
}

pub fn part_one(input: &str) -> Option<i64> {
    let data = parse(input, 1);

    Some(compute_space(&data))
}

pub fn part_two(input: &str) -> Option<i64> {
    let data = parse(input, 2);

    Some(compute_space(&data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
