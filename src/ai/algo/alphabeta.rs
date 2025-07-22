//! Alpha-Beta pruning AI implementation for Othello game.
//!
//! This module implements the alpha-beta pruning algorithm, an optimization
//! of the minimax algorithm that reduces the number of nodes evaluated in
//! the search tree by maintaining lower (alpha) and upper (beta) bounds.

use std::thread;

use crate::{
    ai::{
        action::Action, ai_type::AIType, heuristic::HeuristicType,
        heuristic_matrix::AIHeuristicMatrix,
    },
    game::{board::Board, cell::Cell, history_action::HistoryAction, player::Player},
};

/// An AI player that uses alpha-beta pruning to make optimal moves in Othello.
///
/// The Alpha-Beta algorithm is an optimization of the minimax algorithm that
/// maintains two bounds (alpha and beta) to prune branches of the search tree
/// that cannot improve the current best move. This significantly reduces the
/// number of nodes that need to be evaluated while maintaining the same optimal
/// result as minimax.
///
/// The AI uses multithreading to evaluate multiple possible moves in parallel,
/// improving performance on multi-core systems.
///
/// # Examples
///
/// ```rust
/// // Create a new alpha-beta AI with depth 4
/// let ai = AIAlphaBeta::new(
///     4,
///     HeuristicType::default(),
///     Cell::Black,
///     AIHeuristicMatrix::default(),
/// );
///
/// // Get the AI's color
/// assert_eq!(ai.get_color(), Cell::Black);
/// ```
#[derive(Clone, Debug)]
pub struct AIAlphaBeta {
    /// Maximum search depth for the alpha-beta algorithm
    depth: usize,
    /// Heuristic function used to evaluate board positions
    heuristic: HeuristicType,
    /// The color (Black or White) that this AI player represents
    color: Cell,
    /// Matrix used by the heuristic function for position evaluation
    matrix: AIHeuristicMatrix,
}

impl AIAlphaBeta {
    /// Creates a new alpha-beta AI player with specified parameters.
    ///
    /// This constructor initializes an AI player that will use the alpha-beta
    /// pruning algorithm to determine the best moves. The AI's strength and
    /// behavior depend on the depth of search and the heuristic function used.
    ///
    /// # Arguments
    ///
    /// * `depth` - Maximum search depth for the alpha-beta algorithm. Higher values
    ///   result in stronger play but slower move calculation.
    /// * `heuristic` - The heuristic function used to evaluate board positions
    /// * `color` - The color (Black or White) that this AI will play as
    /// * `matrix` - Heuristic matrix used for position evaluation
    ///
    /// # Examples
    ///
    /// ```rust
    /// let ai = AIAlphaBeta::new(
    ///     6,  // Search depth of 6 moves ahead
    ///     HeuristicType::default(),
    ///     Cell::Black,
    ///     AIHeuristicMatrix::default(),
    /// );
    /// ```
    pub fn new(
        depth: usize,
        heuristic: HeuristicType,
        color: Cell,
        matrix: AIHeuristicMatrix,
    ) -> Self {
        Self {
            depth,
            heuristic,
            color,
            matrix,
        }
    }

    /// Returns the color (Black or White) that this AI player represents.
    ///
    /// This is useful for determining which player this AI instance is playing as,
    /// which affects move evaluation and game logic.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let ai = AIAlphaBeta::new(4, HeuristicType::default(), Cell::White, AIHeuristicMatrix::default());
    /// assert_eq!(ai.get_color(), Cell::White);
    /// ```
    pub fn get_color(&self) -> Cell {
        self.color
    }

    /// Initializes the alpha-beta search tree with initial bounds.
    ///
    /// This method starts the alpha-beta pruning algorithm with the worst
    /// possible alpha and beta values (MIN and MAX) to ensure the algorithm
    /// can find the optimal move from any position.
    ///
    /// # Arguments
    ///
    /// * `board` - The current game board state to evaluate
    /// * `depth` - Maximum search depth for this evaluation
    ///
    /// # Returns
    ///
    /// The heuristic score of the best move found at the given depth
    ///
    /// # Examples
    ///
    /// ```rust
    /// let ai = AIAlphaBeta::new(4, HeuristicType::default(), Cell::Black, AIHeuristicMatrix::default());
    /// let board = Board::new();
    /// let score = ai.init_tree(&board, 4);
    /// // Score represents the evaluation of the best move at depth 4
    /// ```
    pub fn init_tree(&self, board: &Board, depth: usize) -> isize {
        self.tree_step(board, depth, &isize::MIN, &isize::MAX)
    }

    /// Performs one step of the alpha-beta pruning algorithm.
    ///
    /// This is the core recursive function that implements the alpha-beta pruning
    /// algorithm. It alternates between maximizing and minimizing players based
    /// on the current depth (odd depths are minimizing, even depths are maximizing).
    ///
    /// The algorithm maintains alpha (best value for maximizing player) and beta
    /// (best value for minimizing player) bounds, pruning branches when alpha >= beta.
    ///
    /// # Arguments
    ///
    /// * `board` - Current board state to evaluate
    /// * `depth` - Remaining search depth (decreases with each recursive call)
    /// * `alpha` - Lower bound for the maximizing player
    /// * `beta` - Upper bound for the minimizing player
    ///
    /// # Returns
    ///
    /// The heuristic evaluation score of the best move in this subtree
    ///
    /// # Algorithm Details
    ///
    /// - Base case: When depth reaches 1 or no legal moves exist, return heuristic evaluation
    /// - Recursive case: Try all legal moves and recursively evaluate resulting positions
    /// - Pruning: When alpha >= beta, prune the remaining branches (they won't affect the result)
    /// - Alternating players: Odd depths minimize, even depths maximize
    pub fn tree_step(&self, board: &Board, depth: usize, alpha: &isize, beta: &isize) -> isize {
        let mut alpha_mut = alpha.clone();
        let mut beta_mut = beta.clone();

        if depth == 1 || board.has_legal_moves(board.get_player_turn()) == None {
            // Base case: evaluate the current position using the heuristic function
            let score = self
                .heuristic
                .evaluate(board, self.get_color(), self.matrix.clone());
            return score;
        } else {
            // Recursive case: try all legal moves and find the best one
            for case in board.has_legal_moves(board.get_player_turn()).unwrap() {
                let mut new_board = board.clone();
                match new_board.try_play_move(case.0, case.1, board.get_player_turn()) {
                    Ok(_) => {
                        let score = self.tree_step(&new_board, depth - 1, &alpha_mut, &beta_mut);
                        if depth % 2 == 1 {
                            // Minimizing player (odd depth)
                            if score < beta_mut {
                                beta_mut = score
                            }
                            if alpha_mut >= beta_mut {
                                // Alpha-beta pruning: remaining branches won't improve the result
                                return beta_mut;
                            }
                        } else {
                            // Maximizing player (even depth)
                            if score > alpha_mut {
                                alpha_mut = score
                            }
                            if alpha_mut >= beta_mut {
                                // Alpha-beta pruning: remaining branches won't improve the result
                                return alpha_mut;
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            // Return the best value found for the current player
            if depth % 2 == 1 {
                // Minimizing player returns beta
                return beta_mut;
            } else {
                // Maximizing player returns alpha
                return alpha_mut;
            }
        }
    }
}

impl Player for AIAlphaBeta {
    /// Returns false as this is an AI player, not a human player.
    ///
    /// This method is used by the game engine to determine whether to wait for
    /// human input or to automatically trigger AI move calculation.
    fn is_human(&self) -> bool {
        false
    }

    /// Returns the AI type identifier for this player.
    ///
    /// This method allows the game engine to identify which AI algorithm
    /// is being used, which can be useful for UI display or statistics.
    fn get_ai_type(&self) -> Option<AIType> {
        Some(AIType::AlphaBeta)
    }

    /// Returns a copy of the current heuristic matrix.
    ///
    /// The heuristic matrix is used by the evaluation function to assign
    /// different weights to different board positions.
    fn get_heuristic_matrix(&self) -> AIHeuristicMatrix {
        self.matrix.clone()
    }

    /// Updates the heuristic matrix used for position evaluation.
    ///
    /// This allows for dynamic adjustment of the AI's evaluation function
    /// during gameplay, which can be useful for learning algorithms or
    /// adaptive difficulty.
    ///
    /// # Arguments
    ///
    /// * `matrix` - The new heuristic matrix to use for evaluation
    fn set_heuristic_matrix(&mut self, matrix: AIHeuristicMatrix) {
        self.matrix = matrix;
    }

    /// Returns the current heuristic function type.
    ///
    /// This identifies which heuristic algorithm is being used to evaluate
    /// board positions during the search.
    fn get_heuristic(&self) -> HeuristicType {
        self.heuristic.clone()
    }

    /// Updates the heuristic function used for position evaluation.
    ///
    /// This allows switching between different evaluation strategies
    /// during gameplay.
    ///
    /// # Arguments
    ///
    /// * `heuristic` - The new heuristic function to use
    fn set_heuristic(&mut self, heuristic: HeuristicType) {
        self.heuristic = heuristic;
    }

    /// Returns the current search depth.
    ///
    /// The search depth determines how many moves ahead the AI will look
    /// when evaluating positions. Higher depths generally result in
    /// stronger play but slower move calculation.
    fn get_depth(&self) -> usize {
        self.depth
    }

    /// Updates the search depth for future move calculations.
    ///
    /// This allows dynamic adjustment of the AI's strength and speed
    /// during gameplay.
    ///
    /// # Arguments
    ///
    /// * `depth` - The new search depth to use
    fn set_depth(&mut self, depth: usize) {
        self.depth = depth;
    }

    /// Calculates and executes the best move for the current board position.
    ///
    /// This method implements the main AI logic, using multithreading to
    /// evaluate all legal moves in parallel. Each possible move is evaluated
    /// using the alpha-beta pruning algorithm to determine its score, and
    /// the move with the highest score is selected and executed.
    ///
    /// # Arguments
    ///
    /// * `board` - Mutable reference to the current game board
    /// * `_cell` - Optional cell parameter (unused in this implementation)
    ///
    /// # Returns
    ///
    /// * `Ok(HistoryAction)` - Details of the move that was played
    /// * `Err(String)` - Error message if move calculation or execution fails
    ///
    /// # Errors
    ///
    /// This method can return errors in the following cases:
    /// - No legal moves are available
    /// - Move evaluation fails due to invalid board state
    /// - Thread join errors during parallel evaluation
    /// - Move execution fails on the board
    ///
    /// # Performance
    ///
    /// This method uses multithreading to evaluate moves in parallel,
    /// which can significantly improve performance on multi-core systems.
    /// The number of threads created equals the number of legal moves available.
    fn play_turn(
        &self,
        board: &mut Board,
        _cell: Option<(usize, usize)>,
    ) -> Result<HistoryAction, String> {
        // Initialize the best action with minimum score
        let mut best_action = Action {
            pos: (0, 0),
            score: isize::MIN,
        };
        let mut handles = vec![];

        // Evaluate all legal moves in parallel using threads
        for case in board.has_legal_moves(board.get_player_turn()).unwrap() {
            let mut new_board = board.clone();

            match new_board.try_play_move(case.0, case.1, self.get_color()) {
                Ok(_) => {
                    // Clone the AI for the thread and spawn evaluation task
                    let ai_cloned = self.clone();
                    let handle = thread::spawn(move || {
                        (case, ai_cloned.init_tree(&new_board, ai_cloned.depth))
                    });
                    handles.push(handle);
                }
                Err(e) => return Err(format!("Error: {}", e)),
            }
        }

        // Collect results from all threads and find the best move
        for handle in handles {
            match handle.join() {
                Ok((pos, score)) => {
                    if score > best_action.score {
                        best_action = Action { pos, score };
                    }
                }
                Err(_) => {
                    return Err("Thread join error".to_string());
                }
            }
        }

        // Execute the best move on the board and return the action history
        match board.try_play_move(best_action.pos.0, best_action.pos.1, self.get_color()) {
            Ok(gained_discs) => Ok(HistoryAction {
                coordinates: Some(Board::coordinates_to_input(
                    best_action.pos.0,
                    best_action.pos.1,
                )),
                gained_discs: Some(gained_discs),
                color: self.get_color(),
                player_turn: board.get_player_turn(),
                move_number: board.get_turn_number(),
            }),
            Err(e) => Err(format!("Error playing move: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::heuristic::HeuristicType;

    /// Helper function to create a default AIAlphaBeta instance for testing
    fn create_test_ai() -> AIAlphaBeta {
        AIAlphaBeta::new(
            4,
            HeuristicType::Absolute,
            Cell::Black,
            AIHeuristicMatrix::A,
        )
    }

    #[test]
    fn test_new_ai_creation() {
        let ai = create_test_ai();
        assert_eq!(ai.get_depth(), 4);
        assert_eq!(ai.get_color(), Cell::Black);
        assert_eq!(ai.get_heuristic(), HeuristicType::Absolute);
    }

    #[test]
    fn test_constructor_with_different_parameters() {
        // Test with various parameter combinations
        let ai1 = AIAlphaBeta::new(
            1,
            HeuristicType::Absolute,
            Cell::White,
            AIHeuristicMatrix::B,
        );
        assert_eq!(ai1.get_depth(), 1);
        assert_eq!(ai1.get_color(), Cell::White);
        assert_eq!(ai1.get_heuristic(), HeuristicType::Absolute);
        assert!(matches!(ai1.get_heuristic_matrix(), AIHeuristicMatrix::B));

        let ai2 = AIAlphaBeta::new(
            10,
            HeuristicType::Mobility,
            Cell::Black,
            AIHeuristicMatrix::A,
        );
        assert_eq!(ai2.get_depth(), 10);
        assert_eq!(ai2.get_color(), Cell::Black);
        assert_eq!(ai2.get_heuristic(), HeuristicType::Mobility);
        assert!(matches!(ai2.get_heuristic_matrix(), AIHeuristicMatrix::A));
    }

    #[test]
    fn test_depth_edge_cases() {
        let mut ai = create_test_ai();

        // Test setting depth to 0
        ai.set_depth(0);
        assert_eq!(ai.get_depth(), 0);

        // Test setting very high depth
        ai.set_depth(1000);
        assert_eq!(ai.get_depth(), 1000);
    }

    #[test]
    fn test_all_heuristic_types() {
        let mut ai = create_test_ai();

        // Test all available heuristic types
        ai.set_heuristic(HeuristicType::Absolute);
        assert_eq!(ai.get_heuristic(), HeuristicType::Absolute);

        ai.set_heuristic(HeuristicType::Matrix);
        assert_eq!(ai.get_heuristic(), HeuristicType::Matrix);

        ai.set_heuristic(HeuristicType::Mobility);
        assert_eq!(ai.get_heuristic(), HeuristicType::Mobility);

        ai.set_heuristic(HeuristicType::Mixte);
        assert_eq!(ai.get_heuristic(), HeuristicType::Mixte);

        ai.set_heuristic(HeuristicType::Global);
        assert_eq!(ai.get_heuristic(), HeuristicType::Global);
    }

    #[test]
    fn test_multiple_property_changes() {
        let mut ai = create_test_ai();

        // Change multiple properties and verify they persist
        ai.set_depth(7);
        ai.set_heuristic(HeuristicType::Matrix);
        ai.set_heuristic_matrix(AIHeuristicMatrix::B);

        assert_eq!(ai.get_depth(), 7);
        assert_eq!(ai.get_heuristic(), HeuristicType::Matrix);
        assert!(matches!(ai.get_heuristic_matrix(), AIHeuristicMatrix::B));

        // Original properties should remain unchanged
        assert_eq!(ai.get_color(), Cell::Black);
        assert!(!ai.is_human());
        assert_eq!(ai.get_ai_type(), Some(AIType::AlphaBeta));
    }
}
