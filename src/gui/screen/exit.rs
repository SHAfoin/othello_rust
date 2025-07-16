use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    widgets::{Block, BorderType, Padding, Paragraph},
    Frame,
};

use crate::gui::{
    app::App,
    ui::{centered_rect, widget_title},
};

pub fn exit_screen(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(1)])
        .flex(Flex::Center)
        .split(frame.area());

    widget_title(frame, app, chunks[0]);

    let exit_message = Paragraph::new(
        "Are you sure you want to exit?\n\
    y/n",
    )
    .block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title(" Exit Confirmation ")
            .title_alignment(Alignment::Center)
            .padding(Padding::uniform(1)),
    )
    .alignment(Alignment::Center);

    let exit_layout = centered_rect(40, 6, frame.area());
    frame.render_widget(exit_message, exit_layout);
}
