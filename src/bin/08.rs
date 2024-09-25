use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(8);



pub fn part_one(input: &str) -> Option<u32> {
    let (instructions, route_map) = parse_input(input);

    Some(count_steps("AAA", instructions, &route_map))
}

fn parse_input(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let instructions = input.lines().next().unwrap();
    let route_map = input.lines()
        .dropping(2)
        .map(|line| (line.get(0..=2).unwrap(), (line.get(7..=9).unwrap(), line.get(12..=14).unwrap())))
        .collect();
    (instructions, route_map)
}

fn count_steps(start: &str, instructions: &str, route_map: &HashMap<&str, (&str, &str)>) -> u32 {
    let mut steps = 0;
    let mut current = start;
    while !current.ends_with('Z') {
        let route = route_map[current];
        let index = steps % instructions.len();
        steps += 1;
        let instruction = instructions.get(index..=index).unwrap();
        current = if instruction == "L" { route.0 } else { route.1 }
    }
    steps as u32
}

pub fn part_two(input: &str) -> Option<u32> {
    let (instructions, route_map) = parse_input(input);

    let result = route_map.keys()
        .filter(|it| it.ends_with('A'))
        .map(|it| count_steps(it, instructions, &route_map) as u64)
        .reduce(lcm)
        .unwrap();

    Some(result as u32)
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one2() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(6));
    }
}
