advent_of_code::solution!(4);

struct Card {
    id: u32,
    winning: Vec<u32>,
    owned: Vec<u32>,
}

impl Card {
    fn matches(&self) -> u32 {
        let Card { winning, owned, .. } = &self;
        let mut count: u32 = 0;
        let mut owned = owned.clone();

        // Intersection size
        for c in winning {
            match owned.iter().position(|&x| x == *c) {
                Some(i) => {
                    owned.remove(i);
                    count += 1;
                },
                None => (),
            }
        }

        count
    }
}

fn parse(input: &str) -> Vec<Card> {
    input.lines().map(parse_card).collect()
}

fn parse_card(input: &str) -> Card {
    let parts : Vec<&str> = input.strip_prefix("Card ").unwrap().split(":").collect();
    let id: u32 = parts[0].trim().parse().unwrap();
    let nums: Vec<Vec<u32>> = parts[1].split(" | ")
                                      .map(|s| s.trim().split_ascii_whitespace().map(|n| n.parse().unwrap()).collect())
                                      .collect();
    Card { id, winning: nums[0].clone(), owned: nums[1].clone() }
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = parse(input).iter()
                             .map(|c| c.matches())
                             .filter(|&c| c != 0)
                             .map(|n| (2 as u32).pow(n-1))
                             .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards: Vec<u32> = parse(input).iter().map(|c| c.matches()).collect();
    let mut copies = vec![1; cards.len()];

    for (i, &c) in cards.iter().enumerate() {
        for j in 1..=c {
            copies[i+(j as usize)] += copies[i];
        }
    }

    Some(copies.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
