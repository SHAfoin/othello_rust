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
                app.player_1 = Some(Box::new(Human::new(Cell::Black)));
                app.player_2 = Some(Box::new(Human::new(Cell::White)));
                app.start_game();
            }
            Some(1) => {
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
