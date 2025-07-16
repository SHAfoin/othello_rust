use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{Block, BorderType, List, ListItem, ListState, Padding, Paragraph},
    Frame,
};
use tui_big_text::{BigText, PixelSize};

use crate::{
    consts::SIZE,
    game::cell::Cell,
    gui::{app::App, ui::footer},
};

pub fn game_screen(frame: &mut Frame, app: &mut App) {
    // Ecran découpé en zone de base + footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(1)])
        .flex(Flex::Center)
        .split(frame.area());

    // Zone de base découpée en zone gauche / droite
    let main_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(66), Constraint::Percentage(40)])
        .split(chunks[0]);

    // Zone droite découpée en deux : historique et score
    let right_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Percentage(30),
        ])
        .split(main_area[1]);

    // Zone des scores
    let score_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(right_area[3]);

    widget_grid(frame, app, main_area[0]);

    widget_history(frame, app, right_area[0]);

    widget_timer(frame, app, right_area[1]);

    widget_message(frame, app, right_area[2]);

    // Récupérer les scores des joueurs
    let mut black_score = String::new();
    let mut white_score = String::new();

    if let Some(board) = &app.board {
        black_score = board.get_nb_discs(Cell::Black).unwrap().to_string();
        white_score = board.get_nb_discs(Cell::White).unwrap().to_string();
    }

    widget_score(frame, app, score_area[0], black_score, Color::Blue, "BLACK");

    widget_score(
        frame,
        app,
        score_area[1],
        white_score,
        Color::Yellow,
        "WHITE",
    );

    // Footer
    footer(
        frame,
        app,
        chunks[1],
        " (↑↓←→) to choose / (ENTER) to play / (t) for tutorial / (q) to quit ",
    );
}

fn widget_grid(frame: &mut Frame, app: &mut App, area: Rect) {
    // Zone de jeu
    let game_board = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(" Game Board ")
        .title_alignment(Alignment::Center)
        .padding(Padding::uniform(0));

    frame.render_widget(&game_board, area);

    // Centrer la grille horizontalement
    let game_board_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(58),
            Constraint::Fill(1),
        ])
        .split(area);

    // Centrer la grille verticalement
    let game_board_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(27),
            Constraint::Fill(1),
        ])
        .split(game_board_horizontal[1]);

    // Grille de jeu
    if let Some(board) = &app.board {
        let col_constraints = (0..SIZE + 1).map(|_| Constraint::Length(4));
        let row_constraints = (0..SIZE + 1).map(|_| Constraint::Length(2));
        let horizontal = Layout::horizontal(col_constraints).spacing(2);
        let vertical = Layout::vertical(row_constraints).spacing(1);

        let rows = vertical.split(game_board_vertical[1]);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            if i != 0 {
                if i % 9 == 0 && i != 0 {
                    frame.render_widget(
                        Paragraph::new(Span::raw((i / 9).to_string()).into_centered_line())
                            .block(Block::default()),
                        cell,
                    );
                } else if i < 9 {
                    frame.render_widget(
                        Paragraph::new(
                            Span::raw(char::from_u32(i as u32 + 64).unwrap().to_string())
                                .into_centered_line(),
                        )
                        .block(Block::default()),
                        cell,
                    );
                } else {
                    match board.get_cell(i / 9 - 1, i % 9 - 1) {
                        Ok(Cell::Black) => {
                            frame.render_widget(
                                Block::default().style(Style::default().bg(Color::Blue)),
                                cell,
                            );
                        }
                        Ok(Cell::White) => {
                            frame.render_widget(
                                Block::default().style(Style::default().bg(Color::Yellow)),
                                cell,
                            );
                        }
                        _ => {
                            frame.render_widget(Block::bordered().style(Style::default()), cell);
                        }
                    }
                    if app.selected_cell == Some((i / 9 - 1, i % 9 - 1)) {
                        frame.render_widget(
                            Block::new()
                                .border_type(BorderType::Double)
                                .style(Style::default().reversed()),
                            cell,
                        );
                    }
                }
            }
        }
    } else {
        app.game_message = Some("No game board available !".to_string());
    }
}

fn widget_message(frame: &mut Frame, app: &App, area: Rect) {
    // Zone message de jeu
    let game_message = Paragraph::new(app.game_message.clone().unwrap_or("No message".into()))
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .padding(Padding::uniform(1)),
        )
        .alignment(Alignment::Center);

    frame.render_widget(game_message, area);
}

fn widget_history(frame: &mut Frame, app: &App, area: Rect) {
    // Zone historique du jeu
    let history_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title("Game History")
        .title_alignment(Alignment::Center)
        .padding(Padding::uniform(1));

    // Générer l'historique du jeu
    let mut game_history = List::default();
    if let Some(board) = &app.board {
        let mut history_items: Vec<ListItem> = board
            .get_history()
            .iter()
            .enumerate()
            .map(|(index, action)| {
                if action.coordinates.is_none() {
                    return ListItem::new(format!(
                        "Move {}: {} passed (no legal move).",
                        action.move_number, action.player_turn
                    ))
                    .style(Style::default().fg(
                        if action.color == Cell::Black {
                            Color::Blue
                        } else {
                            Color::Yellow
                        },
                    ));
                } else {
                    ListItem::new(format!(
                        "Move {}: {} played at {}. +{} discs.",
                        action.move_number,
                        action.player_turn,
                        action.coordinates.clone().unwrap_or("0".into()),
                        action.gained_discs.unwrap_or(0)
                    ))
                    .style(Style::default().fg(
                        if action.color == Cell::Black {
                            Color::Blue
                        } else {
                            Color::Yellow
                        },
                    ))
                }
            })
            .collect();
        history_items.reverse();
        game_history = List::new(history_items);
    }

    frame.render_stateful_widget(
        game_history.block(history_block),
        area,
        &mut ListState::default(),
    );
}

fn widget_score(frame: &mut Frame, app: &App, area: Rect, score: String, color: Color, name: &str) {
    let player_score_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(color))
        .title(name)
        .title_alignment(Alignment::Center);

    frame.render_widget(player_score_block, area);

    let player_score_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Min(4), Constraint::Fill(1)])
        .split(area);

    let player_score = BigText::builder()
        .alignment(Alignment::Center)
        .pixel_size(PixelSize::Quadrant)
        .style(Style::default().fg(color))
        .lines(vec![format!("{}", score).into()])
        .build();

    frame.render_widget(player_score, player_score_layout[1]);
}

fn widget_timer(frame: &mut Frame, app: &App, area: Rect) {
    if let Some(timer) = &app.timer {
        let elapsed = timer.elapsed();
        let minutes = elapsed.as_secs() / 60;
        let seconds = elapsed.as_secs() % 60;

        let timer_text = format!("{:02}:{:02}", minutes, seconds);
        let timer_paragraph = Paragraph::new(timer_text)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("Timer")
                    .title_alignment(Alignment::Center)
                    .padding(Padding::vertical(1)),
            )
            .alignment(Alignment::Center);

        frame.render_widget(timer_paragraph, area);
    }
}
