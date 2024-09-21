advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let output = input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let first = chars
                .find_map(|char| { char.to_digit(10) })
                .expect("line must contain a digit");
            let last = chars
                .rev()
                .find_map(|char| { char.to_digit(10) })
                .unwrap_or(first);

            last + 10 * first
        })
        .sum::<u32>();
    Some(output)
}

pub fn part_two(input: &str) -> Option<u32> {
    let output = input
        .lines()
        .map(|line| {
            let first = line
                .char_indices()
                .find_map(|(i, c)| c.to_digit(10).or(to_number(line.get(i..).unwrap())))
                .expect("line must contain a digit");
            let last = line
                .char_indices()
                .rev()
                .find_map(|(i, c)| c.to_digit(10).or(to_number(line.get(i..).unwrap())))
                .unwrap_or(first);
            last + 10 * first
        })
        .sum::<u32>();
    Some(output)
}

fn to_number(text: &str) -> Option<u32> {
    let mapping = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    mapping.iter()
        .find(|(word, _)| text.starts_with(word))
        .map(|(_, v)| *v as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}

