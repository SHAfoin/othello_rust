//! Tutorial screen implementation for game rules and instructions.
//!
//! This module provides an in-game tutorial interface that explains Othello
//! rules, gameplay mechanics, and controls to new players. It serves as a
//! helpful reference that can be accessed during gameplay for quick rule
//! clarification and navigation guidance.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    widgets::{Block, BorderType, Padding, Paragraph},
    Frame,
};

use crate::gui::{
    app::App,
    ui::{centered_rect, footer, widget_title},
};

/// Renders the tutorial screen with game rules and instructions.
///
/// This function displays a comprehensive tutorial overlay that explains
/// Othello game rules, objectives, and control instructions. It's designed
/// to be accessible during gameplay to help players understand the game
/// mechanics without interrupting their gaming session.
///
/// # Screen Layout
///
/// The tutorial screen consists of:
/// - **Background title**: Application branding (overlaid by tutorial content)
/// - **Tutorial content**: Centered modal dialog with game rules
/// - **Footer**: Simple navigation instruction to return to game
///
/// # Tutorial Content
///
/// The tutorial covers essential game information:
/// - **Board description**: 8x8 grid gameplay area
/// - **Turn mechanics**: Players alternate placing discs
/// - **Capture rules**: Surrounding opponent discs to flip them
/// - **Move validation**: Must capture at least one disc per move
/// - **Game ending**: When no more moves are possible
/// - **Victory condition**: Player with most discs wins
/// - **Controls**: Arrow key navigation and Enter to place discs
///
/// # Visual Design
///
/// - **Modal overlay**: Tutorial appears over the game interface
/// - **Centered layout**: Large tutorial area (90% width, 14 rows height)
/// - **Clear typography**: Centered text with proper spacing and formatting
/// - **Bordered container**: Rounded border with "Tutorial" title
/// - **Readable format**: Well-structured text with bullet points and spacing
///
/// # User Experience
///
/// The tutorial provides:
/// - **Quick access**: Available during gameplay via 't' key
/// - **Comprehensive rules**: All essential information in one place
/// - **Easy dismissal**: Simple 'q' key to return to game
/// - **Non-intrusive**: Overlays without disrupting game state
///
/// # Arguments
///
/// * `frame` - Ratatui frame for rendering tutorial widgets
/// * `app` - Application state (used for title and potential context)
///
/// # Implementation Notes
///
/// The tutorial uses a two-function approach:
/// - Main function handles overall layout and coordination
/// - Helper function (`widget_tutorial`) renders the tutorial content
///
/// # Examples
///
/// ```rust
/// // Called when user presses 't' during gameplay
/// match app.current_screen {
///     CurrentScreen::Tutorial => tutorial_screen(&mut frame, &mut app),
///     // ... other screen handlers
/// }
/// ```
pub fn tutorial_screen(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(1)])
        .flex(Flex::Center)
        .split(frame.area());

    widget_title(frame, app, chunks[0]);

    // Zone de tutoriel
    widget_tutorial(frame, app, chunks[0]);

    // Footer
    footer(frame, app, chunks[1], "(q) to resume game ");
}

/// Renders the tutorial content widget with game rules and instructions.
///
/// This helper function creates and displays the main tutorial content
/// within a centered, bordered container. It formats and presents all
/// essential Othello game rules and control instructions in a clear,
/// readable format for player reference.
///
/// # Content Organization
///
/// The tutorial text is structured into logical sections:
/// - **Welcome message**: Friendly introduction to the tutorial
/// - **Basic rules**: Core game mechanics and board description
/// - **Gameplay mechanics**: Turn-taking and disc placement rules
/// - **Capture system**: How to surround and flip opponent discs
/// - **Move validation**: Requirements for valid moves
/// - **Game ending**: Conditions for game termination
/// - **Victory condition**: How the winner is determined
/// - **Controls**: Keyboard navigation and action instructions
///
/// # Visual Formatting
///
/// The tutorial uses clear formatting:
/// - **Bullet points**: Easy-to-scan rule listings
/// - **Proper spacing**: Double line breaks between sections
/// - **Emphasis**: Important rules highlighted with formatting
/// - **Logical flow**: Information presented in learning order
///
/// # Layout Specifications
///
/// - **Container**: Rounded border with "Tutorial" title
/// - **Positioning**: Centered on screen (90% width, 14 rows height)
/// - **Padding**: Uniform padding for comfortable reading
/// - **Text alignment**: Centered for visual appeal
///
/// # Tutorial Text Content
///
/// Covers all essential game knowledge:
/// - Board size and structure (8x8 grid)
/// - Player turn mechanics and disc placement
/// - Capture rules and opponent disc flipping
/// - Move validation requirements
/// - Game ending conditions and victory determination
/// - Navigation controls (arrow keys) and action controls (Enter)
///
/// # Arguments
///
/// * `frame` - Ratatui frame for widget rendering
/// * `_app` - Application state (unused but kept for consistency)
/// * `_area` - Screen area parameter (unused, calculates own centered area)
///
/// # Design Philosophy
///
/// The tutorial prioritizes:
/// - **Completeness**: All essential rules covered
/// - **Clarity**: Simple language and clear structure  
/// - **Accessibility**: Easy to read and understand quickly
/// - **Practicality**: Focuses on actionable information for gameplay
///
/// # Examples
///
/// ```rust
/// // Called internally by tutorial_screen()
/// widget_tutorial(&mut frame, &app, tutorial_area);
/// ```
fn widget_tutorial(frame: &mut Frame, _app: &App, _area: Rect) {
    let tutorial_area = centered_rect(90, 14, frame.area());
    let tutorial_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(" Tutorial ")
        .title_alignment(Alignment::Center)
        .padding(Padding::uniform(1));

    let tutorial_text = Paragraph::new(
        "Welcome to the Othello game! Here are the basic rules:\n\n\
        - The game is played on an 8x8 board.\n\
        - Players take turns placing their discs on the board.\n\
        - A player can capture opponent's discs by surrounding them.\n\
        - To play a move, you MUST surround at least one of your opponent's discs.\n\
        - The game ends when no more moves are possible.\n\
        - The player with the most discs at the end wins!\n\n\
        Use the arrow keys to navigate the board, and press Enter to place your disc.",
    )
    .block(tutorial_block)
    .alignment(Alignment::Center);

    frame.render_widget(tutorial_text, tutorial_area);
}
