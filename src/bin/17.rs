use std::{collections::{HashSet, BinaryHeap}, fmt::Debug, cmp::Ordering};

advent_of_code::solution!(17);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Self::North => Self::South,
            Self::West  => Self::East,
            Self::South => Self::North,
            Self::East => Self::West
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    x: usize,
    y: usize
}

type Map = Vec<Vec<u32>>;

#[derive(PartialEq, Eq, Clone)]
struct Crucible {
    loss: u32,
    pos: Coord,
    dir: Direction,
    moves: usize,
}

impl Crucible {
    fn move_dir(&self, dir: Direction, map: &Map) -> Option<Crucible> {
        let (width, height) = (map[0].len(), map.len());

        let new_coords = match dir {
            Direction::North if self.pos.y > 0 =>
                Some(Coord { x: self.pos.x, y: self.pos.y - 1 }),
            Direction::West if self.pos.x > 0 =>
                Some(Coord { x: self.pos.x - 1, y: self.pos.y }),
            Direction::South if self.pos.y + 1 < height =>
                Some(Coord { x: self.pos.x, y: self.pos.y + 1 }),
            Direction::East if self.pos.x + 1 < width =>
                Some(Coord { x: self.pos.x + 1, y: self.pos.y }),
            _ => None
        };

        new_coords.map(|Coord { x, y }|
            Crucible {
                loss: self.loss + map[y][x],
                pos: Coord { x, y },
                dir,
                moves: if self.dir == dir { self.moves + 1 } else { 1 },
            })
    }
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> Ordering {
        other.loss.cmp(&self.loss)
    }
}

impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Map {
    input.lines()
         .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
         .collect()
}

fn neighbours(crucible: Crucible, map: &Map, min_moves: usize, max_moves: usize) -> Vec<Crucible> {
    let Crucible { dir, moves, .. } = crucible;

    let mut ns: Vec<Option<Crucible>> = vec![];

    for d in vec![Direction::North, Direction::West, Direction::South, Direction::East] {
        if moves >= max_moves && dir == d {
            continue;
        }

        if moves < min_moves && dir != d {
            continue;
        }

        if dir.opposite() == d {
            continue;
        }

        ns.push(crucible.move_dir(d, map));
    }

    ns.into_iter().filter_map(|x| x).collect()
}

fn find_path(start: Crucible, goal: Coord, map: &Map, min_moves: usize, max_moves: usize) -> u32 {
    let mut visited: HashSet<(Coord, Direction, usize)> = HashSet::new();
    let mut queue = BinaryHeap::new();

    queue.push(start.move_dir(Direction::East, &map).unwrap());
    queue.push(start.move_dir(Direction::South, &map).unwrap());

    while let Some(current) = queue.pop() {
        if current.pos == goal && current.moves >= min_moves {
            return current.loss;
        }

        for c in neighbours(current, &map, min_moves, max_moves) {
            if visited.insert((c.pos, c.dir, c.moves)) {
                queue.push(c);
            }
        }
    }

    panic!("No path found")
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse(input);

    let crucible = Crucible { loss: 0, pos: Coord { x: 0, y: 0 }, dir: Direction::North, moves: 1 };
    let goal = Coord { x: data[0].len() - 1, y: data.len() - 1 };

    find_path(crucible, goal, &data, 0, 3).into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse(input);

    let crucible = Crucible { loss: 0, pos: Coord { x: 0, y: 0 }, dir: Direction::North, moves: 1 };
    let goal = Coord { x: data[0].len() - 1, y: data.len() - 1 };

    find_path(crucible, goal, &data, 4, 10).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
