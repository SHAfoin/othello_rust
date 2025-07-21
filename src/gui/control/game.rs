use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::gui::app::{App, CurrentScreen};

pub fn game_control(app: &mut App, key: KeyEvent, its_a_human_player: bool) {
    match key.code {
        KeyCode::Char('q') => {
            app.current_screen = CurrentScreen::Exit;
            app.previous_screen = Some(CurrentScreen::Game);
        }
        KeyCode::Up => {
            if !app.board.as_ref().unwrap().is_game_over() && its_a_human_player {
                app.select_cell_key(KeyCode::Up);
            }
        }
        KeyCode::Down => {
            if !app.board.as_ref().unwrap().is_game_over() && its_a_human_player {
                app.select_cell_key(KeyCode::Down);
            }
        }
        KeyCode::Left => {
            if !app.board.as_ref().unwrap().is_game_over() && its_a_human_player {
                app.select_cell_key(KeyCode::Left);
            }
        }
        KeyCode::Right => {
            if !app.board.as_ref().unwrap().is_game_over() && its_a_human_player {
                app.select_cell_key(KeyCode::Right);
            }
        }
        KeyCode::Enter => {
            if !app.board.as_ref().unwrap().is_game_over() && its_a_human_player {
                if let Some(_) = app.selected_cell.as_ref() {
                    app.gui_play_turn();
                }
            }
        }
        KeyCode::Char('t') => {
            app.current_screen = CurrentScreen::Tutorial;
            app.previous_screen = Some(CurrentScreen::Game);
        }
        _ => {}
    }
}
