//! AI vs AI configuration screen implementation.
//!
//! This module provides the user interface screen for configuring AI vs AI
//! game matches. It allows users to set up and customize AI parameters for
//! both players before starting an automated game between two AI opponents.

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

/// Renders the AI vs AI configuration screen.
///
/// This function creates and displays the configuration interface for setting up
/// an AI vs AI game. It provides interactive controls for configuring both AI
/// players' parameters including algorithm type, search depth, heuristics, and
/// performance settings.
///
/// # Screen Layout
///
/// The screen is organized into several sections:
/// - **Title area**: Game title and branding
/// - **Configuration list**: Interactive parameter settings for both AIs
/// - **Message area**: Error messages and status information
/// - **Footer**: Navigation and control instructions
///
/// # AI Configuration Options
///
/// For each AI player, the following parameters can be configured:
/// - **AI Type**: Algorithm selection (MinMax, Alpha-Beta, Q-Learning)
/// - **Search Depth**: How many moves ahead to analyze (tree-based algorithms)
/// - **Heuristic Type**: Evaluation function for position assessment
/// - **Heuristic Matrix**: Strategic focus matrix for evaluation
/// - **Multi-threading**: Performance optimization for compatible algorithms
///
/// # Visual Feedback
///
/// - **Disabled options**: Grayed out when not applicable to selected AI type
/// - **Highlighting**: Current selection highlighted with yellow background
/// - **Conditional display**: Options only shown when relevant to AI algorithm
///
/// # Arguments
///
/// * `frame` - Ratatui frame for rendering widgets
/// * `app` - Mutable application state containing AI configurations
///
/// # UI Behavior
///
/// - Options are dynamically enabled/disabled based on AI type compatibility
/// - Q-Learning AIs have depth and heuristic options disabled
/// - Multi-threading only available for MinMax algorithm
/// - Matrix heuristics disabled for Absolute and Mobility heuristic types
///
/// # Examples
///
/// ```rust
/// // This function is called by the main UI loop
/// match app.current_screen {
///     CurrentScreen::AIvsAI => ai_vs_ai_screen(&mut frame, &mut app),
///     // ... other screens
/// }
/// ```
pub fn ai_vs_ai_screen(frame: &mut Frame, app: &mut App) {
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

    let is_qlearning_1 = app.player_1.as_ref().unwrap().get_ai_type().unwrap() == AIType::QLearning;
    let is_qlearning_2 = app.player_2.as_ref().unwrap().get_ai_type().unwrap() == AIType::QLearning;
    let heuristic_dont_use_matrix_1 = app.player_1.as_ref().unwrap().get_heuristic()
        == HeuristicType::Absolute
        || app.player_1.as_ref().unwrap().get_heuristic() == HeuristicType::Mobility;
    let heuristic_dont_use_matrix_2 = app.player_2.as_ref().unwrap().get_heuristic()
        == HeuristicType::Absolute
        || app.player_2.as_ref().unwrap().get_heuristic() == HeuristicType::Mobility;
    let can_use_double_threading_1 =
        app.player_1.as_ref().unwrap().get_ai_type().unwrap() == AIType::MinMax;
    let can_use_double_threading_2 =
        app.player_2.as_ref().unwrap().get_ai_type().unwrap() == AIType::MinMax;

    let items = [
        Span::from(format!(
            "{:<30}{:>20}",
            "1 - AI Type",
            format!(
                "< {} >",
                app.player_1.as_ref().unwrap().get_ai_type().unwrap()
            )
        )),
        Span::from(format!(
            "{:<30}{:>20}",
            "1 - Depth of tree",
            format!("< {} >", app.player_1.as_ref().unwrap().get_depth())
        ))
        .style(if is_qlearning_1 {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        }),
        Span::from(format!(
            "{:<30}{:>20}",
            "1 - Heuristic Type",
            format!("< {} >", app.player_1.as_ref().unwrap().get_heuristic())
        ))
        .style(if is_qlearning_1 {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        }),
        Span::from(format!(
            "{:<30}{:>20}",
            "1 - Matrix Heuristic",
            format!(
                "< {} >",
                app.player_1.as_ref().unwrap().get_heuristic_matrix()
            )
        ))
        .style(if is_qlearning_1 || heuristic_dont_use_matrix_1 {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        }),
        Span::from(format!(
            "{:<30}{:>20}",
            "1 - Double Threading",
            format!(
                "< {} >",
                app.player_1.as_ref().unwrap().get_double_threading()
            )
        ))
        .style(if !can_use_double_threading_1 {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        }),
        Span::from(format!(
            "{:<30}{:>20}",
            "2 - AI Type",
            format!(
                "< {} >",
                app.player_2.as_ref().unwrap().get_ai_type().unwrap()
            )
        )),
        Span::from(format!(
            "{:<30}{:>20}",
            "2 - Depth of tree",
            format!("< {} >", app.player_2.as_ref().unwrap().get_depth())
        ))
        .style(if is_qlearning_2 {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        }),
        Span::from(format!(
            "{:<30}{:>20}",
            "2 - Heuristic Type",
            format!("< {} >", app.player_2.as_ref().unwrap().get_heuristic())
        ))
        .style(if is_qlearning_2 {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        }),
        Span::from(format!(
            "{:<30}{:>20}",
            "2 - Matrix Heuristic",
            format!(
                "< {} >",
                app.player_2.as_ref().unwrap().get_heuristic_matrix()
            )
        ))
        .style(if is_qlearning_2 || heuristic_dont_use_matrix_2 {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        }),
        Span::from(format!(
            "{:<30}{:>20}",
            "2 - Double Threading",
            format!(
                "< {} >",
                app.player_2.as_ref().unwrap().get_double_threading()
            )
        ))
        .style(if !can_use_double_threading_2 {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        }),
        Span::from(format!("{:<50}", "Play")),
    ];

    let layout = centered_rect(60, 15, chunks[1]);

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
