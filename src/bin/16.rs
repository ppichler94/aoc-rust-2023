use advent_of_code::util::grid2d::Grid2d;
use advent_of_code::util::position::{Position, EAST, NORTH, SOUTH, WEST};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid2d::of_lines(input);
    let result = energize(Position::from((0, 0)), EAST, &grid);
    Some(result)
}

fn movement(tile: char, direction: Position) -> Vec<Position> {
    match (tile, direction) {
        ('|', NORTH | SOUTH) => vec![direction],
        ('|', EAST | WEST) => vec![NORTH, SOUTH],
        ('-', EAST | WEST) => vec![direction],
        ('-', NORTH | SOUTH) => vec![WEST, EAST],
        ('/', EAST) => vec![NORTH],
        ('/', NORTH) => vec![EAST],
        ('/', WEST) => vec![SOUTH],
        ('/', SOUTH) => vec![WEST],
        ('\\', EAST) => vec![SOUTH],
        ('\\', SOUTH) => vec![EAST],
        ('\\', WEST) => vec![NORTH],
        ('\\', NORTH) => vec![WEST],
        ('.', _) => vec![direction],
        _ => vec![],
    }
}

fn energize(start: Position, start_direction: Position, grid: &Grid2d<char>) -> u32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    visited.insert((start, start_direction));
    queue.push_back((start, start_direction));

    while !queue.is_empty() {
        let (current, direction) = queue.pop_front().unwrap();
        let next_directions = movement(grid.get(&current), direction);
        next_directions.iter()
            .for_each(|&dir| {
                let next_pos = current + dir;
                if !visited.contains(&(next_pos, dir)) && next_pos.is_within(grid) {
                    queue.push_back((next_pos, dir));
                    visited.insert((next_pos, dir));
                }
            })
    }

    visited.iter().map(|(pos, _)| *pos).unique().count() as u32
}

#[allow(unused)]
fn print_visited(grid: &Grid2d<char>, visited: &HashSet<Position>) {
    grid.for_each(|x, y, c| {
        if x == 0 {
            println!()
        }
        if visited.contains(&Position::at(x as i64, y as i64)) {
            print!("#");
        } else {
            print!(".");
        }
    });
    print!("\n\n");
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid2d::of_lines(input);
    let (width, height) = grid.size();

    let mut starts = Vec::new();
    starts.extend((0..width).map(|it| (Position::from((it, 0)), SOUTH)));
    starts.extend((0..width).map(|it| (Position::from((it, height - 1)), NORTH)));
    starts.extend((0..height).map(|it| (Position::from((0, it)), EAST)));
    starts.extend((0..height).map(|it| (Position::from((width - 1, it)), WEST)));

    let result = starts.iter().map(|(start, dir)| energize(*start, *dir, &grid)).max()?;
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
