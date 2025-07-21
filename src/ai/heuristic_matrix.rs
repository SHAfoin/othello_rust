//! Heuristic matrices for AI position evaluation.
//!
//! This module provides different heuristic matrices that assign values to
//! board positions in Othello. These matrices are used by AI algorithms to
//! evaluate the strategic value of different board positions, with corners
//! typically having high values and adjacent positions having negative values.

use std::fmt::Display;

use crate::consts::SIZE;

/// Enumeration of available heuristic matrices for position evaluation.
///
/// Different matrices provide different strategic evaluations of board positions.
/// Matrix A uses more conservative values while Matrix B uses more aggressive
/// valuations, particularly for central positions.
///
/// # Examples
///
/// ```rust
/// let matrix = AIHeuristicMatrix::A;
/// println!("Using {}", matrix);
///
/// // Get the actual matrix values
/// let values = matrix.value();
/// assert_eq!(values[0][0], 100); // Corner position has high value
/// ```
#[derive(Clone, Debug)]
pub enum AIHeuristicMatrix {
    /// Matrix A - Conservative position evaluation with standard corner values
    A,
    /// Matrix B - Aggressive position evaluation with higher central values
    B,
}

impl Display for AIHeuristicMatrix {
    /// Formats the heuristic matrix for display purposes.
    ///
    /// Provides user-friendly names for each matrix type, suitable for
    /// UI display and logging.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(format!("{}", AIHeuristicMatrix::A), "Matrix A");
    /// assert_eq!(format!("{}", AIHeuristicMatrix::B), "Matrix B");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AIHeuristicMatrix::A => write!(f, "Matrix A"),
            AIHeuristicMatrix::B => write!(f, "Matrix B"),
        }
    }
}

impl AIHeuristicMatrix {
    /// Returns the actual matrix values for position evaluation.
    ///
    /// This method provides the 8x8 matrix of integer values used to evaluate
    /// board positions. Each matrix has different strategic characteristics:
    ///
    /// - **Matrix A**: Conservative evaluation with standard Othello strategy
    /// - **Matrix B**: Aggressive evaluation with higher central values
    ///
    /// # Matrix Strategy
    ///
    /// - **Corners (100/500)**: Highest value as they cannot be captured
    /// - **Corner adjacent (-20/-150)**: Negative value as they give opponent corner access
    /// - **Edges (10/30)**: Moderate positive value as they're relatively stable
    /// - **Center (varies)**: Different strategies between matrices
    ///
    /// # Examples
    ///
    /// ```rust
    /// let matrix_a = AIHeuristicMatrix::A;
    /// let values = matrix_a.value();
    /// assert_eq!(values[0][0], 100); // Top-left corner
    /// assert_eq!(values[0][1], -20); // Corner adjacent
    ///
    /// let matrix_b = AIHeuristicMatrix::B;
    /// let values_b = matrix_b.value();
    /// assert_eq!(values_b[0][0], 500); // Higher corner value
    /// assert_eq!(values_b[3][3], 16);  // Central position
    /// ```
    pub fn value(&self) -> [[isize; SIZE]; SIZE] {
        match self {
            AIHeuristicMatrix::A => [
                [100, -20, 10, 5, 5, 10, -20, 100],
                [-20, -50, -2, -2, -2, -2, -50, -20],
                [10, -2, -1, -1, -1, -1, -2, 10],
                [5, -2, -1, -1, -1, -1, -2, 5],
                [5, -2, -1, -1, -1, -1, -2, 5],
                [10, -2, -1, -1, -1, -1, -2, 10],
                [-20, -50, -2, -2, -2, -2, -50, -20],
                [100, -20, 10, 5, 5, 10, -20, 100],
            ],
            AIHeuristicMatrix::B => [
                [500, -150, 30, 10, 10, 30, -150, 500],
                [-150, -250, 0, 0, 0, 0, -250, -150],
                [30, 0, 1, 2, 2, 1, 0, 30],
                [10, 0, 2, 16, 16, 2, 0, 10],
                [10, 0, 2, 16, 16, 2, 0, 10],
                [30, 0, 1, 2, 2, 1, 0, 30],
                [-150, -250, 0, 0, 0, 0, -250, -150],
                [500, -150, 30, 10, 10, 30, -150, 500],
            ],
        }
    }

    /// Returns the next matrix in the cycle.
    ///
    /// This method allows cycling between different heuristic matrices,
    /// useful for UI controls that let users switch between evaluation
    /// strategies.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let matrix = AIHeuristicMatrix::A;
    /// assert_eq!(matrix.next(), AIHeuristicMatrix::B);
    /// assert_eq!(matrix.next().next(), AIHeuristicMatrix::A);
    /// ```
    pub fn next(&self) -> AIHeuristicMatrix {
        match self {
            AIHeuristicMatrix::A => AIHeuristicMatrix::B,
            AIHeuristicMatrix::B => AIHeuristicMatrix::A,
        }
    }

    /// Returns the previous matrix in the cycle.
    ///
    /// This method allows cycling backwards between different heuristic matrices,
    /// useful for UI controls that let users navigate between evaluation
    /// strategies in both directions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let matrix = AIHeuristicMatrix::A;
    /// assert_eq!(matrix.previous(), AIHeuristicMatrix::B);
    /// assert_eq!(matrix.previous().previous(), AIHeuristicMatrix::A);
    /// ```
    pub fn previous(&self) -> AIHeuristicMatrix {
        match self {
            AIHeuristicMatrix::A => AIHeuristicMatrix::B,
            AIHeuristicMatrix::B => AIHeuristicMatrix::A,
        }
    }
}
