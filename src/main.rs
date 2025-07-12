//  ===================================================================
//
//  ███████╗██╗  ██╗ █████╗         ███████╗ ██████╗ ██╗███╗   ██╗
//  ██╔════╝██║  ██║██╔══██╗        ██╔════╝██╔═══██╗██║████╗  ██║
//  ███████╗███████║███████║        █████╗  ██║   ██║██║██╔██╗ ██║
//  ╚════██║██╔══██║██╔══██║        ██╔══╝  ██║   ██║██║██║╚██╗██║
//  ███████║██║  ██║██║  ██║███████╗██║     ╚██████╔╝██║██║ ╚████║
//  ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝      ╚═════╝ ╚═╝╚═╝  ╚═══╝
//
//  @file : src\main.rs
//  @description : Othello game implementation in Rust.
//  @author : SALTEL Baptiste
//  @date : 08/07/2025
//  @version : 1.0
//  @license : none
//
//  ===================================================================

mod ai;
mod consts;
mod game;
mod gui;
mod human;

use crate::{
    ai::{alphabeta::AIAlphaBeta, common::HeuristicType, minmax::AIMinMax, qlearning::QLearning},
    consts::{MATRIX_B, MAX_DEPTH},
    game::{
        board::{Board, Player},
        cell::Cell,
    },
    gui::{
        app::{self, App, CurrentScreen},
        ui::ui,
    },
    human::Human,
};

use ratatui::crossterm::event;
use ratatui::crossterm::event::DisableMouseCapture;
use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyEventKind;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use ratatui::crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use ratatui::prelude::Backend;
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use std::{error::Error, time::Instant};
use std::{io, time::Duration};

pub fn start_game() {
    let mut board = Board::new();
    // let player1 = Human::new(Cell::Black);
    // let player1 = AIAlphaBeta::new(
    //     MAX_DEPTH,               // Depth of the search tree
    //     HeuristicType::Absolute, // Heuristic type to use
    //     Cell::Black,
    //     None,
    // );
    // let player2 = Human::new(player1.get_color().get_opponent());
    let player2 = AIAlphaBeta::new(
        MAX_DEPTH,            // Depth of the search tree
        HeuristicType::Mixte, // Heuristic type to use
        Cell::White,
        Some(MATRIX_B),
    );

    let mut qplayer = QLearning::new(
        1000,                  // Maximum number of steps
        HeuristicType::Global, // Heuristic type to use
        10000,                 // Number of epochs
        Cell::Black,           // Color of the player
    );

    qplayer.import_q_table("foo.txt");
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to Othello!\n");
    println!("================\n");

    // start_game();
    // q.try_q_learning();

    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Bien désactiver tout ça à la fin !
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let mut player_turn;
    let mut its_a_human_player = false;

    loop {
        // Dessiner en boucle sur le terminal

        terminal.draw(|f| ui(f, app))?;
        match app.current_screen {
            CurrentScreen::Game => {
                match app.board.as_ref().unwrap().get_player_turn() {
                    Cell::Black => {
                        player_turn = &app.player_1;
                    }
                    Cell::White => {
                        player_turn = &app.player_2;
                    }
                    _ => {
                        player_turn = &None;
                    }
                }
                if let Some(player) = player_turn {
                    its_a_human_player = player.is_human();
                    if !player.is_human() && !app.board.as_ref().unwrap().is_game_over() {
                        gui_play_turn(app);
                    }
                }
            }
            _ => {}
        }

        // Gérer les events
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    // Skip events that are not KeyEventKind::Press
                    continue;
                }
                // Gestion selon l'écran actuel
                match app.current_screen {
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Up => {
                            app.current_mode.select_previous();
                        }
                        KeyCode::Down => {
                            app.current_mode.select_next();
                        }
                        KeyCode::Enter => match app.current_mode.selected() {
                            Some(0) => {
                                app.current_screen = CurrentScreen::Game;
                                app.board = Some(Board::new());
                                app.player_1 = Some(Box::new(Human::new(Cell::Black)));
                                app.player_2 = Some(Box::new(Human::new(Cell::White)));
                                app.game_message = Some(format!(
                                    "It's {} turn !",
                                    app.board.as_ref().unwrap().get_player_turn()
                                ));
                            }
                            Some(1) => {
                                app.current_screen = CurrentScreen::Game;
                                app.board = Some(Board::new());
                                app.player_1 = Some(Box::new(Human::new(Cell::Black)));
                                app.player_2 = Some(Box::new(AIMinMax::new(
                                    3,
                                    HeuristicType::Absolute,
                                    Cell::White,
                                    None,
                                )));
                                app.game_message = Some(format!(
                                    "It's {} turn !",
                                    app.board.as_ref().unwrap().get_player_turn()
                                ));
                            }
                            Some(2) => {
                                app.current_screen = CurrentScreen::Game;
                                app.board = Some(Board::new());
                                app.player_1 = Some(Box::new(AIMinMax::new(
                                    3,
                                    HeuristicType::Absolute,
                                    Cell::Black,
                                    None,
                                )));
                                app.player_2 = Some(Box::new(AIMinMax::new(
                                    3,
                                    HeuristicType::Absolute,
                                    Cell::White,
                                    None,
                                )));
                                app.game_message = Some(format!(
                                    "It's {} turn !",
                                    app.board.as_ref().unwrap().get_player_turn()
                                ));
                            }
                            Some(3) => {
                                app.current_screen = CurrentScreen::QLearningParameters;
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    CurrentScreen::Game => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Up => {
                            if !app.board.as_ref().unwrap().is_game_over() && its_a_human_player {
                                app.select_cell_key(KeyCode::Up);
                            }
                        }
                        KeyCode::Down => {
                            if !app.board.as_ref().unwrap().is_game_over() && its_a_human_player {
                                app.select_cell_key(KeyCode::Down);
                            }
                        }
                        KeyCode::Left => {
                            if !app.board.as_ref().unwrap().is_game_over() && its_a_human_player {
                                app.select_cell_key(KeyCode::Left);
                            }
                        }
                        KeyCode::Right => {
                            if !app.board.as_ref().unwrap().is_game_over() && its_a_human_player {
                                app.select_cell_key(KeyCode::Right);
                            }
                        }
                        KeyCode::Enter => {
                            if its_a_human_player {
                                if let Some(_) = app.selected_cell.as_ref() {
                                    gui_play_turn(app);
                                }
                            }
                        }
                        _ => {}
                    },
                    // CurrentScreen::Tutorial => match key.code {},
                    // CurrentScreen::HumanVsAI => match key.code {},
                    // CurrentScreen::AIvsAI => match key.code {},
                    // CurrentScreen::QLearningParameters => match key.code {},
                    _ => return Ok(()),
                }
            }
        }
    }

    fn gui_play_turn(app: &mut App) {
        let mut play_turn_result = Err("Error in Enter".to_string());
        let mut new_message = None;
        if let Some(board) = &mut app.board {
            if !board.is_game_over() {
                match board.get_player_turn() {
                    Cell::Black => {
                        if let Some(player) = &app.player_1 {
                            play_turn_result = player.play_turn(board, app.selected_cell);
                        }
                    }
                    Cell::White => {
                        if let Some(player) = &app.player_2 {
                            play_turn_result = player.play_turn(board, app.selected_cell);
                        }
                    }
                    _ => {
                        play_turn_result = Err("Invalid player turn".to_string());
                    }
                }

                match play_turn_result {
                    Err(e) => {
                        app.set_game_message(Some(e));
                    }
                    Ok(history_action) => {
                        board.add_to_history(history_action);

                        if board.check_game_over() {
                            if let Some(winner) = board.get_winner() {
                                new_message = Some(format!("Game over! {} is the WINNER!", winner));
                            } else {
                                new_message = Some("Game over! It's a draw!".to_string());
                            }
                            if let Some(message) = new_message {
                                app.set_game_message(Some(message));
                            }
                        } else {
                            board.next_turn();
                            new_message = Some(format!("It's {} turn !", board.get_player_turn()));
                            if let Some(message) = new_message {
                                app.set_game_message(Some(message));
                            }
                        }
                    }
                }
            }
        }
    }
}
