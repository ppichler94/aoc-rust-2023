use advent_of_code::util::position::Position;
use itertools::Itertools;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let galaxies = find_galaxies(input);
    let expansions_x = find_expansions(&galaxies, 2, &|it| it.x);
    let expansions_y = find_expansions(&galaxies, 2, &|it| it.y);
    let expanded_galaxies = expand(galaxies, expansions_x, expansions_y);
    let result = calculate_distances(&expanded_galaxies);

    Some(result as u32)
}

fn calculate_distances(expanded_galaxies: &[Position]) -> u64 {
    (0..expanded_galaxies.len())
        .permutations(2)
        .map(|it| it.into_iter().sorted().collect::<Vec<_>>())
        .unique()
        .map(|it| expanded_galaxies[it[0]].distance_manhatten(&expanded_galaxies[it[1]]))
        .sum::<i64>() as u64
}

fn find_galaxies(input: &str) -> Vec<Position> {
    let mut galaxies = Vec::new();
    input
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    galaxies.push(Position::from((x, y)));
                }
            });
        });
    galaxies
}

fn find_expansions(galaxies: &[Position], factor: i64, key_extractor: &dyn Fn(&Position) -> i64) -> Vec<i64> {
    let max_pos = galaxies.iter().max_by_key(|&it| key_extractor(it)).unwrap();
    let max_index = key_extractor(max_pos);

    let mut expansions = Vec::new();

    let mut space = 0;
    let mut total = 0;
    for x in 0..max_index {
        if !galaxies.iter().any(|it| key_extractor(it) == x) {
            space += 1;
        } else {
            total += space * (factor - 1);
            space = 0;
        }
        expansions.push(total);
    }
    expansions.push(total + space * (factor - 1));

    expansions
}

fn expand(galaxies: Vec<Position>, expansions_x: Vec<i64>, expansions_y: Vec<i64>) -> Vec<Position> {
    galaxies.into_iter()
        .map(|it| Position { x: it.x + expansions_x[it.x as usize], y: it.y + expansions_y[it.y as usize] })
        .collect()
}

pub fn part_two(input: &str) -> Option<u64> {
    let galaxies = find_galaxies(input);
    let expansions_x = find_expansions(&galaxies, 1000000, &|it| it.x);
    let expansions_y = find_expansions(&galaxies, 1000000, &|it| it.y);
    let expanded_galaxies = expand(galaxies, expansions_x, expansions_y);
    let result = calculate_distances(&expanded_galaxies);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
