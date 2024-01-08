advent_of_code::solution!(15);

enum Operation {
    Remove,
    Set(u32)
}

struct Lens {
    label: String,
    op: Operation,
}

fn parse(input: &str) -> Vec<Lens> {
    input.trim()
         .split(",")
         .map(|i| {
             let parts: Vec<_> = i.split(&['-', '='][..]).filter(|s| !s.is_empty()).collect();
             if parts.len() == 1 {
                 Lens { label: parts[0].to_string(), op: Operation::Remove }
             } else {
                 let val: u32 = parts[1].parse::<u32>().unwrap();
                 Lens { label: parts[0].to_string(), op: Operation::Set(val) }
             }
         })
         .collect()
}

fn hash(input: &str) -> u32 {
    input.bytes().fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

pub fn part_one(input: &str) -> Option<u32> {
    input.trim()
         .split(",")
         .map(|s| hash(s))
         .sum::<u32>()
         .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse(input);
    let mut slots: [Vec<(String, u32)>; 256] = std::array::from_fn(|_| vec![]);

    for lens in data {
        let Lens { label, op } = lens;
        let hashed_label = hash(&label) as usize;

        if let Operation::Set(val) = op {
            if let Some(i) = slots[hashed_label].iter().position(|(l, _)| *l == label) {
                slots[hashed_label][i] = (label, val);
            } else {
                slots[hashed_label].push((label, val));
            }
        } else {
            slots[hashed_label].retain(|l| l.0 != label);
        }
    };

    slots.iter()
         .enumerate()
         .map(|(b, s)| {
             s.iter()
              .enumerate()
              .map(|(i, &(_, fl))| (1 + b as u32) * (i as u32 + 1) * fl)
              .sum::<u32>()
         })
         .sum::<u32>()
         .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
