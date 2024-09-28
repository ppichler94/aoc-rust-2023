use crate::util::position::Position;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Grid2d<T> {
    content: Vec<Vec<T>>,
}

impl<T: Copy + PartialEq> Grid2d<T> {
    pub fn size(&self) -> (usize, usize) {
        (self.content.len(), self.content[0].len())
    }

    pub fn get(&self, position: &Position) -> T {
        self.content[position.y as usize][position.x as usize]
    }

    pub fn set(&mut self, position: &Position, data: T) {
        self.content[position.y as usize][position.x as usize] = data
    }

    pub fn of_lines(text: &str) -> Grid2d<char> {
        Grid2d { content: text.lines().map(|line| line.chars().collect()).collect() }
    }

    pub fn find_all(&self, target: T) -> Vec<Position> {
        let mut result = Vec::new();
        self.content.iter()
            .enumerate()
            .for_each(|(y, row)| {
                row.iter()
                    .enumerate()
                    .for_each(|(x, &data)| {
                        if data == target {
                            result.push(Position::from((x, y)));
                        }
                    })
            });
        result
    }
}
