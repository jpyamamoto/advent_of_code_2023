use std::{ops::Range, str::Lines};
use rayon::prelude::*;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Map {
    source_range: Range<usize>,
    offset: i64,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    seed_soil: Vec<Map>,
    soil_fertilizer: Vec<Map>,
    fertilizer_water: Vec<Map>,
    water_light: Vec<Map>,
    light_temperature: Vec<Map>,
    temperature_humidity: Vec<Map>,
    humidity_location: Vec<Map>,
}

impl Map {
    fn send(&self, x: usize) -> usize {
        if self.source_range.contains(&x) {
            (x as i64 + self.offset) as usize
        } else {
            x
        }
    }

    fn contains(&self, x: usize) -> bool {
        self.source_range.contains(&x)
    }
}

impl Almanac {
    fn take_map(&self, x: usize, mappings: &Vec<Map>) -> usize {
        for map in mappings {
            if map.contains(x) {
                return map.send(x);
            }
        }

        x
    }

    fn get_location(&self, seed: usize) -> usize {
        let soil = self.take_map(seed, &self.seed_soil);
        let fertilizer = self.take_map(soil, &self.soil_fertilizer);
        let water = self.take_map(fertilizer, &self.fertilizer_water);
        let light = self.take_map(water, &self.water_light);
        let temperature = self.take_map(light, &self.light_temperature);
        let humidity = self.take_map(temperature, &self.temperature_humidity);
        let location = self.take_map(humidity, &self.humidity_location);

        location
    }
}

fn parse_ranges(input: &mut Lines) -> Vec<Map> {
    input.next();

    let mut ranges: Vec<Map> = vec![];

    while let Some(s) = input.next() {
        if s.is_empty() {
            break;
        }

        let parts : Vec<usize> = s.split(" ").map(|n| n.parse().unwrap()).collect();

        let offset: i64 = (parts[0] as i64) - (parts[1] as i64);
        let source_range: Range<usize> = parts[1]..(parts[1]+parts[2]);

        ranges.push(Map { source_range, offset });
    }

    ranges.sort_by_key(|m| m.source_range.start);

    ranges
}

fn parse(input: &str) -> Almanac {
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();

    lines.next();

    let seeds : Vec<usize> = first_line.split(": ")
                                       .collect::<Vec<&str>>()[1]
                                       .split(" ")
                                       .map(|s| s.parse().unwrap())
                                       .collect();

    let seed_soil = parse_ranges(&mut lines);
    let soil_fertilizer = parse_ranges(&mut lines);
    let fertilizer_water = parse_ranges(&mut lines);
    let water_light = parse_ranges(&mut lines);
    let light_temperature = parse_ranges(&mut lines);
    let temperature_humidity = parse_ranges(&mut lines);
    let humidity_location = parse_ranges(&mut lines);

    Almanac { seeds,
              seed_soil,
              soil_fertilizer,
              fertilizer_water,
              water_light,
              light_temperature,
              temperature_humidity,
              humidity_location
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let almanac = parse(input);

    let result = almanac.seeds
                        .iter()
                        .map(|&s| almanac.get_location(s))
                        .min()
                        .unwrap();

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let almanac = parse(input);

    let seed_ranges : Vec<_> = almanac.seeds
                                      .chunks(2)
                                      .into_iter()
                                      .map(|pair| pair[0]..(pair[0]+pair[1]))
                                      .collect();

    let result = seed_ranges.par_iter()
                            .flat_map(|r| r.clone().into_iter())
                            .map(|s| almanac.get_location(s))
                            .min()
                            .unwrap();

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
