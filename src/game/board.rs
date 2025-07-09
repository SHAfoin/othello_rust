use std::fmt;

use crate::game::{cell::Cell, consts::SIZE};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    cells: [[Cell; SIZE]; SIZE],
    nb_discs: [usize; 2],
    nb_legal_moves: [Option<usize>; 2],
    turn_number: usize,
}

impl Board {
    pub fn new() -> Self {
        let mut cells = [[Cell::Empty; SIZE]; SIZE];
        let center1 = SIZE / 2 - 1;
        let center2 = SIZE / 2;

        cells[center1][center1] = Cell::White;
        cells[center1][center2] = Cell::Black;
        cells[center2][center1] = Cell::Black;
        cells[center2][center2] = Cell::White;

        Board {
            cells,
            nb_discs: [2, 2],
            nb_legal_moves: [Some(4), Some(4)], // Initial legal moves for both players
            turn_number: 1,
        }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Result<Cell, String> {
        if row < SIZE && col < SIZE {
            Ok(self.cells[row][col])
        } else {
            Err("Index out of bounds".to_string())
        }
    }

    pub fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
        self.cells[row][col] = cell;
    }

    pub fn get_turn_number(&self) -> usize {
        self.turn_number
    }

    pub fn get_nb_legal_moves(&self, color: Cell) -> Result<Option<usize>, String> {
        let index = match color {
            Cell::Black => 0,
            Cell::White => 1,
            _ => return Err("Invalid color".to_string()),
        };

        Ok(self.nb_legal_moves[index])
    }

    pub fn set_nb_legal_moves(
        &mut self,
        color: Cell,
        nb_moves: Option<usize>,
    ) -> Result<(), String> {
        let index = match color {
            Cell::Black => 0,
            Cell::White => 1,
            _ => return Err("Invalid color".to_string()),
        };
        self.nb_legal_moves[index] = nb_moves;
        Ok(())
    }

    pub fn get_nb_discs(&self, color: Cell) -> Result<usize, String> {
        match color {
            Cell::Black => Ok(self.nb_discs[0]),
            Cell::White => Ok(self.nb_discs[1]),
            _ => Err("Invalid color".to_string()),
        }
    }

    pub fn get_cells(&self) -> &[[Cell; SIZE]; SIZE] {
        &self.cells
    }

    pub fn try_play_move(
        &mut self,
        row: usize,
        col: usize,
        color: Cell,
    ) -> Result<(usize), String> {
        match self.can_play(row, col, color) {
            Ok(directions) => {
                self.set_cell(row, col, color);
                let mut flipped_count = 0;
                for direction in directions {
                    flipped_count += self.flip_discs(row, col, color, direction);
                }

                // Update disc counts
                let index = match color {
                    Cell::Black => 0,
                    Cell::White => 1,
                    _ => return Err("Invalid color".to_string()),
                };
                let opponent_index = 1 - index;

                self.nb_discs[index] += 1 + flipped_count; // New disc + flipped discs
                self.nb_discs[opponent_index] -= flipped_count; // Remove flipped discs from opponent

                let my_moves = match self.has_legal_moves(color) {
                    Some(moves) => Some(moves.len()),
                    None => None,
                };
                self.set_nb_legal_moves(color, my_moves).unwrap();
                let opponent_moves = match self.has_legal_moves(color.get_opponent()) {
                    Some(moves) => Some(moves.len()),
                    None => None,
                };
                self.set_nb_legal_moves(color.get_opponent(), opponent_moves)
                    .unwrap();

                Ok(flipped_count + 1)
            }
            Err(e) => return Err(e),
        }
    }

    pub fn can_play(
        &self,
        row: usize,
        col: usize,
        color: Cell,
    ) -> Result<Vec<(isize, isize)>, String> {
        if row >= SIZE || col >= SIZE {
            Err("Index out of bounds".to_string())
        } else if self.cells[row][col] != Cell::Empty {
            Err("Cell is not empty".to_string())
        } else if color != Cell::Black && color != Cell::White {
            Err("Invalid color".to_string())
        } else {
            let directions = self.get_valid_directions(row, col, color);
            if directions.is_empty() {
                Err("Invalid move".to_string())
            } else {
                Ok(directions)
            }
        }
    }

    pub fn get_valid_directions(&self, row: usize, col: usize, color: Cell) -> Vec<(isize, isize)> {
        let mut valid_directions = Vec::new();

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue; // Skip the current cell
                }

                if self.is_move_valid_recursive(row as isize, col as isize, color, 1, (i, j)) {
                    valid_directions.push((i, j));
                }
            }
        }

        valid_directions
    }

    pub fn is_move_valid_recursive(
        &self,
        row: isize,
        col: isize,
        color: Cell,
        index: isize,
        direction: (isize, isize),
    ) -> bool {
        let next_row = row + direction.0 * index;
        let next_col = col + direction.1 * index;
        if next_row < 0 || next_row >= SIZE as isize || next_col < 0 || next_col >= SIZE as isize {
            return false; // Out of bounds
        } else if index == 1 && self.get_cell(next_row as usize, next_col as usize) == Ok(color) {
            return false; // Found the same color right next to the move in the direction
        } else if self.get_cell(next_row as usize, next_col as usize) == Ok(color) {
            return true; // Found a cell of the same color after Ok other color
        } else if self.get_cell(next_row as usize, next_col as usize) == Ok(Cell::Empty) {
            return false; // Found an empty cell
        } else {
            return self.is_move_valid_recursive(row, col, color, index + 1, direction);
        }
    }

    fn flip_discs(
        &mut self,
        row: usize,
        col: usize,
        color: Cell,
        direction: (isize, isize),
    ) -> usize {
        let mut next_row = row as isize + direction.0;
        let mut next_col = col as isize + direction.1;
        let mut flipped_count = 0;

        while self.get_cell(next_row as usize, next_col as usize) != Ok(color) {
            self.set_cell(next_row as usize, next_col as usize, color);
            flipped_count += 1;
            next_row += direction.0;
            next_col += direction.1;
        }

        flipped_count
    }

    pub fn has_legal_moves(&self, color: Cell) -> Option<Vec<(usize, usize)>> {
        let mut legal_moves: Vec<(usize, usize)> = Vec::new();
        for row in 0..SIZE {
            for col in 0..SIZE {
                if self.can_play(row, col, color).is_ok() {
                    legal_moves.push((row, col));
                }
            }
        }
        if legal_moves.len() > 0 {
            Some(legal_moves)
        } else {
            None
        }
    }

    pub fn input_to_coordinates(input: &str) -> Option<(usize, usize)> {
        if input.len() != 2 {
            return None;
        }
        let row = input.chars().nth(0)?.to_digit(10)? as usize;
        let col = input.chars().nth(1)?.to_ascii_uppercase() as usize - 'A' as usize;
        if row < SIZE && col < SIZE {
            Some((row, col))
        } else {
            None
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.get_nb_legal_moves(Cell::Black).unwrap().is_none()
            && self.get_nb_legal_moves(Cell::White).unwrap().is_none()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display = String::new();
        display.push_str("   A B C D E F G H\n\n");
        for (index, row) in self.cells.iter().enumerate() {
            display.push_str(&format!("{}  ", index));
            for cell in row.iter() {
                let symbol = match cell {
                    Cell::Empty => "* ",
                    Cell::Black => "B ",
                    Cell::White => "W ",
                };
                display.push_str(symbol);
            }
            display.push_str("\n");
        }
        write!(f, "{}", display)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_cell_out_of_bounds() {
        let board = Board::new();
        assert!(board.get_cell(SIZE, 0).is_err());
        assert!(board.get_cell(0, SIZE).is_err());
        assert!(board.get_cell(SIZE, SIZE).is_err());
    }

    #[test]
    fn get_cell() {
        let board = Board::new();
        assert_eq!(board.get_cell(0, 0), Ok(Cell::Empty));
        assert_eq!(board.get_cell(3, 3), Ok(Cell::White));
        assert_eq!(board.get_cell(3, 4), Ok(Cell::Black));
        assert_eq!(board.get_cell(4, 3), Ok(Cell::Black));
        assert_eq!(board.get_cell(4, 4), Ok(Cell::White));
        assert_eq!(board.get_cell(7, 7), Ok(Cell::Empty));
    }

    #[test]
    fn try_play_move_turn_valid_move() {
        let mut board = Board::new();
        // Black should be able to play at (2, 3)
        assert!(board.try_play_move(2, 3, Cell::Black).is_ok());
        // Verify the cell was set to Black
        assert_eq!(board.get_cell(2, 3), Ok(Cell::Black));
        // Verify the White disc at (3, 3) was flipped to Black
        assert_eq!(board.get_cell(3, 3), Ok(Cell::Black));
    }

    #[test]
    fn try_play_move_turn_invalid_move() {
        let mut board = Board::new();

        assert!(board.try_play_move(0, 0, Cell::Black).is_err());
        assert!(board.try_play_move(3, 3, Cell::Black).is_err());
    }

    #[test]
    fn test_get_nb_discs() {
        let board = Board::new();
        assert_eq!(board.get_nb_discs(Cell::Black), Ok(2));
        assert_eq!(board.get_nb_discs(Cell::White), Ok(2));
        assert!(board.get_nb_discs(Cell::Empty).is_err());
    }

    #[test]
    fn test_get_nb_discs_after_move() {
        let mut board = Board::new();
        board.try_play_move(2, 3, Cell::Black).unwrap();
        assert_eq!(board.get_nb_discs(Cell::Black), Ok(4)); // 2 initial + 1 new + 1 flipped
        assert_eq!(board.get_nb_discs(Cell::White), Ok(1)); // 2 initial - 1 flipped
    }

    #[test]
    fn test_has_legal_moves() {
        let board = Board::new();
        assert!(board.has_legal_moves(Cell::Black).is_some());
        assert!(board.has_legal_moves(Cell::White).is_some());

        let moves_black = board.has_legal_moves(Cell::Black).unwrap();
        let moves_white = board.has_legal_moves(Cell::White).unwrap();
        assert_eq!(moves_black.len(), 4); // 4 coups légaux initiaux pour noir
        assert_eq!(moves_white.len(), 4); // 4 coups légaux initiaux pour blanc
    }

    #[test]
    fn test_input_to_coordinates() {
        assert_eq!(Board::input_to_coordinates("3D"), Some((3, 3)));
        assert_eq!(Board::input_to_coordinates("0A"), Some((0, 0)));
        assert_eq!(Board::input_to_coordinates("7H"), Some((7, 7)));
        assert_eq!(Board::input_to_coordinates("2d"), Some((2, 3))); // lowercase should work

        // Invalid inputs
        assert_eq!(Board::input_to_coordinates(""), None);
        assert_eq!(Board::input_to_coordinates("3"), None);
        assert_eq!(Board::input_to_coordinates("3DD"), None);
        assert_eq!(Board::input_to_coordinates("9A"), None); // Out of bounds row
        assert_eq!(Board::input_to_coordinates("0I"), None); // Out of bounds column
        assert_eq!(Board::input_to_coordinates("XY"), None); // Invalid format
    }

    #[test]
    fn test_can_play() {
        let board = Board::new();

        // Valid moves for Black at start
        assert!(board.can_play(2, 3, Cell::Black).is_ok());
        assert!(board.can_play(3, 2, Cell::Black).is_ok());
        assert!(board.can_play(4, 5, Cell::Black).is_ok());
        assert!(board.can_play(5, 4, Cell::Black).is_ok());

        // Invalid moves
        assert!(board.can_play(0, 0, Cell::Black).is_err());
        assert!(board.can_play(3, 3, Cell::Black).is_err()); // Occupied cell
        assert!(board.can_play(8, 8, Cell::Black).is_err()); // Out of bounds
        assert!(board.can_play(2, 3, Cell::Empty).is_err()); // Invalid color
    }

    #[test]
    fn test_get_valid_directions() {
        let board = Board::new();
        let directions = board.get_valid_directions(2, 3, Cell::Black);
        assert_eq!(directions.len(), 1);
        assert_eq!(directions[0], (1, 0)); // Should be able to go down

        let directions = board.get_valid_directions(3, 2, Cell::Black);
        assert_eq!(directions.len(), 1);
        assert_eq!(directions[0], (0, 1)); // Should be able to go right
    }

    #[test]
    fn test_flip_discs() {
        let mut board = Board::new();

        // Play a move and verify flipping
        board.try_play_move(2, 3, Cell::Black).unwrap();
        assert_eq!(board.get_cell(2, 3), Ok(Cell::Black)); // New piece
        assert_eq!(board.get_cell(3, 3), Ok(Cell::Black)); // Flipped piece
        assert_eq!(board.get_cell(4, 3), Ok(Cell::Black)); // Original piece
    }

    #[test]
    fn test_is_move_valid_recursive() {
        let board = Board::new();

        // Test a valid move
        assert!(board.is_move_valid_recursive(2, 3, Cell::Black, 1, (1, 0)));

        // Test an invalid move (out of bounds)
        assert!(!board.is_move_valid_recursive(0, 0, Cell::Black, 1, (-1, 0)));

        // Test invalid move (same color adjacent)
        assert!(!board.is_move_valid_recursive(3, 3, Cell::White, 1, (1, 0)));
    }
    #[test]
    fn test_multiple_moves_game() {
        let mut board = Board::new();

        // Play a sequence of moves
        assert!(board.try_play_move(2, 3, Cell::Black).is_ok());
        assert!(board.try_play_move(2, 2, Cell::White).is_ok());
        assert!(board.try_play_move(3, 2, Cell::Black).is_ok());

        // Verify the board state
        assert_eq!(board.get_cell(2, 3), Ok(Cell::Black));
        assert_eq!(board.get_cell(2, 2), Ok(Cell::White));
        assert_eq!(board.get_cell(3, 2), Ok(Cell::Black));
    }

    #[test]
    fn test_no_legal_moves_scenario() {
        let mut board = Board::new();

        // Create a scenario where a player has no legal moves
        // This is a simplified test - in a real game this would be more complex
        for row in 0..SIZE {
            for col in 0..SIZE {
                if board.get_cell(row, col) == Ok(Cell::Empty) {
                    board.set_cell(row, col, Cell::Black);
                }
            }
        }

        // White should have no legal moves
        assert_eq!(board.has_legal_moves(Cell::White), None);
    }
}
