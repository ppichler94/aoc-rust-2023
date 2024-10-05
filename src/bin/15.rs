use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Rem;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.replace("\n", "").split(',').map(hash).sum();
    Some(result)
}

fn hash(step: &str) -> u32 {
    step.chars().fold(0, |acc, c| {
        let mut tmp = acc + c as u32;
        tmp *= 17;
        tmp.rem(256)
    })
}

struct Lens {
    label: String,
    focal_length: u32,
}

struct Box {
    number: u32,
    lenses: Vec<Lens>,
}

impl Box {
    fn insert_lens(&mut self, label: &str, focal_length: u32) {
        let result = self.lenses.iter().find_position(|&it| it.label == label);
        if let Some((i, _)) = result {
            self.lenses[i] = Lens { label: label.to_string(), focal_length };
        } else {
            self.lenses.push(Lens { label: label.to_string(), focal_length });
        }
    }

    fn remove_lens(&mut self, label: &str) {
        let result = self.lenses.iter().find_position(|&it| it.label == label);
        if let Some((i, _)) = result {
            self.lenses.remove(i);
        }
    }

    fn focus_power(&self) -> u32 {
        self.lenses.iter()
            .enumerate()
            .map(|(index, lens)| (self.number + 1) * (index as u32 + 1) * lens.focal_length)
            .sum()
    }

    fn at(number: u32) -> Box {
        Box { number, lenses: Vec::new() }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes = HashMap::<u32, Box>::new();
    input
        .replace("\n", "")
        .split(',')
        .for_each(|step| {
            if step.contains("=") {
                let (label, focal_length) = step.split_once('=').unwrap();
                let focal_length: u32 = focal_length.parse().unwrap();
                let number = hash(label);
                boxes.entry(number)
                    .or_insert(Box::at(number))
                    .insert_lens(label, focal_length);
            } else {
                let label = step.trim_end_matches("-");
                let number = hash(label);
                boxes.entry(number)
                    .or_insert(Box::at(number))
                    .remove_lens(label);
            }
        });

    let result = boxes.into_values()
        .map(|b| b.focus_power())
        .sum();

    Some(result)
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
