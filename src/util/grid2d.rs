use crate::util::position::Position;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Grid2d<T> {
    content: Vec<Vec<T>>,
}

impl Grid2d<char> {
    /// Creates a grid from a string by splitting the string into lines and
    /// then splitting the lines into characters.
    /// Each line must be of equal length.
    ///
    pub fn of_lines(text: &str) -> Grid2d<char> {
        Grid2d { content: text.lines().map(|line| line.chars().collect()).collect() }
    }
}

impl<T> Grid2d<T> {
    /// Returns the dimensions of the grid as `(width, height)`.
    ///
    pub fn size(&self) -> (usize, usize) {
        (self.content.len(), self.content[0].len())
    }
}

impl<T: Copy + PartialEq> Grid2d<T> {
    pub fn get(&self, position: &Position) -> T {
        self.content[position.y as usize][position.x as usize]
    }

    pub fn set(&mut self, position: &Position, data: T) {
        self.content[position.y as usize][position.x as usize] = data
    }

    pub fn swap(&mut self, a: &Position, b: &Position) {
        let temp = self.content[a.y as usize][a.x as usize];
        self.content[a.y as usize][a.x as usize] = self.content[b.y as usize][b.x as usize];
        self.content[b.y as usize][b.x as usize] = temp;
    }


    /// Returns the positions for all occurrences of the `target`.
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

    pub fn for_each<F>(&self, f: F)
    where
        F: Fn(usize, usize, T),
    {
        for (i, row) in self.content.iter().enumerate() {
            for (j, &item) in row.iter().enumerate() {
                f(j, i, item);
            }
        }
    }
}
