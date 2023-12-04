use std::cmp::max;
advent_of_code::solution!(2);

struct Game {
    id: u32,
    samples: Vec<Sample>,
}

struct Sample {
    blue: u32,
    red: u32,
    green: u32,
}

fn parse(input: &str) -> Vec<Game> {
    input.lines().map(parse_game).collect()
}

fn parse_game(input: &str) -> Game {
    let input = input.strip_prefix("Game ").unwrap();
    let sections: Vec<&str> = input.split(": ").collect();
    let id: u32 = sections[0].parse().unwrap();

    Game { id, samples: sections[1].split("; ").map(parse_sample).collect() }
}

fn parse_sample(input: &str) -> Sample {
    let counts: Vec<&str> = input.split(", ").collect();
    let mut sample = Sample { blue: 0, red: 0, green: 0 };

    for c in counts {
        match c {
            _ if c.ends_with("blue") => sample.blue = c.strip_suffix(" blue").unwrap().parse().unwrap(),
            _ if c.ends_with("red")  => sample.red = c.strip_suffix(" red").unwrap().parse().unwrap(),
            _                        => sample.green = c.strip_suffix(" green").unwrap().parse().unwrap(),
        }
    };

    sample
}

fn game_possible(game: &Game) -> bool {
    game.samples
        .iter()
        .all(|&Sample { green, red, blue }| blue <= 14 && green <= 13 && red <= 12)
}

fn min_sample(Game { samples, .. }: Game) -> Sample {
    samples
        .iter()
        .fold(Sample { blue: 0, red: 0, green: 0 },
              |acc, s| Sample { green: max(acc.green, s.green), red: max(acc.red, s.red), blue: max(acc.blue, s.blue) })
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = parse(input).into_iter()
                             .filter(game_possible)
                             .map(|g| g.id)
                             .sum::<u32>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = parse(input).into_iter()
                             .map(min_sample)
                             .map(|Sample { green: g, red: r, blue: b }| g * r * b)
                             .sum::<u32>();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
