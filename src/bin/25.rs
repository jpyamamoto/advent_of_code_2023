use std::convert::Infallible;
use std::collections::HashMap;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::Undirected;
use rustworkx_core::petgraph::csr::Csr;

advent_of_code::solution!(25);

type Graph<'a> = Csr<&'a str, (), Undirected>;

fn parse_wiring(input: &str) -> (&str, Vec<&str>) {
    let (key, wires) = input.split_once(": ").unwrap();
    (key, wires.split_ascii_whitespace().collect())
}

fn parse(input: &str) -> Graph {
    let wires = input.lines()
                     .map(|l| parse_wiring(l))
                     .collect::<Vec<_>>();

    let mut nodes: HashMap<&str, u32> = HashMap::new();
    let mut graph = Csr::new();

    for (wire, _) in wires.iter() {
        nodes.insert(wire, graph.add_node(*wire));
    }

    for (wire, conns) in wires {
        let wire_idx = nodes[wire];

        for conn in conns {
            let conn_idx = nodes.entry(conn).or_insert_with(|| graph.add_node(conn));
            graph.add_edge(wire_idx, *conn_idx, ());
        }
    }

    graph
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph = parse(input);

    let (_, connected_set) = stoer_wagner_min_cut(&graph, |_| Ok::<i32, Infallible>(1)).unwrap().unwrap();

    let size1 = connected_set.len();
    let size2 = graph.node_count() - size1;

    Some((size1 * size2) as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
