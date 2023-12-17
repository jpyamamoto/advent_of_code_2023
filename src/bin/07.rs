use std::{str::FromStr, cmp::Ordering, iter::zip};
use itertools::Itertools;

advent_of_code::solution!(7);

const JOKER: &char = &'X';
const CARDS: &'static [char] = &[*JOKER, '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Card {
    id: char
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u32
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl TryInto<Card> for char {
    type Error = ParseCardError;

    fn try_into(self) -> Result<Card, Self::Error> {
        if CARDS.contains(&self) {
            Ok(Card { id: self })
        } else {
            Err(ParseCardError)
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let index_self = CARDS.iter().position(|&x| x == self.id);
        let index_other = CARDS.iter().position(|&x| x == other.id);

        index_self.cmp(&index_other)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let index_self = self.rank_type();
        let index_other = other.rank_type();

        if index_self != index_other {
            return index_self.cmp(&index_other);
        }

        for (x, y) in zip(self.cards, other.cards) {
            if x == y {
                continue;
            } else {
                return x.cmp(&y);
            }
        };

        Ordering::Equal
    }
}

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        let bid: u32 = parts[1].parse().map_err(|_| ParseHandError)?;

        if parts[0].len() == 5 {
            let mut cards: [Card; 5] = [Card { id: 'A' }; 5];

            for (i, c) in parts[0].chars().enumerate() {
                cards[i] = c.try_into().unwrap();
            };

            Ok(Hand { cards, bid })
        } else {
            Err(ParseHandError)
        }
    }
}

impl Hand {
    fn rank_type(&self) -> u32 {
        let jokers: usize = self.cards.into_iter().filter(|c| c.id == *JOKER).count();

        let mut groups: Vec<usize> = self.cards
                                         .into_iter()
                                         .filter(|c| c.id != *JOKER)
                                         .into_group_map_by(|c| c.id)
                                         .into_iter()
                                         .map(|(_, v)| v.len())
                                         .sorted()
                                         .collect();

        if groups.is_empty() {
            groups.push(jokers);
        } else {
            let i = groups.len();
            groups[i-1] += jokers;
        }

        match groups[..] {
            [5]          => 6,
            [1, 4]       => 5,
            [2, 3]       => 4,
            [1, 1, 3]    => 3,
            [1, 2, 2]    => 2,
            [1, 1, 1, 2] => 1,
            _            => 0
        }
    }
}

fn parse(input: &str) -> Vec<Hand> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut data: Vec<Hand> = parse(input);
    data.sort();

    data.iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i as u32 + 1))
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut data: Vec<Hand> = parse(&input.replace("J", &JOKER.to_string()));
    data.sort();

    data.iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i as u32 + 1))
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
