use std::collections::{HashMap, VecDeque};
use num::integer::lcm;

advent_of_code::solution!(20);

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Strength {
    Low,
    High
}

#[derive(Debug)]
struct Pulse<'a> {
    from: &'a str,
    to: &'a str,
    strength: Strength
}

#[derive(Debug)]
enum Module<'a> {
    Broadcaster {
        outputs: Vec<&'a str>
    },
    FlipFlop {
        state: bool,
        outputs: Vec<&'a str>
    },
    Conjunction {
        memory: HashMap<&'a str, Strength>,
        outputs: Vec<&'a str>
    }
}

impl<'a> Module<'a> {
    fn outputs(&self) -> &Vec<&'a str> {
        match self {
            Module::Broadcaster { outputs } => outputs,
            Module::FlipFlop { outputs, .. } => outputs,
            Module::Conjunction { outputs, .. } => outputs,
        }
    }
}

impl<'a> Pulse<'a> {
    fn process(self, modules: &mut HashMap<&'a str, Module<'a>>, queue: &mut VecDeque<Pulse<'a>>) {
        if !modules.contains_key(&self.to) {
            return;
        }

        let dest = modules.get_mut(&self.to).unwrap();

        let pulse = match dest {
            Module::Broadcaster { .. } => Some(self.strength),
            Module::FlipFlop { state, .. } => {
                match self.strength {
                    Strength::High => None,
                    Strength::Low  => {
                        *state = !*state;
                        if *state { Strength::High } else { Strength::Low }.into()
                    }
                }
            },
            Module::Conjunction { memory, .. } => {
                *memory.get_mut(self.from).unwrap() = self.strength;

                let all_high = memory.values().all(|s| *s == Strength::High);

                if all_high { Strength::Low } else { Strength::High }.into()
            }
        };

        if let Some(new_pulse) = pulse {
            for d in dest.outputs() {
                queue.push_back(Pulse { from: self.to, to: d, strength: new_pulse })
            }
        }
    }
}

fn parse(input: &str) -> HashMap<&str, Module> {
    input.lines()
         .fold(HashMap::new(), |mut hm, l| {
             let (name_str, output_str) = l.split_once(" -> ").unwrap();
             let outputs: Vec<&str> = output_str.split(", ").collect();
             let name = if &name_str[0..1] == "b" { &name_str[..] } else { &name_str[1..] };

             let module = match &name_str[0..1] {
                 "b" => Module::Broadcaster { outputs },
                 "%" => Module::FlipFlop { state: false, outputs },
                 "&" => Module::Conjunction { memory: HashMap::new(), outputs },
                 _   => panic!("Invalid gate")
             };

             hm.insert(name, module);
             hm
         })
}

fn initialize_memories<'a>(modules: &mut HashMap<&'a str, Module<'a>>) {
    let destinations: HashMap<&str, Vec<&str>> = modules.iter()
        .fold(HashMap::new(), |hm, (name, module)| {
            module.outputs().iter().fold(hm, |mut hm2, dest| {

                if !hm2.contains_key(dest) {
                    hm2.insert(&dest, vec![]);
                }

                hm2.get_mut(dest).unwrap().push(&name);
                hm2
            })
        });

    for (name, module) in modules.iter_mut() {
        if let Module::Conjunction { memory, .. } = module {
            for d in destinations[name].iter() {
                memory.insert(&d, Strength::Low);
            }
        }
    };
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut modules = parse(input);
    initialize_memories(&mut modules);

    let mut low = 0;
    let mut high = 0;

    for _ in 0..1000 {
        let mut queue: VecDeque<Pulse> = VecDeque::new();
        queue.push_back(Pulse { from: "button", to: "broadcaster", strength: Strength::Low });

        while let Some(pulse) = queue.pop_front() {
            match pulse.strength {
                Strength::Low => { low += 1; },
                Strength::High => { high += 1; },
            }

            pulse.process(&mut modules, &mut queue);
        }
    }

    Some(low * high)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut modules = parse(input);
    initialize_memories(&mut modules);

    let (prior, Module::Conjunction { memory, .. }) = modules.iter()
                                                             .find(|(_, module)| module.outputs().contains(&"rx"))
                                                             .unwrap()
                                                             else { panic!("Should be a conjunction") };

    let prior_name = &prior[..];

    let mut feeder: HashMap<&str, Option<u64>> = memory.keys().map(|&m| (m, None)).collect();

    for p in 1.. {
        let mut queue: VecDeque<Pulse> = VecDeque::new();
        queue.push_back(Pulse { from: "button", to: "broadcaster", strength: Strength::Low });

        while let Some(pulse) = queue.pop_front() {
            if pulse.to == prior_name && pulse.strength == Strength::High {
                feeder.insert(pulse.from, Some(p));

                if feeder.values().all(|v| v.is_some()) {
                    return feeder.values().fold(1, |acc, v| lcm(v.unwrap(), acc)).into();
                }
            }

            pulse.process(&mut modules, &mut queue);
        }
    }

    panic!("No result");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32000000));
    }

    // No tests for part 2.
}
