use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Gauge},
    Frame,
};

use crate::gui::{app::App, ui::footer};

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
