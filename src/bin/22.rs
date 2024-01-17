use std::cmp::{max, min};
use std::collections::{VecDeque, HashSet};

advent_of_code::solution!(22);

struct Brick {
    x: usize,
    y: usize,
    z: usize,
    ex: usize,
    ey: usize,
    ez: usize
}

impl Brick {
    fn overlap_plane(&self, other: &Brick) -> bool {
        max(self.x, other.x) <= min(self.ex, other.ex) &&  // Overlap in x
            max(self.y, other.y) <= min(self.ey, other.ey) // Overlap in y
    }
}

fn parse(input: &str) -> Vec<Brick> {
    input.lines()
         .map(|l| {
             let parts: Vec<usize> = l.split(&[',', '~']).map(|c| c.parse().unwrap()).collect();

             Brick { x: parts[0],
                     y: parts[1],
                     z: parts[2],
                     ex: parts[3],
                     ey: parts[4],
                     ez: parts[5]
             }
         })
         .collect()
}

fn fall_bricks(bricks: &mut Vec<Brick>) {
    bricks.sort_by_key(|b| b.z);

    for i in 0..bricks.len() {
        let mut top_z: usize = 1;

        let bricks_below: &[Brick] = &bricks[..i];

        for b in bricks_below {
            if bricks[i].overlap_plane(b) {
                top_z = max(top_z, b.ez + 1);
            }
        }

        let brick = &mut bricks[i];

        brick.ez -= brick.z - top_z;
        brick.z = top_z;
    }

    bricks.sort_by_key(|b| b.z);
}

fn compute_supports(bricks: &Vec<Brick>) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut supported_by: Vec<Vec<usize>> = vec![vec![]; bricks.len()];
    let mut supports: Vec<Vec<usize>> = vec![vec![]; bricks.len()];

    for (j, brick) in bricks.iter().enumerate() {
        for (i, below) in bricks[..j].iter().enumerate() {
            if brick.overlap_plane(below) && brick.z == below.ez + 1 {
                supported_by[j].push(i);
                supports[i].push(j);
            }
        }
    }

    (supported_by, supports)
}

fn compute_falls(brick: usize, supported_by: &Vec<Vec<usize>>, supports: &Vec<Vec<usize>>) -> u32 {
    let will_fall: Vec<usize> = supports[brick].iter().filter(|&b| supported_by[*b].len() == 1).cloned().collect();
    let mut queue = VecDeque::from_iter(will_fall.iter().cloned());

    let mut falling: HashSet<usize> = HashSet::from_iter(will_fall.into_iter());
    falling.insert(brick);

    while let Some(other) = queue.pop_front() {
        for &b in &supports[other] {
            if !falling.contains(&b) {
                if supported_by[b].iter().all(|s| falling.contains(s)) {
                    queue.push_back(b);
                    falling.insert(b);
                }
            }
        }
    }

    (falling.len() - 1) as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut bricks = parse(input);
    fall_bricks(&mut bricks);

    let (supported_by, supports) = compute_supports(&bricks);

    let result: usize = supports.iter()
                                .filter(|bricks|
                                        bricks.iter().all(|&i| supported_by[i].len() >= 2))
                                .count();

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bricks = parse(input);
    fall_bricks(&mut bricks);

    let (supported_by, supports) = compute_supports(&bricks);

    (0..bricks.len())
        .map(|i| compute_falls(i, &supported_by, &supports))
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
