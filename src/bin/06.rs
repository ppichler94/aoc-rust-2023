use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let (times_text, distances_text) = input.split_once("\n").unwrap();
    let times = extract_values(times_text);
    let distances = extract_values(distances_text);
    let result = times.iter().zip(distances)
        .map(|(time, distance)| find_max_distance(*time, distance))
        .product();

    Some(result)
}

fn extract_values(text: &str) -> Vec<u64> {
    text
        .split_once(":").unwrap().1
        .split_whitespace()
        .map(|it| it.parse().unwrap())
        .collect()
}

fn find_max_distance(time: u64, distance: u64) -> u32 {
    (1..time)
        .filter(|it| ((time - it) * it) > distance)
        .count() as u32
}

pub fn part_two(input: &str) -> Option<u32> {
    let (times_text, distances_text) = input.split_once("\n").unwrap();
    let time = extract_value(times_text);
    let distance = extract_value(distances_text);
    Some(find_max_distance(time, distance))
}

fn extract_value(text: &str) -> u64 {
    text
        .split_once(":").unwrap().1
        .split_whitespace()
        .join("")
        .parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
