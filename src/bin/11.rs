use itertools::Itertools;

advent_of_code::solution!(11);

const OFFSET_P1: usize = 2 - 1;
const OFFSET_P2: usize = 1000000 - 1;

fn parse(input: &str) -> Vec<(usize, usize)> {
    input.lines()
         .enumerate()
         .flat_map(|(row, l)| {
             l.chars()
              .enumerate()
              .filter(|&(_col, c)| c == '#')
              .map(|(col, _c)| (row, col))
              .collect::<Vec<_>>()
         })
         .collect()
}

fn solve(input: &str, offset: usize) -> u64 {
    let (rows, cols) = (input.lines().count(), input.lines().nth(0).unwrap().len());
    let data = parse(input);

    let mut offsets_rows: Vec<usize> = vec![0];
    let mut offsets_cols: Vec<usize> = vec![0];

    let mut rows_data: Vec<usize> = data.iter().map(|&(row, _)| row).collect();
    rows_data.sort_unstable();

    let mut cols_data: Vec<usize> = data.iter().map(|&(_, col)| col).collect();
    cols_data.sort_unstable();

    for r in 1..rows {
        let prev_offset = offsets_rows[r-1];
        offsets_rows.push(prev_offset + if rows_data.binary_search(&r).is_ok() { 0 } else { offset });
    };

    for c in 1..cols {
        let prev_offset = offsets_cols[c-1];
        offsets_cols.push(prev_offset + if cols_data.binary_search(&c).is_ok() { 0 } else { offset });
    };

    let galaxies = data.iter()
                       .map(|&(row, col)| (row + offsets_rows[row], col + offsets_cols[col]))
                       .collect::<Vec<(usize, usize)>>();

    let result = galaxies.iter()
                         .combinations(2)
                         .map(|gs| {
                             let &(r1, c1) = gs[0];
                             let &(r2, c2) = gs[1];
                             r1.abs_diff(r2) + c1.abs_diff(c2)
                         })
                         .sum::<usize>();

    result as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, OFFSET_P1))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, OFFSET_P2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
