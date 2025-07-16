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
