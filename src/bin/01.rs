use std::iter::zip;
use std::cmp::{min, max};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let first = input.lines()
                     .map(|l| string_int_at(l, l.find(|c: char| c.is_ascii_digit()).unwrap()));

    let last = input.lines()
                    .map(|l| string_int_at(l, l.rfind(|c: char| c.is_ascii_digit()).unwrap()));

    Some(zip(first, last).map(|(x, y)| x*10+y).sum())
}

fn string_int_at(text: &str, i: usize) -> u32 {
    text.chars().nth(i).map(|c| c.to_digit(10).unwrap()).unwrap()
}

const NUMBERS: &'static [&'static str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn part_two(input: &str) -> Option<u32> {
    let first = input.lines()
                     .map(|l| string_val_at(l, find_index(l)));

    let last = input.lines()
                    .map(|l| string_val_at(l, rfind_index(l)));

    Some(zip(first, last).map(|(x, y)| x*10+y).sum())
}

fn find_index(text: &str) -> usize {
    NUMBERS.iter().fold(text.find(|c: char| c.is_ascii_digit()),
                        |curr, p| text.find(p).map_or(curr, |i| Some(curr.map_or(i, |x| min(x, i)))))
                  .unwrap()
}

fn rfind_index(text: &str) -> usize {
    NUMBERS.iter().fold(text.rfind(|c: char| c.is_ascii_digit()),
                        |curr, p| text.rfind(p).map_or(curr, |i| Some(curr.map_or(i, |x| max(x, i)))))
                  .unwrap()
}

fn string_val_at(text: &str, i: usize) -> u32 {
    let c_i = text.chars().nth(i).unwrap();

    if char::is_ascii_digit(&c_i) {
        string_int_at(text, i)
    } else {
        (0..9).fold(0, |acc, n| if text[i..].starts_with(NUMBERS[n]) { (n + 1) as u32 } else { acc })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
