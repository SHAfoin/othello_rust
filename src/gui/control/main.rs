//! Input control handler for the main menu screen.
//!
//! This module provides keyboard input handling for the main menu interface.
//! It manages navigation through game mode options and initializes players
//! for different game types including Human vs Human, Human vs AI, AI vs AI,
//! and Q-Learning training modes.

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    ai::{
        algo::{alphabeta::AIAlphaBeta, minmax::AIMinMax, qlearning::QLearning},
        heuristic::HeuristicType,
        heuristic_matrix::AIHeuristicMatrix,
    },
    consts::MAX_DEPTH,
    game::cell::Cell,
    gui::app::{App, CurrentScreen},
    human::Human,
};

/// Handles keyboard input for the main menu screen.
///
/// This function processes user input on the main menu, allowing navigation
/// through different game modes and initializing the appropriate players
/// and game configurations based on the selected option.
///
/// # Arguments
///
/// * `app` - Mutable reference to the application state
/// * `key` - The keyboard event to process
///
/// # Key Bindings
///
/// * `Up/Down` - Navigate through menu options
/// * `Enter` - Select the currently highlighted menu option
///
/// # Menu Options
///
/// * **Option 0**: Human vs Human - Creates two human players and starts game immediately
/// * **Option 1**: Human vs AI - Creates human player 1 and AI player 2, goes to configuration screen
/// * **Option 2**: AI vs AI - Creates two AI players with default settings, goes to configuration screen
/// * **Option 3**: Q-Learning Training - Sets up Q-Learning parameters and goes to training configuration
///
/// # Player Initialization
///
/// Each game mode initializes players with specific default configurations:
/// - **Human players**: Simple human controllers for manual input
/// - **AI players**: Pre-configured with optimal settings (AlphaBeta, MinMax)
/// - **Q-Learning**: Configurable training parameters and heuristics
///
/// # Examples
///
/// ```
/// let mut app = App::new();
/// let key_event = KeyEvent::from(KeyCode::Enter);
/// main_control(&mut app, key_event);
/// ```
pub fn main_control(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Up => {
            app.current_mode.select_previous();
        }
        KeyCode::Down => {
            app.current_mode.select_next();
        }
        KeyCode::Enter => match app.current_mode.selected() {
            Some(0) => {
                // Human vs Human - start game immediately
                app.player_1 = Some(Box::new(Human::new(Cell::Black)));
                app.player_2 = Some(Box::new(Human::new(Cell::White)));
                app.start_game();
            }
            Some(1) => {
                // Human vs AI - go to configuration screen
                app.player_1 = Some(Box::new(Human::new(Cell::Black)));
                app.player_2 = Some(Box::new(AIMinMax::new(
                    3,
                    HeuristicType::Absolute,
                    Cell::White,
                    AIHeuristicMatrix::A,
                    false,
                )));

                app.current_screen = CurrentScreen::HumanVsAI;
                app.current_mode.select_first();
            }
            Some(2) => {
                // AI vs AI - go to configuration screen
                app.player_1 = Some(Box::new(AIAlphaBeta::new(
                    MAX_DEPTH,
                    HeuristicType::Mixte,
                    Cell::Black,
                    AIHeuristicMatrix::A,
                )));
                app.player_2 = Some(Box::new(AIAlphaBeta::new(
                    MAX_DEPTH,
                    HeuristicType::Mixte,
                    Cell::White,
                    AIHeuristicMatrix::B,
                )));
                app.current_screen = CurrentScreen::AIVsAI;
                app.current_mode.select_first();
            }
            Some(3) => {
                // Q-Learning Training - go to parameters screen
                app.current_screen = CurrentScreen::QLearningParameters;
                app.qlearning_parameters = Some(QLearning::new(
                    64,
                    HeuristicType::Global,
                    AIHeuristicMatrix::A,
                    10000,
                    Cell::Black,
                ));
                app.current_mode.select_first();
            }
            _ => {}
        },
        _ => {}
    };
}
