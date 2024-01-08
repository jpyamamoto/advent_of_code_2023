use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(16);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    MirrorForward,
    MirrorBackward,
    SplitterHorizontal,
    SplitterVertical
}

type Coord = (isize, isize);
type Map = Vec<Vec<Tile>>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Beam {
    pos: Coord,
    dir: Direction
}

fn parse(input: &str) -> Map {
    input.lines()
         .map(|line| {
             line.chars()
                 .map(|c| match c {
                     '.'  => Tile::Empty,
                     '/'  => Tile::MirrorForward,
                     '\\' => Tile::MirrorBackward,
                     '-'  => Tile::SplitterHorizontal,
                     _    => Tile::SplitterVertical
                 })
                 .collect()
         })
         .collect()
}

fn neighbours(loc: Beam, map: &Map) -> Vec<Beam> {
    let (col, row) = loc.pos;
    let (width, height) = (map[0].len() as isize, map.len() as isize);
    let tile = map[row as usize][col as usize];

    let ns: Vec<Beam> = match (tile, loc.dir) {
        (Tile::Empty, Direction::North) => vec![Beam { pos: (col, row - 1), dir: loc.dir }],
        (Tile::Empty, Direction::West) => vec![Beam { pos: (col - 1, row), dir: loc.dir }],
        (Tile::Empty, Direction::South) => vec![Beam { pos: (col, row + 1), dir: loc.dir }],
        (Tile::Empty, Direction::East) => vec![Beam { pos: (col + 1, row), dir: loc.dir }],
        (Tile::MirrorForward, Direction::North) => vec![Beam { pos: (col + 1, row), dir: Direction::East }],
        (Tile::MirrorForward, Direction::West) => vec![Beam { pos: (col, row + 1), dir: Direction::South }],
        (Tile::MirrorForward, Direction::South) => vec![Beam { pos: (col - 1, row), dir: Direction::West }],
        (Tile::MirrorForward, Direction::East) => vec![Beam { pos: (col, row - 1), dir: Direction::North }],
        (Tile::MirrorBackward, Direction::North) => vec![Beam { pos: (col - 1, row), dir: Direction::West }],
        (Tile::MirrorBackward, Direction::West) => vec![Beam { pos: (col, row - 1), dir: Direction::North }],
        (Tile::MirrorBackward, Direction::South) => vec![Beam { pos: (col + 1, row), dir: Direction::East }],
        (Tile::MirrorBackward, Direction::East) => vec![Beam { pos: (col, row + 1), dir: Direction::South }],
        (Tile::SplitterHorizontal, Direction::North) | (Tile::SplitterHorizontal, Direction::South) =>
            vec![Beam { pos: (col - 1, row), dir: Direction::West }, Beam { pos: (col + 1, row), dir: Direction::East }],
        (Tile::SplitterHorizontal, Direction::West) => vec![Beam { pos: (col - 1, row), dir: loc.dir }],
        (Tile::SplitterHorizontal, Direction::East) => vec![Beam { pos: (col + 1, row), dir: loc.dir }],
        (Tile::SplitterVertical, Direction::West) | (Tile::SplitterVertical, Direction::East) =>
            vec![Beam { pos: (col, row - 1), dir: Direction::North }, Beam { pos: (col, row + 1), dir: Direction::South }],
        (Tile::SplitterVertical, Direction::North) => vec![Beam { pos: (col, row - 1), dir: loc.dir }],
        (Tile::SplitterVertical, Direction::South) => vec![Beam { pos: (col, row + 1), dir: loc.dir }],
    };

    ns.into_iter()
      .filter(|Beam { pos: (x, y), .. }| !x.is_negative() && !y.is_negative() && x < &width && y < &height)
      .collect()
}

fn flood(start: Beam, map: &Map) -> u32 {
    let mut visited: HashSet<Beam> = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);

        for b in neighbours(current, &map) {
            queue.push_back(b);
        }
    }

    visited.iter()
           .fold(HashSet::new(), |mut hs, Beam { pos, .. }| {
               hs.insert(pos);
               hs
           })
           .len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse(input);

    flood(Beam { pos: (0,0), dir: Direction::East }, &data).into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse(input);
    let (width, height) = (data[0].len() as isize, data.len() as isize);

    let top_row = (0..width).map(|col| flood(Beam { pos: (col, 0), dir: Direction::South }, &data)).max();
    let bottom_row = (0..width).map(|col| flood(Beam { pos: (col, height - 1), dir: Direction::North }, &data)).max();
    let left_col = (0..height).map(|row| flood(Beam { pos: (0, row), dir: Direction::East }, &data)).max();
    let right_col = (0..height).map(|row| flood(Beam { pos: (width - 1, row), dir: Direction::West }, &data)).max();

    vec![top_row, bottom_row, left_col, right_col].into_iter().max().unwrap().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
