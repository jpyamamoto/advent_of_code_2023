use std::cmp::max;
use std::collections::{HashSet, HashMap};

advent_of_code::solution!(23);

#[derive(PartialEq)]
enum Tile {
    Path,
    Forest,
    SlopeUp,
    SlopeLeft,
    SlopeDown,
    SlopeRight
}

type Map = Vec<Vec<Tile>>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord(isize, isize);

type Graph = HashMap<Coord, HashMap<Coord, usize>>;

impl Coord {
    fn neighbours(&self, map: &Map) -> Vec<Coord> {
        let Coord(x, y) = *self;

        let ns: Vec<(isize, isize)> = match map[y as usize][x as usize] {
            Tile::Path => vec![(x, y - 1), (x - 1, y), (x, y + 1), (x + 1, y)],
            Tile::Forest => vec![],
            Tile::SlopeUp => vec![(x, y - 1)],
            Tile::SlopeLeft => vec![(x - 1, y)],
            Tile::SlopeDown => vec![(x, y + 1)],
            Tile::SlopeRight => vec![(x + 1, y)],
        };

        ns.into_iter()
          .filter(|&(x,y)| x >= 0 && y >= 0 && x < map[0].len() as isize && y < map.len() as isize)
          .filter(|&(x,y)| map[y as usize][x as usize] != Tile::Forest)
          .map(|(x, y)| Coord(x, y))
          .collect()
    }
}

fn parse(input: &str) -> Map {
    input.lines()
         .map(|l| l.chars().map(|c| match c {
             '.' => Tile::Path,
             '#' => Tile::Forest,
             '^' => Tile::SlopeUp,
             '<' => Tile::SlopeLeft,
             'v' => Tile::SlopeDown,
             '>' => Tile::SlopeRight,
             _   => panic!("Invalid char")
         }).collect())
         .collect()
}

fn build_graph(start: Coord, end: Coord, map: &Map) -> Graph {
    let mut graph: Graph = HashMap::new();

    let crossroads: Vec<Coord> = (0..map.len()).flat_map(|r|
                                                         (0..map[0].len()).map(move |c| Coord(c as isize, r as isize)))
                                               .filter(|c| c.neighbours(map).len() >= 3)
                                               .chain([start, end])
                                               .collect();

    for c in crossroads.iter().cloned() {
        graph.insert(c, HashMap::new());
    }

    for c in crossroads.iter().cloned() {
        let mut stack: Vec<(usize, Coord)> = vec![(0, c)];
        let mut seen: HashSet<Coord> = HashSet::new();
        seen.insert(c);

        while let Some((distance, coords)) = stack.pop() {
            if distance != 0 && crossroads.contains(&coords) {
                graph.get_mut(&c).unwrap().insert(coords, distance);
                continue;
            }

            for n in coords.neighbours(map) {
                if !seen.contains(&n) {
                    stack.push((distance + 1, n));
                    seen.insert(n);
                }
            }
        }
    }

    graph
}

fn longest_path(start: Coord, end: Coord, visited: &mut Vec<Coord>, graph: &Graph) -> Option<usize> {
    if start == end {
        return Some(0);
    }

    let mut longest: Option<usize> = None;

    visited.push(start);

    for (coord, distance) in &graph[&start] {
        if !visited.contains(coord) {
            longest = max(longest, longest_path(*coord, end, visited, graph).map(|v| v + distance));
        }
    }

    visited.pop();

    longest
}

fn solve(input: &str) -> Option<u32> {
    let data = parse(input);

    let starting_col = data[0].iter().position(|t| *t == Tile::Path).unwrap();
    let ending_col = data[data.len() - 1].iter().position(|t| *t == Tile::Path).unwrap();

    let graph = build_graph(
        Coord(starting_col as isize, 0),
        Coord(ending_col as isize, data.len() as isize - 1),
        &data);

    longest_path(
        Coord(starting_col as isize, 0),
        Coord(ending_col as isize, data.len() as isize - 1),
        &mut vec![],
        &graph
    ).map(|v| v as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    let new_input = input.replace("^", ".").replace("<", ".").replace("v", ".").replace(">", ".");
    solve(&new_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
