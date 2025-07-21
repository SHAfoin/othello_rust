//! Human player implementation for Othello game.
//!
//! This module provides the `Human` struct that implements the `Player` trait
//! for human players. It handles user input through the terminal interface,
//! allowing players to enter moves in algebraic notation and providing
//! help commands and error handling.

use crate::game::{board::Board, cell::Cell, history_action::HistoryAction, player::Player};

/// Represents a human player in the Othello game.
///
/// The `Human` struct implements the `Player` trait to handle human input
/// through the terminal. It provides an interactive interface where players
/// can enter moves using algebraic notation (e.g., "3D"), access help commands,
/// and exit the game gracefully.
///
/// # Examples
///
/// ```
/// let human_player = Human::new(Cell::Black);
/// ```
pub struct Human {
    /// The color of the discs this human player controls
    color: Cell,
}

impl Human {
    /// Creates a new human player with the specified disc color.
    ///
    /// # Arguments
    ///
    /// * `color` - The color of discs this player will control (Black or White)
    ///
    /// # Returns
    ///
    /// A new `Human` instance configured with the given color
    ///
    /// # Examples
    ///
    /// ```
    /// let black_player = Human::new(Cell::Black);
    /// let white_player = Human::new(Cell::White);
    /// ```
    pub fn new(color: Cell) -> Self {
        Self { color }
    }

    /// Returns the disc color of this human player.
    ///
    /// # Returns
    ///
    /// The `Cell` color (Black or White) that this player controls
    ///
    /// # Examples
    ///
    /// ```
    /// let player = Human::new(Cell::Black);
    /// assert_eq!(player.get_color(), Cell::Black);
    /// ```
    fn get_color(&self) -> Cell {
        self.color
    }

    /// Prompts the human player to enter a move through terminal input.
    ///
    /// This method provides an interactive loop that:
    /// - Prompts the player to enter a move in algebraic notation (e.g., "3D")
    /// - Handles special commands like "exit" and "help"
    /// - Validates input format and converts to board coordinates
    /// - Provides error messages for invalid input
    ///
    /// # Returns
    ///
    /// * `Some((row, col))` - Valid board coordinates if input is successful
    /// * `None` - This method loops until valid input or exit, so None is not returned
    ///
    /// # Special Commands
    ///
    /// * "exit" - Terminates the game immediately
    /// * "help" - Displays available commands and input format
    ///
    /// # Examples
    ///
    /// ```
    /// let player = Human::new(Cell::Black);
    /// // Player enters "3D" -> Some((2, 3))
    /// // Player enters "help" -> displays help and continues loop
    /// // Player enters "exit" -> terminates program
    /// ```
    fn get_player_move(&self) -> Option<(usize, usize)> {
        loop {
            println!(
                "{} : Enter your move (row and column, e.g., '3D'): ",
                self.get_color()
            );

            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let input = input.trim();

                    if input == "exit" {
                        println!("Exiting the game.");
                        std::process::exit(0);
                    } else if input == "help" {
                        println!("Available commands:");
                        println!("  - Enter your move in 'rowColumn' format (e.g., '3D').");
                        println!("  - Type 'exit' to quit the game.");
                        continue;
                    }

                    match Board::input_to_coordinates(input) {
                        Some(coords) => return Some(coords),
                        None => println!(
                            "Invalid input format. Please use 'rowColumn' format (e.g., '3D')."
                        ),
                    }
                }
                Err(e) => {
                    println!("Error reading input: {}", e);
                    continue;
                }
            }
        }
    }
}

/// Implementation of the `Player` trait for human players.
///
/// This implementation provides the interface for human players to interact
/// with the game through the terminal. It differs from AI players by requiring
/// external input (cell coordinates) rather than computing moves automatically.
impl Player for Human {
    /// Identifies this player as a human player.
    ///
    /// This method is used by the game engine to determine whether
    /// to prompt for input or let the AI compute moves automatically.
    ///
    /// # Returns
    ///
    /// Always returns `true` for human players
    ///
    /// # Examples
    ///
    /// ```
    /// let human = Human::new(Cell::Black);
    /// assert!(human.is_human());
    /// ```
    fn is_human(&self) -> bool {
        true
    }

    /// Executes a move for the human player on the game board.
    ///
    /// This method takes pre-validated coordinates (typically from GUI input)
    /// and attempts to place a disc at that position. It handles the complete
    /// move process including disc placement, capturing opponent discs, and
    /// creating a history record of the action.
    ///
    /// # Arguments
    ///
    /// * `board` - Mutable reference to the game board
    /// * `cell` - Optional coordinates (row, col) where the player wants to move
    ///
    /// # Returns
    ///
    /// * `Ok(HistoryAction)` - Move was successful, returns action details
    /// * `Err(String)` - Move failed, returns error description
    ///
    /// # Errors
    ///
    /// * Returns error if no cell coordinates provided
    /// * Returns error if the move is invalid (occupied cell, no captures, etc.)
    /// * Returns error if board state prevents the move
    ///
    /// # Examples
    ///
    /// ```
    /// let mut board = Board::new();
    /// let human = Human::new(Cell::Black);
    ///
    /// match human.play_turn(&mut board, Some((2, 3))) {
    ///     Ok(action) => println!("Move successful: {:?}", action),
    ///     Err(e) => println!("Move failed: {}", e),
    /// }
    /// ```
    fn play_turn(
        &self,
        board: &mut Board,
        cell: Option<(usize, usize)>,
    ) -> Result<HistoryAction, String> {
        if let Some((row, col)) = cell {
            match board.try_play_move(row, col, self.get_color()) {
                Ok(gained_discs) => Ok(HistoryAction {
                    coordinates: Some(Board::coordinates_to_input(row, col)),
                    gained_discs: Some(gained_discs),
                    color: self.get_color(),
                    player_turn: board.get_player_turn(),
                    move_number: board.get_turn_number(),
                }),
                Err(e) => Err(format!("Error playing move: {}", e)),
            }
        } else {
            Err("No cell selected.".to_string())
        }
    }
}
