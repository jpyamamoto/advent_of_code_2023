advent_of_code::solution!(9);

fn parse(input: &str) -> Vec<Vec<i64>> {
    input.lines()
         .map(|line| line.split_whitespace()
                         .map(|num| num.parse::<i64>().unwrap())
                         .collect())
         .collect()
}

fn extrapolate(nums: &Vec<i64>) -> i64 {
    if nums.iter().all(|n| *n == 0) {
        return 0;
    }

    let diffs: Vec<i64> = nums.windows(2).map(|w| w[1] - w[0]).collect();
    let forecast = extrapolate(&diffs);

    *nums.last().unwrap() + forecast
}

pub fn part_one(input: &str) -> Option<i64> {
    let result = parse(input).iter()
                             .map(|nums| extrapolate(nums))
                             .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let result = parse(input).iter_mut()
                             .map(|nums| {
                                 nums.reverse();
                                 extrapolate(&nums)
                             })
                             .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
