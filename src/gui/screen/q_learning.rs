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

    let layout = centered_rect(60, 8, chunks[1]);

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
