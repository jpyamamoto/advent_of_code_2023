use std::iter::zip;

advent_of_code::solution!(6);

struct Race {
    time: u64,
    distance: u64
}

fn parse_part1(input: &str) -> Vec<Race> {
    let lines: Vec<Vec<u64>> = input.lines()
                                    .map(|l| l.split_ascii_whitespace()
                                              .skip(1)
                                              .map(|s| s.parse().unwrap())
                                              .collect())
                                    .collect();

    zip(lines[0].iter(), lines[1].iter())
        .into_iter()
        .map(|(&time, &distance)| Race { time, distance })
        .collect()
}

fn parse_part2(input: &str) -> Race {
    let lines: Vec<u64> = input.lines()
                               .map(|l| l.chars()
                                         .filter(char::is_ascii_digit)
                                         .collect::<String>()
                                         .parse()
                                         .unwrap())
                               .collect();

    Race { time: lines[0], distance: lines[1] }
}

fn count_wins(race: &Race) -> u64 {
    let time = race.time as f64;
    let distance = race.distance as f64;

    let root1 = (time - (time.powi(2) - 4.0f64 * distance).sqrt()) / 2.0f64;
    let root2 = (time + (time.powi(2) - 4.0f64 * distance).sqrt()) / 2.0f64;

    let start: u64 = if root1.fract() == 0.0 { root1 as u64 + 1 } else { root1.ceil() as u64 };
    let end: u64   = if root2.fract() == 0.0 { root2 as u64 - 1 } else { root2.floor() as u64 };

    end - start + 1
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_part1(input);

    let result = data.iter()
                     .map(|r| count_wins(r))
                     .product();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let race = parse_part2(input);

    Some(count_wins(&race))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
