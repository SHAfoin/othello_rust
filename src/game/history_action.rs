//! Move history tracking for Othello game.
//!
//! This module defines the `HistoryAction` struct used to record information
//! about moves played during an Othello game. This enables features like
//! move replay, game analysis, and undo functionality.

use crate::game::cell::Cell;

/// Represents a recorded move in the game history.
///
/// A `HistoryAction` captures all the essential information about a move
/// that was played, including the position, the number of discs gained,
/// and the game state at the time of the move. This is used for game
/// replay, analysis, and potentially undo functionality.
///
/// # Examples
///
/// ```rust
/// // Create a history action for a move at position "2D"
/// let action = HistoryAction {
///     coordinates: Some("2D".to_string()),
///     gained_discs: Some(3),
///     color: Cell::Black,
///     move_number: 5,
///     player_turn: Cell::Black,
/// };
///
/// // Create a pass action (no move available)
/// let pass_action = HistoryAction {
///     coordinates: None,
///     gained_discs: None,
///     color: Cell::White,
///     move_number: 6,
///     player_turn: Cell::White,
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryAction {
    /// The board coordinates where the move was played (e.g., "3D"), or None for a pass
    pub coordinates: Option<String>,
    /// The number of opponent discs that were flipped by this move, or None for a pass
    pub gained_discs: Option<usize>,
    /// The color of the disc that was placed (the player who made the move)
    pub color: Cell,
    /// The sequential number of this move in the game
    pub move_number: usize,
    /// The player whose turn it was when this move was made
    pub player_turn: Cell,
}
