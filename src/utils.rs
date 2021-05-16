use std::fmt;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {

    pub fn new(row: usize, col: usize) -> Position {
        Position {row, col}
    }

    pub fn is_valid(&self, size: usize) -> bool {
        (0 <= self.row && self.row < size)
            && (0 <= self.col && self.col < size)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col);
        Ok(())
    }
}