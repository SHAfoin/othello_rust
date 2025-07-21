//! Input control handler for the tutorial/help screen.
//!
//! This module provides keyboard input handling for the tutorial screen.
//! It offers a simple interface for users to view game instructions
//! and return to the previous screen when finished.

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::gui::app::{App, CurrentScreen};

/// Handles keyboard input for the tutorial screen.
///
/// This function processes user input while viewing the tutorial/help screen.
/// It provides a minimal interface focused on returning to the game or
/// previous screen once the user has finished reading the instructions.
///
/// # Arguments
///
/// * `app` - Mutable reference to the application state
/// * `key` - The keyboard event to process
///
/// # Key Bindings
///
/// * `q` - Return to the game screen
/// * Other keys - Ignored (no action)
///
/// # Behavior
///
/// The tutorial screen is designed as a simple overlay that users can
/// dismiss when they're done reading. The 'q' key immediately returns
/// to the game screen, allowing players to continue where they left off.
///
/// # Examples
///
/// ```
/// let mut app = App::new();
/// let key_event = KeyEvent::from(KeyCode::Char('q'));
/// tutorial_control(&mut app, key_event); // Returns to game
/// ```
pub fn tutorial_control(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.current_screen = CurrentScreen::Game, // Return to game
        _ => {}
    }
}
