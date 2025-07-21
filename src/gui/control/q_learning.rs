use std::sync::mpsc;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    ai::heuristic::HeuristicType,
    consts::QLEARNING_MAX_EPOCHS,
    game::player::Player,
    gui::app::{App, CurrentScreen},
};

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
                if app.qlearning_parameters.as_ref().unwrap().get_epochs() > 500 {
                    let current_epochs = app.qlearning_parameters.as_ref().unwrap().get_epochs();
                    app.qlearning_parameters
                        .as_mut()
                        .unwrap()
                        .set_epochs(current_epochs - 500);
                }
            }
            Some(1) => {
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
                if app.qlearning_parameters.as_ref().unwrap().get_epochs() < QLEARNING_MAX_EPOCHS {
                    let current_epochs = app.qlearning_parameters.as_ref().unwrap().get_epochs();
                    app.qlearning_parameters
                        .as_mut()
                        .unwrap()
                        .set_epochs(current_epochs + 500);
                }
            }
            Some(1) => {
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
