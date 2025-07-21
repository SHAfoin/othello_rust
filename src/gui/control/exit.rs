use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::gui::app::{App, CurrentScreen};

pub fn exit_control(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('q') => app.quit_game(),
        KeyCode::Char('n') => app.current_screen = CurrentScreen::Game,
        _ => {}
    }
}
