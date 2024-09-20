use std::collections::HashMap;
use std::ops;

advent_of_code::solution!(2);

struct Game {
    id: u32,
    sets: Vec<Set>
}

struct Set {
    red: u32,
    green: u32,
    blue: u32
}

impl ops::Add<Set> for Set {
    type Output = Set;
    fn add(self, rhs: Set) -> Self::Output {
        Set {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue
        }
    }
}

fn build_set(text: &str) -> Set {
    let cubes: HashMap<_, _> = text
        .trim()
        .split(",")
        .map(|it| it.trim().split_once(" ").unwrap())
        .map(|(a, b)| (b, a.parse::<u32>().unwrap()))
        .collect();
    Set {
        red: *cubes.get("red").unwrap_or(&0u32),
        green: *cubes.get("green").unwrap_or(&0),
        blue: *cubes.get("blue").unwrap_or(&0)
    }
}

fn build_games(text: &str) -> Vec<Game> {
    text
        .lines()
        .map(|line| {
            let (game, rest) = line.split_once(":").unwrap();
            let (_, id) = game.split_once(" ").unwrap();
            let sets = rest
                .split(";")
                .map(build_set)
                .collect();
            Game { id: id.parse().unwrap(), sets }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = build_games(input)
        .iter()
        .filter(|game| game.sets.iter().all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14))
        .map(|game| game.id)
        .sum();
    
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = build_games(input)
        .iter()
        .map(|game| {
            let red = game.sets.iter().map(|set| set.red).max().unwrap();
            let green = game.sets.iter().map(|set| set.green).max().unwrap();
            let blue = game.sets.iter().map(|set| set.blue).max().unwrap();
            red * green * blue
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
