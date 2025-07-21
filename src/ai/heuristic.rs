//! Heuristic evaluation functions for AI position assessment.
//!
//! This module provides various heuristic functions that AI algorithms use to
//! evaluate board positions in Othello. Each heuristic focuses on different
//! aspects of the game such as piece count, position value, mobility, or
//! combinations thereof.

use std::fmt::Display;

use crate::ai::heuristic_matrix::AIHeuristicMatrix;
use crate::consts::SIZE;
use crate::game::{board::Board, cell::Cell};

/// Enumeration of available heuristic evaluation functions.
///
/// Each heuristic type represents a different strategy for evaluating board
/// positions, focusing on different aspects of Othello gameplay:
///
/// - **Absolute**: Simple piece count difference
/// - **Matrix**: Position-based evaluation using strategic matrices
/// - **Mobility**: Move availability and tactical flexibility
/// - **Mixte**: Adaptive strategy that changes based on game phase
/// - **Global**: Combined evaluation using multiple heuristics
///
/// # Examples
///
/// ```rust
/// let heuristic = HeuristicType::Matrix;
/// let board = Board::new();
/// let player = Cell::Black;
/// let matrix = AIHeuristicMatrix::A;
///
/// let score = heuristic.evaluate(&board, player, matrix);
/// println!("Position evaluation: {}", score);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum HeuristicType {
    /// Absolute piece count difference (own pieces - opponent pieces)
    Absolute,
    /// Matrix-based position evaluation using strategic weight matrices
    Matrix,
    /// Mobility-based evaluation focusing on available moves
    Mobility,
    /// Mixed strategy that adapts based on game phase
    Mixte,
    /// Global evaluation combining all heuristic types
    Global,
}

impl Display for HeuristicType {
    /// Formats the heuristic type for display purposes.
    ///
    /// Provides user-friendly names for each heuristic type, suitable for
    /// UI display and logging.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(format!("{}", HeuristicType::Absolute), "Absolute");
    /// assert_eq!(format!("{}", HeuristicType::Matrix), "Matrix");
    /// assert_eq!(format!("{}", HeuristicType::Mobility), "Mobility");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeuristicType::Absolute => write!(f, "Absolute"),
            HeuristicType::Matrix => write!(f, "Matrix"),
            HeuristicType::Mobility => write!(f, "Mobility"),
            HeuristicType::Mixte => write!(f, "Mixte"),
            HeuristicType::Global => write!(f, "Global"),
        }
    }
}

impl HeuristicType {
    /// Evaluates a board position using the selected heuristic function.
    ///
    /// This method applies the chosen heuristic strategy to evaluate how good
    /// the current board position is for the specified player. Higher scores
    /// indicate better positions for the player.
    ///
    /// # Arguments
    ///
    /// * `board` - The current board state to evaluate
    /// * `player` - The player from whose perspective to evaluate the position
    /// * `matrix` - The heuristic matrix to use for position-based evaluations
    ///
    /// # Returns
    ///
    /// An integer score representing the position's value for the player.
    /// Positive values favor the player, negative values favor the opponent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let board = Board::new();
    /// let player = Cell::Black;
    /// let matrix = AIHeuristicMatrix::A;
    ///
    /// let absolute_score = HeuristicType::Absolute.evaluate(&board, player, matrix.clone());
    /// let matrix_score = HeuristicType::Matrix.evaluate(&board, player, matrix);
    /// ```
    pub fn evaluate(&self, board: &Board, player: Cell, matrix: AIHeuristicMatrix) -> isize {
        match self {
            HeuristicType::Absolute => heuristic_absolute(board, player),
            HeuristicType::Matrix => heuristic_matrix(board, player, &matrix),
            HeuristicType::Mobility => heuristic_mobility(board, player),
            HeuristicType::Mixte => heuristic_mixte(board, player, &matrix),
            HeuristicType::Global => {
                // Global heuristic combines all heuristics
                heuristic_absolute(board, player)
                    + heuristic_matrix(board, player, &matrix)
                    + heuristic_mobility(board, player)
            }
        }
    }

    /// Returns the next heuristic type in the cycle.
    ///
    /// This method allows cycling through different heuristic types in a
    /// predefined order, useful for UI controls that let users switch
    /// between evaluation strategies.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let heuristic = HeuristicType::Absolute;
    /// assert_eq!(heuristic.next(), HeuristicType::Matrix);
    /// assert_eq!(heuristic.next().next(), HeuristicType::Mobility);
    /// ```
    pub fn next(&self) -> HeuristicType {
        match self {
            HeuristicType::Absolute => HeuristicType::Matrix,
            HeuristicType::Matrix => HeuristicType::Mobility,
            HeuristicType::Mobility => HeuristicType::Mixte,
            HeuristicType::Mixte => HeuristicType::Global,
            HeuristicType::Global => HeuristicType::Absolute,
        }
    }

    /// Returns the previous heuristic type in the cycle.
    ///
    /// This method allows cycling backwards through different heuristic types,
    /// useful for UI controls that let users navigate between evaluation
    /// strategies in both directions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let heuristic = HeuristicType::Absolute;
    /// assert_eq!(heuristic.previous(), HeuristicType::Global);
    /// assert_eq!(heuristic.previous().previous(), HeuristicType::Mixte);
    /// ```
    pub fn previous(&self) -> HeuristicType {
        match self {
            HeuristicType::Absolute => HeuristicType::Global,
            HeuristicType::Matrix => HeuristicType::Absolute,
            HeuristicType::Mobility => HeuristicType::Matrix,
            HeuristicType::Mixte => HeuristicType::Mobility,
            HeuristicType::Global => HeuristicType::Mixte,
        }
    }
}

/// Absolute heuristic: evaluates based on piece count difference.
///
/// This is the simplest heuristic that calculates the difference between
/// the number of pieces the player has and the number of pieces the opponent has.
/// A positive value means the player has more pieces, negative means the opponent
/// has more pieces.
///
/// # Arguments
///
/// * `board` - The current board state
/// * `player` - The player from whose perspective to evaluate
///
/// # Returns
///
/// The piece count difference (player pieces - opponent pieces)
///
/// # Examples
///
/// ```rust
/// let board = Board::new();
/// let player = Cell::Black;
/// let score = heuristic_absolute(&board, player);
/// // At start, both players have 2 pieces, so score is 0
/// assert_eq!(score, 0);
/// ```
fn heuristic_absolute(board: &Board, player: Cell) -> isize {
    board.get_nb_discs(player).unwrap() as isize
        - board.get_nb_discs(player.get_opponent()).unwrap() as isize
}

/// Matrix heuristic: evaluates based on strategic position values.
///
/// This heuristic assigns different values to different board positions based
/// on their strategic importance. Corners have high values, edges have moderate
/// values, and positions adjacent to corners have negative values.
///
/// # Arguments
///
/// * `board` - The current board state
/// * `player` - The player from whose perspective to evaluate
/// * `matrix` - The heuristic matrix containing position values
///
/// # Returns
///
/// The sum of matrix values for all positions occupied by the player
///
/// # Strategy
///
/// - Corners (100/500): Cannot be flipped, highly valuable
/// - Edges (10/30): Relatively stable positions
/// - Corner adjacent (-20/-150): Give opponent access to corners
/// - Center (varies): Different values based on matrix type
fn heuristic_matrix(board: &Board, player: Cell, matrix: &AIHeuristicMatrix) -> isize {
    let mut score = 0;
    for row in 0..SIZE {
        for col in 0..SIZE {
            if board.get_cell(row, col).unwrap() == player {
                score += matrix.value()[row][col];
            }
        }
    }
    score
}

/// Mobility heuristic: evaluates based on available moves.
///
/// This heuristic considers the number of legal moves available to each player.
/// Having more moves available is generally advantageous as it provides more
/// options and flexibility. This is particularly important in the middle game.
///
/// # Arguments
///
/// * `board` - The current board state
/// * `player` - The player from whose perspective to evaluate
///
/// # Returns
///
/// The mobility difference (player moves - opponent moves)
///
/// # Strategy
///
/// - More moves = more options and control
/// - Fewer opponent moves = restricting opponent's choices
/// - Particularly important in middle game phases
fn heuristic_mobility(board: &Board, player: Cell) -> isize {
    let nb_moves_player = board.get_nb_legal_moves(player).unwrap().unwrap_or(0) as isize;
    let nb_moves_opponent = board
        .get_nb_legal_moves(player.get_opponent())
        .unwrap()
        .unwrap_or(0) as isize;
    nb_moves_player - nb_moves_opponent
}

/// Mixed heuristic: adaptive strategy based on game phase.
///
/// This heuristic changes its evaluation strategy based on the current turn number,
/// adapting to different phases of the game:
///
/// - **Early game** (turns 1-19): Focus on positional play using matrix values
/// - **Middle game** (turns 20-39): Focus on mobility and tactical flexibility
/// - **Late game** (turns 40+): Focus on piece count for endgame advantage
///
/// # Arguments
///
/// * `board` - The current board state
/// * `player` - The player from whose perspective to evaluate
/// * `matrix` - The heuristic matrix for positional evaluation
///
/// # Returns
///
/// The evaluation score based on the current game phase
///
/// # Strategy Phases
///
/// 1. **Opening** (< 20 moves): Secure good positions, avoid bad ones
/// 2. **Middle game** (20-40 moves): Maintain mobility, control opponent options
/// 3. **Endgame** (> 40 moves): Maximize piece count for final advantage
fn heuristic_mixte(board: &Board, player: Cell, matrix: &AIHeuristicMatrix) -> isize {
    if board.get_turn_number() < 20 {
        heuristic_matrix(board, player, matrix)
    } else if board.get_turn_number() < 40 {
        heuristic_mobility(board, player)
    } else {
        heuristic_absolute(board, player)
    }
}
