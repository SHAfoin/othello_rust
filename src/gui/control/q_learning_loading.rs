//! Input control handler for the Q-Learning training progress screen.
//!
//! This module provides keyboard input handling during Q-Learning AI training.
//! It offers minimal interaction options while the training process runs
//! in the background, allowing users to exit or monitor progress.

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::gui::app::{App, CurrentScreen};

/// Handles keyboard input during Q-Learning training progress display.
///
/// This function processes user input while Q-Learning training is in progress.
/// It provides limited interaction to avoid interfering with the background
/// training process while still allowing users to exit if needed.
///
/// # Arguments
///
/// * `app` - Mutable reference to the application state
/// * `key` - The keyboard event to process
///
/// # Key Bindings
///
/// * `q` - Exit to confirmation screen (stops training)
/// * Other keys - Ignored (no action)
///
/// # Behavior
///
/// During Q-Learning training, most interactions are disabled to prevent
/// interference with the training process. The only available action is
/// to request exit, which leads to a confirmation dialog that will
/// terminate the training if confirmed.
///
/// # Training State
///
/// The function respects the background training process by:
/// - Limiting available actions during active training
/// - Preserving training state when navigating to exit confirmation
/// - Allowing graceful termination through the exit dialog
///
/// # Examples
///
/// ```
/// let mut app = App::new();
/// let key_event = KeyEvent::from(KeyCode::Char('q'));
/// q_learning_loading_control(&mut app, key_event); // Request exit
/// ```
pub fn q_learning_loading_control(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => {
            // Exit - go to confirmation (stops training)
            app.current_screen = CurrentScreen::Exit;
            app.previous_screen = Some(CurrentScreen::QLearningLoading);
        }
        _ => {}
    }
}
