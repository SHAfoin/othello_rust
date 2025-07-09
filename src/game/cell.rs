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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_opponent() {
        assert_eq!(Cell::Black.get_opponent(), Cell::White);
        assert_eq!(Cell::White.get_opponent(), Cell::Black);
        assert_eq!(Cell::Empty.get_opponent(), Cell::Empty);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Cell::Black), "BLACK");
        assert_eq!(format!("{}", Cell::White), "WHITE");
        assert_eq!(format!("{}", Cell::Empty), "*");
    }
}
