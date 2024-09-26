use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.lines()
        .map(|it| it.split_whitespace().map(|it| it.parse().unwrap()).collect::<Vec<i32>>())
        .map(generate_sequences)
        .map(extrapolate)
        .sum::<i32>();
    Some(result as u32)
}

fn generate_sequences(history: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sequences = Vec::new();
    let mut current = history;
    while current.iter().all_equal_value() != Ok(&0) {
        let next_sequence = current.windows(2)
            .map(|it| it[1] - it[0])
            .collect();

        sequences.push(current.clone());
        current = next_sequence;
    }
    sequences.push(current.clone());
    sequences
}

fn extrapolate(mut sequences: Vec<Vec<i32>>) -> i32 {
    let last_index = sequences.len() - 1;
    sequences[last_index].push(0);
    for i in (0..last_index).rev() {
        let v = sequences[i].last().unwrap() + sequences[i + 1].last().unwrap();
        sequences[i].push(v)
    }

    *sequences[0].last().unwrap()
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input.lines()
        .map(|it| it.split_whitespace().map(|it| it.parse().unwrap()).rev().collect::<Vec<i32>>())
        .map(generate_sequences)
        .map(extrapolate)
        .sum::<i32>();
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
