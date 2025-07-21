use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::gui::app::{App, CurrentScreen};

pub fn q_learning_loading_control(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => {
            app.current_screen = CurrentScreen::Exit;
            app.previous_screen = Some(CurrentScreen::QLearningLoading);
        }
        _ => {}
    }
}
