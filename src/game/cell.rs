//! Cell state representation for Othello game board.
//!
//! This module defines the `Cell` enum that represents the three possible
//! states of a cell on the Othello board: Empty, Black, or White. It provides
//! essential functionality for game logic including opponent determination
//! and display formatting.

use std::fmt;

/// Represents the state of a cell on the Othello game board.
///
/// Each cell on the 8x8 Othello board can be in one of three states:
/// - `Empty`: No disc is placed on this cell
/// - `Black`: A black disc is placed on this cell
/// - `White`: A white disc is placed on this cell
///
/// The enum is designed to be efficiently stored and compared, with
/// integer values assigned for potential serialization or storage needs.
///
/// # Examples
///
/// ```rust
/// let empty_cell = Cell::Empty;
/// let black_disc = Cell::Black;
/// let white_disc = Cell::White;
///
/// // Get the opponent of a player
/// assert_eq!(black_disc.get_opponent(), Cell::White);
/// assert_eq!(white_disc.get_opponent(), Cell::Black);
///
/// // Display cell state
/// println!("Current cell: {}", black_disc); // Prints "BLACK"
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    /// Empty cell with no disc placed
    Empty = 0,
    /// Cell containing a black disc
    Black = 1,
    /// Cell containing a white disc
    White = 2,
}

impl Cell {
    /// Returns the opponent color for the current cell.
    ///
    /// This method is essential for Othello game logic, as many operations
    /// need to determine the opponent's color when evaluating moves or
    /// flipping discs.
    ///
    /// # Returns
    ///
    /// - `Cell::White` if the current cell is `Cell::Black`
    /// - `Cell::Black` if the current cell is `Cell::White`
    /// - `Cell::Empty` if the current cell is `Cell::Empty` (no opponent for empty cells)
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(Cell::Black.get_opponent(), Cell::White);
    /// assert_eq!(Cell::White.get_opponent(), Cell::Black);
    /// assert_eq!(Cell::Empty.get_opponent(), Cell::Empty);
    /// ```
    pub fn get_opponent(self) -> Cell {
        match self {
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
            Cell::Empty => Cell::Empty,
        }
    }
}

impl fmt::Display for Cell {
    /// Formats the cell for display purposes.
    ///
    /// This implementation provides human-readable text representation
    /// of each cell state, suitable for console output, debugging,
    /// and user interfaces.
    ///
    /// # Format
    ///
    /// - `Cell::Black` displays as "BLACK"
    /// - `Cell::White` displays as "WHITE"
    /// - `Cell::Empty` displays as "*"
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(format!("{}", Cell::Black), "BLACK");
    /// assert_eq!(format!("{}", Cell::White), "WHITE");
    /// assert_eq!(format!("{}", Cell::Empty), "*");
    /// ```
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
