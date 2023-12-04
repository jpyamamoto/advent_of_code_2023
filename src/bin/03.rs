advent_of_code::solution!(3);

#[derive(PartialEq)]
enum Coord {
    Num(u32),
    Symbol(char),
    Void,
}

impl Coord {
    fn is_num(&self) -> bool {
        match self {
            Coord::Num(_) => true,
            _             => false,
        }
    }

    fn is_symbol(&self) -> bool {
        match self {
            Coord::Symbol(_) => true,
            _                => false,
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Coord>> {
    input.lines().map(|l| l.chars().map(|c| match c {
        '.' => Coord::Void,
        c if c.is_ascii_digit() => Coord::Num(c.to_digit(10).unwrap()),
        c   => Coord::Symbol(c)
    }).collect()).collect()
}

fn is_part(plot: &Vec<Vec<Coord>>, x: usize, y: usize) -> bool {
    if let Coord::Num(_) = plot[y][x] {
        for i in -1..=1 {
            for j in -1..=1 {
                if y.saturating_add_signed(i) < plot.len() && x.saturating_add_signed(j) < plot[y].len() {
                  if plot[y.saturating_add_signed(i)][x.saturating_add_signed(j)].is_symbol() {
                      return true;
                  }
                }
            }
        }
    }

    false
}

fn parse_whole_number(plot: &Vec<Vec<Coord>>, x: usize, y: usize) -> u32 {
    let mut x = x;

    while x > 0 && plot[y][x].is_num() {
        x -= 1;
    }

    if !plot[y][x].is_num() {
        x += 1
    }

    let mut result: u32 = 0;

    while x < plot[y].len() {
        if let Coord::Num(n) = plot[y][x] {
            result = result*10 + n;
        } else { break; }

        x += 1;
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);

    let mut result: u32 = 0;

    for y in 0..input.len() {
        let mut flag = false;

        for x in 0..input[y].len() {
            if flag {
                match input[y][x] {
                    Coord::Num(_) => continue,
                    _             => flag = false,
                }
            }

            if is_part(&input, x, y) {
                flag = true;
                result += parse_whole_number(&input, x, y);
            }
        }
    }

    Some(result)
}

fn gear_ratio(plot: &Vec<Vec<Coord>>, x: usize, y: usize) -> u32 {
    let mut nums: Vec<u32> = vec![];

    for i in [-1, 1].iter() {
        if plot[y][x.saturating_add_signed(*i)].is_num() {
            nums.push(parse_whole_number(plot, x.saturating_add_signed(*i), y));
        }
    }

    for i in [-1, 1] {
        if plot[y.saturating_add_signed(i)][x].is_num() {
            nums.push(parse_whole_number(plot, x, y.saturating_add_signed(i)));
        } else {
            if plot[y.saturating_add_signed(i)][x.saturating_add_signed(i)].is_num() {
                nums.push(parse_whole_number(plot, x.saturating_add_signed(i), y.saturating_add_signed(i)));
            }

            if plot[y.saturating_add_signed(i)][x.saturating_add_signed(-i)].is_num() {
                nums.push(parse_whole_number(plot, x.saturating_add_signed(-i), y.saturating_add_signed(i)));
            }
        }
    }

    if nums.len() == 2 {
        nums[0] * nums[1]
    } else {
        0
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);

    let mut result: u32 = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if let Coord::Symbol('*') = input[y][x] {
                result += gear_ratio(&input, x, y);
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
