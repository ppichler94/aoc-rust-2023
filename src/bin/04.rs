use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(4);

#[derive(Debug, Eq, PartialEq, Hash)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    own_numbers: Vec<u32>,
}

impl Card {
    fn from_line(line: &str) -> Card {
        let (card_text, numbers) = line.split_once(": ").unwrap();
        let id_text = card_text.split_whitespace().get(1..=1).next().unwrap();
        let id = id_text.parse().unwrap();
        let (winning_text, own_text) = numbers.split_once(" | ").unwrap();
        let winning_numbers = winning_text
            .split_whitespace()
            .filter_map(|it| it.parse().ok())
            .collect();
        let own_numbers = own_text
            .split_whitespace()
            .filter_map(|it| it.parse().ok())
            .collect();

        Card { id, winning_numbers, own_numbers }
    }

    fn points(self) -> u32 {
        self.own_numbers.iter()
            .filter(|it| self.winning_numbers.contains(it))
            .count() as u32
    }

    fn matches(&self) -> usize {
        let winning = self.winning_numbers.iter().collect::<HashSet<_>>();
        let own = self.own_numbers.iter().collect::<HashSet<_>>();
        winning.intersection(&own).count()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(Card::from_line)
        .map(|it| it.points())
        .filter(|it| *it > 0)
        .map(|it| 2u32.pow(it - 1))
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards: Vec<Card> = input
        .lines()
        .map(Card::from_line)
        .collect();

    let mut copies: Vec<u32> = vec![1; cards.len()];

    for i in 0..cards.len() {
        for j in i + 1..=i + cards[i].matches() {
            copies[j] += copies[i]
        }
    }

    Some(copies.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
