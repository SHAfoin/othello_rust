use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    widgets::{Block, BorderType, Padding, Paragraph},
    Frame,
};

use crate::gui::{
    app::App,
    ui::{centered_rect, footer, widget_title},
};

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

fn widget_tutorial(frame: &mut Frame, app: &App, area: Rect) {
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
