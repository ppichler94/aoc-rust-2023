use advent_of_code::util::grid2d::Grid2d;
use advent_of_code::util::position::{Position, EAST, NORTH, SOUTH, WEST};
use pathfinding::prelude::dijkstra;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid2d::of_lines(input);
    let (width, height) = grid.size();
    let goal = Position::from((width - 1, height - 1));
    let start1 = State { position: Position::from((0, 0)), direction: EAST, steps: 1 };
    let start2 = State { position: Position::from((0, 0)), direction: SOUTH, steps: 1 };
    let result = dijkstra(&start1, |state| state.successor(&grid, |next_dir, dir, steps| next_dir == dir && steps >= 3), |state| state.position == goal);
    let loss1 = result.map(|(_, it)| it);
    let result = dijkstra(&start2, |state| state.successor(&grid, |next_dir, dir, steps| next_dir == dir && steps >= 3), |state| state.position == goal);
    let loss2 = result.map(|(_, it)| it);

    let result = loss1.min(loss2).unwrap();
    Some(result)
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct State {
    position: Position,
    direction: Position,
    steps: u32,
}

impl State {
    fn successor<F>(&self, grid: &Grid2d<char>, filter: F) -> Vec<(State, u32)>
    where
        F: Fn(Position, Position, u32) -> bool,
    {
        let mut successors = Vec::new();
        let directions = directions(&self.direction);
        for direction in directions {
            if filter(direction, self.direction, self.steps) {
                continue;
            }
            let next_position = self.position + direction;
            if next_position.is_within(grid) {
                let next_state = self.next(direction);
                let next_cost = grid.get(&next_state.position).to_digit(10).unwrap();
                successors.push((next_state, next_cost));
            }
        }
        successors
    }

    fn next(&self, direction: Position) -> State {
        State {
            position: self.position + direction,
            direction,
            steps: if self.direction == direction { self.steps + 1 } else { 1 },
        }
    }
}

fn directions(direction: &Position) -> Vec<Position> {
    match *direction {
        NORTH => vec![NORTH, EAST, WEST],
        EAST => vec![EAST, NORTH, SOUTH],
        SOUTH => vec![SOUTH, EAST, WEST],
        WEST => vec![WEST, NORTH, SOUTH],
        _ => panic!("Illegal input direction")
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid2d::of_lines(input);
    let (width, height) = grid.size();
    let goal = Position::from((width - 1, height - 1));
    let start1 = State { position: Position::from((0, 0)), direction: EAST, steps: 1 };
    let start2 = State { position: Position::from((0, 0)), direction: SOUTH, steps: 1 };
    let result = dijkstra(&start1,
                          |state| state.successor(&grid, |next_dir, dir, steps| (next_dir != dir && steps <= 3) || (next_dir == dir && steps > 9)),
                          |state| state.position == goal && state.steps > 3);
    let loss1 = result.map(|(_, it)| it);
    let result = dijkstra(&start2,
                          |state| state.successor(&grid, |next_dir, dir, steps| (next_dir != dir && steps <= 3) || (next_dir == dir && steps > 9)),
                          |state| state.position == goal && state.steps > 3);
    let loss2 = result.map(|(_, it)| it);

    let result = loss1.min(loss2).unwrap();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two2() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(71));
    }
}
