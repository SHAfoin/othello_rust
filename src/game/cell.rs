use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty = 0,
    Black = 1,
    White = 2,
}

impl Cell {
    pub fn get_opponent(self) -> Cell {
        match self {
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
            Cell::Empty => Cell::Empty,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Cell::Black => "BLACK",
            Cell::White => "WHITE",
            _ => "*",
        };
        write!(f, "{}", symbol)
    }
}
