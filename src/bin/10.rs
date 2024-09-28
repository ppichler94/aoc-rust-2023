use advent_of_code::util::position::Position;
use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

const NORTH: Position = Position { x: 0, y: -1 };
const SOUTH: Position = Position { x: 0, y: 1 };
const EAST: Position = Position { x: 1, y: 0 };
const WEST: Position = Position { x: -1, y: 0 };

type Map = Vec<Vec<char>>;

fn movement(tile: char, direction: Position) -> Option<Position> {
    match (tile, direction) {
        ('|', NORTH) => Some(NORTH),
        ('|', SOUTH) => Some(SOUTH),
        ('-', EAST) => Some(EAST),
        ('-', WEST) => Some(WEST),
        ('L', SOUTH) => Some(EAST),
        ('L', WEST) => Some(NORTH),
        ('J', SOUTH) => Some(WEST),
        ('J', EAST) => Some(NORTH),
        ('7', NORTH) => Some(WEST),
        ('7', EAST) => Some(SOUTH),
        ('F', NORTH) => Some(EAST),
        ('F', WEST) => Some(SOUTH),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (width, height, start, mut map) = parse_map(input);

    let result = traverse_pipe(&mut map, start, width, height, &|_, _, _, _| {}).len();
    Some((result / 2) as u32)
}

fn traverse_pipe(map: &mut Map, start: Position, width: usize, height: usize, pre_move: &dyn Fn(&Position, &Position, &Position, &mut Map)) -> HashSet<Position> {
    let mut current = start.neighbors(Position::moves(false, false))
        .into_iter()
        .filter(|it| it.is_safe(width as i64, height as i64))
        .find(|&pos| {
            let dir = pos - start;
            movement(map[pos.y as usize][pos.x as usize], dir).is_some()
        }).unwrap();

    let mut direction = current - start;
    let mut pipe = HashSet::new();
    pipe.insert(start);
    while current != start {
        pipe.insert(current);
        let tile = map[current.y as usize][current.x as usize];
        let next_direction = movement(tile, direction).expect("Invalid movement");
        pre_move(&current, &direction, &next_direction, map);
        direction = next_direction;
        current = current + direction;
    }
    pipe
}

fn parse_map(input: &str) -> (usize, usize, Position, Map) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let map: Map = input.lines()
        .map(|it| it.chars().collect())
        .collect();
    let start_index = input.find('S').unwrap();
    let start_x = start_index % (width + 1); // add +1 to account for newlines
    let start_y = start_index / (width + 1);
    let start = Position::from((start_x, start_y));
    (width, height, start, map)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (width, height, start, mut map) = parse_map(input);

    let pipe = traverse_pipe(&mut map, start, width, height, &|_, _, _, _| {});
    map.iter_mut()
        .enumerate()
        .for_each(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .for_each(|(x, c)| {
                    let pos = Position::from((x, y));
                    if !pipe.contains(&pos) {
                        *c = '.'
                    }
                })
        });

    let empty_corner = vec![Position::at(0, 0), Position::at(0, (height - 1) as i64), Position::at((width - 1) as i64, 0), Position::at(width as i64, height as i64)]
        .into_iter().find(|it| map[it.y as usize][it.x as usize] == '.')
        .unwrap();

    let corners = HashSet::from(['7', 'L', 'J', 'F']);
    traverse_pipe(&mut map, start, width, height, &|pos, dir, next_dir, map| {
        fill_area(map, *pos + marking_direction(*dir), 'O', width as i64, height as i64);
        if corners.contains(&map[pos.y as usize][pos.x as usize]) {
            fill_area(map, *pos + marking_direction(*next_dir), 'O', width as i64, height as i64);
        }
    });

    let target = if map[empty_corner.y as usize][empty_corner.x as usize] == 'O' { '.' } else { 'O' };
    Some(map.iter().flatten().filter(|c| **c == target).count() as u32)
}

fn marking_direction(dir: Position) -> Position {
    match dir {
        NORTH => WEST,
        SOUTH => EAST,
        WEST => SOUTH,
        EAST => NORTH,
        _ => panic!("Invalid direction to mark")
    }
}

fn fill_area(map: &mut Map, pos: Position, c: char, width: i64, height: i64) {
    if !pos.is_safe(width, height) {
        return;
    }

    let mut queue = VecDeque::<Position>::new();
    queue.push_back(pos);
    while !queue.is_empty() {
        let next = queue.pop_front().unwrap();
        if !next.is_safe(width, height) {
            continue;
        }
        if map[next.y as usize][next.x as usize] == '.' {
            map[next.y as usize][next.x as usize] = c;
            queue.extend(next.neighbors(Position::moves(false, false)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one2() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two2() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 4));
        assert_eq!(result, Some(8));
    }
}
