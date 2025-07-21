//! Human vs AI configuration screen implementation.
//!
//! This module provides the user interface for configuring Human vs AI games.
//! It allows users to set up AI opponent parameters while playing as the human
//! player, offering customization of AI difficulty and behavior settings.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{Block, BorderType, List, Padding, Paragraph},
    Frame,
};

use crate::{
    ai::{ai_type::AIType, heuristic::HeuristicType},
    gui::{
        app::App,
        ui::{centered_rect, footer, widget_title},
    },
};

/// Renders the Human vs AI configuration screen.
///
/// This function creates and displays the configuration interface for setting up
/// a Human vs AI game. It provides interactive controls for customizing the AI
/// opponent's parameters, allowing players to adjust difficulty and playing style
/// to match their preferences.
///
/// # Screen Layout
///
/// The screen is organized into four main sections:
/// - **Title area**: Application branding and screen identification
/// - **Configuration list**: Interactive AI parameter settings
/// - **Message area**: Status messages and error notifications
/// - **Footer**: Navigation instructions and available controls
///
/// # AI Configuration Options
///
/// Players can configure the following AI parameters:
/// - **AI Type**: Choose algorithm (MinMax, Alpha-Beta, Q-Learning)
/// - **Search Depth**: Set analysis depth for tree-based algorithms
/// - **Heuristic Type**: Select evaluation function approach
/// - **Heuristic Matrix**: Choose strategic focus for position evaluation
/// - **Multi-threading**: Enable performance optimization where applicable
///
/// # Dynamic UI Behavior
///
/// The interface adapts based on selected AI type:
/// - **Q-Learning**: Depth and heuristic options are disabled (grayed out)
/// - **MinMax/Alpha-Beta**: All options available, multi-threading for MinMax only
/// - **Matrix heuristics**: Disabled for Absolute and Mobility heuristic types
///
/// # Visual Feedback
///
/// - **Option availability**: Incompatible options shown in dark gray
/// - **Current selection**: Highlighted with yellow background
/// - **Value display**: Current settings shown in angle brackets format
/// - **Status messages**: Important information displayed in message area
///
/// # User Experience
///
/// The screen provides an intuitive interface for:
/// - Customizing AI difficulty by adjusting search depth
/// - Selecting different AI personalities through algorithm choice
/// - Fine-tuning AI behavior via heuristic configuration
/// - Understanding option availability through visual cues
///
/// # Arguments
///
/// * `frame` - Ratatui frame for rendering UI widgets
/// * `app` - Mutable application state containing AI configuration
///
/// # Layout Dimensions
///
/// - **Configuration area**: Centered rectangle (60% width, 12 rows height)
/// - **Responsive design**: Adapts to various terminal sizes
/// - **Consistent spacing**: Maintains visual balance across screen sections
///
/// # Examples
///
/// ```rust
/// // Called by the main UI router when on Human vs AI screen
/// match app.current_screen {
///     CurrentScreen::HumanVsAI => human_vs_ai_screen(&mut frame, &mut app),
///     // ... other screen handlers
/// }
/// ```
pub fn human_vs_ai_screen(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .flex(Flex::Center)
        .split(frame.area());

    widget_title(frame, app, chunks[0]);

    let is_qlearning = app.player_2.as_ref().unwrap().get_ai_type().unwrap() == AIType::QLearning;
    let can_use_double_threading =
        app.player_2.as_ref().unwrap().get_ai_type().unwrap() == AIType::MinMax;
    let heuristic_dont_use_matrix = app.player_1.as_ref().unwrap().get_heuristic()
        == HeuristicType::Absolute
        || app.player_2.as_ref().unwrap().get_heuristic() == HeuristicType::Mobility;

    let items = [
        Span::from(format!(
            "{:<30}{:>20}",
            "AI Type",
            format!(
                "< {} >",
                app.player_2.as_ref().unwrap().get_ai_type().unwrap()
            )
        )),
        Span::from(format!(
            "{:<30}{:>20}",
            "Depth of tree",
            format!("< {} >", app.player_2.as_ref().unwrap().get_depth())
        ))
        .style(if is_qlearning {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        }),
        Span::from(format!(
            "{:<30}{:>20}",
            "Heuristic Type",
            format!("< {} >", app.player_2.as_ref().unwrap().get_heuristic())
        ))
        .style(if is_qlearning {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        }),
        Span::from(format!(
            "{:<30}{:>20}",
            "Matrix Heuristic",
            format!(
                "< {} >",
                app.player_2.as_ref().unwrap().get_heuristic_matrix()
            )
        ))
        .style(if is_qlearning || !heuristic_dont_use_matrix {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        }),
        Span::from(format!(
            "{:<30}{:>20}",
            "Double Threading",
            format!(
                "< {} >",
                app.player_1.as_ref().unwrap().get_double_threading()
            )
        ))
        .style(if !can_use_double_threading {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        }),
        Span::from(format!("{:<50}", "Play")),
    ];

    let layout = centered_rect(60, 10, chunks[1]);

    let list = List::new(items)
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(" AI parameters ")
                .title_alignment(Alignment::Center)
                .padding(Padding::uniform(1)),
        )
        .highlight_style(Style::new().bg(Color::Yellow).fg(Color::Black))
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(list, layout, &mut app.current_mode);

    // Zone de message
    let error_message = app.game_message.clone().unwrap_or("".into());
    let error_message_block = Paragraph::new(Span::from(error_message).into_centered_line())
        .yellow()
        .block(Block::default());

    frame.render_widget(error_message_block, chunks[2]);

    // Footer
    footer(
        frame,
        app,
        chunks[3],
        " (↑↓←→) to choose / (ENTER) to validate / (q) to return to main menu ",
    );
}
