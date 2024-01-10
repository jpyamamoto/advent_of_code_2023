use std::{collections::HashMap, ops::RangeInclusive};

advent_of_code::solution!(19);

#[derive(PartialEq, Eq, Clone)]
enum State {
    Accept,
    Reject,
    Flow(String)
}

#[derive(Clone, Copy)]
enum Operation {
    GT,
    LT
}

enum RatingVar {
    X,
    M,
    A,
    S
}

struct Rule {
    ratingvar: RatingVar,
    op: Operation,
    val: u64,
    then: State
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default: State
}

struct Rating {
    x: u64,
    m: u64,
    a: u64,
    s: u64
}

#[derive(Clone)]
struct RatingRange {
    x: RangeInclusive<u64>,
    m: RangeInclusive<u64>,
    a: RangeInclusive<u64>,
    s: RangeInclusive<u64>
}

fn parse_op(input: char) -> Operation {
    match input {
        '>' => Operation::GT,
        '<' => Operation::LT,
        _   => panic!("Invalid operation")
    }
}

fn parse_state(input: &str) -> State {
    match input {
        "A" => State::Accept,
        "R" => State::Reject,
        _   => State::Flow(input.to_string())
    }
}

fn parse_rating(input: &str) -> Rating {
    let parts: Vec<u64> = input[1..input.len()-1]
        .split(",")
        .map(|p| p[2..].parse().unwrap())
        .collect();

    Rating { x: parts[0], m: parts[1], a: parts[2], s: parts[3] }
}

fn parse_ratingvar(input: char) -> RatingVar {
    match input {
        'x' => RatingVar::X,
        'm' => RatingVar::M,
        'a' => RatingVar::A,
        's' => RatingVar::S,
        _   => panic!("Invalid char")
    }
}

fn parse_rule(input: &str) -> Rule {
    let mut rule = input.to_string();
    let ratingvar = parse_ratingvar(rule.remove(0));
    let op = parse_op(rule.remove(0));
    let mut parts = rule.split(":");
    let val = parts.next().unwrap().parse().unwrap();
    let then = parse_state(parts.next().unwrap());

    Rule { ratingvar, op, val, then }
}

fn parse_workflow(input: &str) -> Workflow {
    let mut parts = input[0..input.len()-1].split(&['{', ',']);
    let name = parts.next().unwrap().to_string();

    let mut rest: Vec<&str> = parts.collect();
    let default = parse_state(rest.pop().unwrap());

    let rules = rest.iter().map(|r| parse_rule(r)).collect();

    Workflow { name, rules, default }
}

fn parse(input: &str) -> (HashMap<String, Workflow>, Vec<Rating>) {
    let mut parts = input.split("\n\n");

    let workflows = parts.next().unwrap().lines().fold(HashMap::new(), |mut hm, w| {
        let workflow = parse_workflow(w);
        hm.insert(workflow.name.clone(), workflow);
        hm
    });

    let ratings = parts.next().unwrap().lines().map(|r| parse_rating(r)).collect();

    (workflows, ratings)
}

fn execute_workflow(rating: &Rating, workflow: &Workflow) -> State {
    for rule in workflow.rules.iter() {
        let Rule { ratingvar, op, val, then } = rule;

        let accepted = match ratingvar {
            RatingVar::X => compare(rating.x, *val, *op),
            RatingVar::M => compare(rating.m, *val, *op),
            RatingVar::A => compare(rating.a, *val, *op),
            RatingVar::S => compare(rating.s, *val, *op),
        };

        if accepted {
            return then.clone();
        }
    }

    workflow.default.clone()
}

fn compare(x: u64, y: u64, op: Operation) -> bool {
    match op {
        Operation::GT => x > y,
        Operation::LT => x < y,
    }
}

fn execute(rating: &Rating, workflows: &HashMap<String, Workflow>) -> bool {
    let mut curr_workflow: State = State::Flow("in".to_string());

    loop {
        if curr_workflow == State::Accept {
            return true;
        }

        if curr_workflow == State::Reject {
            return false;
        }

        match &curr_workflow {
            State::Accept => { return true; },
            State::Reject => { return false; },
            State::Flow(workflow) => {
                curr_workflow = execute_workflow(rating, &workflows[workflow]);
            },
        }
    }
}

impl RatingRange {
    fn combinations(&self) -> u64 {
        let RatingRange { x, m, a, s } = self;

        let mut product = 1;

        product *= x.end() - x.start() + 1;
        product *= m.end() - m.start() + 1;
        product *= a.end() - a.start() + 1;
        product *= s.end() - s.start() + 1;

        product as u64
    }
}

fn split_range(range: &RangeInclusive<u64>, val: u64, op: Operation) -> (RangeInclusive<u64>, RangeInclusive<u64>) {
    match op {
        Operation::LT => (*range.start()..=val-1, val..=*range.end()),
        Operation::GT => (val+1..=*range.end(), *range.start()..=val),
    }
}

fn get_ranges(ranges: &RatingRange, state: State, workflows: &HashMap<String, Workflow>) -> u64 {
    match state {
        State::Reject => 0,
        State::Accept => ranges.combinations(),
        State::Flow(workflow) => {
            let Workflow { rules, default, .. } = &workflows[&workflow];

            let mut curr_ranges = ranges.clone();
            let mut total: u64 = 0;

            for rule in rules.iter() {
                let Rule { ratingvar, op, val, then } = rule;
                let mut accepted_ranges = curr_ranges.clone();

                match ratingvar {
                    RatingVar::X => {
                        let (accepted, rejected) = split_range(&curr_ranges.x, *val, *op);
                        accepted_ranges.x = accepted;
                        curr_ranges.x = rejected;
                    },
                    RatingVar::M => {
                        let (accepted, rejected) = split_range(&curr_ranges.m, *val, *op);
                        accepted_ranges.m = accepted;
                        curr_ranges.m = rejected;
                    },
                    RatingVar::A => {
                        let (accepted, rejected) = split_range(&curr_ranges.a, *val, *op);
                        accepted_ranges.a = accepted;
                        curr_ranges.a = rejected;
                    },
                    RatingVar::S => {
                        let (accepted, rejected) = split_range(&curr_ranges.s, *val, *op);
                        accepted_ranges.s = accepted;
                        curr_ranges.s = rejected;
                    },
                };

                total += get_ranges(&accepted_ranges, then.clone(), workflows);
            }

            total + get_ranges(&curr_ranges, default.clone(), workflows)
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (workflows, ratings) = parse(input);

    let result = ratings.iter()
                        .filter(|r| execute(r, &workflows))
                        .map(|r| r.x + r.m + r.a + r.s)
                        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (workflows, _) = parse(input);

    let range = RatingRange {
        x: (1..=4000),
        m: (1..=4000),
        a: (1..=4000),
        s: (1..=4000),
    };

    get_ranges(&range, State::Flow("in".to_string()), &workflows).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
