//! Input control handler for Q-Learning training parameter configuration.
//!
//! This module provides keyboard input handling for the Q-Learning parameter
//! configuration screen. It manages the adjustment of training parameters
//! such as epochs, max steps, heuristics, and matrix settings before
//! initiating the training process.

use std::sync::mpsc;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    ai::heuristic::HeuristicType,
    consts::QLEARNING_MAX_EPOCHS,
    game::player::Player,
    gui::app::{App, CurrentScreen},
};

/// Handles keyboard input for Q-Learning training parameter configuration.
///
/// This function processes user input for configuring Q-Learning training
/// parameters before starting the training process. It provides comprehensive
/// control over training settings including epochs, step limits, heuristics,
/// and matrix configurations.
///
/// # Arguments
///
/// * `app` - Mutable reference to the application state
/// * `key` - The keyboard event to process
///
/// # Key Bindings
///
/// * `q` - Return to main menu
/// * `Up/Down` - Navigate between parameter options
/// * `Left/Right` - Adjust selected parameter values
/// * `Enter` - Start training (when "Start Training" is selected)
///
/// # Configuration Parameters
///
/// The function manages 5 different configuration options (0-4):
/// * **Option 0**: Training epochs (500-QLEARNING_MAX_EPOCHS, step: 500)
/// * **Option 1**: Max steps per episode (1-64, step: 1)
/// * **Option 2**: Heuristic type (cycles through available types)
/// * **Option 3**: Heuristic matrix (when supported by heuristic)
/// * **Option 4**: Start training option
///
/// # Parameter Constraints
///
/// Each parameter has specific validation rules:
/// - **Epochs**: Minimum 500, maximum defined by QLEARNING_MAX_EPOCHS constant
/// - **Max Steps**: Range from 1 to 64 steps per training episode
/// - **Heuristics**: Some types (Absolute, Mobility) don't support matrix changes
/// - **Matrix**: Only available for compatible heuristic types
///
/// # Training Initialization
///
/// When training starts (Option 4 + Enter):
/// - Creates a communication channel for progress updates
/// - Spawns a background thread for training execution
/// - Transitions to the loading screen with progress display
/// - Preserves parameter configuration for potential restarts
///
/// # Examples
///
/// ```
/// let mut app = App::new();
/// let key_event = KeyEvent::from(KeyCode::Right);
/// q_learning_parameters_control(&mut app, key_event); // Adjust parameter
///
/// let key_event = KeyEvent::from(KeyCode::Enter);
/// q_learning_parameters_control(&mut app, key_event); // Start training
/// ```
pub fn q_learning_parameters_control(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => {
            app.current_mode.select_first();
            app.game_message = None;
            app.current_screen = CurrentScreen::Main;
        }
        KeyCode::Up => {
            app.current_mode.select_previous();
        }
        KeyCode::Down => {
            app.current_mode.select_next();
        }
        KeyCode::Left => match app.current_mode.selected() {
            Some(0) => {
                // Training Epochs - decrease by 500
                if app.qlearning_parameters.as_ref().unwrap().get_epochs() > 500 {
                    let current_epochs = app.qlearning_parameters.as_ref().unwrap().get_epochs();
                    app.qlearning_parameters
                        .as_mut()
                        .unwrap()
                        .set_epochs(current_epochs - 500);
                }
            }
            Some(1) => {
                // Max Steps - decrease by 1
                if app.qlearning_parameters.as_ref().unwrap().get_max_step() > 1 {
                    let current_max_step =
                        app.qlearning_parameters.as_ref().unwrap().get_max_step();
                    app.qlearning_parameters
                        .as_mut()
                        .unwrap()
                        .set_max_step(current_max_step - 1);
                }
            }
            Some(2) => {
                // Heuristic Type - cycle to previous
                let previous_heuristic = app
                    .qlearning_parameters
                    .as_ref()
                    .unwrap()
                    .get_heuristic()
                    .previous();
                app.qlearning_parameters
                    .as_mut()
                    .unwrap()
                    .set_heuristic(previous_heuristic);
            }
            Some(3) => {
                // Heuristic Matrix - cycle to previous
                if app.qlearning_parameters.as_ref().unwrap().get_heuristic()
                    == HeuristicType::Absolute
                    || app.qlearning_parameters.as_ref().unwrap().get_heuristic()
                        == HeuristicType::Mobility
                {
                    app.set_game_message(Some(
                        "This heuristic do not support heuristic matrix change".to_string(),
                    ));
                } else {
                    let previous_matrix = app
                        .qlearning_parameters
                        .as_ref()
                        .unwrap()
                        .get_heuristic_matrix()
                        .previous();
                    app.qlearning_parameters
                        .as_mut()
                        .unwrap()
                        .set_heuristic_matrix(previous_matrix);
                }
            }
            _ => {}
        },
        KeyCode::Right => match app.current_mode.selected() {
            Some(0) => {
                // Training Epochs - increase by 500
                if app.qlearning_parameters.as_ref().unwrap().get_epochs() < QLEARNING_MAX_EPOCHS {
                    let current_epochs = app.qlearning_parameters.as_ref().unwrap().get_epochs();
                    app.qlearning_parameters
                        .as_mut()
                        .unwrap()
                        .set_epochs(current_epochs + 500);
                }
            }
            Some(1) => {
                // Max Steps - increase by 1
                if app.qlearning_parameters.as_ref().unwrap().get_max_step() < 64 {
                    let current_max_step =
                        app.qlearning_parameters.as_ref().unwrap().get_max_step();
                    app.qlearning_parameters
                        .as_mut()
                        .unwrap()
                        .set_max_step(current_max_step + 1);
                }
            }
            Some(2) => {
                // Heuristic Type - cycle to next
                let next_heuristic = app
                    .qlearning_parameters
                    .as_ref()
                    .unwrap()
                    .get_heuristic()
                    .next();
                app.qlearning_parameters
                    .as_mut()
                    .unwrap()
                    .set_heuristic(next_heuristic);
            }
            Some(3) => {
                // Heuristic Matrix - cycle to next
                if app.qlearning_parameters.as_ref().unwrap().get_heuristic()
                    == HeuristicType::Absolute
                    || app.qlearning_parameters.as_ref().unwrap().get_heuristic()
                        == HeuristicType::Mobility
                {
                    app.set_game_message(Some(
                        "This heuristic do not support heuristic matrix change".to_string(),
                    ));
                } else {
                    let next_matrix = app
                        .qlearning_parameters
                        .as_ref()
                        .unwrap()
                        .get_heuristic_matrix()
                        .next();
                    app.qlearning_parameters
                        .as_mut()
                        .unwrap()
                        .set_heuristic_matrix(next_matrix);
                }
            }
            _ => {}
        },
        KeyCode::Enter => match app.current_mode.selected() {
            Some(4) => {
                // Start Training - begin Q-Learning process
                app.current_screen = CurrentScreen::QLearningLoading;
                app.previous_screen = Some(CurrentScreen::QLearningParameters);
                let (tx, rx) = mpsc::channel();
                let mut qlearning_params = app.qlearning_parameters.take().unwrap();
                std::thread::spawn(move || {
                    qlearning_params.try_q_learning(tx);
                });
                app.qlearning_channel = Some(rx);
            }
            _ => {}
        },
        _ => return,
    }
}
