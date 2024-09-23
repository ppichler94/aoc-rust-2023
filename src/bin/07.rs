use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

advent_of_code::solution!(7);

struct Hand {
    cards: String,
    bid: u32,
    strength: u32,
    card_values: HashMap<char, u32>,
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.bid == other.bid && self.strength == other.strength && self.cards == other.cards
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.strength != other.strength {
            return Ord::cmp(&self.strength, &other.strength);
        }

        let difference = self.cards.chars().zip(other.cards.chars())
            .find(|(a, b)| a != b);

        if difference.is_some() {
            return Ord::cmp(&self.card_values[&difference.unwrap().0], &self.card_values[&difference.unwrap().1]);
        }

        Ordering::Equal
    }
}

impl Eq for Hand {}

impl Hand {
    fn from_line1(line: &str) -> Hand {
        let (cards, bid) = line.split_once(" ").unwrap();
        let mut counts: HashMap<_, _> = cards.chars()
            .sorted()
            .chunk_by(|it| *it)
            .into_iter()
            .map(|(card, chunk)| (card, chunk.count()))
            .collect();
        let high_card = *counts.iter().max_by_key(|(_, count)| **count).unwrap().0;
        let max_count = counts.remove(&high_card).unwrap_or(0);
        let strength = match max_count {
            5 => 7,
            4 => 6,
            3 if counts.values().contains(&2) => 5,
            3 if !counts.values().contains(&2) => 4,
            2 if counts.values().contains(&2) => 3,
            2 if !counts.values().contains(&2) => 2,
            _ => 1
        };
        let card_values = HashMap::from([
            ('A', 14),
            ('K', 13),
            ('Q', 12),
            ('J', 11),
            ('T', 10),
            ('9', 9),
            ('8', 8),
            ('7', 7),
            ('6', 6),
            ('5', 5),
            ('4', 4),
            ('3', 3),
            ('2', 2),
        ]);

        Hand { cards: cards.to_string(), bid: bid.parse().unwrap(), strength, card_values }
    }

    fn from_line2(line: &str) -> Hand {
        let (cards, bid) = line.split_once(" ").unwrap();
        let mut counts: HashMap<_, _> = cards.chars()
            .sorted()
            .chunk_by(|it| *it)
            .into_iter()
            .map(|(card, chunk)| (card, chunk.count()))
            .collect();
        let jokers = counts.remove(&'J').unwrap_or(0);
        let high_card = *counts.iter().max_by_key(|(_, count)| **count).unwrap_or((&'x', &0)).0;
        let max_count = counts.remove(&high_card).unwrap_or(0);
        let strength = match max_count + jokers {
            5 => 7,
            4 => 6,
            3 if counts.values().contains(&2) => 5,
            3 if !counts.values().contains(&2) => 4,
            2 if counts.values().contains(&2) => 3,
            2 if !counts.values().contains(&2) => 2,
            _ => 1
        };
        let card_values = HashMap::from([
            ('A', 14),
            ('K', 13),
            ('Q', 12),
            ('J', 1),
            ('T', 10),
            ('9', 9),
            ('8', 8),
            ('7', 7),
            ('6', 6),
            ('5', 5),
            ('4', 4),
            ('3', 3),
            ('2', 2),
        ]);

        Hand { cards: cards.to_string(), bid: bid.parse().unwrap(), strength, card_values }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(Hand::from_line1)
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(Hand::from_line2)
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
