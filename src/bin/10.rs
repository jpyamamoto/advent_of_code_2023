advent_of_code::solution!(10);

#[derive(PartialEq)]
enum Tile {
    NS,
    WE,
    NW,
    NE,
    SW,
    SE,
    Ground,
    Start
}

#[derive(PartialEq, Debug)]
enum Dir {
    North,
    South,
    West,
    East
}

#[derive(Clone, Copy, Debug)]
struct Point(usize, usize);

fn next(tile: &Tile, from: &Dir) -> Dir {
    match (tile, from) {
        (Tile::NS, Dir::North) => Dir::North,
        (Tile::NS, Dir::South) => Dir::South,
        (Tile::WE, Dir::West)  => Dir::West,
        (Tile::WE, Dir::East)  => Dir::East,
        (Tile::NW, Dir::South) => Dir::West,
        (Tile::NW, Dir::East)  => Dir::North,
        (Tile::NE, Dir::South) => Dir::East,
        (Tile::NE, Dir::West)  => Dir::North,
        (Tile::SW, Dir::North) => Dir::West,
        (Tile::SW, Dir::East)  => Dir::South,
        (Tile::SE, Dir::North) => Dir::East,
        (Tile::SE, Dir::West)  => Dir::South,
        _                      => Dir::South
    }
}

fn parse(input: &str) -> ((usize, usize), Vec<Vec<Tile>>) {
    let mut start = (0,0);

    let tiles = input.lines()
                     .map(|line| line.chars().map(|c| match c {
                         '|' => Tile::NS,
                         '-' => Tile::WE,
                         'L' => Tile::NE,
                         'J' => Tile::NW,
                         '7' => Tile::SW,
                         'F' => Tile::SE,
                         '.' => Tile::Ground,
                         _   => Tile::Start
                     }).collect::<Vec<_>>())
                     .collect::<Vec<_>>();

    for i in 0..(tiles.len()) {
        for j in 0..(tiles[0].len()) {
            if tiles[i][j] == Tile::Start {
                start = (i, j);
                break;
            }
        }
    };

    (start, tiles)
}

pub fn part_one(input: &str) -> Option<u32> {
    let ((start_y, start_x), tiles) = parse(input);

    let mut heading = Dir::South;
    let (mut y, mut x) = (start_y, start_x);
    let mut count = 0;

    loop {
        heading = next(&tiles[y][x], &heading);

        let (diff_x, diff_y): (isize, isize) = match heading {
            Dir::North => (0, -1),
            Dir::South => (0, 1),
            Dir::West  => (-1, 0),
            Dir::East  => (1, 0)
        };

        (x, y) = (x.saturating_add_signed(diff_x), y.saturating_add_signed(diff_y));
        count += 1;

        if (start_y, start_x) == (y, x) {
            break;
        }
    };

    Some(count / 2)
}

fn polygon_area(points: Vec<Point>) -> i64 {
    let twice_area = points.windows(2)
                           .map(|p| {
                               let Point(x1, y1) = p[0];
                               let Point(x2, y2) = p[1];
                               (x1*y2) as i64 - (y1*x2) as i64
                           })
                           .sum::<i64>()
                           .abs();

    twice_area / 2
}

pub fn part_two(input: &str) -> Option<i64> {
    let ((start_y, start_x), tiles) = parse(input);

    let mut points: Vec<Point> = vec![];

    let mut heading = Dir::South;
    let (mut y, mut x) = (start_y, start_x);
    let mut count = 0;

    loop {
        points.push(Point(x, y));

        heading = next(&tiles[y][x], &heading);

        let (diff_x, diff_y): (isize, isize) = match heading {
            Dir::North => (0, -1),
            Dir::South => (0, 1),
            Dir::West  => (-1, 0),
            Dir::East  => (1, 0)
        };

        (x, y) = (x.saturating_add_signed(diff_x), y.saturating_add_signed(diff_y));
        count += 1;

        if (start_y, start_x) == (y, x) {
            break;
        }
    };

    points.push(points[0]);

    let area: i64 = polygon_area(points);
    let circ: i64 = count / 2;

    Some(area - circ + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
