use itertools::Itertools;
use ndarray::{Array1, arr1, arr2};
use ndarray_linalg::Solve;
use std::ops::RangeInclusive;

advent_of_code::solution!(24);

#[derive(Debug)]
struct HailStone {
    position: Array1<f64>,
    direction: Array1<f64>,
}

impl HailStone {
    fn eval(&self, time: f64) -> Array1<f64> {
        &self.position + time * &self.direction
    }
}

fn parse_hailstone(input: &str) -> HailStone {
    let mut parts = input.split(" @ ")
                         .map(|p| p.split(", ")
                                   .map(|s| s.trim().parse().unwrap())
                                   .collect());

    let position = parts.next().unwrap();
    let direction = parts.next().unwrap();

    HailStone { position, direction }
}

fn find_intersection(a: &HailStone, b: &HailStone) -> Option<(f64, f64)> {
    // A * a = B

    let matrix_a = arr2(&[[a.direction[0] as f64, -b.direction[0] as f64],
                          [a.direction[1] as f64, -b.direction[1] as f64]]);

    let vector_b = arr1(&[b.position[0] as f64 - a.position[0] as f64,
                          b.position[1] as f64 - a.position[1] as f64]);

    matrix_a.solve_into(vector_b)
            .ok()
            .map(|m| (m[0], m[1]))
}

fn parse(input: &str) -> Vec<HailStone> {
    input.lines()
         .map(|l| parse_hailstone(l))
         .collect()
}

fn the_part_one(input: &str, boundaries: RangeInclusive<f64>) -> Option<u64> {
    let data = parse(input);

    let result = data.iter()
                     .tuple_combinations()
                     .filter_map(|(h1, h2)| find_intersection(h1, h2).map(|(t1, t2)| (t1, t2, h1.eval(t1))))
                     .filter(|(t1, t2, p)| {
                         t1 >= &0. && t2 >= &0. && boundaries.contains(&p[0]) && boundaries.contains(&p[1])
                     })
                     .count();

    Some(result as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    the_part_one(input, 200000000000000f64..=400000000000000f64)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Inspiration from: https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/khlrstp/
    let data = parse(input);
    let h0 = &data[0];
    let h1 = &data[1];
    let h2 = &data[2];
    let h3 = &data[3];

    let (px0, py0, pz0, vx0, vy0, vz0) = (h0.position[0], h0.position[1], h0.position[2], h0.direction[0], h0.direction[1], h0.direction[2]);
    let (px1, py1, pz1, vx1, vy1, vz1) = (h1.position[0], h1.position[1], h1.position[2], h1.direction[0], h1.direction[1], h1.direction[2]);
    let (px2, py2, pz2, vx2, vy2, vz2) = (h2.position[0], h2.position[1], h2.position[2], h2.direction[0], h2.direction[1], h2.direction[2]);
    let (px3, py3, pz3, vx3, vy3, vz3) = (h3.position[0], h3.position[1], h3.position[2], h3.direction[0], h3.direction[1], h3.direction[2]);

    let matrix_a = arr2(&[
        [vy0 - vy1, vx1 - vx0, 0.       , py1 - py0, px0 - px1, 0.       ],
        [vz0 - vz1, 0.       , vx1 - vx0, pz1 - pz0, 0.       , px0 - px1],
        [vy0 - vy2, vx2 - vx0, 0.       , py2 - py0, px0 - px2, 0.       ],
        [vz0 - vz2, 0.       , vx2 - vx0, pz2 - pz0, 0.       , px0 - px2],
        [vy0 - vy3, vx3 - vx0, 0.       , py3 - py0, px0 - px3, 0.       ],
        [vz0 - vz3, 0.       , vx3 - vx0, pz3 - pz0, 0.       , px0 - px3],
    ]);

    let vector_b = arr1(&[
        px0 * vy0 - py0 * vx0 - px1 * vy1 + py1 * vx1,
        px0 * vz0 - pz0 * vx0 - px1 * vz1 + pz1 * vx1,
        px0 * vy0 - py0 * vx0 - px2 * vy2 + py2 * vx2,
        px0 * vz0 - pz0 * vx0 - px2 * vz2 + pz2 * vx2,
        px0 * vy0 - py0 * vx0 - px3 * vy3 + py3 * vx3,
        px0 * vz0 - pz0 * vx0 - px3 * vz3 + pz3 * vx3,
    ]);

    let result = matrix_a.solve_into(vector_b).ok().unwrap();

    let (x, y, z) = (result[0].round() as i64, result[1].round() as i64, result[2].round() as i64);

    Some((x + y + z) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = the_part_one(&advent_of_code::template::read_file("examples", DAY), 7f64..=27f64);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47));
    }
}
