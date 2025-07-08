use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty = 0,
    Black = 1,
    White = 2,
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
