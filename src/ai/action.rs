//! Action representation for AI move evaluation.
//!
//! This module defines the `Action` struct used by AI algorithms to represent
//! a potential move along with its evaluation score. This is essential for
//! game tree search algorithms that need to compare different moves.

/// Represents a potential move with its evaluation score.
///
/// An `Action` combines a board position with its calculated score, allowing
/// AI algorithms to compare different moves and select the best one. This is
/// used by search algorithms like minimax and alpha-beta pruning.
///
/// # Examples
///
/// ```rust
/// // Create an action representing a move at position (3, 4) with score 100
/// let action = Action {
///     pos: (3, 4),
///     score: 100,
/// };
///
/// // Compare two actions
/// let action1 = Action { pos: (2, 3), score: 50 };
/// let action2 = Action { pos: (4, 5), score: 75 };
/// assert!(action2.score > action1.score);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Action {
    /// The board position (row, column) where the move would be played
    pub pos: (usize, usize),
    /// The heuristic score assigned to this move by the AI evaluation function
    pub score: isize,
}
