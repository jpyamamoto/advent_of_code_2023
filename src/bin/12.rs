use std::fmt::Debug;
use std::cmp;
use std::collections::HashMap;

advent_of_code::solution!(12);

#[derive(PartialEq, Eq, Clone)]
enum Spring {
    Operational,
    Damaged,
    Unknown
}

impl Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Operational => write!(f, "."),
            Self::Damaged => write!(f, "#"),
            Self::Unknown => write!(f, "?"),
        }
    }
}

struct Record {
    springs: Vec<Spring>,
    damaged: Vec<usize>
}

fn parse_record(input: &str) -> Record {
    let mut parts = input.split_ascii_whitespace();
    let springs_data = parts.next().unwrap();
    let damaged_data = parts.next().unwrap();

    let damaged = damaged_data.split(',')
                              .map(|c| c.parse().unwrap())
                              .collect();

    let springs = springs_data.chars().map(|c| {
        match c {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            _   => Spring::Unknown
        }
    }).collect();

    Record { springs, damaged }
}

fn parse(input: &str) -> Vec<Record> {
    input.lines()
         .map(|l| parse_record(l))
         .collect()
}

// fn arrangements(springs: &[Spring], damaged: &[usize], i: usize, j: usize) -> usize {
//     if i >= springs.len() {
//         return if j >= damaged.len() { 1 } else { 0 }
//     }
//
//     if j >= damaged.len() {
//         return if springs[i..].contains(&Spring::Damaged) { 0 } else { 1 }
//     }
//
//     match springs[i] {
//         Spring::Operational => arrangements(springs, damaged, i+1, j),
//         Spring::Unknown => {
//             let mut new_springs: Vec<Spring> = springs.iter().map(|s| s.clone()).collect();
//             new_springs[i] = Spring::Operational;
//
//             let arr1 = arrangements(&new_springs, damaged, i, j);
//
//             new_springs = springs.iter().map(|s| s.clone()).collect();
//             new_springs[i] = Spring::Damaged;
//
//             let arr2 = arrangements(&new_springs, damaged, i, j);
//             arr1 + arr2
//         },
//         Spring::Damaged => {
//             let springs_len = springs.len() - i;
//             let group_size = damaged[j];
//
//             if group_size > springs_len {
//                 return 0;
//             }
//
//             let group = &springs[i..i+group_size];
//
//             if group.contains(&Spring::Operational) {
//                 return 0;
//             }
//
//             if group_size < springs_len && springs[i+group_size] == Spring::Damaged {
//                 return 0;
//             }
//
//             arrangements(springs, damaged, i+group_size+1, j+1)
//         }
//     }
// }

fn to_hash(springs: &[Spring], damaged: &[usize]) -> (String, String) {
    let springs: String = springs.iter().map(|s| match s {
        Spring::Damaged => '#',
        Spring::Operational => '.',
        Spring::Unknown => '?'
    }).collect();

    let damaged = damaged.iter().map(|u| u.to_string()).collect::<Vec<_>>().join(",");

    (springs, damaged)
}

fn arrangements(springs: &mut [Spring], damaged: &mut [usize], cache: &mut HashMap<(String, String), u64>) -> u64 {
    let hash = to_hash(springs, damaged);
    if cache.contains_key(&hash) {
        return *cache.get(&hash).unwrap();
    }

    if springs.is_empty() {
        return if damaged.is_empty() { 1 } else { 0 };
    }

    if damaged.is_empty() {
        let result = if springs.contains(&Spring::Damaged) { 0 } else { 1 };
        cache.insert(hash, result);
        return result;
    }

    match springs[0] {
        Spring::Operational => {
            let result = arrangements(&mut springs[1..], damaged, cache);
            cache.insert(hash, result);
            result
        },
        Spring::Unknown => {
            springs[0] = Spring::Operational;
            let arr1 = arrangements(springs, damaged, cache);
            springs[0] = Spring::Damaged;
            let arr2 = arrangements(springs, damaged, cache);
            springs[0] = Spring::Unknown;

            let result = arr1 + arr2;
            cache.insert(hash, result);
            result
        },
        Spring::Damaged => {
            let springs_len = springs.len();
            let group_size = damaged[0];

            if group_size > springs_len {
                return 0;
            }

            let group = &springs[..group_size];

            if group.contains(&Spring::Operational) {
                return 0;
            }

            if group_size < springs_len && springs[group_size] == Spring::Damaged {
                return 0;
            }

            let result = arrangements(&mut springs[cmp::min(group_size+1, springs_len)..], &mut damaged[1..], cache);
            cache.insert(hash, result);
            result
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut data = parse(input);

    let result = data.iter_mut()
                     .map(|r| arrangements(&mut r.springs, &mut r.damaged, &mut HashMap::new()))
                     .sum::<u64>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut data = parse(input);

    let result = data.iter_mut()
                     .map(|Record { springs, damaged }| {
                         let mut springs: Vec<Spring> = springs.iter().chain([Spring::Unknown].iter())
                                                                      .cycle()
                                                                      .take((springs.len() + 1) * 5 - 1)
                                                                      .cloned()
                                                                      .collect();
                         let mut damaged: Vec<usize> = damaged.iter().cycle()
                                                                     .take(damaged.len() * 5).cloned().collect();

                         arrangements(&mut springs, &mut damaged, &mut HashMap::new())
                     })
                     .sum::<u64>();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
