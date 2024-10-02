use advent_of_code::util::grid2d::Grid2d;
use advent_of_code::util::position::{Position, EAST, NORTH, SOUTH, WEST};
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid2d::of_lines(input);
    tilt_grid(&mut grid, &NORTH);

    Some(score(&grid))
}

fn progressions(grid: &Grid2d<char>) -> HashMap<Position, Vec<Position>> {
    let (width, height) = grid.size();
    HashMap::from([
        (NORTH, (0..height).flat_map(|y| {
            (0..width).map(move |x| Position::from((x, y)))
        }).collect()),
        (WEST, (0..width).flat_map(|x| {
            (0..height).map(move |y| Position::from((x, y)))
        }).collect()),
        (SOUTH, (0..height).rev().flat_map(|y| {
            (0..width).map(move |x| Position::from((x, y)))
        }).collect()),
        (EAST, (0..width).rev().flat_map(|x| {
            (0..height).map(move |y| Position::from((x, y)))
        }).collect()),
    ])
}

fn tilt_grid(grid: &mut Grid2d<char>, direction: &Position) {
    let stones: Vec<Position> = progressions(grid)[direction].clone().into_iter()
        .filter(|it| grid.get(it) == 'O')
        .collect();
    stones.iter().for_each(|it| tilt_rock(grid, it, direction))
}

fn tilt_rock(grid: &mut Grid2d<char>, position: &Position, direction: &Position) {
    let mut current = *position;
    let (width, height) = grid.size();
    while (current + *direction).is_safe(width as i64, height as i64) && grid.get(&(&current + direction)) == '.' {
        grid.swap(&current, &(&current + direction));
        current = &current + direction;
    }
}

fn score(grid: &Grid2d<char>) -> u32 {
    let (_, height) = grid.size();
    grid.find_all('O').iter()
        .map(|it| height as i64 - it.y)
        .sum::<i64>() as u32
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = Grid2d::of_lines(input);
    Some(solve_cycles(&mut grid, 1_000_000_000))
}

fn solve_cycles(grid: &mut Grid2d<char>, goal: u32) -> u32 {
    let mut seen = HashMap::<u64, u32>::new();
    let mut cycle = 1;
    while cycle <= goal {
        tilt_grid(grid, &NORTH);
        tilt_grid(grid, &WEST);
        tilt_grid(grid, &SOUTH);
        tilt_grid(grid, &EAST);
        let state = hash(grid);
        if let Vacant(e) = seen.entry(state) {
            e.insert(cycle);
            cycle += 1;
        } else {
            let cycle_length = cycle - seen[&state];
            let remaining_cycles = (goal - cycle) % cycle_length;
            (0..remaining_cycles).for_each(|_| {
                tilt_grid(grid, &NORTH);
                tilt_grid(grid, &WEST);
                tilt_grid(grid, &SOUTH);
                tilt_grid(grid, &EAST);
            });
            return score(grid);
        }
    }
    score(grid)
}

fn hash(grid: &Grid2d<char>) -> u64 {
    let mut s = DefaultHasher::new();
    grid.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
