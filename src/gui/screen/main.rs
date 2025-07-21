//! Main menu screen implementation.
//!
//! This module provides the main menu interface for the Othello game application.
//! It displays the primary navigation options allowing users to choose between
//! different game modes and access various features of the application.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, List, Padding},
    Frame,
};

use crate::gui::{
    app::App,
    ui::{footer, widget_title},
};

/// Renders the main menu screen of the Othello application.
///
/// This function creates and displays the primary navigation interface,
/// presenting users with all available game modes and features. It serves
/// as the central hub for accessing different parts of the application.
///
/// # Screen Layout
///
/// The main menu is organized into several vertical sections:
/// - **Title section**: Application branding and logo
/// - **Spacer**: Visual separation (flexible height)
/// - **Menu section**: Interactive list of game mode options
/// - **Spacer**: Additional visual separation (flexible height)
/// - **Footer**: Navigation instructions and controls
///
/// # Available Options
///
/// The main menu provides access to:
/// - **Human vs Human**: Two-player local multiplayer mode
/// - **Human vs AI**: Single-player mode against computer opponent
/// - **AI vs AI**: Automated game between two AI players
/// - **Q-Learning Training**: AI training and learning interface
///
/// # Visual Design
///
/// - **Centered layout**: Menu items centered horizontally on screen
/// - **Flexible spacing**: Adapts to different terminal sizes
/// - **Highlighted selection**: Current option highlighted for clarity
/// - **Consistent styling**: Matches application's overall visual theme
///
/// # User Interaction
///
/// Navigation is controlled through keyboard input:
/// - **↑↓ arrows**: Move selection up and down through menu items
/// - **ENTER**: Activate selected menu option
/// - **'q'**: Quit application
///
/// # Arguments
///
/// * `frame` - Ratatui frame for rendering widgets to the terminal
/// * `app` - Mutable application state containing current selection and settings
///
/// # Layout Behavior
///
/// The layout uses flexible constraints to maintain proper spacing:
/// - Fixed heights for title (8 lines), menu (8 lines), and footer (1 line)
/// - Flexible fill areas provide responsive spacing between sections
/// - Center flex alignment ensures optimal visual balance
///
/// # Examples
///
/// ```rust
/// // This function is called by the main UI loop when on the main screen
/// match app.current_screen {
///     CurrentScreen::Main => main_screen(&mut frame, &mut app),
///     // ... other screen handlers
/// }
/// ```
pub fn main_screen(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8), // au moins 1 ligne de hauteur pour la liste, prend plus si possible
            Constraint::Fill(1),
            Constraint::Length(8), // au moins 1 ligne de hauteur pour la liste, prend plus si possible
            Constraint::Fill(1),
            Constraint::Length(1), // 3 lignes FIXES de hauteur pour le footer
        ])
        .flex(Flex::Center)
        .split(frame.area());

    widget_title(frame, app, chunks[0]);

    widget_menu(frame, app, chunks[2]);

    footer(
        frame,
        app,
        chunks[4],
        " (↑↓) to choose / (ENTER) to validate / (q) to quit ",
    );
}

/// Renders the interactive menu widget for game mode selection.
///
/// This helper function creates and displays the central menu list that allows
/// users to select between different game modes. It handles the visual
/// presentation, layout, and styling of the menu options.
///
/// # Menu Options
///
/// The menu displays four primary game modes:
/// - **"Human vs Human"**: Local multiplayer for two human players
/// - **"Human vs AI"**: Single-player mode against computer opponent
/// - **"AI vs AI"**: Automated match between two AI players
/// - **"Q-Learning Training"**: AI training and development interface
///
/// # Visual Layout
///
/// The menu uses a three-column horizontal layout:
/// - **Left spacer**: Flexible spacing for centering
/// - **Menu content**: Fixed 40-character width for menu items
/// - **Right spacer**: Flexible spacing for centering
///
/// # Styling Features
///
/// - **Bordered container**: Rounded border with "Main Menu" title
/// - **Highlight system**: Yellow background with black text for selection
/// - **Selection indicator**: ">>" symbol marks the current selection
/// - **Uniform padding**: Consistent spacing within the menu container
///
/// # Interactive Behavior
///
/// The menu responds to the application's current selection state:
/// - Highlights the currently selected option
/// - Updates visual feedback based on user navigation
/// - Maintains selection state across screen redraws
///
/// # Arguments
///
/// * `frame` - Ratatui frame for widget rendering
/// * `app` - Application state containing current menu selection
/// * `area` - Screen rectangle area allocated for the menu widget
///
/// # Layout Calculation
///
/// The function calculates centered positioning by:
/// 1. Creating horizontal layout with flexible side margins
/// 2. Allocating fixed 40-character width for menu content
/// 3. Using the middle section for actual menu rendering
///
/// # Examples
///
/// ```rust
/// // This function is called internally by main_screen()
/// widget_menu(&mut frame, &mut app, menu_area);
/// ```
fn widget_menu(frame: &mut Frame, app: &mut App, area: Rect) {
    let items = [
        "Human vs Human",
        "Human vs AI",
        "AI vs AI",
        "Q-Learning Training",
    ];

    let middle_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(40),
            Constraint::Fill(1),
        ])
        .split(area);

    let list = List::new(items)
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Choose a gamemode")
                .title_alignment(Alignment::Center)
                .padding(Padding::uniform(1)),
        )
        .highlight_style(Style::new().bg(Color::Yellow).fg(Color::Black))
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(list, middle_layout[1], &mut app.current_mode);
}
