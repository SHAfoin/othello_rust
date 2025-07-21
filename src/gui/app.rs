//! Application state management and core GUI logic.
//!
//! This module defines the main application structure and state management
//! for the Othello game's graphical user interface. It handles screen
//! navigation, game state coordination, player management, and user
//! interaction processing for the terminal-based interface.

use ratatui::{crossterm::event::KeyCode, widgets::ListState};

use crate::{
    ai::algo::qlearning::QLearning,
    consts::SIZE,
    game::{board::Board, cell::Cell, player::Player, timer::Timer},
};

/// Enumeration of all possible application screens.
///
/// This enum represents the different screens/states that the application
/// can display to the user. It's used for navigation control and determines
/// which UI components are rendered at any given time.
///
/// # Screen Types
///
/// - **Main**: Primary menu for selecting game modes
/// - **Game**: Active gameplay interface with board and controls
/// - **Tutorial**: Help screen with game rules and instructions
/// - **HumanVsAI**: Configuration screen for Human vs AI games
/// - **AIVsAI**: Configuration screen for AI vs AI games
/// - **QLearningParameters**: Training configuration for Q-Learning AI
/// - **Exit**: Confirmation dialog for application termination
///
/// # Navigation Flow
///
/// ```text
/// Main → HumanVsAI/AIVsAI/QLearningParameters → Game
///   ↓                                            ↓
/// Exit ←                                    Tutorial
/// ```
///
/// # Usage
///
/// The current screen is stored in `App.current_screen` and used by
/// the UI rendering system to determine which screen to display.
///
/// # Examples
///
/// ```rust
/// let mut app = App::new();
/// app.current_screen = CurrentScreen::Game;
/// // UI will now render the game screen
/// ```
pub enum CurrentScreen {
    Main,
    Game,
    Tutorial,
    HumanVsAI,
    AIVsAI,
    QLearningParameters,
    QLearningLoading,
    Exit,
}

/// Main application state container for the Othello game GUI.
///
/// The `App` struct serves as the central state manager for the entire
/// application, coordinating between the game logic, user interface,
/// player management, and various application modes. It maintains all
/// necessary state for seamless navigation and gameplay.
///
/// # Core Responsibilities
///
/// - **Screen Management**: Controls which UI screen is currently displayed
/// - **Game State**: Manages active games, boards, and game progression
/// - **Player Management**: Handles both human and AI players
/// - **User Input**: Processes keyboard input and user interactions
/// - **Message System**: Manages status messages and error notifications
/// - **Timing**: Tracks game duration and session timing
///
/// # State Components
///
/// The application state includes:
/// - Current screen and UI navigation state
/// - Optional game board for active games
/// - Player instances (human or AI implementations)
/// - User interface state (selected cells, list selections)
/// - Timing information for game sessions
/// - Configuration for AI training and parameters
///
/// # Lifecycle Management
///
/// The App manages the complete lifecycle of game sessions:
/// 1. **Initialization**: Set up default state and UI
/// 2. **Configuration**: Player setup and game mode selection
/// 3. **Gameplay**: Active game management and turn processing
/// 4. **Completion**: Game ending and result presentation
/// 5. **Cleanup**: State reset for new games
///
/// # Examples
///
/// ```rust
/// // Create new application instance
/// let mut app = App::new();
///
/// // Start a new game
/// app.start_game();
///
/// // Process user input
/// app.select_cell_key(KeyCode::Right);
/// app.gui_play_turn();
/// ```
pub struct App {
    /// Current screen being displayed to the user.
    ///
    /// Controls which UI components are rendered and determines
    /// the available user interactions and navigation options.
    pub current_screen: CurrentScreen,

    /// Current selection state for list-based UI components.
    ///
    /// Manages which item is currently selected in menus, configuration
    /// lists, and other selectable UI elements. Used by ratatui's
    /// stateful widgets for maintaining selection across redraws.
    pub current_mode: ListState,

    /// Active game board state, if a game is in progress.
    ///
    /// Contains the complete Othello game state including disc positions,
    /// move history, current player turn, and game rules enforcement.
    /// None when not in an active game session.
    pub board: Option<Board>,

    /// Current status or error message to display to the user.
    ///
    /// Used for showing game status updates, error notifications,
    /// player turn information, and other important messages.
    /// None when no message needs to be displayed.
    pub game_message: Option<String>,

    /// First player instance (typically Black player).
    ///
    /// Can be either a human player or any AI implementation.
    /// Boxed trait object allows for different player types while
    /// maintaining a uniform interface.
    pub player_1: Option<Box<dyn Player>>,

    /// Second player instance (typically White player).
    ///
    /// Can be either a human player or any AI implementation.
    /// Supports the same flexibility as player_1 for mixed human/AI games.
    pub player_2: Option<Box<dyn Player>>,

    /// Currently selected board cell coordinates for user input.
    ///
    /// Represents the cell (row, col) that the user has navigated to
    /// on the game board. Used for move input and visual highlighting.
    /// None when no cell is selected or not in game mode.
    pub selected_cell: Option<(usize, usize)>,

    /// Game session timer for tracking elapsed time.
    ///
    /// Tracks the total time spent in the current game session,
    /// useful for performance analysis and time management.
    /// None when not timing a game session.
    pub timer: Option<Timer>,

    /// Q-Learning training configuration parameters.
    ///
    /// Contains all settings for Q-Learning AI training sessions
    /// including epoch count, learning rates, and heuristic settings.
    /// None when not configuring or running Q-Learning training.
    pub qlearning_parameters: Option<QLearning>,

    pub qlearning_loading: Option<f64>,

    pub qlearning_channel: Option<std::sync::mpsc::Receiver<f64>>,
}

impl App {
    /// Creates a new application instance with default state.
    ///
    /// This constructor initializes the application with safe default values,
    /// setting up the main screen, empty game state, and no active players.
    /// The application is ready for user interaction and screen navigation.
    ///
    /// # Initial State
    ///
    /// - **Screen**: Main menu (CurrentScreen::Main)
    /// - **Selection**: First menu item selected (index 0)
    /// - **Game**: No active game board or players
    /// - **UI**: No selected cell or messages
    /// - **Timing**: No active timer
    /// - **AI**: No Q-Learning parameters configured
    ///
    /// # Return Value
    ///
    /// Returns a fully initialized App instance ready for use with the UI loop.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let app = App::new();
    /// assert_eq!(app.current_screen, CurrentScreen::Main);
    /// assert!(app.board.is_none());
    /// assert!(app.player_1.is_none());
    /// ```
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            current_mode: ListState::default().with_selected(Some(0)), // Sélectionner le premier élément par défaut
            board: None,
            game_message: None,
            player_1: None,             // Initialiser sans joueur
            player_2: None,             // Initialiser sans joueur
            selected_cell: None,        // Aucune cellule sélectionnée par défaut
            timer: None,                // Pas de timer initialement
            qlearning_parameters: None, // Pas de paramètres QLearning initialement
            qlearning_loading: None,
            qlearning_channel: None, // Pas de canal QLearning initialement
        }
    }

    /// Initializes and starts a new game session.
    ///
    /// This method transitions the application from configuration screens
    /// to active gameplay by creating a new game board, setting up the
    /// initial game state, and starting the game timer. It assumes that
    /// players have already been configured.
    ///
    /// # State Changes
    ///
    /// - **Screen**: Switches to CurrentScreen::Game
    /// - **Board**: Creates new Board with standard Othello starting position
    /// - **Message**: Displays initial turn message for the starting player
    /// - **Timer**: Starts a new Timer to track game duration
    ///
    /// # Game Initialization
    ///
    /// The new game starts with:
    /// - Standard 8x8 Othello board with initial disc placement
    /// - Black player's turn (as per Othello rules)
    /// - Fresh move history
    /// - Active game timer
    ///
    /// # Prerequisites
    ///
    /// This method should be called after players have been configured
    /// through the appropriate setup screens (HumanVsAI, AIVsAI, etc.).
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut app = App::new();
    /// // ... configure players ...
    /// app.start_game();
    /// assert_eq!(app.current_screen, CurrentScreen::Game);
    /// assert!(app.board.is_some());
    /// assert!(app.timer.is_some());
    /// ```
    pub fn start_game(&mut self) {
        self.current_screen = CurrentScreen::Game;
        self.board = Some(Board::new());
        self.game_message = Some(format!(
            "It's {} turn !",
            self.board.as_ref().unwrap().get_player_turn()
        ));
        self.timer = Some(Timer::new());
    }

    /// Processes a player's turn in the GUI context.
    ///
    /// This method handles the complete turn execution process, including
    /// player input processing, move validation, game state updates, and
    /// turn transition. It coordinates between the UI state and game logic
    /// to provide seamless gameplay experience.
    ///
    /// # Turn Processing Flow
    ///
    /// 1. **Player Identification**: Determines current player based on board state
    /// 2. **Move Execution**: Calls appropriate player's play_turn method
    /// 3. **Result Processing**: Handles success/failure of move attempt
    /// 4. **Game State Update**: Updates board state and move history
    /// 5. **End Game Check**: Checks for game completion conditions
    /// 6. **Turn Transition**: Advances to next player or ends game
    /// 7. **UI Update**: Updates messages and visual feedback
    ///
    /// # Player Coordination
    ///
    /// - **Black player**: Uses player_1 instance
    /// - **White player**: Uses player_2 instance  
    /// - **Human players**: Receives selected_cell coordinates
    /// - **AI players**: Calculates own moves (ignores selected_cell)
    ///
    /// # Error Handling
    ///
    /// Invalid moves and errors are handled gracefully:
    /// - **Invalid moves**: Display error message, no state change
    /// - **Missing players**: Show appropriate error message
    /// - **Game over**: Prevent further moves, show final status
    ///
    /// # Game Completion
    ///
    /// When game ends:
    /// - **Timer**: Stops automatically
    /// - **Winner determination**: Calculates and displays winner
    /// - **Draw detection**: Handles tie game scenarios
    /// - **Final message**: Shows game over status with result
    ///
    /// # State Updates
    ///
    /// Successful moves trigger:
    /// - Move addition to game history
    /// - Board state advancement to next turn
    /// - Status message updates
    /// - Game over checking and handling
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut app = App::new();
    /// // ... setup game and players ...
    /// app.selected_cell = Some((2, 3)); // For human player
    /// app.gui_play_turn(); // Processes the move
    /// ```
    pub fn gui_play_turn(&mut self) {
        let mut play_turn_result = Err("Error in Enter".to_string());
        let mut new_message = None;
        if let Some(board) = &mut self.board {
            if !board.is_game_over() {
                match board.get_player_turn() {
                    Cell::Black => {
                        if let Some(player) = &self.player_1 {
                            play_turn_result = player.play_turn(board, self.selected_cell);
                        }
                    }
                    Cell::White => {
                        if let Some(player) = &self.player_2 {
                            play_turn_result = player.play_turn(board, self.selected_cell);
                        }
                    }
                    _ => {
                        play_turn_result = Err("Invalid player turn".to_string());
                    }
                }

                match play_turn_result {
                    Err(e) => {
                        self.set_game_message(Some(e));
                    }
                    Ok(history_action) => {
                        board.add_to_history(history_action);

                        if board.check_game_over() {
                            self.timer.as_mut().unwrap().stop();
                            if let Some(winner) = board.get_winner() {
                                new_message = Some(format!("Game over! {} is the WINNER!", winner));
                            } else {
                                new_message = Some("Game over! It's a draw!".to_string());
                            }
                            if let Some(message) = new_message {
                                self.set_game_message(Some(message));
                            }
                        } else {
                            board.next_turn();
                            new_message = Some(format!("It's {} turn !", board.get_player_turn()));
                            if let Some(message) = new_message {
                                self.set_game_message(Some(message));
                            }
                        }
                    }
                }
            }
        }
    }

    /// Terminates the current game and returns to the main menu.
    ///
    /// This method performs complete cleanup of the current game session,
    /// resetting all game-related state and returning the application to
    /// its initial state. It's called when the user quits a game or when
    /// transitioning back to the main menu.
    ///
    /// # State Cleanup
    ///
    /// The method resets the following components:
    /// - **Screen**: Returns to CurrentScreen::Main
    /// - **Game board**: Clears current board state
    /// - **Players**: Removes both player instances
    /// - **UI state**: Clears selected cell and messages
    /// - **Timer**: Stops and removes game timer
    ///
    /// # Memory Management
    ///
    /// All dynamically allocated game resources are properly cleaned up:
    /// - Player trait objects are dropped
    /// - Board state is deallocated
    /// - Timer resources are freed
    ///
    /// # Use Cases
    ///
    /// - User quits current game via 'q' key
    /// - Game completion and return to menu
    /// - Error recovery requiring state reset
    /// - Preparation for new game configuration
    ///
    /// # Post-Cleanup State
    ///
    /// After calling this method, the application is in the same state
    /// as a freshly created App instance, ready for new game setup.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut app = App::new();
    /// // ... play a game ...
    /// app.quit_game();
    /// assert_eq!(app.current_screen, CurrentScreen::Main);
    /// assert!(app.board.is_none());
    /// ```
    pub fn quit_game(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.board = None;
        self.game_message = None;
        self.player_1 = None;
        self.player_2 = None;
        self.selected_cell = None;
        self.timer = None;
    }

    /// Updates the current game message displayed to the user.
    ///
    /// This method provides a simple interface for updating the status
    /// message shown in the game interface. Messages are used for player
    /// notifications, error reporting, and game status updates.
    ///
    /// # Message Types
    ///
    /// Common message categories include:
    /// - **Turn notifications**: "It's Black's turn!"
    /// - **Error messages**: "Invalid move", "Cell already occupied"
    /// - **Game status**: "Game over! Black wins!"
    /// - **System messages**: "No game board available"
    ///
    /// # Message Persistence
    ///
    /// Messages persist until explicitly updated or cleared:
    /// - **None**: Clears the current message
    /// - **Some(String)**: Sets new message content
    /// - Messages remain visible across UI redraws until changed
    ///
    /// # Arguments
    ///
    /// * `message` - Optional string message to display
    ///   - `Some(String)`: Sets new message content
    ///   - `None`: Clears current message
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut app = App::new();
    ///
    /// // Set a status message
    /// app.set_game_message(Some("It's your turn!".to_string()));
    ///
    /// // Clear the message
    /// app.set_game_message(None);
    /// ```
    pub fn set_game_message(&mut self, message: Option<String>) {
        self.game_message = message;
    }

    /// Handles keyboard navigation for board cell selection.
    ///
    /// This method processes arrow key input to move the cell selection
    /// cursor around the game board. It provides intuitive navigation
    /// for human players to select where they want to place their disc.
    ///
    /// # Navigation Controls
    ///
    /// - **Up Arrow**: Move selection up one row (decrease row index)
    /// - **Down Arrow**: Move selection down one row (increase row index)  
    /// - **Left Arrow**: Move selection left one column (decrease column index)
    /// - **Right Arrow**: Move selection right one column (increase column index)
    /// - **Other keys**: Ignored (no movement)
    ///
    /// # Boundary Handling
    ///
    /// The method enforces board boundaries (0 to SIZE-1):
    /// - **Top edge**: Up arrow at row 0 has no effect
    /// - **Bottom edge**: Down arrow at row 7 has no effect
    /// - **Left edge**: Left arrow at column 0 has no effect
    /// - **Right edge**: Right arrow at column 7 has no effect
    ///
    /// # Initialization Behavior
    ///
    /// If no cell is currently selected (selected_cell is None):
    /// - Automatically initializes selection to top-left corner (0, 0)
    /// - Subsequent navigation operates normally from this position
    ///
    /// # Visual Feedback
    ///
    /// The selected cell is highlighted in the game board display:
    /// - **Highlighting**: Double border around selected cell
    /// - **Real-time updates**: Selection changes immediately visible
    /// - **Persistent state**: Selection maintained across screen redraws
    ///
    /// # Use Cases
    ///
    /// - **Human player input**: Navigate to desired move position
    /// - **Move selection**: Choose where to place disc before confirming
    /// - **Board exploration**: Examine different board positions
    /// - **Accessibility**: Keyboard-only navigation support
    ///
    /// # Arguments
    ///
    /// * `key` - KeyCode representing the pressed arrow key or other input
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut app = App::new();
    ///
    /// // Initialize selection (will set to (0,0))
    /// app.select_cell_key(KeyCode::Right);
    /// assert_eq!(app.selected_cell, Some((0, 1)));
    ///
    /// // Navigate around the board
    /// app.select_cell_key(KeyCode::Down);
    /// assert_eq!(app.selected_cell, Some((1, 1)));
    /// ```
    pub fn select_cell_key(&mut self, key: KeyCode) {
        if self.selected_cell.is_none() {
            self.selected_cell = Some((0, 0)); // Initialiser la cellule sélectionnée si elle est None
        } else {
            let (row, col) = self.selected_cell.unwrap();
            match key {
                KeyCode::Up => {
                    if row > 0 {
                        self.selected_cell = Some((row - 1, col));
                    }
                }
                KeyCode::Down => {
                    if row < SIZE - 1 {
                        self.selected_cell = Some((row + 1, col));
                    }
                }
                KeyCode::Left => {
                    if col > 0 {
                        self.selected_cell = Some((row, col - 1));
                    }
                }
                KeyCode::Right => {
                    if col < SIZE - 1 {
                        self.selected_cell = Some((row, col + 1));
                    }
                }
                _ => {}
            }
        }
    }
}
