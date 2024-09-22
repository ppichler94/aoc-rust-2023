use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::ops::Range;

advent_of_code::solution!(5);

struct Lut {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl Lut {
    fn source_range(&self) -> Range<u64> {
        self.source_start..(self.source_start + self.length)
    }

    fn destination_range(&self) -> Range<u64> {
        self.destination_start..(self.destination_start + self.length)
    }
}

struct Converter {
    luts: Vec<Lut>,
    destination: String,
}

impl Converter {
    fn convert(&self, source: &u64) -> u64 {
        let lut = self.luts.iter()
            .find(|it| it.source_range().contains(&source));
        if lut.is_none() {
            return *source;
        }
        lut.unwrap().destination_start + (source - lut.unwrap().source_start)
    }

    fn parse(text: &str) -> (String, Converter) {
        let re = Regex::new(r"(\w+)-to-(\w+).*").unwrap();
        let result = re.captures(text.lines().take(1).next().unwrap()).unwrap();
        let source = result.get(1).unwrap().as_str().to_string();
        let destination = result.get(2).unwrap().as_str().to_string();
        let luts = text.lines().dropping(1)
            .map(|line| {
                let numbers: Vec<_> = line.split_whitespace().map(|it| it.parse::<u64>().unwrap()).collect();
                Lut { destination_start: numbers[0], source_start: numbers[1], length: numbers[2] }
            })
            .collect();
        (source, Converter { destination, luts })
    }
}

fn parse_input(input: &str) -> (Vec<u64>, HashMap<String, Converter>) {
    let parts: Vec<_> = input.split("\n\n").collect();
    let (_, seeds_text) = parts[0].split_once(": ").unwrap();
    let seeds: Vec<_> = seeds_text.split_whitespace().map(|it| it.parse::<u64>().unwrap()).collect();
    let converters: HashMap<_, _> = parts.iter().dropping(1)
        .map(|it| Converter::parse(*it))
        .collect();
    (seeds, converters)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, converters) = parse_input(input);

    Some(seeds.iter()
        .map(|it| convert_to_location(it, &converters))
        .min().unwrap() as u32)
}

fn convert_to_location(seed: &u64, converters: &HashMap<String, Converter>) -> u64 {
    let mut category = "seed".to_string();
    let mut number = *seed;

    while category != "location" {
        number = converters[&category].convert(&number);
        category = converters[&category].destination.clone();
    }

    number
}

pub fn part_two(input: &str) -> Option<u32> {
    let (seeds, converters) = parse_input(input);
    let result = seeds.chunks(2)
        .map(|it| it[0]..(it[0] + it[1]))
        .map(|it| it.map(|i| convert_to_location(&i, &converters)).min().unwrap())
        .min().unwrap();

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
