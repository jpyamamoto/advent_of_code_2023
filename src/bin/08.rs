use std::{str::FromStr, collections::HashMap};

use num::integer::lcm;

advent_of_code::solution!(8);

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl Into<Direction> for char {
    fn into(self) -> Direction {
        match self {
            'L' => Direction::Left,
            _   => Direction::Right,
        }
    }
}

#[derive(Clone)]
struct Node {
    id: String,
    left: String,
    right: String,
}

impl Node {
    fn get_next(&self, dir: Direction) -> String {
        match dir {
            Direction::Left => self.left.to_owned(),
            Direction::Right => self.right.to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseNodeError;

impl FromStr for Node {
    type Err = ParseNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" = ").collect();

        let id = parts[0].to_string();
        let mut dirs: String = parts[1].to_string();
        dirs.pop();
        dirs = dirs[1..].to_string();

        let parts2: Vec<&str> = dirs.split(", ").collect();
        let left = parts2[0].to_string();
        let right = parts2[1].to_string();

        Ok(Node { id, left, right })
    }
}

fn parse(input: &str) -> (Vec<Direction>, HashMap<String, Node>) {
    let mut lines = input.lines();

    let dirs: Vec<Direction> = lines.next()
                                    .unwrap()
                                    .chars()
                                    .map(|c| c.into())
                                    .collect();

    lines.next();

    let nodes: Vec<Node> = lines.map(|l| l.parse().unwrap()).collect();

    let mut graph: HashMap<String, Node> = HashMap::new();

    for n in nodes.iter() {
        graph.insert(n.id.clone(), n.to_owned());
    };

    (dirs, graph)
}

fn count_steps(graph: &HashMap<String, Node>, directions: &Vec<Direction>, start: &str, single: bool) -> u64 {
    let mut dirs = directions.iter().cycle();

    let mut k: String = start.to_string();
    let mut steps = 0;

    while let Some(node) = graph.get(&k) {
        if single {
            if node.id == "ZZZ" { break; }
        } else {
            if node.id.ends_with("Z") { break; }
        }

        let curr_dir = dirs.next().unwrap();

        k = node.get_next(*curr_dir);
        steps += 1;
    };

    steps
}

pub fn part_one(input: &str) -> Option<u64> {
    let (directions, graph) = parse(input);

    Some(count_steps(&graph, &directions, "AAA", true))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (directions, graph) = parse(input);

    let starting_nodes: Vec<String> = graph.keys()
                                           .filter(|k| k.ends_with("A"))
                                           .map(|s| s.to_owned())
                                           .collect();

    let mut steps = vec![];

    for node in starting_nodes {
        steps.push(count_steps(&graph, &directions, node.as_str(), false));
    }

    let lcm = steps.iter().fold(1, |s, acc| lcm(s, *acc));

    Some(lcm)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(6));
    }
}
