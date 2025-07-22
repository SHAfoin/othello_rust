//! Input control handler for the exit confirmation screen.
//!
//! This module provides keyboard input handling for the exit confirmation dialog.
//! It processes user responses to confirm or cancel game termination,
//! ensuring users don't accidentally quit the application.

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::gui::app::App;

/// Handles keyboard input for the exit confirmation screen.
///
/// This function processes user input when the exit confirmation dialog is displayed.
/// It provides a simple yes/no interface to confirm game termination or return
/// to the previous screen.
///
/// # Arguments
///
/// * `app` - Mutable reference to the application state
/// * `key` - The keyboard event to process
///
/// # Key Bindings
///
/// * `y` or `q` - Confirm exit and terminate the application
/// * `n` - Cancel exit and return to the previous screen
/// * Other keys - Ignored (no action)
///
/// # Behavior
///
/// When the user confirms exit (y/q), the function calls `app.quit_game()`
/// which sets the application's running state to false and begins cleanup.
/// When the user cancels (n), the function restores the previous screen
/// from `app.previous_screen`.
///
/// # Examples
///
/// ```
/// let mut app = App::new();
/// let key_event = KeyEvent::from(KeyCode::Char('y'));
/// exit_control(&mut app, key_event); // Will quit the game
///
/// let key_event = KeyEvent::from(KeyCode::Char('n'));
/// exit_control(&mut app, key_event); // Will return to previous screen
/// ```
pub fn exit_control(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('q') => app.quit_game(),
        KeyCode::Char('n') => app.current_screen = (*app.previous_screen.as_ref().unwrap()).clone(),
        _ => {}
    }
}
