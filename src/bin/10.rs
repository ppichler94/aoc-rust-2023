use std::collections::{HashSet, VecDeque};
use std::ops::{Add, Sub};

advent_of_code::solution!(10);

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Copy)]
struct Vec2d(i32, i32);

impl Vec2d {
    fn is_safe(&self, size: &Vec2d) -> bool {
        let &Vec2d(x, y) = self;
        let &Vec2d(width, height) = size;
        x >= 0 && y >= 0 && x < width && y < height
    }
}

impl Add for Vec2d {
    type Output = Vec2d;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2d(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vec2d {
    type Output = Vec2d;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2d(self.0 - rhs.0, self.1 - rhs.1)
    }
}

const NORTH: Vec2d = Vec2d(0, -1);
const SOUTH: Vec2d = Vec2d(0, 1);
const EAST: Vec2d = Vec2d(1, 0);
const WEST: Vec2d = Vec2d(-1, 0);

type Map = Vec<Vec<char>>;

fn movement(tile: char, direction: Vec2d) -> Option<Vec2d> {
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

fn traverse_pipe(map: &mut Map, start: Vec2d, width: usize, height: usize, pre_move: &dyn Fn(&Vec2d, &Vec2d, &Vec2d, &mut Map)) -> HashSet<Vec2d> {
    let neighbors = [start + NORTH, start + SOUTH, start + EAST, start + WEST];
    let size = Vec2d(width as i32, height as i32);

    let mut current = *neighbors.iter()
        .filter(|it| it.is_safe(&size))
        .find(|&&pos| {
            let dir = pos - start;
            movement(map[pos.1 as usize][pos.0 as usize], dir).is_some()
        }).unwrap();

    let mut direction = current - start;
    let mut pipe = HashSet::new();
    pipe.insert(start);
    while current != start {
        pipe.insert(current);
        let tile = map[current.1 as usize][current.0 as usize];
        let next_direction = movement(tile, direction).expect("Invalid movement");
        pre_move(&current, &direction, &next_direction, map);
        direction = next_direction;
        current = current + direction;
    }
    pipe
}

fn parse_map(input: &str) -> (usize, usize, Vec2d, Map) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let map: Map = input.lines()
        .map(|it| it.chars().collect())
        .collect();
    let start_index = input.find('S').unwrap();
    let start_x = start_index % (width + 1); // add +1 to account for newlines
    let start_y = start_index / (width + 1);
    let start = Vec2d(start_x as i32, start_y as i32);
    (width, height, start, map)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (width, height, start, mut map) = parse_map(input);
    let size = Vec2d(width as i32, height as i32);

    let pipe = traverse_pipe(&mut map, start, width, height, &|_, _, _, _| {});
    map.iter_mut()
        .enumerate()
        .for_each(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .for_each(|(x, c)| {
                    let pos = Vec2d(x as i32, y as i32);
                    if !pipe.contains(&pos) {
                        *c = '.'
                    }
                })
        });

    let empty_corner = vec![Vec2d(0, 0), Vec2d(0, (height - 1) as i32), Vec2d((width - 1) as i32, 0), Vec2d(width as i32, height as i32)]
        .into_iter().find(|it| map[it.1 as usize][it.0 as usize] == '.')
        .unwrap();

    let corners = HashSet::from(['7', 'L', 'J', 'F']);
    traverse_pipe(&mut map, start, width, height, &|pos, dir, next_dir, map| {
        fill_area(map, *pos + marking_direction(*dir), 'O', &size);
        if corners.contains(&map[pos.1 as usize][pos.0 as usize]) {
            fill_area(map, *pos + marking_direction(*next_dir), 'O', &size);
        }
    });

    let target = if map[empty_corner.1 as usize][empty_corner.0 as usize] == 'O' { '.' } else { 'O' };
    Some(map.iter().flatten().filter(|c| **c == target).count() as u32)
}

fn marking_direction(dir: Vec2d) -> Vec2d {
    match dir {
        NORTH => WEST,
        SOUTH => EAST,
        WEST => SOUTH,
        EAST => NORTH,
        _ => Vec2d(0, 0)
    }
}

fn fill_area(map: &mut Map, pos: Vec2d, c: char, size: &Vec2d) {
    if !pos.is_safe(size) {
        return;
    }

    let mut queue = VecDeque::<Vec2d>::new();
    queue.push_back(pos);
    while !queue.is_empty() {
        let next = queue.pop_front().unwrap();
        if !next.is_safe(size) {
            continue;
        }
        if map[next.1 as usize][next.0 as usize] == '.' {
            map[next.1 as usize][next.0 as usize] = c;
            queue.push_back(next + NORTH);
            queue.push_back(next + SOUTH);
            queue.push_back(next + EAST);
            queue.push_back(next + WEST);
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
