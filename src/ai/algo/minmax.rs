//! Minimax algorithm AI implementation for Othello game.
//!
//! This module implements the minimax algorithm, a classic game tree search algorithm
//! that assumes both players play optimally. The algorithm alternates between maximizing
//! and minimizing players to find the best possible move for the current player.

use std::{thread, vec};

use crate::{
    ai::{
        action::Action, ai_type::AIType, heuristic::HeuristicType,
        heuristic_matrix::AIHeuristicMatrix,
    },
    consts::MAX_DEPTH,
    game::{board::Board, cell::Cell, history_action::HistoryAction, player::Player},
};

/// An AI player that uses the minimax algorithm to make optimal moves in Othello.
///
/// The Minimax algorithm is a decision-making algorithm that evaluates all possible
/// moves in a game tree, assuming both players play optimally. It alternates between
/// maximizing the score for the current player and minimizing it for the opponent.
///
/// This implementation includes optional multithreading support for improved performance
/// on multi-core systems when evaluating moves at the maximum depth.
///
/// # Examples
///
/// ```rust
/// // Create a new minimax AI with depth 5 and threading enabled
/// let ai = AIMinMax::new(
///     5,
///     HeuristicType::default(),
///     Cell::Black,
///     AIHeuristicMatrix::default(),
///     true,
/// );
///
/// // Get the AI's color
/// assert_eq!(ai.get_color(), Cell::Black);
/// ```
#[derive(Clone, Debug)]
pub struct AIMinMax {
    /// Maximum search depth for the minimax algorithm
    depth: usize,
    /// Heuristic function used to evaluate board positions
    heuristic: HeuristicType,
    /// The color (Black or White) that this AI player represents
    color: Cell,
    /// Matrix used by the heuristic function for position evaluation
    matrix: AIHeuristicMatrix,
    /// Whether to use multithreading for move evaluation
    double_threading: bool,
}

impl AIMinMax {
    /// Creates a new minimax AI player with specified parameters.
    ///
    /// This constructor initializes an AI player that will use the minimax
    /// algorithm to determine the best moves. The AI's strength depends on
    /// the depth of search and the heuristic function used.
    ///
    /// # Arguments
    ///
    /// * `depth` - Maximum search depth for the minimax algorithm. Higher values
    ///   result in stronger play but exponentially slower move calculation.
    /// * `heuristic` - The heuristic function used to evaluate board positions
    /// * `color` - The color (Black or White) that this AI will play as
    /// * `matrix` - Heuristic matrix used for position evaluation
    /// * `double_threading` - Whether to enable multithreading for move evaluation
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Create a strong AI with threading enabled
    /// let ai = AIMinMax::new(
    ///     6,  // Search depth of 6 moves ahead
    ///     HeuristicType::default(),
    ///     Cell::White,
    ///     AIHeuristicMatrix::default(),
    ///     true,  // Enable multithreading
    /// );
    /// ```
    pub fn new(
        depth: usize,
        heuristic: HeuristicType,
        color: Cell,
        matrix: AIHeuristicMatrix,
        double_threading: bool,
    ) -> Self {
        Self {
            depth,
            heuristic,
            color,
            matrix,
            double_threading,
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
    /// let ai = AIMinMax::new(4, HeuristicType::default(), Cell::Black, AIHeuristicMatrix::default(), false);
    /// assert_eq!(ai.get_color(), Cell::Black);
    /// ```
    pub fn get_color(&self) -> Cell {
        self.color
    }

    /// Initializes the minimax search tree from the root position.
    ///
    /// This method starts the minimax algorithm from the current board state,
    /// evaluating all possible moves to the specified depth to find the best
    /// move's evaluation score.
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
    /// let ai = AIMinMax::new(4, HeuristicType::default(), Cell::Black, AIHeuristicMatrix::default(), false);
    /// let board = Board::new();
    /// let score = ai.init_tree(&board, 4);
    /// // Score represents the minimax evaluation at depth 4
    /// ```
    pub fn init_tree(&self, board: &Board, depth: usize) -> isize {
        self.tree_step(board, depth)
    }

    /// Performs one step of the minimax algorithm recursively.
    ///
    /// This is the core recursive function that implements the minimax algorithm.
    /// It alternates between maximizing and minimizing players based on the current
    /// depth (even depths maximize, odd depths minimize).
    ///
    /// At the maximum depth, if multithreading is enabled, it will evaluate all
    /// possible moves in parallel for improved performance.
    ///
    /// # Arguments
    ///
    /// * `board` - Current board state to evaluate
    /// * `depth` - Remaining search depth (decreases with each recursive call)
    ///
    /// # Returns
    ///
    /// The minimax evaluation score for the current position
    ///
    /// # Algorithm Details
    ///
    /// - Base case: When depth reaches 1 or no legal moves exist, return heuristic evaluation
    /// - Recursive case: Try all legal moves and find the best/worst score depending on player
    /// - Maximizing player: Seeks the highest score among all possible moves
    /// - Minimizing player: Seeks the lowest score among all possible moves
    /// - Multithreading: At maximum depth, evaluates moves in parallel if enabled
    pub fn tree_step(&self, board: &Board, depth: usize) -> isize {
        // Determine comparison function and initial best score based on player type
        let comparation_function: fn(isize, isize) -> isize;
        let mut best_score;
        if depth % 2 == 0 {
            // Maximizing player (even depth)
            comparation_function = isize::max;
            best_score = isize::MIN;
        } else {
            // Minimizing player (odd depth)
            comparation_function = isize::min;
            best_score = isize::MAX;
        }

        if depth == 1 || board.has_legal_moves(board.get_player_turn()) == None {
            // Base case: evaluate the current position using the heuristic function
            let score = self
                .heuristic
                .evaluate(board, self.get_color(), self.matrix.clone());
            return score;
        } else if depth == MAX_DEPTH && self.double_threading {
            // Use multithreading at maximum depth for improved performance
            let mut handles = vec![];
            for case in board.has_legal_moves(board.get_player_turn()).unwrap() {
                let mut new_board = board.clone();
                match new_board.try_play_move(case.0, case.1, board.get_player_turn()) {
                    Ok(_) => {
                        let ai_cloned = self.clone();
                        let handle =
                            thread::spawn(move || ai_cloned.tree_step(&new_board, depth - 1));
                        handles.push(handle);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }

            // Collect results from all threads
            for handle in handles {
                match handle.join() {
                    Ok(score) => {
                        best_score = comparation_function(best_score, score);
                    }
                    Err(_) => {
                        println!("Thread panicked");
                    }
                }
            }
            best_score
        } else {
            // Sequential evaluation of all possible moves
            for case in board.has_legal_moves(board.get_player_turn()).unwrap() {
                let mut new_board = board.clone();
                match new_board.try_play_move(case.0, case.1, board.get_player_turn()) {
                    Ok(_) => {
                        let score = self.tree_step(&new_board, depth - 1);
                        best_score = comparation_function(best_score, score);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            best_score
        }
    }
}

impl Player for AIMinMax {
    /// Returns false as this is an AI player, not a human player.
    ///
    /// This method is used by the game engine to determine whether to wait for
    /// human input or to automatically trigger AI move calculation.
    fn is_human(&self) -> bool {
        false
    }

    /// Returns whether multithreading is enabled for this AI.
    ///
    /// When enabled, the AI will use multiple threads to evaluate moves
    /// in parallel at the maximum search depth, improving performance.
    fn get_double_threading(&self) -> bool {
        self.double_threading
    }

    /// Enables or disables multithreading for move evaluation.
    ///
    /// # Arguments
    ///
    /// * `double_threading` - Whether to enable multithreading
    fn set_double_threading(&mut self, double_threading: bool) {
        self.double_threading = double_threading;
    }

    /// Returns the AI type identifier for this player.
    ///
    /// This method allows the game engine to identify which AI algorithm
    /// is being used, which can be useful for UI display or statistics.
    fn get_ai_type(&self) -> Option<AIType> {
        Some(AIType::MinMax)
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
    /// when evaluating positions. Higher depths result in stronger play
    /// but exponentially slower move calculation.
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
    /// using the minimax algorithm to determine its score, and the move
    /// with the highest score is selected and executed.
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
                Err(e) => {
                    return Err(format!("Error: {}", e));
                }
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
