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
            "1 - Double Threading",
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
