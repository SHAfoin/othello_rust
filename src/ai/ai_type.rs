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

impl AIType {
    /// Returns the next AI type in the cycle.
    ///
    /// This method allows cycling through different AI types in a predefined
    /// order, useful for UI controls that let users switch between AI algorithms.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let ai = AIType::AlphaBeta;
    /// assert_eq!(ai.next(), AIType::MinMax);
    /// assert_eq!(ai.next().next(), AIType::QLearning);
    /// assert_eq!(ai.next().next().next(), AIType::AlphaBeta);
    /// ```
    pub fn next(&self) -> AIType {
        match self {
            AIType::AlphaBeta => AIType::MinMax,
            AIType::MinMax => AIType::QLearning,
            AIType::QLearning => AIType::AlphaBeta,
        }
    }

    /// Returns the previous AI type in the cycle.
    ///
    /// This method allows cycling backwards through different AI types,
    /// useful for UI controls that let users navigate between AI algorithms
    /// in both directions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let ai = AIType::AlphaBeta;
    /// assert_eq!(ai.previous(), AIType::QLearning);
    /// assert_eq!(ai.previous().previous(), AIType::MinMax);
    /// assert_eq!(ai.previous().previous().previous(), AIType::AlphaBeta);
    /// ```
    pub fn previous(&self) -> AIType {
        match self {
            AIType::AlphaBeta => AIType::QLearning,
            AIType::MinMax => AIType::AlphaBeta,
            AIType::QLearning => AIType::MinMax,
        }
    }
}
