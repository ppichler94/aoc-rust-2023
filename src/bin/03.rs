use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(3);

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy, Ord, PartialOrd)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn neighbors(&self) -> HashSet<Position> {
        let mut result = HashSet::new();
        if self.x > 0 {
            result.insert(Position { x: self.x - 1, y: self.y });
            result.insert(Position { x: self.x - 1, y: self.y + 1 });
        }

        if self.y > 0 {
            result.insert(Position { x: self.x, y: self.y - 1 });
            result.insert(Position { x: self.x + 1, y: self.y - 1 });
        }

        if self.x > 0 && self.y > 0 {
            result.insert(Position { x: self.x - 1, y: self.y - 1 });
        }
        result.insert(Position { x: self.x, y: self.y });
        result.insert(Position { x: self.x + 1, y: self.y });
        result.insert(Position { x: self.x, y: self.y + 1 });
        result.insert(Position { x: self.x + 1, y: self.y + 1 });
        result
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Number {
    value: u32,
    neighbors: HashSet<Position>,
}

fn build_number(value: u32, positions: &[Position]) -> Number {
    let neighbors = positions.iter().flat_map(|it| it.neighbors()).collect();
    Number { value, neighbors }
}

fn find_numbers(text: &str) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();
    text
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            let mut acc = String::new();
            let mut positions: Vec<Position> = Vec::new();
            line
                .char_indices()
                .for_each(|(x, c)| {
                    if c.is_ascii_digit() {
                        acc.push_str(&c.to_string());
                        positions.push(Position { x, y })
                    }
                    if !c.is_ascii_digit() && !acc.is_empty() {
                        numbers.push(build_number(acc.parse().unwrap(), &positions));
                        acc.clear();
                        positions.clear()
                    }
                });
            if !acc.is_empty() {
                numbers.push(build_number(acc.parse().unwrap(), &positions));
                acc.clear();
                positions.clear()
            }
        });
    numbers
}


pub fn part_one(input: &str) -> Option<u32> {
    let numbers: Vec<Number> = find_numbers(input);
    let mut symbols = HashSet::new();
    input
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line
                .char_indices()
                .for_each(|(x, c)| {
                    if !c.is_ascii_digit() && c != '.' {
                        symbols.insert(Position { x, y });
                    }
                });
        });

    let result = numbers.iter()
        .filter(|number| !number.neighbors.is_disjoint(&symbols))
        .map(|number| number.value)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers: Vec<Number> = find_numbers(input);
    let mut gears = HashSet::new();
    input
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line
                .char_indices()
                .for_each(|(x, c)| {
                    if c == '*' {
                        gears.insert(Position { x, y });
                    }
                });
        });

    let next_to_gear: Vec<(&Number, &Position)> = numbers.iter()
        .map(|number| (number, number.neighbors.intersection(&gears).collect::<HashSet<_>>()))
        .flat_map(|(number, gears)| gears.iter().map(|it| (number, *it)).collect::<Vec<_>>())
        .collect();

    let result: u32 = next_to_gear.iter()
        .sorted_by_key(|x| x.1)
        .chunk_by(|x| x.1)
        .into_iter()
        .filter_map(|(_, it)| {
            let iter: Vec<_> = it.collect();
            if iter.len() != 2 {
                return None;
            }
            Some(iter
                .iter()
                .map(|(num, _)| num.value)
                .product::<u32>())
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
