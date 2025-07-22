//! AI algorithm type enumeration and utilities.
//!
//! This module defines the different types of AI algorithms available in the
//! Othello game, providing a way to identify and cycle through different AI
//! implementations for gameplay and comparison purposes.

/// Enumeration of available AI algorithm types.
///
/// This enum represents the different AI algorithms that can be used to play
/// Othello. Each variant corresponds to a specific AI implementation with
/// different strengths and characteristics.
///
/// # Examples
///
/// ```rust
/// let ai_type = AIType::AlphaBeta;
/// println!("Using AI: {}", ai_type);
///
/// // Cycle through AI types
/// let next_ai = ai_type.next();
/// assert_eq!(next_ai, AIType::MinMax);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum AIType {
    /// Alpha-Beta pruning algorithm - optimized minimax with branch pruning
    AlphaBeta,
    /// Minimax algorithm - classic game tree search
    MinMax,
    /// Q-Learning algorithm - reinforcement learning approach
    QLearning,
}

impl std::fmt::Display for AIType {
    /// Formats the AI type for display purposes.
    ///
    /// Provides user-friendly names for each AI algorithm type, suitable
    /// for UI display and logging.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(format!("{}", AIType::AlphaBeta), "Alpha-Beta");
    /// assert_eq!(format!("{}", AIType::MinMax), "Min-Max");
    /// assert_eq!(format!("{}", AIType::QLearning), "Q-Learning");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AIType::AlphaBeta => write!(f, "Alpha-Beta"),
            AIType::MinMax => write!(f, "Min-Max"),
            AIType::QLearning => write!(f, "Q-Learning"),
        }
    }
}
