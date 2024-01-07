use std::collections::HashMap;

advent_of_code::solution!(14);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Tile {
    Space,
    Round,
    Cube
}

type Map = Vec<Vec<Tile>>;

fn parse(input: &str) -> Map {
    input.lines()
         .map(|l| l.chars().map(|c| match c {
             '#' => Tile::Cube,
             'O' => Tile::Round,
             '.' => Tile::Space,
             _   => panic!("Invalid character")
         }).collect())
         .collect()
}

fn transpose(matrix: &mut Vec<Vec<Tile>>) -> &Vec<Vec<Tile>> {
    for i in 0..matrix.len() {
        for j in 0..i {
            let temp = matrix[i][j].clone();
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }

    matrix
}

fn mirror(matrix: &mut Vec<Vec<Tile>>) -> &Vec<Vec<Tile>> {
    matrix.iter_mut().for_each(|r| r.reverse());
    matrix
}

fn rotate(matrix: &mut Vec<Vec<Tile>>) -> &Vec<Vec<Tile>> {
    transpose(matrix);
    mirror(matrix);
    matrix
}

fn tilt(matrix: &mut Vec<Vec<Tile>>) -> &Vec<Vec<Tile>> {
    let (height, width) = (matrix.len(), matrix[0].len());

    let mut top: Vec<isize> = vec![-1; width];

    for r in 0..height {
        for c in 0..width {
            let t = &matrix[r][c];

            match t {
                Tile::Space => (),
                Tile::Cube  => { top[c] = r as isize },
                Tile::Round => {
                    let pos: u32 = (top[c] + 1) as u32;
                    top[c] = pos as isize;
                    matrix[r][c] = Tile::Space;
                    matrix[pos as usize][c] = Tile::Round;
                }
            }
        }
    };

    matrix
}

fn cycle(matrix: &mut Vec<Vec<Tile>>) -> &Vec<Vec<Tile>> {
    for _ in 0..4 {
        tilt(matrix);
        rotate(matrix);
    }

    matrix
}

fn calculate_load(matrix: &Vec<Vec<Tile>>) -> u32 {
    let (height, width) = (matrix.len(), matrix[0].len());

    let mut result: u32 = 0;

    for r in 0..height {
        for c in 0..width {
            let t = &matrix[r][c];

            result += match t {
                Tile::Space => 0,
                Tile::Cube  => 0,
                Tile::Round => height as u32 - r as u32
            }
        }
    };

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut data = parse(input);
    tilt(&mut data);

    Some(calculate_load(&data))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut data = parse(input);

    let mut seen: HashMap<Vec<Vec<Tile>>, usize> = HashMap::new();
    let mut i = 0;

    let begin_loop = loop {
        cycle(&mut data);

        if seen.contains_key(&data) {
            break i - seen[&data];
        }

        seen.insert(data.clone(), i);
        i += 1;
    };

    let remaining = 1000000000 - (i+1);
    let missing = remaining % begin_loop;

    for _ in 0..missing {
        cycle(&mut data);
    };

    Some(calculate_load(&data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
