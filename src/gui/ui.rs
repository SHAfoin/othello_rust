use ratatui::{
    crossterm::terminal,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Clear, List, ListItem, ListState, Padding, Paragraph, Widget,
    },
    Frame,
};
use tui_big_text::{BigText, PixelSize};

use crate::{
    consts::SIZE,
    game::board,
    gui::app::{App, CurrentScreen},
};

use crate::game::{board::Board, cell::Cell};

// WIDGET TOUT FAIT POUR DES POPUPS CENTRÉES
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Coupe le rectangle verticalement en trois parties, avec la partie du milieu ayant la hauteur de `percent_y` centrée
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // On prend la partie du milieu (dans le split) et on la centre horizontalement aussi, et on retourne que le milieu
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

// Dessiner l'UI
pub fn ui(frame: &mut Frame, app: &mut App) {
    // Set background color for the entire frame
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
        frame.area(),
    );

    frame.render_widget(Clear, frame.area());

    match app.current_screen {
        CurrentScreen::Main => {
            main_screen(frame, app);
        }
        CurrentScreen::Game => {
            game_screen(frame, app);
        }
        CurrentScreen::Tutorial => {
            tutorial_screen(frame, app);
        }
        CurrentScreen::HumanVsAI => {
            human_vs_ai_screen(frame, app);
        }
        CurrentScreen::AIvsAI => {
            ai_vs_ai_screen(frame, app);
        }
        CurrentScreen::QLearningParameters => {
            q_learning_parameters_screen(frame, app);
        }
    }
}

pub fn main_screen(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(8), // au moins 1 ligne de hauteur pour la liste, prend plus si possible
            Constraint::Length(8), // au moins 1 ligne de hauteur pour la liste, prend plus si possible
            Constraint::Fill(1),
            Constraint::Length(1), // 3 lignes FIXES de hauteur pour le footer
        ])
        .flex(Flex::Center)
        .split(frame.area());

    let big_text = BigText::builder()
        .alignment(Alignment::Center)
        .pixel_size(PixelSize::Full)
        .lines(vec!["Othello".into()])
        .build();

    frame.render_widget(big_text, chunks[1]);

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
        .split(chunks[2]);

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

    let footer = footer(
        frame,
        app,
        " (↑↓) to choose / (ENTER) to validate / (q) to quit ",
    );
    frame.render_widget(footer, chunks[4]);
}

pub fn footer<'a>(frame: &mut Frame, app: &App, text: &'a str) -> Paragraph<'a> {
    let current_navigation_text =
        Span::styled(text, Style::default().fg(Color::Red)).into_centered_line();
    let mode_footer = Paragraph::new(Line::from(current_navigation_text)).block(Block::default());
    mode_footer
}

pub fn game_screen(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(1)])
        .flex(Flex::Center)
        .split(frame.area());

    let main_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(66), Constraint::Percentage(40)])
        .split(chunks[0]);

    let left_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(30), Constraint::Length(5)])
        .split(main_area[0]);

    let right_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(main_area[1]);

    let game_board = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(" Game Board ")
        .title_alignment(Alignment::Center)
        .padding(Padding::uniform(0));

    frame.render_widget(&game_board, left_area[0]);

    let game_board_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(58),
            Constraint::Fill(1),
        ])
        .split(left_area[0]);

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

    let game_message = Paragraph::new(app.game_message.clone().unwrap_or("No message".into()))
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .padding(Padding::uniform(1)),
        )
        .alignment(Alignment::Center);

    frame.render_widget(game_message, left_area[1]);

    let history_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title("Game History")
        .title_alignment(Alignment::Center)
        .padding(Padding::uniform(1));

    let mut game_history = List::default();
    if let Some(board) = &app.board {
        let history_items: Vec<ListItem> = board
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
        game_history = List::new(history_items);
    }

    frame.render_stateful_widget(
        game_history.block(history_block),
        right_area[0],
        &mut ListState::default(),
    );

    let game_score = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(right_area[1]);

    let mut black_score = String::new();
    let mut white_score = String::new();

    if let Some(board) = &app.board {
        black_score = board.get_nb_discs(Cell::Black).unwrap().to_string();
        white_score = board.get_nb_discs(Cell::White).unwrap().to_string();
    }

    let player1_score_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Blue))
        .title("BLACK")
        .title_alignment(Alignment::Center);
    let player2_score_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Yellow))
        .title("WHITE")
        .title_alignment(Alignment::Center);

    frame.render_widget(player1_score_block, game_score[0]);
    frame.render_widget(player2_score_block, game_score[1]);

    let player1_score_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Min(4), Constraint::Fill(1)])
        .split(game_score[0]);
    let player2_score_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Min(4), Constraint::Fill(1)])
        .split(game_score[1]);

    let player1_score = BigText::builder()
        .alignment(Alignment::Center)
        .pixel_size(PixelSize::Quadrant)
        .style(Style::default().fg(Color::Blue))
        .lines(vec![format!("{}", black_score).into()])
        .build();

    let player2_score = BigText::builder()
        .alignment(Alignment::Center)
        .pixel_size(PixelSize::Quadrant)
        .style(Style::default().fg(Color::Yellow))
        .lines(vec![format!("{}", white_score).into()])
        .build();

    frame.render_widget(player1_score, player1_score_layout[1]);
    frame.render_widget(player2_score, player2_score_layout[1]);

    let footer = footer(
        frame,
        app,
        " (↑↓←→) to choose / (ENTER) to play / (q) to quit ",
    );
    frame.render_widget(footer, chunks[1]);
}

pub fn tutorial_screen(frame: &mut Frame, app: &App) {}

pub fn human_vs_ai_screen(frame: &mut Frame, app: &App) {}

pub fn ai_vs_ai_screen(frame: &mut Frame, app: &App) {}

pub fn q_learning_parameters_screen(frame: &mut Frame, app: &App) {}
