use ratatui::{
    layout::{Constraint, Direction, Flex, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge},
    Frame,
};

use crate::gui::app::App;

pub fn q_learning_loading_screen(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(5),
            Constraint::Fill(1),
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
        .block(
            Block::default()
                .title("Loading Q-Learning")
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
        .ratio(app.qlearning_loading.unwrap_or(0.0) as f64)
        .label(label);
    frame.render_widget(gauge, chunks[1]);
}
