//! Q-Learning training configuration screen implementation.
//!
//! This module provides the user interface for configuring and initiating
//! Q-Learning AI training sessions. It allows users to set training parameters
//! and monitor the reinforcement learning process for developing improved
//! AI strategies through self-play and experimentation.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{Block, BorderType, List, Padding, Paragraph},
    Frame,
};

use crate::{
    ai::heuristic::HeuristicType,
    game::player::Player,
    gui::{
        app::App,
        ui::{centered_rect, footer, widget_title},
    },
};

/// Renders the Q-Learning training parameters configuration screen.
///
/// This function creates and displays the interface for configuring Q-Learning
/// AI training sessions. It provides controls for setting training parameters
/// that determine the learning process, duration, and strategy development
/// approach for the reinforcement learning algorithm.
///
/// # Screen Layout
///
/// The screen is organized into four main sections:
/// - **Title area**: Application branding and screen identification
/// - **Parameter list**: Interactive training configuration options
/// - **Message area**: Status updates and error notifications
/// - **Footer**: Navigation instructions and available controls
///
/// # Training Parameters
///
/// Users can configure the following Q-Learning parameters:
/// - **Number of epochs**: Total training iterations/sessions to run
/// - **Max steps per epoch**: Maximum moves allowed in each training game
/// - **Heuristic Type**: Evaluation function for position assessment during learning
/// - **Matrix Heuristic**: Strategic focus matrix for position evaluation
///
/// # Learning Configuration
///
/// The interface allows fine-tuning of the learning process:
/// - **Epoch count**: Controls training duration and convergence time
/// - **Step limits**: Prevents infinite games and controls training efficiency
/// - **Heuristic selection**: Influences how the AI evaluates positions during learning
/// - **Matrix compatibility**: Some heuristics don't use matrix evaluation
///
/// # Dynamic UI Behavior
///
/// The interface adapts based on selected heuristic type:
/// - **Matrix heuristics**: Disabled (grayed out) for Absolute and Mobility types
/// - **Parameter validation**: Ensures compatible combinations of settings
/// - **Visual feedback**: Unavailable options shown in dark gray
///
/// # Training Process
///
/// When training is initiated:
/// 1. Q-Learning AI plays games against itself or opponents
/// 2. Algorithm learns from win/loss outcomes and position evaluations
/// 3. Q-table is updated with improved strategy knowledge
/// 4. Process repeats for specified number of epochs
///
/// # Performance Considerations
///
/// Training parameters affect performance and results:
/// - **Higher epochs**: Better learning but longer training time
/// - **More steps**: Allows complex games but increases computation
/// - **Heuristic choice**: Influences learning speed and strategy quality
///
/// # Arguments
///
/// * `frame` - Ratatui frame for rendering UI widgets
/// * `app` - Mutable application state containing Q-Learning configuration
///
/// # Layout Specifications
///
/// - **Configuration area**: Centered rectangle (60% width, 8 rows height)
/// - **Responsive design**: Adapts to various terminal dimensions
/// - **Visual hierarchy**: Clear separation between sections
///
/// # Examples
///
/// ```rust
/// // Called by the main UI router when on Q-Learning screen
/// match app.current_screen {
///     CurrentScreen::QLearning => q_learning_parameters_screen(&mut frame, &mut app),
///     // ... other screen handlers
/// }
/// ```
pub fn q_learning_parameters_screen(frame: &mut Frame, app: &mut App) {
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

    let heuristic_dont_use_matrix = app.qlearning_parameters.as_ref().unwrap().get_heuristic()
        == HeuristicType::Absolute
        || app.qlearning_parameters.as_ref().unwrap().get_heuristic() == HeuristicType::Mobility;

    let items = [
        Span::from(format!(
            "{:<30}{:>20}",
            "Number of epoch",
            format!(
                "< {} >",
                app.qlearning_parameters.as_ref().unwrap().get_epochs()
            )
        )),
        Span::from(format!(
            "{:<30}{:>20}",
            "Max steps per epoch",
            format!(
                "< {} >",
                app.qlearning_parameters.as_ref().unwrap().get_max_step()
            )
        )),
        Span::from(format!(
            "{:<30}{:>20}",
            "Heuristic Type",
            format!(
                "< {} >",
                app.qlearning_parameters.as_ref().unwrap().get_heuristic()
            )
        )),
        Span::from(format!(
            "{:<30}{:>20}",
            "Matrix Heuristic",
            format!(
                "< {} >",
                app.qlearning_parameters
                    .as_ref()
                    .unwrap()
                    .get_heuristic_matrix()
            )
        ))
        .style(if heuristic_dont_use_matrix {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        }),
        Span::from(format!("{:<50}", "Start training")),
    ];

    let layout = centered_rect(60, 9, chunks[1]);

    let list = List::new(items)
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(" QLearning training parameters ")
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
