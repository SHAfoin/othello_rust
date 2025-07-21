//! Q-Learning training progress screen for the Othello game GUI.
//!
//! This module provides the visual interface for displaying Q-Learning AI training progress.
//! It shows a progress bar, completion percentage, status messages, and handles
//! real-time updates from the training thread through channel communication.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Gauge},
    Frame,
};

use crate::gui::{app::App, ui::footer};

/// Renders the Q-Learning training progress screen.
///
/// This function displays a real-time progress interface for Q-Learning AI training,
/// including a visual progress bar, percentage completion, status messages, and
/// navigation instructions. It communicates with the training thread through
/// a channel to receive progress updates.
///
/// # Arguments
///
/// * `frame` - Mutable reference to the terminal frame for rendering
/// * `app` - Mutable reference to the application state
///
/// # Layout Structure
///
/// The screen is organized in a vertical layout with:
/// - Fill space (top padding)
/// - Progress bar section (5 lines)
/// - Status message line (1 line)
/// - Fill space (bottom padding)
/// - Footer with navigation instructions (1 line)
///
/// # Progress Communication
///
/// The function uses `app.qlearning_channel` to receive training progress:
/// - Uses `try_recv()` for non-blocking progress updates
/// - Updates `app.qlearning_loading` with the latest progress value
/// - Handles channel communication errors gracefully
///
/// # Visual Elements
///
/// * **Progress Bar**: Yellow gauge with percentage label showing training completion
/// * **Status Messages**:
///   - "Training in progress, please wait..." during training
///   - "Training complete! The Q table has been saved as 'q_table_player_1.json'." when finished
/// * **Footer**: Navigation instructions for returning to main menu
///
/// # Examples
///
/// ```
/// let mut frame = Frame::new();
/// let mut app = App::new();
/// q_learning_loading_screen(&mut frame, &mut app);
/// ```
pub fn q_learning_loading_screen(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(5),
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .flex(Flex::Center)
        .split(frame.area());

    // use try_recv to get the current progress
    if let Some(rx) = &app.qlearning_channel {
        if let Ok(progress) = rx.try_recv() {
            app.qlearning_loading = Some(progress);
        }
    }

    let label = format!(
        "{:.1}%",
        app.qlearning_loading.unwrap_or(0.0) as f64 * 100.0
    );
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow).bg(Color::Black))
        .ratio(app.qlearning_loading.unwrap_or(0.0) as f64)
        .label(label);
    frame.render_widget(gauge, chunks[1]);

    let mut message = "Training in progress, please wait...";

    if let Some(progress) = app.qlearning_loading {
        if progress >= 1.0 {
            message = "Training complete! The Q table has been saved as 'q_table_player_1.json'.";
        }
    }

    let message_widget = Line::from(message)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center);

    frame.render_widget(message_widget, chunks[2]);

    footer(frame, app, chunks[4], "(q) to return to main menu ");
}
