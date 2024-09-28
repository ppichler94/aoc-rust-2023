use itertools::Itertools;

advent_of_code::solution!(11);

type Vec2d = (u64, u64);

pub fn part_one(input: &str) -> Option<u32> {
    let galaxies = find_galaxies(input);
    let (expansions_x, expansions_y) = find_expansions(&galaxies, 2);
    let expanded_galaxies = expand(galaxies, expansions_x, expansions_y);
    let result = calculate_distances(&expanded_galaxies);

    Some(result as u32)
}

fn calculate_distances(expanded_galaxies: &[Vec2d]) -> u64 {
    (0..expanded_galaxies.len())
        .permutations(2)
        .map(|it| it.into_iter().sorted().collect::<Vec<_>>())
        .unique()
        .map(|it| distance(&expanded_galaxies[it[0]], &expanded_galaxies[it[1]]))
        .sum()
}

fn find_galaxies(input: &str) -> Vec<Vec2d> {
    let mut galaxies = Vec::new();
    input
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    galaxies.push((x as u64, y as u64));
                }
            });
        });
    galaxies
}

fn find_expansions(galaxies: &[Vec2d], factor: u64) -> (Vec<u64>, Vec<u64>) {
    let width = galaxies.iter().max_by_key(|&&it| it.0).unwrap().0;
    let height = galaxies.iter().max_by_key(|&&it| it.1).unwrap().1;

    let mut expansions_x = Vec::new();
    let mut expansions_y = Vec::new();

    let mut space = 0;
    let mut total = 0;
    for x in 0..width {
        if !galaxies.iter().any(|&it| it.0 == x) {
            space += 1;
        } else {
            total += space * (factor - 1);
            space = 0;
        }
        expansions_x.push(total);
    }
    expansions_x.push(total + space * (factor - 1));

    space = 0;
    total = 0;
    for y in 0..height {
        if !galaxies.iter().any(|&it| it.1 == y) {
            space += 1;
        } else {
            total += space * (factor - 1);
            space = 0;
        }
        expansions_y.push(total);
    }
    expansions_y.push(total + space * (factor - 1));

    (expansions_x, expansions_y)
}

fn expand(galaxies: Vec<Vec2d>, expansions_x: Vec<u64>, expansions_y: Vec<u64>) -> Vec<Vec2d> {
    galaxies.into_iter()
        .map(|it| (it.0 + expansions_x[it.0 as usize], it.1 + expansions_y[it.1 as usize]))
        .collect()
}

fn distance(a: &Vec2d, b: &Vec2d) -> u64 {
    ((a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs()) as u64
}


pub fn part_two(input: &str) -> Option<u64> {
    let galaxies = find_galaxies(input);
    let (expansions_x, expansions_y) = find_expansions(&galaxies, 1000000);
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
