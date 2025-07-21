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

pub fn human_vs_ai_control(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => {
            app.current_mode.select_first();
            app.game_message = None;
            app.current_screen = CurrentScreen::Main;
        }

        KeyCode::Enter => match app.current_mode.selected() {
            Some(5) => {
                let mut game_ready = false;

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
                // changer l'IA en gardant les mêmes paramètres, sauf Q learning
                match app.player_2.as_mut().unwrap().get_ai_type() {
                    Some(AIType::AlphaBeta) => {
                        app.player_2 = Some(Box::new(AIMinMax::new(
                            app.player_2.as_ref().unwrap().get_depth(),
                            app.player_2.as_ref().unwrap().get_heuristic(),
                            Cell::White,
                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                            false,
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
            Some(1) => {
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
            Some(2) => {
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
            Some(3) => {
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
            Some(4) => {
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
            Some(1) => {
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
            Some(2) => {
                if app.player_2.as_ref().unwrap().get_ai_type().unwrap() != AIType::QLearning {
                    let next_heuristic = app.player_2.as_ref().unwrap().get_heuristic().next();
                    app.player_2.as_mut().unwrap().set_heuristic(next_heuristic);
                } else {
                    app.set_game_message(Some(
                        "QLearning does not support heuristic change".to_string(),
                    ));
                }
            }
            Some(3) => {
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
            Some(4) => {
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
