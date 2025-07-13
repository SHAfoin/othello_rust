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
    ai::{
        alphabeta::AIAlphaBeta,
        common::{AIHeuristicMatrix, AIType, HeuristicType},
        minmax::AIMinMax,
        qlearning::QLearning,
    },
    consts::MAX_DEPTH,
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
use std::{error::Error, thread::current, time::Instant};
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
        Some(AIHeuristicMatrix::B),
    );

    let mut qplayer = QLearning::new(
        1000,                       // Maximum number of steps
        HeuristicType::Global,      // Heuristic type to use
        Some(AIHeuristicMatrix::A), // Heuristic matrix to use
        10000,                      // Number of epochs
        Cell::Black,                // Color of the player
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
                                app.player_1 = Some(Box::new(Human::new(Cell::Black)));
                                app.player_2 = Some(Box::new(Human::new(Cell::White)));
                                app.start_game();
                            }
                            Some(1) => {
                                app.player_1 = Some(Box::new(Human::new(Cell::Black)));
                                app.player_2 = Some(Box::new(AIMinMax::new(
                                    3,
                                    HeuristicType::Absolute,
                                    Cell::White,
                                    None,
                                    false,
                                )));

                                app.current_screen = CurrentScreen::HumanVsAI;
                                app.current_mode.select_first();
                            }
                            Some(2) => {
                                // app.start_game(
                                //     Box::new(AIAlphaBeta::new(
                                //         MAX_DEPTH,
                                //         HeuristicType::Mixte,
                                //         Cell::Black,
                                //         Some(AIHeuristicMatrix::A),
                                //         false,
                                //     )),
                                //     Box::new(AIAlphaBeta::new(
                                //         MAX_DEPTH,
                                //         HeuristicType::Mixte,
                                //         Cell::White,
                                //         Some(AIHeuristicMatrix::B),
                                //         false,
                                //     )),
                                // );
                                app.current_screen = CurrentScreen::AIVsAI;
                                app.current_mode.select_first();
                            }
                            Some(3) => {
                                app.current_screen = CurrentScreen::QLearningParameters;
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    CurrentScreen::Game => match key.code {
                        KeyCode::Char('q') => app.current_screen = CurrentScreen::Exit,
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
                        KeyCode::Char('t') => {
                            app.current_screen = CurrentScreen::Tutorial;
                        }
                        _ => {}
                    },
                    CurrentScreen::Tutorial => match key.code {
                        KeyCode::Char('q') => app.current_screen = CurrentScreen::Game,
                        _ => {}
                    },
                    CurrentScreen::Exit => match key.code {
                        KeyCode::Char('y') | KeyCode::Char('q') => app.quit_game(),
                        KeyCode::Char('n') => app.current_screen = CurrentScreen::Game,
                        _ => {}
                    },
                    CurrentScreen::HumanVsAI => match key.code {
                        KeyCode::Char('q') => {
                            app.current_mode.select_first();
                            app.current_screen = CurrentScreen::Main;
                        }

                        KeyCode::Enter => match app.current_mode.selected() {
                            Some(4) => {
                                app.current_screen = CurrentScreen::Game;
                                app.current_mode.select_first();
                                app.start_game();
                            }
                            _ => {}
                        },

                        KeyCode::Up => {
                            app.current_mode.select_previous();
                        }
                        KeyCode::Down => {
                            app.current_mode.select_next();
                        }
                        KeyCode::Left => match app.current_mode.selected() {
                            Some(0) => {
                                // changer l'IA en gardant les mêmes paramètres, sauf Q learning
                                match app.player_2.as_mut().unwrap().get_ai_type() {
                                    Some(AIType::AlphaBeta) => {
                                        app.player_2 = Some(Box::new(AIMinMax::new(
                                            app.player_2
                                                .as_ref()
                                                .unwrap()
                                                .get_depth()
                                                .unwrap_or(MAX_DEPTH),
                                            app.player_2
                                                .as_ref()
                                                .unwrap()
                                                .get_heuristic()
                                                .unwrap_or(HeuristicType::Mixte),
                                            Cell::White,
                                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                                            false,
                                        )));
                                    }
                                    Some(AIType::MinMax) => {
                                        app.player_2 = Some(Box::new(QLearning::new(
                                            1000,
                                            app.player_2
                                                .as_ref()
                                                .unwrap()
                                                .get_heuristic()
                                                .unwrap_or(HeuristicType::Mixte),
                                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                                            10000,
                                            Cell::White,
                                        )));
                                    }
                                    Some(AIType::QLearning) => {
                                        app.player_2 = Some(Box::new(AIAlphaBeta::new(
                                            app.player_2
                                                .as_ref()
                                                .unwrap()
                                                .get_depth()
                                                .unwrap_or(MAX_DEPTH),
                                            app.player_2
                                                .as_ref()
                                                .unwrap()
                                                .get_heuristic()
                                                .unwrap_or(HeuristicType::Mixte),
                                            Cell::White,
                                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                                        )));
                                    }
                                    _ => {}
                                }
                            }
                            Some(1) => {
                                // si pas Q learning
                                // diminuer la depth de 1 si c'est pas inférieur à 1
                                // sinon : message error, pas adpaté au qlearning
                            }
                            Some(2) => {
                                // si pas Q learning
                                // heuristique previous
                                // sinon : message error, pas adpaté au qlearning
                            }
                            Some(3) => {
                                // si l'heuristique le demande && pas Q learning:
                                // matrice précédente
                                // sinon : message error, pas adpaté pour cette heuristique
                                // sinon : message error, pas adpaté au qlearning
                            }
                            _ => {}
                        },
                        KeyCode::Right => match app.current_mode.selected() {
                            Some(0) => {
                                // changer l'IA en gardant les mêmes paramètres, sauf Q learning
                                match app.player_2.as_mut().unwrap().get_ai_type() {
                                    Some(AIType::AlphaBeta) => {
                                        app.player_2 = Some(Box::new(QLearning::new(
                                            1000,
                                            app.player_2
                                                .as_ref()
                                                .unwrap()
                                                .get_heuristic()
                                                .unwrap_or(HeuristicType::Mixte),
                                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                                            10000,
                                            Cell::White,
                                        )));
                                    }
                                    Some(AIType::MinMax) => {
                                        app.player_2 = Some(Box::new(AIAlphaBeta::new(
                                            app.player_2
                                                .as_ref()
                                                .unwrap()
                                                .get_depth()
                                                .unwrap_or(MAX_DEPTH),
                                            app.player_2
                                                .as_ref()
                                                .unwrap()
                                                .get_heuristic()
                                                .unwrap_or(HeuristicType::Mixte),
                                            Cell::White,
                                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                                        )));
                                    }
                                    Some(AIType::QLearning) => {
                                        app.player_2 = Some(Box::new(AIMinMax::new(
                                            app.player_2
                                                .as_ref()
                                                .unwrap()
                                                .get_depth()
                                                .unwrap_or(MAX_DEPTH),
                                            app.player_2
                                                .as_ref()
                                                .unwrap()
                                                .get_heuristic()
                                                .unwrap_or(HeuristicType::Mixte),
                                            Cell::White,
                                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                                            false,
                                        )));
                                    }
                                    _ => {}
                                }
                            }
                            Some(1) => {
                                // si pas Q learning
                                // augmenter la depth de 1 si c'est pas supérieur à MAX_DEPTH
                                // sinon : message error, pas adpaté au qlearning
                                // sinon : limite par la constante MAX_DEPTH
                            }
                            Some(2) => {
                                // si pas Q learning
                                // heuristique next
                                // sinon : message error, pas adpaté au qlearning
                            }
                            Some(3) => {
                                // si l'heuristique le demande && pas Q learning:
                                // matrice suivante
                                // sinon : message error, pas adpaté pour cette heuristique
                                // sinon : message error, pas adpaté au qlearning
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    CurrentScreen::AIVsAI => match key.code {
                        KeyCode::Char('q') => {
                            app.current_mode.select_first();
                            app.current_screen = CurrentScreen::Main;
                        }

                        KeyCode::Up => {
                            app.current_mode.select_previous();
                        }
                        KeyCode::Down => {
                            app.current_mode.select_next();
                        }
                        _ => {}
                    },
                    // CurrentScreen::QLearningParameters => match key.code {},
                    _ => return Ok(()),
                }
            }
        }
    }
}
