use ratatui::{
    crossterm::terminal,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style, Styled, Stylize},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Clear, List, ListItem, ListState, Padding, Paragraph, Widget,
    },
    Frame,
};
use tui_big_text::{BigText, PixelSize};

use crate::{
    ai::common::{AIType, HeuristicType},
    consts::SIZE,
    game::board,
    gui::app::{App, CurrentScreen},
};

use crate::game::{board::Board, cell::Cell};

// WIDGET TOUT FAIT POUR DES POPUPS CENTRÉES
fn centered_rect(length_x: u16, length_y: u16, r: Rect) -> Rect {
    // Coupe le rectangle verticalement en trois parties, avec la partie du milieu ayant la hauteur de `percent_y` centrée
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(length_y),
            Constraint::Fill(1),
        ])
        .split(r);

    // On prend la partie du milieu (dans le split) et on la centre horizontalement aussi, et on retourne que le milieu
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(length_x),
            Constraint::Fill(1),
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
        CurrentScreen::AIVsAI => {
            ai_vs_ai_screen(frame, app);
        }
        CurrentScreen::QLearningParameters => {
            q_learning_parameters_screen(frame, app);
        }
        CurrentScreen::Exit => {
            exit_screen(frame, app);
        }
    }
}

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

pub fn footer<'a>(frame: &mut Frame, app: &App, area: Rect, text: &'a str) {
    let current_navigation_text =
        Span::styled(text, Style::default().fg(Color::Red)).into_centered_line();
    let mode_footer = Paragraph::new(Line::from(current_navigation_text)).block(Block::default());
    frame.render_widget(mode_footer, area);
}

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
        .style(if is_qlearning {
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

pub fn ai_vs_ai_screen(frame: &mut Frame, app: &mut App) {
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

    let items = [
        format!("{:<30}{:>20}", "AI Type 1", "< MinMax >"),
        format!("{:<30}{:>20}", "Depth of tree 1", "< 10 >"),
        format!("{:<30}{:>20}", "Heuristic Type 1", "< Absolute >"),
        format!("{:<30}{:>20}", "Matrix Heuristic 1", "< A >"),
        format!("{:<30}{:>20}", "AI Type 2", "< MinMax >"),
        format!("{:<30}{:>20}", "Depth of tree 2", "< 10 >"),
        format!("{:<30}{:>20}", "Heuristic Type 2", "< Absolute >"),
        format!("{:<30}{:>20}", "Matrix Heuristic 2", "< A >"),
        format!("{:<50}", "Play"),
    ];

    let layout = centered_rect(60, 13, chunks[1]);

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
    let error_message = "ayoyooo";
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

pub fn q_learning_parameters_screen(frame: &mut Frame, app: &App) {}

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

fn widget_title(frame: &mut Frame, app: &App, area: Rect) {
    let big_text = BigText::builder()
        .alignment(Alignment::Center)
        .pixel_size(PixelSize::Full)
        .lines(vec!["Othello".into()])
        .build();

    frame.render_widget(big_text, area);
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
