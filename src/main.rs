//! Othello Game Implementation in Rust
//!
//! This is a complete implementation of the classic Othello (Reversi) board game in Rust,
//! featuring a terminal-based user interface built with Ratatui. The game supports multiple
//! play modes including Human vs Human, Human vs AI, AI vs AI, and Q-Learning training.
//!
//! # Features
//!
//! * **Multiple Game Modes**: Human vs Human, Human vs AI, AI vs AI, Q-Learning training
//! * **Advanced AI Algorithms**: MinMax, Alpha-Beta pruning, Q-Learning reinforcement learning
//! * **Terminal UI**: Rich interactive terminal interface using Ratatui framework
//! * **Configurable AI**: Adjustable search depth, heuristics, and training parameters
//! * **Real-time Training**: Background Q-Learning with progress visualization
//!
//! # Architecture
//!
//! The application follows a modular architecture with clear separation of concerns:
//! - **Game Logic**: Core game rules, board state, and player management
//! - **AI Systems**: Multiple AI implementations with different strategies
//! - **GUI Layer**: Terminal interface with screens and input controls
//! - **Configuration**: Constants and parameters for AI and game settings
//!
//! # AI Implementations
//!
//! * **MinMax**: Classic minimax algorithm with optional multithreading
//! * **Alpha-Beta**: Optimized minimax with alpha-beta pruning
//! * **Q-Learning**: Reinforcement learning with configurable parameters
//!
//! # Usage
//!
//! Run the application to access the main menu with game mode selection:
//! ```bash
//! cargo run
//! ```
//!
//! Navigate using arrow keys and Enter to select options. Each game mode
//! provides its own configuration interface before starting play.
//!
//! # Author
//!
//! SALTEL Baptiste - July 2025

//  ===================================================================
//
//  ███████╗██╗  ██╗ █████╗         ███████╗ ██████╗ ██╗███╗   ██╗
//  ██╔════╝██║  ██║██╔══██╗        ██╔════╝██╔═══██╗██║████╗  ██║
//  ███████╗███████║███████║        █████╗  ██║   ██║██║██╔██╗ ██║
//  ╚════██║██╔══██║██╔══██║        ██╔══╝  ██║   ██║██║██║╚██╗██║
//  ███████║██║  ██║██║  ██║███████╗██║     ╚██████╔╝██║██║ ╚████║
//  ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝      ╚═════╝ ╚═╝╚═╝  ╚═══╝
//
//  ===================================================================

mod ai;
mod consts;
mod game;
mod gui;
mod human;

use crate::{
    ai::{
        ai_type::AIType,
        algo::{alphabeta::AIAlphaBeta, minmax::AIMinMax, qlearning::QLearning},
        heuristic::HeuristicType,
        heuristic_matrix::AIHeuristicMatrix,
    },
    consts::{MAX_DEPTH, QLEARNING_MAX_EPOCHS},
    game::{board::Board, cell::Cell, player::Player},
    gui::{
        app::{App, CurrentScreen},
        control::{
            ai_vs_ai::{self, ai_vs_ai_control},
            exit::exit_control,
            game::game_control,
            human_vs_ai::human_vs_ai_control,
            main::main_control,
            q_learning::{self, q_learning_parameters_control},
            q_learning_loading::q_learning_loading_control,
            tutorial::tutorial_control,
        },
        screen::q_learning::q_learning_parameters_screen,
        ui::ui,
    },
};

use ratatui::crossterm::event;
use ratatui::crossterm::event::DisableMouseCapture;
use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use ratatui::crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use ratatui::prelude::Backend;
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use std::{error::Error, io, time::Duration};

/// Entry point for the Othello game application.
///
/// This function initializes the terminal interface, sets up the application state,
/// and runs the main game loop. It handles all terminal configuration including
/// raw mode, alternate screen, and mouse capture for the Ratatui interface.
///
/// # Terminal Setup
///
/// The function configures the terminal for interactive use:
/// - Enables raw mode for direct key capture
/// - Switches to alternate screen to preserve terminal state
/// - Enables mouse capture for potential future mouse support
/// - Creates a Crossterm backend for Ratatui
///
/// # Error Handling
///
/// Returns a boxed error if any terminal operations fail during:
/// - Terminal mode configuration
/// - Application execution
/// - Terminal state restoration
///
/// # Cleanup
///
/// Ensures proper cleanup regardless of how the application exits:
/// - Disables raw mode
/// - Restores original screen
/// - Disables mouse capture
/// - Shows cursor
///
/// # Examples
///
/// ```bash
/// cargo run
/// ```
///
/// # Returns
///
/// * `Ok(())` - Application completed successfully
/// * `Err(Box<dyn Error>)` - Terminal or application error occurred
fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let _res = run_app(&mut terminal, &mut app);

    // Bien désactiver tout ça à la fin !
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

/// Main application loop handling UI rendering and event processing.
///
/// This function implements the core game loop that continuously:
/// - Renders the current screen state
/// - Processes player turns (human input or AI computation)
/// - Handles keyboard events and screen transitions
/// - Manages the application state machine
///
/// # Arguments
///
/// * `terminal` - Mutable reference to the terminal interface
/// * `app` - Mutable reference to the application state
///
/// # Game Loop Architecture
///
/// The loop operates in several phases each iteration:
/// 1. **Rendering**: Draws the current screen using the UI system
/// 2. **Turn Management**: Determines current player and triggers AI moves
/// 3. **Event Processing**: Handles keyboard input with 100ms polling
/// 4. **Screen Routing**: Dispatches events to appropriate control handlers
///
/// # Player Turn Logic
///
/// During active gameplay (CurrentScreen::Game):
/// - Determines current player based on board state (Black/White)
/// - Identifies if current player is human or AI
/// - Automatically triggers AI moves when it's an AI player's turn
/// - Prevents moves when game is over
///
/// # Event Handling
///
/// Processes keyboard events based on current screen:
/// - **Main**: Menu navigation and game mode selection
/// - **Game**: Gameplay controls and move execution
/// - **Tutorial**: Help screen navigation
/// - **Exit**: Confirmation dialog
/// - **Configuration Screens**: Parameter adjustment for AI settings
/// - **Training**: Q-Learning progress and control
///
/// # Input Processing
///
/// - Polls for events every 100ms to balance responsiveness and CPU usage
/// - Filters out key release events to prevent duplicate processing
/// - Routes input to screen-specific control handlers
/// - Handles special case of 'q' quit from main menu
///
/// # Examples
///
/// ```
/// let mut terminal = Terminal::new(backend)?;
/// let mut app = App::new();
/// run_app(&mut terminal, &mut app)?;
/// ```
///
/// # Returns
///
/// * `Ok(())` - Application loop completed successfully
/// * `Err(io::Error)` - Terminal or rendering error occurred
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let mut player_turn;
    let mut its_a_human_player = false;

    loop {
        // Render current screen state
        terminal.draw(|f| ui(f, app))?;

        // Handle player turns during active gameplay
        match app.current_screen {
            CurrentScreen::Game => {
                // Determine current player based on board state
                match app.board.as_ref().unwrap().get_player_turn() {
                    Cell::Black => {
                        player_turn = &app.player_1; // Player 1 is always Black
                    }
                    Cell::White => {
                        player_turn = &app.player_2; // Player 2 is always White
                    }
                    _ => {
                        player_turn = &None; // Invalid state
                    }
                }
                // Process AI moves automatically
                if let Some(player) = player_turn {
                    its_a_human_player = player.is_human();
                    if !player.is_human() && !app.board.as_ref().unwrap().is_game_over() {
                        app.gui_play_turn(); // AI makes move automatically
                    }
                }
            }
            _ => {} // Other screens don't need turn processing
        }

        // Handle keyboard events with polling
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    // Skip key release events to prevent duplicate processing
                    continue;
                }
                // Route events to appropriate screen controllers
                match app.current_screen {
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('q') => {
                            // Special case: quit from main menu
                            return Ok(());
                        }
                        _ => main_control(app, key), // Handle menu navigation
                    },

                    CurrentScreen::Game => game_control(app, key, its_a_human_player),
                    CurrentScreen::Tutorial => tutorial_control(app, key),
                    CurrentScreen::Exit => exit_control(app, key),
                    CurrentScreen::HumanVsAI => human_vs_ai_control(app, key),
                    CurrentScreen::AIVsAI => ai_vs_ai_control(app, key),
                    CurrentScreen::QLearningParameters => q_learning_parameters_control(app, key),
                    CurrentScreen::QLearningLoading => q_learning_loading_control(app, key),
                }
            }
        }
    }
}
