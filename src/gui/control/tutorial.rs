use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::gui::app::{App, CurrentScreen};

pub fn tutorial_control(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.current_screen = CurrentScreen::Game,
        _ => {}
    }
}
