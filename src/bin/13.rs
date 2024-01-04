advent_of_code::solution!(13);

#[derive(Debug)]
struct Map {
    rows: Vec<u32>,
    cols: Vec<u32>
}

fn parse_row(input: &str) -> u32 {
    input.chars()
         .fold(0, |acc: u32, c| {
             (acc << 1) + match c {
                 '.' => 0,
                 '#' => 1,
                 _   => panic!("Invalid character")
             }
         })
}

fn parse_map(input: &str) -> Map {
    let rows = input.lines()
                    .map(|l| parse_row(l))
                    .collect::<Vec<u32>>();

    let num_cols = input.lines().nth(0).unwrap().len();

    let mut iters: Vec<_> = input.lines().map(|l| l.chars()).collect();

    let cols = (0..num_cols).map(|_| {
        iters.iter_mut()
             .map(|n| n.next().unwrap())
             .collect::<String>()
    }).map(|s| parse_row(&s))
      .collect();

    Map { rows, cols }
}

fn parse(input: &str) -> Vec<Map> {
    input.split("\n\n")
         .map(|p| parse_map(p))
         .collect()
}

fn is_mirror(rows: &Vec<u32>, i: usize, smudge: bool) -> bool {
    let closer_side = (rows.len() - i).min(i);
    let mut count = 0;

    for j in 0..closer_side {
        if rows[i+j] != rows[i-(j+1)] {
            if smudge {
                let n = rows[i+j] ^ rows[i-(j+1)];

                if (n & n.overflowing_sub(1).0) == 0 {
                    count += 1;

                    if count <= 1 {
                        continue;
                    }
                }
            }

            return false;
        }
    }

    true
}

fn find_mirror(rows: Vec<u32>, smudge: bool) -> Option<usize> {
    (1..rows.len()).find(|&i|
                         (!smudge || !is_mirror(&rows, i, false))
                            && is_mirror(&rows, i, smudge))
}

fn find_symmetry(map: Map, smudge: bool) -> u32 {
    let Map { rows, cols } = map;

    find_mirror(rows, smudge).map(|n| n * 100)
                     .or_else(|| find_mirror(cols, smudge))
                     .map(|n| n as u32)
                     .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse(input);

    data.into_iter()
        .map(|m| find_symmetry(m, false))
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse(input);

    data.into_iter()
        .map(|m| find_symmetry(m, true))
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
