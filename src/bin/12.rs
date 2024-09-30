use itertools::Itertools;
use memoize::memoize;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let result: u64 = input
        .lines()
        .map(parse_line)
        .map(|(springs, damage)| count_arrangements(springs.to_string(), damage))
        .sum();

    Some(result as u32)
}

fn parse_line(line: &str) -> (&str, Vec<i32>) {
    let (springs, damage_text) = line.split_once(" ").unwrap();
    let damage = damage_text.split(",").map(|it| it.parse().unwrap()).collect();
    (springs, damage)
}

#[memoize]
fn count_arrangements(springs: String, damage: Vec<i32>) -> u64 {
    if springs.is_empty() {
        return if damage.is_empty() { 1 } else { 0 };
    }

    match springs.chars().next().unwrap() {
        '.' => count_arrangements(springs.trim_start_matches('.').to_string(), damage),
        '?' => {
            let next_springs = springs.get(1..).unwrap().to_string();
            let next_springs2 = String::from("#") + next_springs.as_str();
            count_arrangements(next_springs, damage.clone())
                + count_arrangements(next_springs2, damage)
        }
        '#' if damage.is_empty() => 0,
        '#' if !damage.is_empty() => {
            let next_damage = damage[0] as usize;
            let remaining_damage = damage.into_iter().dropping(1).collect_vec();
            if next_damage <= springs.len() && springs.chars().take(next_damage).all(|c| c != '.') {
                if next_damage == springs.len() {
                    if remaining_damage.is_empty() { 1 } else { 0 }
                } else if springs.chars().nth(next_damage) == Some('#') {
                    0
                } else {
                    count_arrangements(springs.get(next_damage + 1..).unwrap().to_string(), remaining_damage)
                }
            } else {
                0
            }
        }
        _ => panic!("Unexpected char in springs text")
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .map(parse_line)
        .map(unfold)
        .map(|(springs, damage)| count_arrangements(springs, damage))
        .sum();

    Some(result)
}

fn unfold(input: (&str, Vec<i32>)) -> (String, Vec<i32>) {
    let springs = (0..=4).map(|_| input.0)
        .join("?");
    let damage = (0..=4).flat_map(|_| input.1.clone()).collect_vec();
    (springs, damage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
