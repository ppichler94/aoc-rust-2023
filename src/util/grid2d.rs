use crate::util::position::Position;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Grid2d<T> {
    content: Vec<Vec<T>>,
}

impl<T: Copy> Grid2d<T> {
    pub fn size(&self) -> (usize, usize) {
        (self.content.len(), self.content[0].len())
    }

    pub fn get(&self, position: Position) -> T {
        self.content[position.y as usize][position.x as usize]
    }

    pub fn of_lines(text: &str) -> Grid2d<char> {
        Grid2d { content: text.lines().map(|line| line.chars().collect()).collect() }
    }
}
