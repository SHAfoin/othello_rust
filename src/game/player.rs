//! Player trait definition for Othello game.
//!
//! This module defines the `Player` trait that provides a common interface
//! for all player types in the Othello game, including human players and
//! various AI implementations. The trait defines the core behaviors and
//! configuration options that all players must support.

use crate::{
    ai::{ai_type::AIType, heuristic::HeuristicType, heuristic_matrix::AIHeuristicMatrix},
    game::{board::Board, history_action::HistoryAction},
};

/// Common interface for all player types in the Othello game.
///
/// This trait defines the essential behaviors that all players (human and AI)
/// must implement, as well as optional configuration methods for AI players.
/// It provides a unified interface for game management regardless of player type.
///
/// # Player Types
///
/// - **Human Players**: Interactive players that receive input from users
/// - **AI Players**: Automated players using various algorithms (MinMax, Alpha-Beta, Q-Learning)
///
/// # Core Functionality
///
/// All players must be able to:
/// - Execute a turn and return the resulting action
/// - Indicate whether they are human or AI controlled
///
/// # AI Configuration
///
/// AI players can optionally support:
/// - Algorithm type selection
/// - Search depth configuration
/// - Heuristic function selection
/// - Performance optimization settings
///
/// # Examples
///
/// ```rust
/// // Example of using a player (implementation-specific)
/// let mut board = Board::new();
/// let result = player.play_turn(&mut board, Some((2, 3)));
/// match result {
///     Ok(action) => println!("Move played: {:?}", action),
///     Err(e) => println!("Invalid move: {}", e),
/// }
/// ```
pub trait Player {
    /// Executes a player's turn on the game board.
    ///
    /// This is the core method that all players must implement. It processes
    /// the player's move (either from user input for humans or AI calculation)
    /// and applies it to the board, returning the resulting action.
    ///
    /// # Arguments
    ///
    /// * `board` - Mutable reference to the game board to play on
    /// * `cell` - Optional coordinates (row, col) for the desired move.
    ///   - For human players: represents user's selected position
    ///   - For AI players: typically None (AI calculates its own move)
    ///
    /// # Returns
    ///
    /// * `Ok(HistoryAction)` - Successfully executed move with details
    /// * `Err(String)` - Error message if move is invalid or failed
    ///
    /// # Behavior by Player Type
    ///
    /// - **Human**: Validates and applies the provided cell coordinates
    /// - **AI**: Ignores cell parameter, calculates optimal move using algorithm
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Human player making a move
    /// let result = human_player.play_turn(&mut board, Some((2, 3)));
    ///
    /// // AI player calculating its own move
    /// let result = ai_player.play_turn(&mut board, None);
    /// ```
    fn play_turn(
        &self,
        board: &mut Board,
        cell: Option<(usize, usize)>,
    ) -> Result<HistoryAction, String>;

    /// Imports a Q-table from file for Q-Learning AI players.
    ///
    /// This method allows Q-Learning AI players to load previously trained
    /// Q-tables from files, enabling them to continue learning from past
    /// training sessions or use pre-trained strategies.
    ///
    /// # Arguments
    ///
    /// * `_q_table` - File path or serialized Q-table data to import
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Q-table successfully imported
    /// * `Err(String)` - Import failed or not supported for this player type
    ///
    /// # Default Implementation
    ///
    /// Returns an error indicating Q-table import is not supported.
    /// Only Q-Learning implementations should override this method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Only works for Q-Learning players
    /// match qlearning_player.import_q_table_file("qtable.json") {
    ///     Ok(()) => println!("Q-table loaded successfully"),
    ///     Err(e) => println!("Failed to load Q-table: {}", e),
    /// }
    /// ```
    fn import_q_table_file(&mut self, _q_table: &str) -> Result<(), String> {
        Err("Importing Q-table is not supported for this player type".to_string())
    }

    /// Indicates whether this player requires human input.
    ///
    /// This method distinguishes between human players (requiring user input)
    /// and AI players (autonomous). It's used by the game engine to determine
    /// how to handle turn processing and user interface updates.
    ///
    /// # Returns
    ///
    /// * `true` - Player requires human input and interaction
    /// * `false` - Player is AI-controlled and autonomous
    ///
    /// # Usage
    ///
    /// The game engine uses this to:
    /// - Show/hide input prompts
    /// - Enable/disable move selection UI
    /// - Control turn timing and flow
    ///
    /// # Examples
    ///
    /// ```rust
    /// if player.is_human() {
    ///     // Show move selection interface
    ///     display_move_options(&board);
    /// } else {
    ///     // Let AI calculate automatically
    ///     ai_thinking_indicator();
    /// }
    /// ```
    fn is_human(&self) -> bool;

    /// Gets the AI algorithm type used by this player.
    ///
    /// This method returns the specific AI algorithm implementation used
    /// by the player. It's primarily used for configuration management,
    /// UI display, and algorithm-specific optimizations.
    ///
    /// # Returns
    ///
    /// * `Some(AIType)` - The AI algorithm type (MinMax, AlphaBeta, QLearning, etc.)
    /// * `None` - For human players or when algorithm type is not applicable
    ///
    /// # AI Algorithm Types
    ///
    /// - `AIType::MinMax` - Classic minimax algorithm
    /// - `AIType::AlphaBeta` - Alpha-beta pruning optimization
    /// - `AIType::QLearning` - Reinforcement learning approach
    ///
    /// # Examples
    ///
    /// ```rust
    /// match player.get_ai_type() {
    ///     Some(AIType::AlphaBeta) => println!("Using Alpha-Beta pruning"),
    ///     Some(AIType::QLearning) => println!("Using Q-Learning"),
    ///     None => println!("Human player or unknown AI type"),
    /// }
    /// ```
    fn get_ai_type(&self) -> Option<AIType> {
        None
    }

    /// Gets the multi-threading configuration for AI players.
    ///
    /// This method indicates whether the AI player uses multiple threads
    /// to accelerate move calculation. Multi-threading can significantly
    /// improve performance for complex algorithms like Alpha-Beta pruning.
    ///
    /// # Returns
    ///
    /// * `true` - AI uses multiple threads for parallel processing
    /// * `false` - AI uses single-threaded calculation (default)
    ///
    /// # Performance Impact
    ///
    /// - **Enabled**: Faster calculation, higher CPU usage
    /// - **Disabled**: Slower calculation, lower CPU usage, more predictable timing
    ///
    /// # Examples
    ///
    /// ```rust
    /// if player.get_double_threading() {
    ///     println!("AI is using multi-threading for faster calculations");
    /// }
    /// ```
    fn get_double_threading(&self) -> bool {
        false
    }

    /// Sets the multi-threading configuration for AI players.
    ///
    /// This method enables or disables multi-threading for AI move calculation.
    /// The setting affects performance but may also impact move determinism
    /// and resource usage.
    ///
    /// # Arguments
    ///
    /// * `_double_threading` - Whether to enable multi-threading
    ///   - `true`: Enable parallel processing
    ///   - `false`: Use single-threaded calculation
    ///
    /// # Default Implementation
    ///
    /// Does nothing. AI implementations should override this method
    /// if they support configurable threading.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Enable multi-threading for faster AI
    /// ai_player.set_double_threading(true);
    /// ```
    fn set_double_threading(&mut self, _double_threading: bool) {}

    /// Gets the heuristic matrix used for position evaluation.
    ///
    /// This method returns the current heuristic matrix that the AI uses
    /// to evaluate board positions. Different matrices emphasize different
    /// strategic aspects like corner control, edge play, or mobility.
    ///
    /// # Returns
    ///
    /// The current `AIHeuristicMatrix` variant being used for evaluation.
    /// Default implementation returns `AIHeuristicMatrix::A`.
    ///
    /// # Matrix Types
    ///
    /// Different matrices provide different strategic approaches:
    /// - Corner-focused matrices prioritize corner capture
    /// - Mobility matrices emphasize available moves
    /// - Stability matrices focus on disc permanence
    ///
    /// # Examples
    ///
    /// ```rust
    /// let current_matrix = ai_player.get_heuristic_matrix();
    /// println!("AI is using matrix: {:?}", current_matrix);
    /// ```
    fn get_heuristic_matrix(&self) -> AIHeuristicMatrix {
        AIHeuristicMatrix::A
    }

    /// Sets the heuristic matrix for position evaluation.
    ///
    /// This method allows changing the strategic focus of the AI by
    /// selecting different heuristic matrices. Each matrix represents
    /// a different approach to evaluating board positions.
    ///
    /// # Arguments
    ///
    /// * `_matrix` - The heuristic matrix to use for position evaluation
    ///
    /// # Strategic Impact
    ///
    /// Different matrices can significantly change AI behavior:
    /// - Aggressive matrices: Focus on quick wins
    /// - Defensive matrices: Prioritize position safety
    /// - Balanced matrices: Combine multiple factors
    ///
    /// # Default Implementation
    ///
    /// Does nothing. AI implementations should override this method
    /// if they support configurable heuristic matrices.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Switch to a corner-focused strategy
    /// ai_player.set_heuristic_matrix(AIHeuristicMatrix::CornerFocus);
    /// ```
    fn set_heuristic_matrix(&mut self, _matrix: AIHeuristicMatrix) {}

    /// Gets the current heuristic evaluation function type.
    ///
    /// This method returns the type of heuristic function used to evaluate
    /// board positions. Different heuristic types use different algorithms
    /// and criteria for position assessment.
    ///
    /// # Returns
    ///
    /// The current `HeuristicType` being used for position evaluation.
    /// Default implementation returns `HeuristicType::Absolute`.
    ///
    /// # Heuristic Types
    ///
    /// - `Absolute`: Direct position value calculation
    /// - `Relative`: Comparative position assessment
    /// - `Weighted`: Multi-factor weighted evaluation
    ///
    /// # Examples
    ///
    /// ```rust
    /// let heuristic_type = ai_player.get_heuristic();
    /// println!("Using heuristic: {:?}", heuristic_type);
    /// ```
    fn get_heuristic(&self) -> HeuristicType {
        HeuristicType::Absolute
    }

    /// Sets the heuristic evaluation function type.
    ///
    /// This method allows changing the evaluation algorithm used by the AI
    /// to assess board positions. Different heuristic types can lead to
    /// different playing styles and strategic preferences.
    ///
    /// # Arguments
    ///
    /// * `_heuristic` - The heuristic evaluation type to use
    ///
    /// # Evaluation Impact
    ///
    /// Different heuristics affect how the AI:
    /// - Weighs position factors
    /// - Balances short vs long-term goals
    /// - Handles risk assessment
    ///
    /// # Default Implementation
    ///
    /// Does nothing. AI implementations should override this method
    /// if they support configurable heuristic functions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Use weighted multi-factor evaluation
    /// ai_player.set_heuristic(HeuristicType::Weighted);
    /// ```
    fn set_heuristic(&mut self, _heuristic: HeuristicType) {}

    /// Gets the current search depth for tree-based AI algorithms.
    ///
    /// This method returns the maximum depth that tree-search algorithms
    /// (like MinMax and Alpha-Beta) will explore when calculating moves.
    /// Higher depths provide stronger play but require more computation time.
    ///
    /// # Returns
    ///
    /// The current search depth as a positive integer.
    /// Default implementation returns 1 (single-level lookahead).
    ///
    /// # Depth vs Performance
    ///
    /// - **Depth 1-3**: Fast calculation, basic strategy
    /// - **Depth 4-6**: Good balance of speed and strength
    /// - **Depth 7+**: Strong play, slower calculation
    ///
    /// # Algorithm Support
    ///
    /// - **MinMax/Alpha-Beta**: Direct depth control
    /// - **Q-Learning**: May use depth for simulation
    /// - **Human**: Not applicable (returns default)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let current_depth = ai_player.get_depth();
    /// println!("AI search depth: {} moves ahead", current_depth);
    /// ```
    fn get_depth(&self) -> usize {
        1
    }

    /// Sets the search depth for tree-based AI algorithms.
    ///
    /// This method configures how many moves ahead the AI will analyze
    /// when calculating its next move. Higher depths provide stronger
    /// play at the cost of increased computation time.
    ///
    /// # Arguments
    ///
    /// * `_depth` - The maximum search depth to use (positive integer)
    ///
    /// # Performance Considerations
    ///
    /// Increasing depth has exponential impact on calculation time:
    /// - Each additional depth level multiplies calculation time
    /// - Consider hardware capabilities and time constraints
    /// - Balance playing strength with responsiveness
    ///
    /// # Recommended Values
    ///
    /// - **Beginner AI**: 1-2 (fast, educational)
    /// - **Intermediate AI**: 3-4 (balanced performance)
    /// - **Advanced AI**: 5-7 (strong, slower)
    /// - **Expert AI**: 8+ (tournament strength, very slow)
    ///
    /// # Default Implementation
    ///
    /// Does nothing. AI implementations should override this method
    /// if they support configurable search depth.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Set AI to look 5 moves ahead
    /// ai_player.set_depth(5);
    /// ```
    fn set_depth(&mut self, _depth: usize) {}
}
