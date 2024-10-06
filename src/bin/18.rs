use advent_of_code::util::position::{Position, EAST, NORTH, SOUTH, WEST};
use itertools::Itertools;

advent_of_code::solution!(18);

struct Step {
    direction: Position,
    steps: u64,
}

impl Step {
    fn parse(line: &str) -> Step {
        let mut parts = line.split_whitespace();
        let direction = match parts.next().unwrap() {
            "U" => NORTH,
            "R" => EAST,
            "D" => SOUTH,
            "L" => WEST,
            _ => panic!("Illegal direction"),
        };
        let steps: u64 = parts.next().unwrap().parse().unwrap();

        Step { direction, steps }
    }

    fn decode(line: &str) -> Step {
        let hexcode = line.split_whitespace().last().unwrap();
        let steps = hexcode.get(2..=6).unwrap();
        let steps = u64::from_str_radix(steps, 16).unwrap();
        let direction = match hexcode.get(7..=7).unwrap() {
            "3" => NORTH,
            "0" => EAST,
            "1" => SOUTH,
            "2" => WEST,
            _ => panic!("Illegal direction"),
        };
        Step { direction, steps }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let steps = input.lines().map(Step::parse).collect_vec();
    let result = calculate_area(steps);
    Some(result as u32)
}

fn calculate_area(steps: Vec<Step>) -> u64 {
    let mut current = Position { x: 0, y: 0 };
    let mut visited = vec![current];

    steps.iter().for_each(|step| {
        let new_position = current + step.direction * step.steps as i64;
        visited.push(current);
        current = new_position;
    });
    visited.push(Position { x: 0, y: 0 });

    // use shoelace formula to calculate the are
    // https://en.wikipedia.org/wiki/Shoelace_formula#Shoelace_formula
    let area = visited.windows(2)
        .fold(0, |acc, p| acc + p[0].x * p[1].y - p[1].x * p[0].y) as u64 / 2;
    let perimeter = steps.iter().fold(0, |acc, step: &Step| acc + step.steps);

    // use pick's theorem to get the result (A = i + b/2 - 1)
    // we want to calculate i + b = A + b/2 + 1
    area + perimeter / 2 + 1
}


pub fn part_two(input: &str) -> Option<u64> {
    let steps = input.lines().map(Step::decode).collect_vec();
    let result = calculate_area(steps);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
