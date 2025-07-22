//! Input control handler for AI vs AI game mode configuration.
//!
//! This module provides keyboard input handling for the AI vs AI configuration screen.
//! It manages navigation through AI settings, parameter adjustments for both players,
//! and game initialization. The interface allows users to configure AI types,
//! search depths, heuristics, threading options, and start the game.

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    ai::{
        ai_type::AIType,
        algo::{alphabeta::AIAlphaBeta, minmax::AIMinMax, qlearning::QLearning},
        heuristic::HeuristicType,
    },
    consts::MAX_DEPTH,
    game::cell::Cell,
    gui::app::{App, CurrentScreen},
};

/// Handles keyboard input for the AI vs AI configuration screen.
///
/// This function processes all user input for configuring AI players before starting
/// an AI vs AI game. It manages:
/// - Navigation between different configuration options
/// - AI type switching (AlphaBeta, MinMax, Q-Learning)
/// - Parameter adjustments (depth, heuristics, threading)
/// - Game initialization and Q-table loading for Q-Learning AIs
/// - Error handling and user feedback
///
/// # Arguments
///
/// * `app` - Mutable reference to the application state
/// * `key` - The keyboard event to process
///
/// # Key Bindings
///
/// * `q` - Return to main menu
/// * `Enter` - Start game (when "Start Game" is selected)
/// * `Up/Down` - Navigate between configuration options
/// * `Left/Right` - Adjust selected parameter values
///
/// # Configuration Options
///
/// The function handles 11 different configuration options (0-10):
/// - 0, 5: AI type for player 1 and 2
/// - 1, 6: Search depth for player 1 and 2
/// - 2, 7: Heuristic type for player 1 and 2
/// - 3, 8: Heuristic matrix for player 1 and 2
/// - 4, 9: Double threading for player 1 and 2
/// - 10: Start game option
///
/// # Q-Learning Support
///
/// When Q-Learning AIs are selected, the function:
/// - Disables parameter changes (depth, heuristics, threading)
/// - Attempts to load pre-trained Q-tables from JSON files
/// - Provides appropriate error messages for unsupported operations
///
/// # Examples
///
/// ```
/// let mut app = App::new();
/// let key_event = KeyEvent::from(KeyCode::Enter);
/// ai_vs_ai_control(&mut app, key_event);
/// ```
pub fn ai_vs_ai_control(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => {
            app.current_mode.select_first();
            app.game_message = None;
            app.current_screen = CurrentScreen::Main;
        }

        KeyCode::Enter => match app.current_mode.selected() {
            Some(10) => {
                // Start Game option
                let mut game_ready = true;
                if app.player_1.as_ref().unwrap().get_ai_type().unwrap() == AIType::QLearning {
                    match app
                        .player_1
                        .as_mut()
                        .unwrap()
                        .import_q_table_file("q_table_player_1.json")
                    {
                        Ok(_) => {
                            game_ready = true;
                        }
                        Err(e) => {
                            game_ready = false;
                            app.set_game_message(Some(e));
                        }
                    }
                }
                if app.player_2.as_ref().unwrap().get_ai_type().unwrap() == AIType::QLearning {
                    match app
                        .player_2
                        .as_mut()
                        .unwrap()
                        .import_q_table_file("q_table_player_2.json")
                    {
                        Ok(_) => {
                            game_ready = true;
                        }
                        Err(e) => {
                            game_ready = false;
                            app.set_game_message(Some(e));
                        }
                    }
                }
                if game_ready {
                    app.current_screen = CurrentScreen::Game;
                    app.previous_screen = Some(CurrentScreen::AIVsAI);
                    app.current_mode.select_first();
                    app.start_game();
                }
            }
            _ => {}
        },

        KeyCode::Up => {
            app.current_mode.select_previous();
        }
        KeyCode::Down => {
            app.current_mode.select_next();
        }
        KeyCode::Left => match app.current_mode.selected() {
            Some(0) => {
                // Player 1 AI Type - cycle to previous AI type
                // changer l'IA en gardant les mêmes paramètres, sauf Q learning
                match app.player_1.as_mut().unwrap().get_ai_type() {
                    Some(AIType::AlphaBeta) => {
                        app.player_1 = Some(Box::new(AIMinMax::new(
                            app.player_1.as_ref().unwrap().get_depth(),
                            app.player_1.as_ref().unwrap().get_heuristic(),
                            Cell::Black,
                            app.player_1.as_ref().unwrap().get_heuristic_matrix(),
                            app.player_1.as_ref().unwrap().get_double_threading(),
                        )));
                    }
                    Some(AIType::MinMax) => {
                        app.player_1 = Some(Box::new(QLearning::new(
                            1000,
                            app.player_1.as_ref().unwrap().get_heuristic(),
                            app.player_1.as_ref().unwrap().get_heuristic_matrix(),
                            10000,
                            Cell::Black,
                        )));
                    }
                    Some(AIType::QLearning) => {
                        app.player_1 = Some(Box::new(AIAlphaBeta::new(
                            app.player_1.as_ref().unwrap().get_depth(),
                            app.player_1.as_ref().unwrap().get_heuristic(),
                            Cell::Black,
                            app.player_1.as_ref().unwrap().get_heuristic_matrix(),
                        )));
                    }
                    _ => {}
                }
            }
            Some(1) => {
                // Player 1 Depth - decrease search depth
                if app.player_1.as_ref().unwrap().get_ai_type().unwrap() != AIType::QLearning {
                    if app.player_1.as_ref().unwrap().get_depth() > 1 {
                        let current_depth = app.player_1.as_ref().unwrap().get_depth();
                        app.player_1.as_mut().unwrap().set_depth(current_depth - 1);
                    }
                } else {
                    app.set_game_message(Some(
                        "QLearning does not support depth change".to_string(),
                    ));
                }
            }
            Some(2) => {
                // Player 1 Heuristic - cycle to previous heuristic
                if app.player_1.as_ref().unwrap().get_ai_type().unwrap() != AIType::QLearning {
                    let previous_heuristic =
                        app.player_1.as_ref().unwrap().get_heuristic().previous();
                    app.player_1
                        .as_mut()
                        .unwrap()
                        .set_heuristic(previous_heuristic);
                } else {
                    app.set_game_message(Some(
                        "QLearning does not support heuristic change".to_string(),
                    ));
                }
            }
            Some(3) => {
                // Player 1 Heuristic Matrix - cycle to previous matrix
                if app.player_1.as_ref().unwrap().get_ai_type().unwrap() != AIType::QLearning {
                    if app.player_1.as_ref().unwrap().get_heuristic() == HeuristicType::Absolute
                        || app.player_1.as_ref().unwrap().get_heuristic() == HeuristicType::Mobility
                    {
                        app.set_game_message(Some(
                            "This heuristic do not support heuristic matrix change".to_string(),
                        ));
                    } else {
                        let previous_matrix = app
                            .player_1
                            .as_ref()
                            .unwrap()
                            .get_heuristic_matrix()
                            .previous();
                        app.player_1
                            .as_mut()
                            .unwrap()
                            .set_heuristic_matrix(previous_matrix);
                    }
                } else {
                    app.set_game_message(Some(
                        "QLearning does not support heuristic matrix change".to_string(),
                    ));
                }
            }
            Some(4) => {
                // Player 1 Double Threading - toggle threading option
                if app.player_1.as_ref().unwrap().get_ai_type().unwrap() == AIType::MinMax {
                    let previous_double_threading =
                        app.player_1.as_ref().unwrap().get_double_threading();
                    app.player_1
                        .as_mut()
                        .unwrap()
                        .set_double_threading(!previous_double_threading);
                } else {
                    app.set_game_message(Some(
                        "Only MinMax AI can use double threading".to_string(),
                    ));
                }
            }
            Some(5) => {
                // Player 2 AI Type - cycle to previous AI type
                // changer l'IA en gardant les mêmes paramètres, sauf Q learning
                match app.player_2.as_mut().unwrap().get_ai_type() {
                    Some(AIType::AlphaBeta) => {
                        app.player_2 = Some(Box::new(AIMinMax::new(
                            app.player_2.as_ref().unwrap().get_depth(),
                            app.player_2.as_ref().unwrap().get_heuristic(),
                            Cell::White,
                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                            app.player_2.as_ref().unwrap().get_double_threading(),
                        )));
                    }
                    Some(AIType::MinMax) => {
                        app.player_2 = Some(Box::new(QLearning::new(
                            1000,
                            app.player_2.as_ref().unwrap().get_heuristic(),
                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                            10000,
                            Cell::White,
                        )));
                    }
                    Some(AIType::QLearning) => {
                        app.player_2 = Some(Box::new(AIAlphaBeta::new(
                            app.player_2.as_ref().unwrap().get_depth(),
                            app.player_2.as_ref().unwrap().get_heuristic(),
                            Cell::White,
                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                        )));
                    }
                    _ => {}
                }
            }
            Some(6) => {
                // Player 2 Depth - decrease search depth
                if app.player_2.as_ref().unwrap().get_ai_type().unwrap() != AIType::QLearning {
                    if app.player_2.as_ref().unwrap().get_depth() > 1 {
                        let current_depth = app.player_2.as_ref().unwrap().get_depth();
                        app.player_2.as_mut().unwrap().set_depth(current_depth - 1);
                    }
                } else {
                    app.set_game_message(Some(
                        "QLearning does not support depth change".to_string(),
                    ));
                }
            }
            Some(7) => {
                // Player 2 Heuristic - cycle to previous heuristic
                if app.player_2.as_ref().unwrap().get_ai_type().unwrap() != AIType::QLearning {
                    let previous_heuristic =
                        app.player_2.as_ref().unwrap().get_heuristic().previous();
                    app.player_2
                        .as_mut()
                        .unwrap()
                        .set_heuristic(previous_heuristic);
                } else {
                    app.set_game_message(Some(
                        "QLearning does not support heuristic change".to_string(),
                    ));
                }
            }
            Some(8) => {
                // Player 2 Heuristic Matrix - cycle to previous matrix
                if app.player_2.as_ref().unwrap().get_ai_type().unwrap() != AIType::QLearning {
                    if app.player_2.as_ref().unwrap().get_heuristic() == HeuristicType::Absolute
                        || app.player_2.as_ref().unwrap().get_heuristic() == HeuristicType::Mobility
                    {
                        app.set_game_message(Some(
                            "This heuristic do not support heuristic matrix change".to_string(),
                        ));
                    } else {
                        let previous_matrix = app
                            .player_2
                            .as_ref()
                            .unwrap()
                            .get_heuristic_matrix()
                            .previous();
                        app.player_2
                            .as_mut()
                            .unwrap()
                            .set_heuristic_matrix(previous_matrix);
                    }
                } else {
                    app.set_game_message(Some(
                        "QLearning does not support heuristic matrix change".to_string(),
                    ));
                }
            }
            Some(9) => {
                // Player 2 Double Threading - toggle threading option
                if app.player_2.as_ref().unwrap().get_ai_type().unwrap() == AIType::MinMax {
                    let previous_double_threading =
                        app.player_2.as_ref().unwrap().get_double_threading();
                    app.player_2
                        .as_mut()
                        .unwrap()
                        .set_double_threading(!previous_double_threading);
                } else {
                    app.set_game_message(Some(
                        "Only MinMax AI can use double threading".to_string(),
                    ));
                }
            }
            _ => {}
        },
        KeyCode::Right => match app.current_mode.selected() {
            Some(0) => {
                // Player 1 AI Type - cycle to next AI type
                // changer l'IA en gardant les mêmes paramètres, sauf Q learning
                match app.player_1.as_mut().unwrap().get_ai_type() {
                    Some(AIType::AlphaBeta) => {
                        app.player_1 = Some(Box::new(QLearning::new(
                            1000,
                            app.player_1.as_ref().unwrap().get_heuristic(),
                            app.player_1.as_ref().unwrap().get_heuristic_matrix(),
                            10000,
                            Cell::Black,
                        )));
                    }
                    Some(AIType::MinMax) => {
                        app.player_1 = Some(Box::new(AIAlphaBeta::new(
                            app.player_1.as_ref().unwrap().get_depth(),
                            app.player_1.as_ref().unwrap().get_heuristic(),
                            Cell::Black,
                            app.player_1.as_ref().unwrap().get_heuristic_matrix(),
                        )));
                    }
                    Some(AIType::QLearning) => {
                        app.player_1 = Some(Box::new(AIMinMax::new(
                            app.player_1.as_ref().unwrap().get_depth(),
                            app.player_1.as_ref().unwrap().get_heuristic(),
                            Cell::Black,
                            app.player_1.as_ref().unwrap().get_heuristic_matrix(),
                            app.player_1.as_ref().unwrap().get_double_threading(),
                        )));
                    }
                    _ => {}
                }
            }
            Some(1) => {
                // Player 1 Depth - increase search depth
                if app.player_1.as_ref().unwrap().get_ai_type().unwrap() != AIType::QLearning {
                    if app.player_1.as_ref().unwrap().get_depth() < MAX_DEPTH {
                        let current_depth = app.player_1.as_ref().unwrap().get_depth();
                        app.player_1.as_mut().unwrap().set_depth(current_depth + 1);
                    } else {
                        app.set_game_message(Some(
                            "Maximum depth reached [see const MAX_DEPTH]".to_string(),
                        ));
                    }
                } else {
                    app.set_game_message(Some(
                        "QLearning does not support depth change".to_string(),
                    ));
                }
            }
            Some(2) => {
                // Player 1 Heuristic - cycle to next heuristic
                if app.player_1.as_ref().unwrap().get_ai_type().unwrap() != AIType::QLearning {
                    let next_heuristic = app.player_1.as_ref().unwrap().get_heuristic().next();
                    app.player_1.as_mut().unwrap().set_heuristic(next_heuristic);
                } else {
                    app.set_game_message(Some(
                        "QLearning does not support heuristic change".to_string(),
                    ));
                }
            }
            Some(3) => {
                // Player 1 Heuristic Matrix - cycle to next matrix
                if app.player_1.as_ref().unwrap().get_ai_type().unwrap() != AIType::QLearning {
                    if app.player_1.as_ref().unwrap().get_heuristic() == HeuristicType::Absolute
                        || app.player_1.as_ref().unwrap().get_heuristic() == HeuristicType::Mobility
                    {
                        app.set_game_message(Some(
                            "This heuristic do not support heuristic matrix change".to_string(),
                        ));
                    } else {
                        let next_matrix =
                            app.player_1.as_ref().unwrap().get_heuristic_matrix().next();
                        app.player_1
                            .as_mut()
                            .unwrap()
                            .set_heuristic_matrix(next_matrix);
                    }
                } else {
                    app.set_game_message(Some(
                        "QLearning does not support heuristic matrix change".to_string(),
                    ));
                }
            }
            Some(4) => {
                // Player 1 Double Threading - toggle threading option
                if app.player_1.as_ref().unwrap().get_ai_type().unwrap() == AIType::MinMax {
                    let previous_double_threading =
                        app.player_1.as_ref().unwrap().get_double_threading();
                    app.player_1
                        .as_mut()
                        .unwrap()
                        .set_double_threading(!previous_double_threading);
                } else {
                    app.set_game_message(Some(
                        "Only MinMax AI can use double threading".to_string(),
                    ));
                }
            }

            Some(5) => {
                // Player 2 AI Type - cycle to next AI type
                // changer l'IA en gardant les mêmes paramètres, sauf Q learning
                match app.player_2.as_mut().unwrap().get_ai_type() {
                    Some(AIType::AlphaBeta) => {
                        app.player_2 = Some(Box::new(QLearning::new(
                            1000,
                            app.player_2.as_ref().unwrap().get_heuristic(),
                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                            10000,
                            Cell::White,
                        )));
                    }
                    Some(AIType::MinMax) => {
                        app.player_2 = Some(Box::new(AIAlphaBeta::new(
                            app.player_2.as_ref().unwrap().get_depth(),
                            app.player_2.as_ref().unwrap().get_heuristic(),
                            Cell::White,
                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                        )));
                    }
                    Some(AIType::QLearning) => {
                        app.player_2 = Some(Box::new(AIMinMax::new(
                            app.player_2.as_ref().unwrap().get_depth(),
                            app.player_2.as_ref().unwrap().get_heuristic(),
                            Cell::White,
                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                            false,
                        )));
                    }
                    _ => {}
                }
            }
            Some(6) => {
                // Player 2 Depth - increase search depth
                if app.player_2.as_ref().unwrap().get_ai_type().unwrap() != AIType::QLearning {
                    if app.player_2.as_ref().unwrap().get_depth() < MAX_DEPTH {
                        let current_depth = app.player_2.as_ref().unwrap().get_depth();
                        app.player_2.as_mut().unwrap().set_depth(current_depth + 1);
                    } else {
                        app.set_game_message(Some(
                            "Maximum depth reached [see const MAX_DEPTH]".to_string(),
                        ));
                    }
                } else {
                    app.set_game_message(Some(
                        "QLearning does not support depth change".to_string(),
                    ));
                }
            }
            Some(7) => {
                // Player 2 Heuristic - cycle to next heuristic
                if app.player_2.as_ref().unwrap().get_ai_type().unwrap() != AIType::QLearning {
                    let next_heuristic = app.player_2.as_ref().unwrap().get_heuristic().next();
                    app.player_2.as_mut().unwrap().set_heuristic(next_heuristic);
                } else {
                    app.set_game_message(Some(
                        "QLearning does not support heuristic change".to_string(),
                    ));
                }
            }
            Some(8) => {
                // Player 2 Heuristic Matrix - cycle to next matrix
                if app.player_2.as_ref().unwrap().get_ai_type().unwrap() != AIType::QLearning {
                    if app.player_2.as_ref().unwrap().get_heuristic() == HeuristicType::Absolute
                        || app.player_2.as_ref().unwrap().get_heuristic() == HeuristicType::Mobility
                    {
                        app.set_game_message(Some(
                            "This heuristic do not support heuristic matrix change".to_string(),
                        ));
                    } else {
                        let next_matrix =
                            app.player_2.as_ref().unwrap().get_heuristic_matrix().next();
                        app.player_2
                            .as_mut()
                            .unwrap()
                            .set_heuristic_matrix(next_matrix);
                    }
                } else {
                    app.set_game_message(Some(
                        "QLearning does not support heuristic matrix change".to_string(),
                    ));
                }
            }
            Some(9) => {
                // Player 2 Double Threading - toggle threading option
                if app.player_2.as_ref().unwrap().get_ai_type().unwrap() == AIType::MinMax {
                    let previous_double_threading =
                        app.player_2.as_ref().unwrap().get_double_threading();
                    app.player_2
                        .as_mut()
                        .unwrap()
                        .set_double_threading(!previous_double_threading);
                } else {
                    app.set_game_message(Some(
                        "Only MinMax AI can use double threading".to_string(),
                    ));
                }
            }
            _ => {}
        },
        _ => {}
    }
}
