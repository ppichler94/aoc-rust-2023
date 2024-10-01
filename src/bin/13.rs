use itertools::Itertools;

advent_of_code::solution!(13);

type Map = Vec<String>;


pub fn part_one(input: &str) -> Option<u32> {
    let result = input.split("\n\n")
        .map(|map| map
            .lines()
            .map(|line| line.to_string())
            .collect()
        )
        .map(|map| find_horizontal_mirror(&map, 0) + find_vertical_mirror(&map, 0))
        .sum();


    Some(result)
}

fn find_vertical_mirror(map: &Map, goal: u32) -> u32 {
    let result = (0..map[0].len() - 1).find(|&start| {
        create_mirror_ranges(start as u32, map[0].len() as u32 - 1)
            .iter()
            .map(|(left, right)| diff(column_as_string(map, *left), column_as_string(map, *right)))
            .sum::<u32>() == goal
    });
    if let Some(value) = result {
        return (value + 1) as u32;
    }
    0
}

/// Creates a list of indices that should match if a mirror is at `start`.
/// For example for `start=4` the list `(4,5), (3,6), (2,7), (1,8), (0,9)` is generated.
///
fn create_mirror_ranges(start: u32, max: u32) -> Vec<(u32, u32)> {
    (0..=start).rev().zip(start + 1..=max).collect()
}

fn column_as_string(map: &Map, index: u32) -> String {
    map.iter().map(|row| row.chars().nth(index as usize).unwrap()).join("")
}

fn diff(lhs: String, rhs: String) -> u32 {
    lhs.chars().zip(rhs.chars()).filter(|(a, b)| a != b).count() as u32 +
        (lhs.len() as i32 - rhs.len() as i32).unsigned_abs()
}

fn find_horizontal_mirror(map: &Map, goal: u32) -> u32 {
    let result = (0..map.len() - 1).find(|&start| {
        create_mirror_ranges(start as u32, map.len() as u32 - 1)
            .iter()
            .map(|(up, down)| diff(map[*up as usize].clone(), map[*down as usize].clone()))
            .sum::<u32>() == goal
    });
    if let Some(value) = result {
        return ((value + 1) * 100) as u32;
    }
    0
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input.split("\n\n")
        .map(|map| map
            .lines()
            .map(|line| line.to_string())
            .collect()
        )
        .map(|map| find_horizontal_mirror(&map, 1) + find_vertical_mirror(&map, 1))
        .sum();


    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
