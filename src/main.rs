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
                        app.gui_play_turn();
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
                                app.start_game(
                                    Box::new(Human::new(Cell::Black)),
                                    Box::new(Human::new(Cell::White)),
                                );
                            }
                            Some(1) => {
                                app.start_game(
                                    Box::new(Human::new(Cell::Black)),
                                    Box::new(AIMinMax::new(
                                        3,
                                        HeuristicType::Absolute,
                                        Cell::White,
                                        None,
                                    )),
                                );
                            }
                            Some(2) => {
                                app.start_game(
                                    Box::new(AIAlphaBeta::new(
                                        MAX_DEPTH,
                                        HeuristicType::Mixte,
                                        Cell::Black,
                                        Some(MATRIX_B),
                                    )),
                                    Box::new(AIAlphaBeta::new(
                                        MAX_DEPTH,
                                        HeuristicType::Mixte,
                                        Cell::White,
                                        Some(MATRIX_B),
                                    )),
                                );
                            }
                            Some(3) => {
                                app.current_screen = CurrentScreen::QLearningParameters;
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    CurrentScreen::Game => match key.code {
                        KeyCode::Char('q') => app.quit_game(),
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
                            if !app.board.as_ref().unwrap().is_game_over() && its_a_human_player {
                                if let Some(_) = app.selected_cell.as_ref() {
                                    app.gui_play_turn();
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
}
