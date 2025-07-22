//! Othello game board implementation and game logic.
//!
//! This module provides the core game board functionality for Othello,
//! including move validation, disc flipping, game state management,
//! and win condition checking. The board manages the 8x8 grid of cells
//! and enforces all Othello game rules.

use std::fmt;

use crate::consts::SIZE;
use crate::game::cell::Cell;
use crate::game::history_action::HistoryAction;

/// Represents the Othello game board and manages game state.
///
/// The `Board` struct encapsulates all aspects of an Othello game including
/// the 8x8 grid of cells, game state tracking, move history, and game logic.
/// It provides methods for move validation, execution, and game state queries.
///
/// # Game Rules Enforced
///
/// - Black always starts first
/// - Players must place discs to flip at least one opponent disc
/// - Players must pass if no legal moves are available
/// - Game ends when neither player can move
/// - Winner is determined by disc count
///
/// # Examples
///
/// ```rust
/// // Create a new game board
/// let mut board = Board::new();
///
/// // Check initial state
/// assert_eq!(board.get_player_turn(), Cell::Black);
/// assert_eq!(board.get_turn_number(), 1);
///
/// // Make a move
/// let result = board.try_play_move(2, 3, Cell::Black);
/// assert!(result.is_ok());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    /// 8x8 grid representing the game board cells
    cells: [[Cell; SIZE]; SIZE],
    /// Number of discs for each player [Black, White]
    nb_discs: [usize; 2],
    /// Number of legal moves available for each player [Black, White]
    nb_legal_moves: [Option<usize>; 2],
    /// Current turn number (starts at 1)
    turn_number: usize,
    /// Current player's turn
    player_turn: Cell,
    /// History of all moves played in the game
    history: Vec<HistoryAction>,
    /// Whether the game has ended
    game_over: bool,
}

impl Board {
    /// Creates a new Othello game board with the standard starting position.
    ///
    /// This constructor initializes an 8x8 board with the classic Othello
    /// starting configuration: four discs in the center (two black, two white)
    /// arranged diagonally, with Black to move first.
    ///
    /// # Initial Setup
    ///
    /// ```text
    ///    A B C D E F G H
    /// 0  . . . . . . . .
    /// 1  . . . . . . . .
    /// 2  . . . . . . . .
    /// 3  . . . W B . . .
    /// 4  . . . B W . . .
    /// 5  . . . . . . . .
    /// 6  . . . . . . . .
    /// 7  . . . . . . . .
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// let board = Board::new();
    /// assert_eq!(board.get_player_turn(), Cell::Black);
    /// assert_eq!(board.get_nb_discs(Cell::Black).unwrap(), 2);
    /// assert_eq!(board.get_nb_discs(Cell::White).unwrap(), 2);
    /// assert_eq!(board.get_turn_number(), 1);
    /// ```
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
            player_turn: Cell::Black, // Black starts first
            history: Vec::new(),
            game_over: false,
        }
    }

    /// Adds a move to the game history.
    ///
    /// This method records a completed move in the game's history for
    /// potential replay, analysis, or undo functionality.
    ///
    /// # Arguments
    ///
    /// * `action` - The `HistoryAction` representing the move to record
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut board = Board::new();
    /// let action = HistoryAction {
    ///     coordinates: Some("2D".to_string()),
    ///     gained_discs: Some(1),
    ///     color: Cell::Black,
    ///     move_number: 1,
    ///     player_turn: Cell::Black,
    /// };
    /// board.add_to_history(action);
    /// assert_eq!(board.get_history().len(), 1);
    /// ```
    pub fn add_to_history(&mut self, action: HistoryAction) {
        self.history.push(action);
    }

    /// Returns the state of a cell at the specified coordinates.
    ///
    /// This method provides safe access to board cells with bounds checking.
    /// It's used throughout the game logic to query cell states for move
    /// validation and game state evaluation.
    ///
    /// # Arguments
    ///
    /// * `row` - The row index (0-7)
    /// * `col` - The column index (0-7)
    ///
    /// # Returns
    ///
    /// * `Ok(Cell)` - The cell state if coordinates are valid
    /// * `Err(String)` - Error message if coordinates are out of bounds
    ///
    /// # Examples
    ///
    /// ```rust
    /// let board = Board::new();
    /// assert_eq!(board.get_cell(3, 3), Ok(Cell::White));
    /// assert_eq!(board.get_cell(3, 4), Ok(Cell::Black));
    /// assert_eq!(board.get_cell(0, 0), Ok(Cell::Empty));
    /// assert!(board.get_cell(8, 8).is_err());
    /// ```
    pub fn get_cell(&self, row: usize, col: usize) -> Result<Cell, String> {
        if row < SIZE && col < SIZE {
            Ok(self.cells[row][col])
        } else {
            Err("Index out of bounds".to_string())
        }
    }

    /// Returns the current player whose turn it is to move.
    ///
    /// This method indicates which player should make the next move.
    /// The game alternates between Black and White players, with
    /// Black always starting first.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let board = Board::new();
    /// assert_eq!(board.get_player_turn(), Cell::Black); // Black starts
    /// ```
    pub fn get_player_turn(&self) -> Cell {
        self.player_turn
    }

    /// Advances the game to the next turn, handling pass situations.
    ///
    /// This method manages turn progression and automatically handles
    /// situations where a player has no legal moves (must pass). If the
    /// next player has no legal moves, their turn is automatically passed
    /// and recorded in the history.
    ///
    /// # Turn Logic
    ///
    /// 1. Increment turn number
    /// 2. Check if opponent has legal moves
    /// 3. If no legal moves: record pass and increment turn again
    /// 4. If legal moves exist: switch to opponent
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut board = Board::new();
    /// assert_eq!(board.get_player_turn(), Cell::Black);
    /// board.next_turn();
    /// assert_eq!(board.get_player_turn(), Cell::White);
    /// ```
    pub fn next_turn(&mut self) {
        self.turn_number += 1;
        if None
            == self
                .get_nb_legal_moves(self.player_turn.get_opponent())
                .unwrap()
        {
            self.add_to_history(HistoryAction {
                coordinates: None,
                gained_discs: None,
                color: self.player_turn.get_opponent(),
                move_number: self.turn_number,
                player_turn: self.player_turn,
            });
            self.turn_number += 1;
        } else {
            self.player_turn = self.player_turn.get_opponent();
        }
    }

    /// Sets the state of a cell at the specified coordinates.
    ///
    /// This is a low-level method used internally for placing discs
    /// and flipping opponent discs during move execution.
    ///
    /// # Arguments
    ///
    /// * `row` - The row index (0-7)
    /// * `col` - The column index (0-7)
    /// * `cell` - The new cell state to set
    ///
    /// # Safety
    ///
    /// This method does not perform bounds checking. Callers must ensure
    /// coordinates are valid (0-7 for both row and col).
    pub fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
        self.cells[row][col] = cell;
    }

    /// Returns the current turn number.
    ///
    /// The turn number starts at 1 and increments with each move.
    /// Pass moves also increment the turn number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let board = Board::new();
    /// assert_eq!(board.get_turn_number(), 1);
    /// ```
    pub fn get_turn_number(&self) -> usize {
        self.turn_number
    }

    /// Returns the number of legal moves available for the specified player.
    ///
    /// This method retrieves the cached count of legal moves for a player.
    /// The count is updated after each move and is used for game logic
    /// and AI evaluation.
    ///
    /// # Arguments
    ///
    /// * `color` - The player color (Black or White)
    ///
    /// # Returns
    ///
    /// * `Ok(Some(count))` - Number of legal moves available
    /// * `Ok(None)` - No legal moves available (player must pass)
    /// * `Err(String)` - Error if invalid color is provided
    ///
    /// # Examples
    ///
    /// ```rust
    /// let board = Board::new();
    /// assert_eq!(board.get_nb_legal_moves(Cell::Black), Ok(Some(4)));
    /// assert_eq!(board.get_nb_legal_moves(Cell::White), Ok(Some(4)));
    /// ```
    pub fn get_nb_legal_moves(&self, color: Cell) -> Result<Option<usize>, String> {
        let index = match color {
            Cell::Black => 0,
            Cell::White => 1,
            _ => return Err("Invalid color".to_string()),
        };

        Ok(self.nb_legal_moves[index])
    }

    /// Updates the number of legal moves available for the specified player.
    ///
    /// This method is used internally to maintain the cached count of legal
    /// moves after board state changes.
    ///
    /// # Arguments
    ///
    /// * `color` - The player color (Black or White)
    /// * `nb_moves` - New number of legal moves (None if no moves available)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Successfully updated
    /// * `Err(String)` - Error if invalid color is provided
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

    /// Returns the number of discs for the specified player.
    ///
    /// This method provides the current count of discs on the board for
    /// a given player. This is essential for determining the winner and
    /// for heuristic evaluation in AI players.
    ///
    /// # Arguments
    ///
    /// * `color` - The player color (Black or White)
    ///
    /// # Returns
    ///
    /// * `Ok(count)` - Number of discs for the player
    /// * `Err(String)` - Error if invalid color is provided
    ///
    /// # Examples
    ///
    /// ```rust
    /// let board = Board::new();
    /// assert_eq!(board.get_nb_discs(Cell::Black), Ok(2));
    /// assert_eq!(board.get_nb_discs(Cell::White), Ok(2));
    /// ```
    pub fn get_nb_discs(&self, color: Cell) -> Result<usize, String> {
        match color {
            Cell::Black => Ok(self.nb_discs[0]),
            Cell::White => Ok(self.nb_discs[1]),
            _ => Err("Invalid color".to_string()),
        }
    }

    /// Returns a reference to the game move history.
    ///
    /// This method provides access to the complete history of moves
    /// played in the game, useful for replay, analysis, or undo functionality.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let board = Board::new();
    /// assert_eq!(board.get_history().len(), 0); // No moves yet
    /// ```
    pub fn get_history(&self) -> &Vec<HistoryAction> {
        &self.history
    }

    /// Attempts to play a move at the specified coordinates.
    ///
    /// This is the main method for executing moves in the game. It validates
    /// the move, places the disc, flips opponent discs in all valid directions,
    /// and updates the game state.
    ///
    /// # Arguments
    ///
    /// * `row` - The row coordinate (0-7)
    /// * `col` - The column coordinate (0-7)
    /// * `color` - The color of the disc to place
    ///
    /// # Returns
    ///
    /// * `Ok(count)` - Number of opponent discs that were flipped
    /// * `Err(String)` - Error message if the move is invalid
    ///
    /// # Errors
    ///
    /// - Coordinates out of bounds (0-7)
    /// - Target cell is not empty
    /// - Invalid player color
    /// - Move doesn't flip any opponent discs
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut board = Board::new();
    /// let result = board.try_play_move(2, 3, Cell::Black);
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap(), 1); // One disc was flipped
    /// ```
    pub fn try_play_move(&mut self, row: usize, col: usize, color: Cell) -> Result<usize, String> {
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

    /// Checks if a move is valid and returns capture directions.
    ///
    /// This method validates if a disc can be placed at the specified position
    /// and returns all valid capture directions if the move is legal. A move
    /// is valid if it captures at least one opponent disc in any direction.
    ///
    /// # Arguments
    ///
    /// * `row` - Row position (0-7)
    /// * `col` - Column position (0-7)
    /// * `color` - Color of the disc to place
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<(isize, isize)>)` - Valid capture directions
    /// * `Err(String)` - Error message if move is invalid
    ///
    /// # Validation Rules
    ///
    /// - Position must be within board bounds
    /// - Target cell must be empty
    /// - Color must be Black or White
    /// - Must capture at least one opponent disc
    ///
    /// # Examples
    ///
    /// ```rust
    /// let board = Board::new();
    /// // Valid opening moves
    /// assert!(board.can_play(2, 3, Cell::Black).is_ok());
    /// assert!(board.can_play(3, 2, Cell::Black).is_ok());
    /// // Invalid move (occupied cell)
    /// assert!(board.can_play(3, 3, Cell::Black).is_err());
    /// ```
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

    /// Gets all directions where placing a disc would capture opponent discs.
    ///
    /// This method scans all eight directions (horizontal, vertical, diagonal)
    /// from a given position to find valid capture lines. A direction is
    /// valid if it forms a line of opponent discs ending with the player's color.
    ///
    /// # Arguments
    ///
    /// * `row` - Row position to check
    /// * `col` - Column position to check
    /// * `color` - Player color to check for
    ///
    /// # Returns
    ///
    /// Vector of direction tuples (row_delta, col_delta) where captures are possible
    ///
    /// # Direction Encoding
    ///
    /// Directions are encoded as (-1, 0, 1) tuples:
    /// - (-1, -1): Up-left diagonal
    /// - (-1, 0): Up
    /// - (-1, 1): Up-right diagonal
    /// - (0, -1): Left
    /// - (0, 1): Right
    /// - (1, -1): Down-left diagonal
    /// - (1, 0): Down
    /// - (1, 1): Down-right diagonal
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

    /// Recursively validates if a move captures discs in a given direction.
    ///
    /// This method implements the core Othello capture validation logic by
    /// walking along a direction from the starting position. It ensures there's
    /// at least one opponent disc followed by a player's disc.
    ///
    /// # Arguments
    ///
    /// * `row` - Starting row position
    /// * `col` - Starting column position
    /// * `color` - Player color
    /// * `index` - Distance from starting position
    /// * `direction` - Direction tuple (row_delta, col_delta)
    ///
    /// # Returns
    ///
    /// `true` if the direction forms a valid capture line, `false` otherwise
    ///
    /// # Validation Logic
    ///
    /// 1. First adjacent cell must be opponent color (not same color or empty)
    /// 2. Continue along direction while finding opponent discs
    /// 3. Must end with player's color disc (not empty or board edge)
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

    /// Flips opponent discs along a capture direction.
    ///
    /// This private method handles the actual disc flipping after a valid move
    /// has been confirmed. It walks along the specified direction from the
    /// move position, flipping all opponent discs until reaching the player's disc.
    ///
    /// # Arguments
    ///
    /// * `row` - Starting row position
    /// * `col` - Starting column position  
    /// * `color` - Player color to flip to
    /// * `direction` - Direction tuple for flipping
    ///
    /// # Returns
    ///
    /// Number of discs flipped in this direction
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

    /// Finds all legal moves for a given player.
    ///
    /// This method scans the entire board to identify all positions where
    /// the specified player can make a valid move. This is essential for
    /// determining if a player has any moves available or if the game should end.
    ///
    /// # Arguments
    ///
    /// * `color` - Player color to check moves for
    ///
    /// # Returns
    ///
    /// * `Some(Vec<(usize, usize)>)` - List of valid move coordinates
    /// * `None` - No legal moves available
    ///
    /// # Strategic Importance
    ///
    /// This method is crucial for:
    /// - AI move generation and evaluation
    /// - Determining when to skip a player's turn
    /// - Detecting game end conditions
    /// - Providing move hints to human players
    ///
    /// # Examples
    ///
    /// ```rust
    /// let board = Board::new();
    /// let black_moves = board.has_legal_moves(Cell::Black);
    /// assert!(black_moves.is_some()); // Black has 4 opening moves
    /// assert_eq!(black_moves.unwrap().len(), 4);
    /// ```
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

    /// Converts human-readable input coordinates to array indices.
    ///
    /// This utility method converts string coordinates (like "3D") into
    /// array indices suitable for board access. The format expects a digit
    /// (0-7) followed by a letter (A-H, case insensitive).
    ///
    /// # Arguments
    ///
    /// * `input` - String coordinates in format "RowColumn" (e.g., "3D")
    ///
    /// # Returns
    ///
    /// * `Some((row, col))` - Valid coordinates as (row, column) indices
    /// * `None` - Invalid input format or coordinates out of bounds
    ///
    /// # Format
    ///
    /// - First character: row number (0-7)
    /// - Second character: column letter (A-H, case insensitive)
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(Board::input_to_coordinates("3D"), Some((3, 3)));
    /// assert_eq!(Board::input_to_coordinates("0A"), Some((0, 0)));
    /// assert_eq!(Board::input_to_coordinates("7H"), Some((7, 7)));
    /// assert_eq!(Board::input_to_coordinates("2d"), Some((2, 3))); // Case insensitive
    /// assert_eq!(Board::input_to_coordinates("9A"), None); // Out of bounds
    /// assert_eq!(Board::input_to_coordinates("3"), None); // Invalid format
    /// ```
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

    /// Determines the winner of the game based on disc count.
    ///
    /// This method compares the number of discs for each player to determine
    /// the winner. The player with the most discs on the board wins.
    ///
    /// # Returns
    ///
    /// * `Some(Cell::Black)` - Black player has more discs
    /// * `Some(Cell::White)` - White player has more discs  
    /// * `None` - Equal number of discs (tie game)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let board = Board::new();
    /// assert_eq!(board.get_winner(), None); // Tie at start (2-2)
    /// ```
    pub fn get_winner(&self) -> Option<Cell> {
        let black_count = self.get_nb_discs(Cell::Black).unwrap();
        let white_count = self.get_nb_discs(Cell::White).unwrap();

        if black_count > white_count {
            Some(Cell::Black)
        } else if white_count > black_count {
            Some(Cell::White)
        } else {
            None
        }
    }

    /// Converts array indices to human-readable coordinate string.
    ///
    /// This utility method converts internal array coordinates to the
    /// human-readable format used in Othello notation. This is the inverse
    /// operation of `input_to_coordinates`.
    ///
    /// # Arguments
    ///
    /// * `row` - Row index (0-7)
    /// * `col` - Column index (0-7)
    ///
    /// # Returns
    ///
    /// String coordinates in format "RowColumn" (e.g., "3D"), or empty string if invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(Board::coordinates_to_input(3, 3), "3D");
    /// assert_eq!(Board::coordinates_to_input(0, 0), "0A");
    /// assert_eq!(Board::coordinates_to_input(7, 7), "7H");
    /// assert_eq!(Board::coordinates_to_input(8, 8), ""); // Invalid coordinates
    /// ```
    pub fn coordinates_to_input(row: usize, col: usize) -> String {
        if row < SIZE && col < SIZE {
            format!("{}{}", row, (col as u8 + b'A') as char)
        } else {
            String::new()
        }
    }

    /// Generates a string hash representation of the board state.
    ///
    /// This method creates a compact string representation of the current
    /// board state that can be used for caching, state comparison, or
    /// transposition tables in AI algorithms. The hash includes both the
    /// board configuration and the current player turn.
    ///
    /// # Returns
    ///
    /// String hash where:
    /// - First character: Current player ('B' for Black, 'W' for White)
    /// - Following 64 characters: Board state row by row
    ///   - '0': Empty cell
    ///   - '1': Black disc
    ///   - '2': White disc
    ///
    /// # Hash Format
    ///
    /// The hash is 65 characters long:
    /// - Character 0: Player turn indicator
    /// - Characters 1-64: Board cells (8x8 = 64 cells)
    ///
    /// # Use Cases
    ///
    /// - AI transposition tables for avoiding recalculation
    /// - Game state serialization and storage
    /// - Position analysis and pattern recognition
    /// - Debugging and state verification
    ///
    /// # Examples
    ///
    /// ```rust
    /// let board = Board::new();
    /// let hash = board.to_hash();
    /// assert_eq!(hash.len(), 65); // 1 turn + 64 cells
    /// assert!(hash.starts_with('B')); // Black starts first
    /// ```
    pub fn to_hash(&self) -> String {
        let mut hash = String::new();
        match self.get_player_turn() {
            Cell::Black => hash.push('B'),
            Cell::White => hash.push('W'),
            _ => hash.push('E'), // Empty or invalid
        }
        for row in self.cells.iter() {
            for cell in row.iter() {
                hash.push(match cell {
                    Cell::Empty => '0',
                    Cell::Black => '1',
                    Cell::White => '2',
                });
            }
        }
        hash
    }

    /// Checks if the game has ended.
    ///
    /// This method returns the current game state without performing any
    /// checks or updates. Use `check_game_over()` to update the game state
    /// before calling this method for accurate results.
    ///
    /// # Returns
    ///
    /// `true` if the game is over, `false` if the game is still in progress
    ///
    /// # Game End Conditions
    ///
    /// The game ends when neither player has any legal moves available.
    /// This can happen when:
    /// - The board is completely filled
    /// - All remaining empty spaces cannot capture any discs
    /// - One player has no discs left (rare but possible)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut board = Board::new();
    /// assert!(!board.is_game_over()); // Game just started
    /// board.check_game_over(); // Update game state
    /// assert!(!board.is_game_over()); // Still moves available
    /// ```
    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    /// Updates and checks if the game has ended.
    ///
    /// This method performs the actual game end condition check by verifying
    /// if both players have legal moves available. It updates the internal
    /// game state and returns the result.
    ///
    /// # Returns
    ///
    /// `true` if the game is over (no legal moves for either player), `false` otherwise
    ///
    /// # Game End Logic
    ///
    /// The game ends when both of these conditions are true:
    /// 1. Black player has no legal moves
    /// 2. White player has no legal moves
    ///
    /// # Side Effects
    ///
    /// Updates the internal `game_over` flag based on current board state
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut board = Board::new();
    /// assert!(!board.check_game_over()); // Both players have moves
    /// ```
    pub fn check_game_over(&mut self) -> bool {
        self.game_over = self.get_nb_legal_moves(Cell::Black).unwrap().is_none()
            && self.get_nb_legal_moves(Cell::White).unwrap().is_none();
        self.game_over
    }
}

/// Implementation of Display trait for Board.
///
/// Provides a human-readable text representation of the Othello board
/// suitable for console output and debugging. The display shows column
/// labels (A-H), row numbers (0-7), and disc positions using symbols.
///
/// # Display Format
///
/// ```text
///    A B C D E F G H
///
/// 0  * * * * * * * *
/// 1  * * * * * * * *
/// 2  * * * * * * * *
/// 3  * * * W B * * *
/// 4  * * * B W * * *
/// 5  * * * * * * * *
/// 6  * * * * * * * *
/// 7  * * * * * * * *
/// ```
///
/// # Symbol Legend
///
/// - `*`: Empty cell
/// - `B`: Black disc
/// - `W`: White disc
///
/// # Usage
///
/// This implementation allows using `print!`, `println!`, and `format!` macros
/// with Board instances for easy visualization during development and gameplay.
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

    #[test]
    fn test_new_board_initial_state() {
        let board = Board::new();

        // Test center setup
        assert_eq!(board.get_cell(3, 3), Ok(Cell::White));
        assert_eq!(board.get_cell(3, 4), Ok(Cell::Black));
        assert_eq!(board.get_cell(4, 3), Ok(Cell::Black));
        assert_eq!(board.get_cell(4, 4), Ok(Cell::White));

        // Test corners are empty
        assert_eq!(board.get_cell(0, 0), Ok(Cell::Empty));
        assert_eq!(board.get_cell(0, 7), Ok(Cell::Empty));
        assert_eq!(board.get_cell(7, 0), Ok(Cell::Empty));
        assert_eq!(board.get_cell(7, 7), Ok(Cell::Empty));

        assert_eq!(board.get_turn_number(), 1);
        assert_eq!(board.get_player_turn(), Cell::Black);
    }

    #[test]
    fn test_coordinates_conversion_edge_cases() {
        // Test boundary coordinates
        assert_eq!(Board::input_to_coordinates("0A"), Some((0, 0)));
        assert_eq!(Board::input_to_coordinates("7H"), Some((7, 7)));
        assert_eq!(Board::coordinates_to_input(0, 0), "0A");
        assert_eq!(Board::coordinates_to_input(7, 7), "7H");

        // Test invalid coordinates (out of bounds)
        assert_eq!(Board::input_to_coordinates("8A"), None);
        assert_eq!(Board::input_to_coordinates("0I"), None);
        assert_eq!(Board::coordinates_to_input(8, 0), "");
        assert_eq!(Board::coordinates_to_input(0, 8), "");

        // Test malformed input
        assert_eq!(Board::input_to_coordinates(""), None);
        assert_eq!(Board::input_to_coordinates("3DD"), None);

        // Test non-digit character for row
        assert_eq!(Board::input_to_coordinates("XA"), None);
    }

    #[test]
    fn test_game_over_detection() {
        let mut board = Board::new();

        // Game should not be over at start
        assert!(!board.check_game_over());
        assert!(!board.is_game_over());

        // Fill most of the board to create an end game scenario
        for row in 0..SIZE {
            for col in 0..SIZE {
                if board.get_cell(row, col) == Ok(Cell::Empty) {
                    // Skip center area to avoid breaking existing setup
                    if row < 2 || row > 5 || col < 2 || col > 5 {
                        board.set_cell(row, col, Cell::Black);
                    }
                }
            }
        }

        // Update legal moves count to reflect filled board
        board.set_nb_legal_moves(Cell::Black, None).unwrap();
        board.set_nb_legal_moves(Cell::White, None).unwrap();

        assert!(board.check_game_over());
        assert!(board.is_game_over());
    }

    #[test]
    fn test_move_validation_edge_cases() {
        let board = Board::new();

        // Test out of bounds moves
        assert!(board.can_play(8, 0, Cell::Black).is_err());
        assert!(board.can_play(0, 8, Cell::Black).is_err());
        assert!(board.can_play(SIZE, SIZE, Cell::Black).is_err());

        // Test occupied cells
        assert!(board.can_play(3, 3, Cell::Black).is_err());
        assert!(board.can_play(3, 4, Cell::Black).is_err());
        assert!(board.can_play(4, 3, Cell::Black).is_err());
        assert!(board.can_play(4, 4, Cell::Black).is_err());

        // Test invalid colors
        assert!(board.can_play(2, 3, Cell::Empty).is_err());
    }

    #[test]
    fn test_disc_flipping_behavior() {
        let mut board = Board::new();

        // Test a simple valid move that flips one disc
        let result = board.try_play_move(2, 3, Cell::Black);
        assert!(result.is_ok());

        // Verify the disc was placed
        assert_eq!(board.get_cell(2, 3), Ok(Cell::Black));

        // Verify the disc below was flipped from White to Black
        assert_eq!(board.get_cell(3, 3), Ok(Cell::Black));

        // Check that the move returned the correct count (1 new disc + 1 flipped)
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_winner_determination_edge_cases() {
        let mut board = Board::new();

        // Initially should be a tie
        assert_eq!(board.get_winner(), None);

        // Make black have more discs
        board.nb_discs[0] = 32; // Black
        board.nb_discs[1] = 31; // White
        assert_eq!(board.get_winner(), Some(Cell::Black));

        // Make white have more discs
        board.nb_discs[0] = 30; // Black
        board.nb_discs[1] = 34; // White
        assert_eq!(board.get_winner(), Some(Cell::White));

        // Tie game
        board.nb_discs[0] = 32; // Black
        board.nb_discs[1] = 32; // White
        assert_eq!(board.get_winner(), None);
    }

    #[test]
    fn test_hash_generation() {
        let board1 = Board::new();
        let board2 = Board::new();

        // Same board state should produce same hash
        assert_eq!(board1.to_hash(), board2.to_hash());

        // Hash should be 65 characters (1 for turn + 64 for cells)
        assert_eq!(board1.to_hash().len(), 65);

        // Hash should start with 'B' for Black's turn
        assert!(board1.to_hash().starts_with('B'));

        // Make a move and verify hash changes
        let mut board3 = Board::new();
        board3.try_play_move(2, 3, Cell::Black).unwrap();
        assert_ne!(board1.to_hash(), board3.to_hash());
    }

    #[test]
    fn test_legal_moves_boundary_conditions() {
        let board = Board::new();

        // Test that initial legal moves are correct
        let black_moves = board.has_legal_moves(Cell::Black).unwrap();
        let white_moves = board.has_legal_moves(Cell::White).unwrap();

        // Should have exactly 4 legal moves each at start
        assert_eq!(black_moves.len(), 4);
        assert_eq!(white_moves.len(), 4);

        // Verify specific starting moves for black
        assert!(black_moves.contains(&(2, 3)));
        assert!(black_moves.contains(&(3, 2)));
        assert!(black_moves.contains(&(4, 5)));
        assert!(black_moves.contains(&(5, 4)));
    }

    #[test]
    fn test_recursive_move_validation_edge_cases() {
        let board = Board::new();

        // Test edge of board
        assert!(!board.is_move_valid_recursive(0, 0, Cell::Black, 1, (-1, -1)));
        assert!(!board.is_move_valid_recursive(7, 7, Cell::Black, 1, (1, 1)));

        // Test same color adjacent (should be invalid)
        assert!(!board.is_move_valid_recursive(3, 4, Cell::Black, 1, (1, 0)));

        // Test valid direction from starting position
        assert!(board.is_move_valid_recursive(2, 3, Cell::Black, 1, (1, 0)));
    }

    #[test]
    fn test_empty_board_scenario() {
        let mut board = Board::new();

        // Clear the board
        for row in 0..SIZE {
            for col in 0..SIZE {
                board.set_cell(row, col, Cell::Empty);
            }
        }

        // No legal moves should exist on empty board
        assert_eq!(board.has_legal_moves(Cell::Black), None);
        assert_eq!(board.has_legal_moves(Cell::White), None);
    }

    #[test]
    fn test_single_disc_scenario() {
        let mut board = Board::new();

        // Clear board and place single black disc
        for row in 0..SIZE {
            for col in 0..SIZE {
                board.set_cell(row, col, Cell::Empty);
            }
        }
        board.set_cell(3, 3, Cell::Black);

        // No legal moves should exist with only one disc
        assert_eq!(board.has_legal_moves(Cell::Black), None);
        assert_eq!(board.has_legal_moves(Cell::White), None);
    }

    #[test]
    fn test_history_tracking() {
        let board = Board::new();

        // Initially empty history
        assert_eq!(board.get_history().len(), 0);

        // Add a move to history
        let mut board_with_history = board;
        let action = HistoryAction {
            coordinates: Some("2D".to_string()),
            gained_discs: Some(1),
            color: Cell::Black,
            player_turn: Cell::Black,
            move_number: 1,
        };
        board_with_history.add_to_history(action);

        assert_eq!(board_with_history.get_history().len(), 1);
        assert_eq!(
            board_with_history.get_history()[0].coordinates,
            Some("2D".to_string())
        );
    }
}
