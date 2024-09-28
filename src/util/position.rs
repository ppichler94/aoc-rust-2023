use std::ops::{Add, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self { x: value.0 as i64, y: value.1 as i64 }
    }
}

impl Position {
    pub fn at(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn moves(diagonals: bool, zero_move: bool) -> Vec<Position> {
        let mut moves = vec![
            Position { x: 0, y: -1 },
            Position { x: 0, y: 1 },
            Position { x: -1, y: 0 },
            Position { x: 1, y: 0 },
        ];
        if diagonals {
            moves.extend_from_slice(&[
                Position { x: -1, y: -1 },
                Position { x: -1, y: 1 },
                Position { x: 1, y: -1 },
                Position { x: 1, y: 1 },
            ]);
        }
        if zero_move {
            moves.push(Position { x: 0, y: 0 });
        }
        moves
    }

    pub fn neighbors_within(&self, moves: Vec<Position>, max_x: usize, max_y: usize) -> Vec<Position> {
        let mut result = Vec::new();
        for mv in moves {
            let new_pos = self + &mv;
            if new_pos.x >= 0 && new_pos.x < max_x as i64 && new_pos.y >= 0 && new_pos.y < max_y as i64 {
                result.push(new_pos);
            }
        }
        result
    }

    pub fn neighbors(&self, moves: Vec<Position>) -> Vec<Position> {
        let mut result = Vec::new();
        for mv in moves {
            let new_pos = self + &mv;
            if new_pos.x >= 0 && new_pos.y >= 0 {
                result.push(new_pos);
            }
        }
        result
    }

    pub fn distance_manhatten(&self, other: &Position) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn is_safe(&self, width: i64, height: i64) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < width && self.y < height
    }
}

impl Add for &Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for &Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
