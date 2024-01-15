use std::collections::{VecDeque, HashSet};

advent_of_code::solution!(21);

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Rock,
    Garden,
}

type Map = Vec<Vec<Tile>>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize
}

impl Coord {
    fn neighbours(&self, map: &Map) -> Vec<Coord> {
        let mut neighs: Vec<Coord> = vec![];

        // No need to check for boundaries in part 1 because
        // they are not reachable in 64 steps.
        // For part 2 there are no boundaries.
        neighs.push(Coord { x: self.x - 1, y: self.y });
        neighs.push(Coord { x: self.x + 1, y: self.y });
        neighs.push(Coord { x: self.x, y: self.y - 1 });
        neighs.push(Coord { x: self.x, y: self.y + 1 });

        neighs.into_iter()
              .filter(|n| n.is_garden(map))
              .collect()
    }

    fn is_garden(&self, map: &Map) -> bool {
        match self.get_infinite(map) {
            Tile::Garden => true,
            Tile::Rock   => false,
        }
    }

    fn get_infinite(&self, map: &Map) -> Tile {
        let (width, height) = (map[0].len() as isize, map.len() as isize);

        map[self.y.rem_euclid(height) as usize][self.x.rem_euclid(width) as usize]
    }
}

fn parse(input: &str) -> (Coord, Map) {
    let mut starting: Coord = Coord { x: 0, y: 0 };

    let map = input.lines()
                   .enumerate()
                   .map(|(row, l)| l.chars()
                        .enumerate()
                        .map(|(col, c)| match c {
                            '.' => Tile::Garden,
                            '#' => Tile::Rock,
                            'S' => { starting = Coord { x: col as isize, y: row as isize };  Tile::Garden },
                            _   => panic!("Invalid char")
                        })
                        .collect())
                   .collect();

    (starting, map)
}

fn bfs(map: &Map, start: Coord, steps: usize) -> usize {
    let mut queue: VecDeque<(Coord, usize)> = VecDeque::new();
    queue.push_back((start, steps));

    let mut seen: HashSet<Coord> = HashSet::new();
    let mut answer: Vec<Coord> = vec![];

    while let Some((current_coord, remaining)) = queue.pop_front() {
        if remaining % 2 == 0 {
            answer.push(current_coord);
        }

        if remaining == 0 {
            continue;
        }

        for n in current_coord.neighbours(map) {
            if seen.insert(n) && n.get_infinite(map) != Tile::Rock {
                queue.push_back((n, remaining - 1));
            }
        }
    }

    answer.len() - if steps % 2 != 0 { 0 } else { 1 }
}

fn the_part_one(input: &str, steps: usize) -> Option<u32> {
    let (starting_point, map) = parse(input);

    let coords = bfs(&map, starting_point, steps);

    Some(coords as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    the_part_one(input, 64)
}

pub fn part_one_test(input: &str) -> Option<u32> {
    the_part_one(input, 6)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (starting_point, map) = parse(input);

    let coords1 = bfs(&map, starting_point, 65);
    let coords2 = bfs(&map, starting_point, 65+131);
    let coords3 = bfs(&map, starting_point, 65+131*2);

    let c = coords1;
    let b = (4 * coords2 - 3 * coords1 - coords3) / 2;
    let a = coords2 - coords1 - b;

    let x = (26501365 - map.len() / 2) / map.len();

    let result = a * x.pow(2) + b * x + c;

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_test(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
