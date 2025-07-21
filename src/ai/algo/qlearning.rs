//! Q-Learning reinforcement learning AI implementation for Othello game.
//!
//! This module implements Q-Learning, a model-free reinforcement learning algorithm
//! that learns to play Othello by exploring different actions and learning from
//! rewards. The AI maintains a Q-table that maps board states to action values,
//! gradually improving its play through experience.

use rand::{rng, Rng};
use std::{collections::HashMap, fs::File, io::Write, sync::mpsc};

use crate::{
    ai::{ai_type::AIType, heuristic::HeuristicType, heuristic_matrix::AIHeuristicMatrix},
    consts::{EPSILON, GAMMA, LAMBDA_LEARN},
    game::{board::Board, cell::Cell, history_action::HistoryAction, player::Player},
};

/// An AI player that uses Q-Learning to learn optimal moves in Othello.
///
/// Q-Learning is a reinforcement learning algorithm that learns to play by
/// exploring different actions and learning from rewards. The AI maintains
/// a Q-table that maps board states to action values, using an epsilon-greedy
/// strategy to balance exploration and exploitation.
///
/// The AI learns through self-play over multiple epochs, gradually improving
/// its understanding of good moves through trial and error.
///
/// # Examples
///
/// ```rust
/// // Create a new Q-Learning AI
/// let mut ai = QLearning::new(
///     1000,  // Maximum steps per game
///     HeuristicType::default(),
///     AIHeuristicMatrix::default(),
///     500,   // Number of training epochs
///     Cell::Black,
/// );
///
/// // Train the AI
/// ai.try_q_learning();
///
/// // The AI can now play using its learned Q-table
/// ```
pub struct QLearning {
    /// Maximum number of steps per training game
    max_step: usize,
    /// Q-table mapping board states to action values
    q_table: HashMap<String, HashMap<String, isize>>,
    /// Heuristic function used for reward calculation
    heuristic: HeuristicType,
    /// Matrix used by the heuristic function
    matrix: AIHeuristicMatrix,
    /// Number of training epochs
    epoch: usize,
    /// Epsilon value for epsilon-greedy exploration
    epsilon: f64,
    /// The color (Black or White) that this AI represents
    color: Cell,
}

impl QLearning {
    /// Creates a new Q-Learning AI with specified parameters.
    ///
    /// This constructor initializes a Q-Learning AI that will learn to play
    /// Othello through reinforcement learning. The AI starts with an empty
    /// Q-table and learns through self-play over multiple epochs.
    ///
    /// # Arguments
    ///
    /// * `max_step` - Maximum number of steps per training game
    /// * `heuristic` - Heuristic function used for reward calculation
    /// * `matrix` - Matrix used by the heuristic function
    /// * `epoch` - Number of training epochs to perform
    /// * `color` - The color (Black or White) that this AI represents
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut ai = QLearning::new(
    ///     2000,  // Maximum 2000 steps per game
    ///     HeuristicType::default(),
    ///     AIHeuristicMatrix::default(),
    ///     1000,  // Train for 1000 epochs
    ///     Cell::Black,
    /// );
    /// ```
    pub fn new(
        max_step: usize,
        heuristic: HeuristicType,
        matrix: AIHeuristicMatrix,
        epoch: usize,
        color: Cell,
    ) -> Self {
        Self {
            max_step: max_step,
            q_table: HashMap::new(),
            heuristic,
            matrix,
            epoch: epoch,
            epsilon: EPSILON,
            color: color,
        }
    }

    /// Returns the number of training epochs.
    ///
    /// This indicates how many complete games the AI will play during training
    /// to learn the Q-table values.
    pub fn get_epochs(&self) -> usize {
        self.epoch
    }

    /// Updates the number of training epochs.
    ///
    /// # Arguments
    ///
    /// * `epoch` - New number of training epochs
    pub fn set_epochs(&mut self, epoch: usize) {
        self.epoch = epoch;
    }

    /// Returns the maximum number of steps per training game.
    ///
    /// This prevents infinite games during training by limiting the maximum
    /// number of moves that can be played in a single game.
    pub fn get_max_step(&self) -> usize {
        self.max_step
    }

    /// Updates the maximum number of steps per training game.
    ///
    /// # Arguments
    ///
    /// * `max_step` - New maximum number of steps per game
    pub fn set_max_step(&mut self, max_step: usize) {
        self.max_step = max_step;
    }

    /// Returns the current heuristic function type.
    ///
    /// The heuristic function is used to calculate rewards during training,
    /// helping the AI learn which board positions are favorable.
    pub fn get_heuristic(&self) -> HeuristicType {
        self.heuristic.clone()
    }

    /// Updates the heuristic function used for reward calculation.
    ///
    /// # Arguments
    ///
    /// * `heuristic` - New heuristic function to use
    pub fn set_heuristic(&mut self, heuristic: HeuristicType) {
        self.heuristic = heuristic;
    }

    /// Returns a copy of the current heuristic matrix.
    ///
    /// The matrix is used by the heuristic function to evaluate board positions.
    pub fn get_matrix(&self) -> AIHeuristicMatrix {
        self.matrix.clone()
    }

    /// Updates the heuristic matrix used for evaluation.
    ///
    /// # Arguments
    ///
    /// * `matrix` - New heuristic matrix to use
    pub fn set_matrix(&mut self, matrix: AIHeuristicMatrix) {
        self.matrix = matrix;
    }

    /// Returns a reference to the current Q-table.
    ///
    /// The Q-table maps board states (as strings) to action values,
    /// representing the AI's learned knowledge about move quality.
    pub fn get_q_table(&self) -> &HashMap<String, HashMap<String, isize>> {
        &self.q_table
    }

    /// Updates or inserts a Q-value for a specific state-action pair.
    ///
    /// This method is used during training to update the AI's knowledge
    /// about the value of taking specific actions in specific states.
    ///
    /// # Arguments
    ///
    /// * `state` - String representation of the board state
    /// * `action` - Tuple containing the action string and its Q-value
    pub fn set_q_table(&mut self, state: String, action: (String, isize)) {
        self.q_table
            .entry(state)
            .or_insert_with(HashMap::new)
            .insert(action.0, action.1);
    }

    /// Returns the current epsilon value for epsilon-greedy exploration.
    ///
    /// Epsilon determines the probability of taking a random action instead
    /// of the best known action, balancing exploration and exploitation.
    pub fn get_epsilon(&self) -> f64 {
        self.epsilon
    }

    /// Updates the epsilon value for exploration.
    ///
    /// # Arguments
    ///
    /// * `epsilon` - New epsilon value (typically between 0.0 and 1.0)
    pub fn set_epsilon(&mut self, epsilon: f64) {
        self.epsilon = epsilon;
    }

    /// Performs one complete Q-learning training game.
    ///
    /// This method plays a single game using the current Q-table and epsilon-greedy
    /// strategy, updating Q-values based on the rewards received. The AI learns
    /// by trying different actions and updating its Q-table based on the outcomes.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// * `isize` - Total reward accumulated during the game
    /// * `bool` - Whether the game ended naturally (true) or hit max steps (false)
    ///
    /// # Algorithm Details
    ///
    /// - Uses epsilon-greedy action selection (random vs best known action)
    /// - Updates Q-values using the Q-learning formula: Q[s,a] = (1-λ)*Q[s,a] + λ*(r + γ*max(Q[s',a']))
    /// - Continues until game over or maximum steps reached
    /// - Applies win/loss bonuses to final rewards
    pub fn q_learning(&mut self) -> (isize, bool) {
        // Initialize game state
        let mut board = Board::new();
        let mut step = 0;
        let mut s = board.to_hash();
        let mut action: (usize, usize);
        let mut total_r = 0;

        // Continue until max steps reached or game over
        while step < self.max_step && !board.check_game_over() {
            step += 1;
            // Choose action using epsilon-greedy strategy
            if let Some(actions) = board.has_legal_moves(board.get_player_turn()) {
                if rng().random::<f64>() < self.epsilon || self.get_q_table().get(&s).is_none() {
                    // Choose random action for exploration
                    action = actions[rng().random_range(0..actions.len())];
                } else {
                    // Choose best known action for exploitation
                    let mut best_action = None;
                    let mut best_value = isize::MIN;
                    if let Some(q_values) = self.get_q_table().get(&s) {
                        for (action, value) in q_values {
                            if *value > best_value {
                                best_value = *value;
                                best_action = Some(action);
                            }
                        }
                    }
                    action = Board::input_to_coordinates(best_action.unwrap().as_str()).unwrap();
                }

                // Execute the chosen action
                board
                    .try_play_move(action.0, action.1, board.get_player_turn())
                    .unwrap();

                // Calculate reward using heuristic function
                let mut r =
                    self.heuristic
                        .evaluate(&board, board.get_player_turn(), self.matrix.clone());

                // Add win/loss bonus if game is over
                if board.check_game_over() {
                    let winner = board.get_winner();

                    if let Some(w) = winner {
                        if w == board.get_player_turn() {
                            r += 1000; // Win bonus
                        } else if w == board.get_player_turn().get_opponent() {
                            r -= 1000; // Loss penalty
                        }
                    } else {
                        r += 0; // Draw - no bonus
                    }
                }

                // Update Q-value using Q-learning formula
                // Q[s, a] = (1-λ)*Q[s, a] + λ*(r + γ * max(Q[new_state, :]))
                let new_s = board.to_hash();

                let q_value = self
                    .get_q_table()
                    .get(&s)
                    .and_then(|q_values| {
                        q_values.get(Board::coordinates_to_input(action.0, action.1).as_str())
                    })
                    .cloned()
                    .unwrap_or(0);
                let new_q_value = (1.0 - LAMBDA_LEARN) * q_value as f64
                    + LAMBDA_LEARN
                        * (r as f64
                            + GAMMA
                                * self
                                    .get_q_table()
                                    .get(&new_s)
                                    .and_then(|q_values| q_values.values().cloned().max())
                                    .unwrap_or(0) as f64);

                self.set_q_table(
                    s.clone(),
                    (
                        Board::coordinates_to_input(action.0, action.1),
                        new_q_value as isize,
                    ),
                );

                // Update state and total reward, advance to next turn
                s = new_s;
                total_r += r;
                board.next_turn();
            }
        }

        (total_r, board.check_game_over())
    }

    /// Trains the AI through multiple epochs of Q-learning.
    ///
    /// This method performs the complete training process by playing multiple
    /// games and learning from each one. The epsilon value is gradually decreased
    /// to reduce exploration over time, allowing the AI to exploit its learned
    /// knowledge more as training progresses.
    ///
    /// After training, the Q-table is exported to a file for future use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut ai = QLearning::new(1000, HeuristicType::default(), AIHeuristicMatrix::default(), 500, Cell::Black);
    /// ai.try_q_learning();  // Train for 500 epochs
    /// ```
    pub fn try_q_learning(&mut self, tx: mpsc::Sender<f64>) {
        for i in 0..self.epoch {
            // Play one training game and learn from it
            let (_total_r, _done) = self.q_learning();
            // Gradually reduce epsilon to favor exploitation over exploration
            self.set_epsilon(self.get_epsilon() * 0.999);

            // Send progress update to the channel
            if let Err(e) = tx.send(i as f64 / self.epoch as f64) {
                eprintln!("Error sending training progress: {}", e);
                break;
            }
        }
        // Export the learned Q-table for future use
        self.export_q_table("q_table.json");
    }

    /// Imports a Q-table from a JSON file.
    ///
    /// This allows loading a previously trained Q-table, enabling the AI
    /// to use pre-learned knowledge without retraining.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the JSON file containing the Q-table
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the Q-table was successfully loaded
    /// * `Err(String)` - If there was an error loading or parsing the file
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut ai = QLearning::new(1000, HeuristicType::default(), AIHeuristicMatrix::default(), 0, Cell::Black);
    /// ai.import_q_table("trained_q_table.json").expect("Failed to load Q-table");
    /// ```
    pub fn import_q_table(&mut self, file_path: &str) -> Result<(), String> {
        match File::open(file_path) {
            Ok(file) => match serde_json::from_reader(file) {
                Ok(q_table) => {
                    self.q_table = q_table;
                    Ok(())
                }
                Err(e) => Err(format!("Could not deserialize Q-table: {}", e)),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    /// Returns the color (Black or White) that this AI player represents.
    ///
    /// This is useful for determining which player this AI instance is playing as.
    pub fn get_color(&self) -> Cell {
        self.color
    }

    /// Exports the current Q-table to a JSON file.
    ///
    /// This saves the AI's learned knowledge to a file, allowing it to be
    /// loaded later without retraining.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path where the Q-table should be saved
    ///
    /// # Examples
    ///
    /// ```rust
    /// let ai = QLearning::new(1000, HeuristicType::default(), AIHeuristicMatrix::default(), 0, Cell::Black);
    /// ai.export_q_table("my_q_table.json");
    /// ```
    pub fn export_q_table(&self, file_path: &str) {
        let json =
            serde_json::to_string_pretty(&self.q_table).expect("Could not serialize Q-table");
        let mut file = File::create(file_path).expect("Could not create file");
        file.write_all(json.as_bytes())
            .expect("Could not write to file");
    }
}

impl Player for QLearning {
    /// Returns false as this is an AI player, not a human player.
    ///
    /// This method is used by the game engine to determine whether to wait for
    /// human input or to automatically trigger AI move calculation.
    fn is_human(&self) -> bool {
        false
    }

    /// Imports a Q-table from a file for use by this AI player.
    ///
    /// This method is called by the game engine to load a pre-trained Q-table,
    /// allowing the AI to use previously learned knowledge.
    ///
    /// # Arguments
    ///
    /// * `q_table` - Path to the Q-table file to import
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the Q-table was successfully loaded
    /// * `Err(String)` - If there was an error loading the file
    fn import_q_table_file(&mut self, q_table: &str) -> Result<(), String> {
        match self.import_q_table(q_table) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to import Q-table {}: {}", q_table, &e)),
        }
    }

    /// Returns the AI type identifier for this player.
    ///
    /// This method allows the game engine to identify that this is a Q-Learning AI,
    /// which can be useful for UI display or statistics.
    fn get_ai_type(&self) -> Option<AIType> {
        Some(AIType::QLearning)
    }

    /// Returns a copy of the current heuristic matrix.
    ///
    /// The heuristic matrix is used by the evaluation function during
    /// training to calculate rewards.
    fn get_heuristic_matrix(&self) -> AIHeuristicMatrix {
        self.matrix.clone()
    }

    /// Updates the heuristic matrix used for reward calculation.
    ///
    /// # Arguments
    ///
    /// * `matrix` - The new heuristic matrix to use
    fn set_heuristic_matrix(&mut self, matrix: AIHeuristicMatrix) {
        self.matrix = matrix;
    }

    /// Returns the current heuristic function type.
    ///
    /// This identifies which heuristic algorithm is being used to calculate
    /// rewards during training.
    fn get_heuristic(&self) -> HeuristicType {
        self.heuristic.clone()
    }

    /// Updates the heuristic function used for reward calculation.
    ///
    /// # Arguments
    ///
    /// * `heuristic` - The new heuristic function to use
    fn set_heuristic(&mut self, heuristic: HeuristicType) {
        self.heuristic = heuristic;
    }

    /// Calculates and executes the best move based on the learned Q-table.
    ///
    /// This method uses the AI's learned Q-table to select the best move for
    /// the current board position. If no Q-values exist for the current state,
    /// it falls back to selecting a random legal move.
    ///
    /// # Arguments
    ///
    /// * `board` - Mutable reference to the current game board
    /// * `_cell` - Optional cell parameter (unused in this implementation)
    ///
    /// # Returns
    ///
    /// * `Ok(HistoryAction)` - Details of the move that was played
    /// * `Err(String)` - Error message if move execution fails
    ///
    /// # Strategy
    ///
    /// 1. Look up the current board state in the Q-table
    /// 2. If Q-values exist, select the action with the highest Q-value
    /// 3. If no Q-values exist, select a random legal move
    /// 4. Execute the selected move on the board
    fn play_turn(
        &self,
        board: &mut Board,
        _cell: Option<(usize, usize)>,
    ) -> Result<HistoryAction, String> {
        let actions = board.has_legal_moves(board.get_player_turn()).unwrap();

        // Select the best action based on Q-table values
        let mut best_action = None;
        let mut best_value = isize::MIN;

        if let Some(q_values) = self.get_q_table().get(&board.to_hash()) {
            // Find the action with the highest Q-value
            for (action, value) in q_values {
                if *value > best_value {
                    best_value = *value;
                    best_action = Some(action.clone());
                }
            }
        } else {
            // No Q-values available, choose a random action
            let random_index = rng().random_range(0..actions.len());
            let random_action =
                Board::coordinates_to_input(actions[random_index].0, actions[random_index].1);
            best_action = Some(random_action);
        }

        // Convert action string to coordinates and execute the move
        let action_coords = Board::input_to_coordinates(best_action.unwrap().as_str()).unwrap();

        match board.try_play_move(action_coords.0, action_coords.1, board.get_player_turn()) {
            Ok(gained_discs) => Ok(HistoryAction {
                coordinates: Some(Board::coordinates_to_input(
                    action_coords.0,
                    action_coords.1,
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
