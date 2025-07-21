//! Input control handler for the active game screen.
//!
//! This module provides keyboard input handling during active gameplay.
//! It manages player movement selection, move execution, and game navigation
//! while respecting game state and player type constraints.

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::gui::app::{App, CurrentScreen};

/// Handles keyboard input during active gameplay.
///
/// This function processes user input while a game is in progress, managing
/// both human player interactions and general game controls. It respects
/// game state (game over, player turn) and player type (human vs AI) to
/// provide appropriate responses to user input.
///
/// # Arguments
///
/// * `app` - Mutable reference to the application state
/// * `key` - The keyboard event to process
/// * `its_a_human_player` - Whether the current player is human (controls input availability)
///
/// # Key Bindings
///
/// ## General Controls (Always Available)
/// * `q` - Quit to exit confirmation screen
/// * `t` - Open tutorial screen
///
/// ## Human Player Controls (Only when `its_a_human_player` is true and game not over)
/// * `Up/Down/Left/Right` - Navigate cell selection on the board
/// * `Enter` - Execute move at selected cell position
///
/// # Behavior
///
/// The function implements different behavior based on context:
/// - **Game Over**: Only general navigation commands (q, t) are processed
/// - **AI Turn**: Only general commands work, movement/selection is disabled
/// - **Human Turn**: Full control including cell selection and move execution
/// - **Move Execution**: Only processes Enter if a cell is currently selected
///
/// # Game State Integration
///
/// The function integrates with several game state components:
/// - Checks `board.is_game_over()` to prevent moves during game end
/// - Uses `selected_cell` to track current cursor position
/// - Calls `gui_play_turn()` to execute moves through the game engine
/// - Manages screen transitions for tutorial and exit confirmation
///
/// # Examples
///
/// ```
/// let mut app = App::new();
/// let key_event = KeyEvent::from(KeyCode::Enter);
///
/// // Human player can make moves
/// game_control(&mut app, key_event, true);
///
/// // AI player cannot control interface
/// game_control(&mut app, key_event, false);
/// ```
pub fn game_control(app: &mut App, key: KeyEvent, its_a_human_player: bool) {
    match key.code {
        KeyCode::Char('q') => {
            // Quit - go to exit confirmation
            app.current_screen = CurrentScreen::Exit;
            app.previous_screen = Some(CurrentScreen::Game);
        }
        KeyCode::Up => {
            // Move selection cursor up
            if !app.board.as_ref().unwrap().is_game_over() && its_a_human_player {
                app.select_cell_key(KeyCode::Up);
            }
        }
        KeyCode::Down => {
            // Move selection cursor down
            if !app.board.as_ref().unwrap().is_game_over() && its_a_human_player {
                app.select_cell_key(KeyCode::Down);
            }
        }
        KeyCode::Left => {
            // Move selection cursor left
            if !app.board.as_ref().unwrap().is_game_over() && its_a_human_player {
                app.select_cell_key(KeyCode::Left);
            }
        }
        KeyCode::Right => {
            // Move selection cursor right
            if !app.board.as_ref().unwrap().is_game_over() && its_a_human_player {
                app.select_cell_key(KeyCode::Right);
            }
        }
        KeyCode::Enter => {
            // Execute move at selected position
            if !app.board.as_ref().unwrap().is_game_over() && its_a_human_player {
                if let Some(_) = app.selected_cell.as_ref() {
                    app.gui_play_turn();
                }
            }
        }
        KeyCode::Char('t') => {
            // Tutorial - open help screen
            app.current_screen = CurrentScreen::Tutorial;
            app.previous_screen = Some(CurrentScreen::Game);
        }
        _ => {}
    }
}
