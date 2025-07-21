//! Application exit confirmation screen implementation.
//!
//! This module provides a simple confirmation dialog for application exit.
//! It displays a centered confirmation message asking the user to confirm
//! their intention to quit the application before actually terminating.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    widgets::{Block, BorderType, Padding, Paragraph},
    Frame,
};

use crate::gui::{
    app::App,
    ui::{centered_rect, widget_title},
};

/// Renders the application exit confirmation screen.
///
/// This function displays a simple confirmation dialog asking the user
/// to confirm their intention to exit the application. It provides a
/// clean, centered interface with clear yes/no options.
///
/// # Screen Layout
///
/// The screen consists of two main areas:
/// - **Title area**: Application title and branding (fills available space)
/// - **Confirmation dialog**: Centered modal-style dialog box
///
/// # Dialog Content
///
/// The confirmation dialog contains:
/// - **Question**: "Are you sure you want to exit?"
/// - **Options**: "y/n" input instruction
/// - **Styling**: Rounded border with centered title and padding
///
/// # User Interaction
///
/// After this screen is displayed, the application waits for user input:
/// - **'y' or 'Y'**: Confirms exit and terminates the application
/// - **'n' or 'N'**: Cancels exit and returns to previous screen
/// - **Other keys**: Typically ignored or treated as 'n'
///
/// # Visual Design
///
/// - **Centered layout**: Dialog appears in the center of the terminal
/// - **Modal appearance**: Overlays the background content
/// - **Clear typography**: Centered text for easy reading
/// - **Consistent styling**: Matches application's visual theme
///
/// # Arguments
///
/// * `frame` - Ratatui frame for rendering widgets to the terminal
/// * `app` - Application state (used for title rendering and potential state)
///
/// # Layout Dimensions
///
/// - **Dialog size**: 40% width, 6 lines height (via `centered_rect(40, 6)`)
/// - **Positioning**: Centered both horizontally and vertically
/// - **Responsive**: Adapts to different terminal sizes
///
/// # Examples
///
/// ```rust
/// // This function is typically called from the main UI router
/// match app.current_screen {
///     CurrentScreen::Exit => exit_screen(&mut frame, &mut app),
///     // ... other screen handlers
/// }
/// ```
pub fn exit_screen(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(1)])
        .flex(Flex::Center)
        .split(frame.area());

    widget_title(frame, app, chunks[0]);

    let exit_message = Paragraph::new(
        "Are you sure you want to exit?\n\
    y/n",
    )
    .block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title(" Exit Confirmation ")
            .title_alignment(Alignment::Center)
            .padding(Padding::uniform(1)),
    )
    .alignment(Alignment::Center);

    let exit_layout = centered_rect(40, 6, frame.area());
    frame.render_widget(exit_message, exit_layout);
}
