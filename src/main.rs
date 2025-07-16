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
        heuristic::{AIHeuristicMatrix, AIType, HeuristicType},
        minmax::AIMinMax,
        qlearning::QLearning,
    },
    consts::{MAX_DEPTH, QLEARNING_MAX_EPOCHS},
    game::{
        board::{Board, Player},
        cell::Cell,
    },
    gui::{
        app::{self, App, CurrentScreen},
        control::main::main_control,
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
        AIHeuristicMatrix::B,
    );

    let mut qplayer = QLearning::new(
        64,                    // Maximum number of steps
        HeuristicType::Global, // Heuristic type to use
        AIHeuristicMatrix::A,  // Heuristic matrix to use
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
                    CurrentScreen::Main => main_control(app, key),
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
                            app.game_message = None;
                            app.current_screen = CurrentScreen::Main;
                        }

                        KeyCode::Enter => match app.current_mode.selected() {
                            Some(5) => {
                                let mut game_ready = false;

                                if app.player_2.as_ref().unwrap().get_ai_type().unwrap()
                                    == AIType::QLearning
                                {
                                    match app
                                        .player_2
                                        .as_mut()
                                        .unwrap()
                                        .import_q_table_file("q_table_player_2.json")
                                    {
                                        Ok(_) => {
                                            game_ready = true;
                                        }
                                        Err(e) => {
                                            game_ready = false;
                                            app.set_game_message(Some(e));
                                        }
                                    }
                                }
                                if game_ready {
                                    app.current_screen = CurrentScreen::Game;
                                    app.current_mode.select_first();
                                    app.start_game();
                                }
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
                                            app.player_2.as_ref().unwrap().get_depth(),
                                            app.player_2.as_ref().unwrap().get_heuristic(),
                                            Cell::White,
                                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                                            false,
                                        )));
                                    }
                                    Some(AIType::MinMax) => {
                                        app.player_2 = Some(Box::new(QLearning::new(
                                            1000,
                                            app.player_2.as_ref().unwrap().get_heuristic(),
                                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                                            10000,
                                            Cell::White,
                                        )));
                                    }
                                    Some(AIType::QLearning) => {
                                        app.player_2 = Some(Box::new(AIAlphaBeta::new(
                                            app.player_2.as_ref().unwrap().get_depth(),
                                            app.player_2.as_ref().unwrap().get_heuristic(),
                                            Cell::White,
                                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                                        )));
                                    }
                                    _ => {}
                                }
                            }
                            Some(1) => {
                                if app.player_2.as_ref().unwrap().get_ai_type().unwrap()
                                    != AIType::QLearning
                                {
                                    if app.player_2.as_ref().unwrap().get_depth() > 1 {
                                        let current_depth =
                                            app.player_2.as_ref().unwrap().get_depth();
                                        app.player_2.as_mut().unwrap().set_depth(current_depth - 1);
                                    }
                                } else {
                                    app.set_game_message(Some(
                                        "QLearning does not support depth change".to_string(),
                                    ));
                                }
                            }
                            Some(2) => {
                                if app.player_2.as_ref().unwrap().get_ai_type().unwrap()
                                    != AIType::QLearning
                                {
                                    let previous_heuristic =
                                        app.player_2.as_ref().unwrap().get_heuristic().previous();
                                    app.player_2
                                        .as_mut()
                                        .unwrap()
                                        .set_heuristic(previous_heuristic);
                                } else {
                                    app.set_game_message(Some(
                                        "QLearning does not support heuristic change".to_string(),
                                    ));
                                }
                            }
                            Some(3) => {
                                if app.player_2.as_ref().unwrap().get_ai_type().unwrap()
                                    != AIType::QLearning
                                {
                                    if app.player_2.as_ref().unwrap().get_heuristic()
                                        == HeuristicType::Absolute
                                        || app.player_2.as_ref().unwrap().get_heuristic()
                                            == HeuristicType::Mobility
                                    {
                                        app.set_game_message(Some(
                                            "This heuristic do not support heuristic matrix change"
                                                .to_string(),
                                        ));
                                        continue;
                                    } else {
                                        let previous_matrix = app
                                            .player_2
                                            .as_ref()
                                            .unwrap()
                                            .get_heuristic_matrix()
                                            .previous();
                                        app.player_2
                                            .as_mut()
                                            .unwrap()
                                            .set_heuristic_matrix(previous_matrix);
                                    }
                                } else {
                                    app.set_game_message(Some(
                                        "QLearning does not support heuristic matrix change"
                                            .to_string(),
                                    ));
                                }
                            }
                            Some(4) => {
                                if app.player_2.as_ref().unwrap().get_ai_type().unwrap()
                                    == AIType::MinMax
                                {
                                    let previous_double_threading =
                                        app.player_2.as_ref().unwrap().get_double_threading();
                                    app.player_2
                                        .as_mut()
                                        .unwrap()
                                        .set_double_threading(!previous_double_threading);
                                } else {
                                    app.set_game_message(Some(
                                        "Only MinMax AI can use double threading".to_string(),
                                    ));
                                }
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
                                            app.player_2.as_ref().unwrap().get_heuristic(),
                                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                                            10000,
                                            Cell::White,
                                        )));
                                    }
                                    Some(AIType::MinMax) => {
                                        app.player_2 = Some(Box::new(AIAlphaBeta::new(
                                            app.player_2.as_ref().unwrap().get_depth(),
                                            app.player_2.as_ref().unwrap().get_heuristic(),
                                            Cell::White,
                                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                                        )));
                                    }
                                    Some(AIType::QLearning) => {
                                        app.player_2 = Some(Box::new(AIMinMax::new(
                                            app.player_2.as_ref().unwrap().get_depth(),
                                            app.player_2.as_ref().unwrap().get_heuristic(),
                                            Cell::White,
                                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                                            false,
                                        )));
                                    }
                                    _ => {}
                                }
                            }
                            Some(1) => {
                                if app.player_2.as_ref().unwrap().get_ai_type().unwrap()
                                    != AIType::QLearning
                                {
                                    if app.player_2.as_ref().unwrap().get_depth() < MAX_DEPTH {
                                        let current_depth =
                                            app.player_2.as_ref().unwrap().get_depth();
                                        app.player_2.as_mut().unwrap().set_depth(current_depth + 1);
                                    } else {
                                        app.set_game_message(Some(
                                            "Maximum depth reached [see const MAX_DEPTH]"
                                                .to_string(),
                                        ));
                                    }
                                } else {
                                    app.set_game_message(Some(
                                        "QLearning does not support depth change".to_string(),
                                    ));
                                }
                            }
                            Some(2) => {
                                if app.player_2.as_ref().unwrap().get_ai_type().unwrap()
                                    != AIType::QLearning
                                {
                                    let next_heuristic =
                                        app.player_2.as_ref().unwrap().get_heuristic().next();
                                    app.player_2.as_mut().unwrap().set_heuristic(next_heuristic);
                                } else {
                                    app.set_game_message(Some(
                                        "QLearning does not support heuristic change".to_string(),
                                    ));
                                }
                            }
                            Some(3) => {
                                if app.player_2.as_ref().unwrap().get_ai_type().unwrap()
                                    != AIType::QLearning
                                {
                                    if app.player_2.as_ref().unwrap().get_heuristic()
                                        == HeuristicType::Absolute
                                        || app.player_2.as_ref().unwrap().get_heuristic()
                                            == HeuristicType::Mobility
                                    {
                                        app.set_game_message(Some(
                                            "This heuristic do not support heuristic matrix change"
                                                .to_string(),
                                        ));
                                        continue;
                                    } else {
                                        let next_matrix = app
                                            .player_2
                                            .as_ref()
                                            .unwrap()
                                            .get_heuristic_matrix()
                                            .next();
                                        app.player_2
                                            .as_mut()
                                            .unwrap()
                                            .set_heuristic_matrix(next_matrix);
                                    }
                                } else {
                                    app.set_game_message(Some(
                                        "QLearning does not support heuristic matrix change"
                                            .to_string(),
                                    ));
                                }
                            }
                            Some(4) => {
                                if app.player_2.as_ref().unwrap().get_ai_type().unwrap()
                                    == AIType::MinMax
                                {
                                    let previous_double_threading =
                                        app.player_2.as_ref().unwrap().get_double_threading();
                                    app.player_2
                                        .as_mut()
                                        .unwrap()
                                        .set_double_threading(!previous_double_threading);
                                } else {
                                    app.set_game_message(Some(
                                        "Only MinMax AI can use double threading".to_string(),
                                    ));
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    CurrentScreen::AIVsAI => match key.code {
                        KeyCode::Char('q') => {
                            app.current_mode.select_first();
                            app.game_message = None;
                            app.current_screen = CurrentScreen::Main;
                        }

                        KeyCode::Enter => match app.current_mode.selected() {
                            Some(10) => {
                                let mut game_ready = false;
                                if app.player_1.as_ref().unwrap().get_ai_type().unwrap()
                                    == AIType::QLearning
                                {
                                    match app
                                        .player_1
                                        .as_mut()
                                        .unwrap()
                                        .import_q_table_file("q_table_player_1.json")
                                    {
                                        Ok(_) => {
                                            game_ready = true;
                                        }
                                        Err(e) => {
                                            game_ready = false;
                                            app.set_game_message(Some(e));
                                        }
                                    }
                                }
                                if app.player_2.as_ref().unwrap().get_ai_type().unwrap()
                                    == AIType::QLearning
                                {
                                    match app
                                        .player_2
                                        .as_mut()
                                        .unwrap()
                                        .import_q_table_file("q_table_player_2.json")
                                    {
                                        Ok(_) => {
                                            game_ready = true;
                                        }
                                        Err(e) => {
                                            game_ready = false;
                                            app.set_game_message(Some(e));
                                        }
                                    }
                                }
                                if game_ready {
                                    app.current_screen = CurrentScreen::Game;
                                    app.current_mode.select_first();
                                    app.start_game();
                                }
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
                                match app.player_1.as_mut().unwrap().get_ai_type() {
                                    Some(AIType::AlphaBeta) => {
                                        app.player_1 = Some(Box::new(AIMinMax::new(
                                            app.player_1.as_ref().unwrap().get_depth(),
                                            app.player_1.as_ref().unwrap().get_heuristic(),
                                            Cell::Black,
                                            app.player_1.as_ref().unwrap().get_heuristic_matrix(),
                                            app.player_1.as_ref().unwrap().get_double_threading(),
                                        )));
                                    }
                                    Some(AIType::MinMax) => {
                                        app.player_1 = Some(Box::new(QLearning::new(
                                            1000,
                                            app.player_1.as_ref().unwrap().get_heuristic(),
                                            app.player_1.as_ref().unwrap().get_heuristic_matrix(),
                                            10000,
                                            Cell::Black,
                                        )));
                                    }
                                    Some(AIType::QLearning) => {
                                        app.player_1 = Some(Box::new(AIAlphaBeta::new(
                                            app.player_1.as_ref().unwrap().get_depth(),
                                            app.player_1.as_ref().unwrap().get_heuristic(),
                                            Cell::Black,
                                            app.player_1.as_ref().unwrap().get_heuristic_matrix(),
                                        )));
                                    }
                                    _ => {}
                                }
                            }
                            Some(1) => {
                                if app.player_1.as_ref().unwrap().get_ai_type().unwrap()
                                    != AIType::QLearning
                                {
                                    if app.player_1.as_ref().unwrap().get_depth() > 1 {
                                        let current_depth =
                                            app.player_1.as_ref().unwrap().get_depth();
                                        app.player_1.as_mut().unwrap().set_depth(current_depth - 1);
                                    }
                                } else {
                                    app.set_game_message(Some(
                                        "QLearning does not support depth change".to_string(),
                                    ));
                                }
                            }
                            Some(2) => {
                                if app.player_1.as_ref().unwrap().get_ai_type().unwrap()
                                    != AIType::QLearning
                                {
                                    let previous_heuristic =
                                        app.player_1.as_ref().unwrap().get_heuristic().previous();
                                    app.player_1
                                        .as_mut()
                                        .unwrap()
                                        .set_heuristic(previous_heuristic);
                                } else {
                                    app.set_game_message(Some(
                                        "QLearning does not support heuristic change".to_string(),
                                    ));
                                }
                            }
                            Some(3) => {
                                if app.player_1.as_ref().unwrap().get_ai_type().unwrap()
                                    != AIType::QLearning
                                {
                                    if app.player_1.as_ref().unwrap().get_heuristic()
                                        == HeuristicType::Absolute
                                        || app.player_1.as_ref().unwrap().get_heuristic()
                                            == HeuristicType::Mobility
                                    {
                                        app.set_game_message(Some(
                                            "This heuristic do not support heuristic matrix change"
                                                .to_string(),
                                        ));
                                        continue;
                                    } else {
                                        let previous_matrix = app
                                            .player_1
                                            .as_ref()
                                            .unwrap()
                                            .get_heuristic_matrix()
                                            .previous();
                                        app.player_1
                                            .as_mut()
                                            .unwrap()
                                            .set_heuristic_matrix(previous_matrix);
                                    }
                                } else {
                                    app.set_game_message(Some(
                                        "QLearning does not support heuristic matrix change"
                                            .to_string(),
                                    ));
                                }
                            }
                            Some(4) => {
                                if app.player_1.as_ref().unwrap().get_ai_type().unwrap()
                                    == AIType::MinMax
                                {
                                    let previous_double_threading =
                                        app.player_1.as_ref().unwrap().get_double_threading();
                                    app.player_1
                                        .as_mut()
                                        .unwrap()
                                        .set_double_threading(!previous_double_threading);
                                } else {
                                    app.set_game_message(Some(
                                        "Only MinMax AI can use double threading".to_string(),
                                    ));
                                }
                            }
                            Some(5) => {
                                // changer l'IA en gardant les mêmes paramètres, sauf Q learning
                                match app.player_2.as_mut().unwrap().get_ai_type() {
                                    Some(AIType::AlphaBeta) => {
                                        app.player_2 = Some(Box::new(AIMinMax::new(
                                            app.player_2.as_ref().unwrap().get_depth(),
                                            app.player_2.as_ref().unwrap().get_heuristic(),
                                            Cell::White,
                                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                                            app.player_2.as_ref().unwrap().get_double_threading(),
                                        )));
                                    }
                                    Some(AIType::MinMax) => {
                                        app.player_2 = Some(Box::new(QLearning::new(
                                            1000,
                                            app.player_2.as_ref().unwrap().get_heuristic(),
                                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                                            10000,
                                            Cell::White,
                                        )));
                                    }
                                    Some(AIType::QLearning) => {
                                        app.player_2 = Some(Box::new(AIAlphaBeta::new(
                                            app.player_2.as_ref().unwrap().get_depth(),
                                            app.player_2.as_ref().unwrap().get_heuristic(),
                                            Cell::White,
                                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                                        )));
                                    }
                                    _ => {}
                                }
                            }
                            Some(6) => {
                                if app.player_2.as_ref().unwrap().get_ai_type().unwrap()
                                    != AIType::QLearning
                                {
                                    if app.player_2.as_ref().unwrap().get_depth() > 1 {
                                        let current_depth =
                                            app.player_2.as_ref().unwrap().get_depth();
                                        app.player_2.as_mut().unwrap().set_depth(current_depth - 1);
                                    }
                                } else {
                                    app.set_game_message(Some(
                                        "QLearning does not support depth change".to_string(),
                                    ));
                                }
                            }
                            Some(7) => {
                                if app.player_2.as_ref().unwrap().get_ai_type().unwrap()
                                    != AIType::QLearning
                                {
                                    let previous_heuristic =
                                        app.player_2.as_ref().unwrap().get_heuristic().previous();
                                    app.player_2
                                        .as_mut()
                                        .unwrap()
                                        .set_heuristic(previous_heuristic);
                                } else {
                                    app.set_game_message(Some(
                                        "QLearning does not support heuristic change".to_string(),
                                    ));
                                }
                            }
                            Some(8) => {
                                if app.player_2.as_ref().unwrap().get_ai_type().unwrap()
                                    != AIType::QLearning
                                {
                                    if app.player_2.as_ref().unwrap().get_heuristic()
                                        == HeuristicType::Absolute
                                        || app.player_2.as_ref().unwrap().get_heuristic()
                                            == HeuristicType::Mobility
                                    {
                                        app.set_game_message(Some(
                                            "This heuristic do not support heuristic matrix change"
                                                .to_string(),
                                        ));
                                        continue;
                                    } else {
                                        let previous_matrix = app
                                            .player_2
                                            .as_ref()
                                            .unwrap()
                                            .get_heuristic_matrix()
                                            .previous();
                                        app.player_2
                                            .as_mut()
                                            .unwrap()
                                            .set_heuristic_matrix(previous_matrix);
                                    }
                                } else {
                                    app.set_game_message(Some(
                                        "QLearning does not support heuristic matrix change"
                                            .to_string(),
                                    ));
                                }
                            }
                            Some(9) => {
                                if app.player_2.as_ref().unwrap().get_ai_type().unwrap()
                                    == AIType::MinMax
                                {
                                    let previous_double_threading =
                                        app.player_2.as_ref().unwrap().get_double_threading();
                                    app.player_2
                                        .as_mut()
                                        .unwrap()
                                        .set_double_threading(!previous_double_threading);
                                } else {
                                    app.set_game_message(Some(
                                        "Only MinMax AI can use double threading".to_string(),
                                    ));
                                }
                            }
                            _ => {}
                        },
                        KeyCode::Right => match app.current_mode.selected() {
                            Some(0) => {
                                // changer l'IA en gardant les mêmes paramètres, sauf Q learning
                                match app.player_1.as_mut().unwrap().get_ai_type() {
                                    Some(AIType::AlphaBeta) => {
                                        app.player_1 = Some(Box::new(QLearning::new(
                                            1000,
                                            app.player_1.as_ref().unwrap().get_heuristic(),
                                            app.player_1.as_ref().unwrap().get_heuristic_matrix(),
                                            10000,
                                            Cell::Black,
                                        )));
                                    }
                                    Some(AIType::MinMax) => {
                                        app.player_1 = Some(Box::new(AIAlphaBeta::new(
                                            app.player_1.as_ref().unwrap().get_depth(),
                                            app.player_1.as_ref().unwrap().get_heuristic(),
                                            Cell::Black,
                                            app.player_1.as_ref().unwrap().get_heuristic_matrix(),
                                        )));
                                    }
                                    Some(AIType::QLearning) => {
                                        app.player_1 = Some(Box::new(AIMinMax::new(
                                            app.player_1.as_ref().unwrap().get_depth(),
                                            app.player_1.as_ref().unwrap().get_heuristic(),
                                            Cell::Black,
                                            app.player_1.as_ref().unwrap().get_heuristic_matrix(),
                                            app.player_1.as_ref().unwrap().get_double_threading(),
                                        )));
                                    }
                                    _ => {}
                                }
                            }
                            Some(1) => {
                                if app.player_1.as_ref().unwrap().get_ai_type().unwrap()
                                    != AIType::QLearning
                                {
                                    if app.player_1.as_ref().unwrap().get_depth() < MAX_DEPTH {
                                        let current_depth =
                                            app.player_1.as_ref().unwrap().get_depth();
                                        app.player_1.as_mut().unwrap().set_depth(current_depth + 1);
                                    } else {
                                        app.set_game_message(Some(
                                            "Maximum depth reached [see const MAX_DEPTH]"
                                                .to_string(),
                                        ));
                                    }
                                } else {
                                    app.set_game_message(Some(
                                        "QLearning does not support depth change".to_string(),
                                    ));
                                }
                            }
                            Some(2) => {
                                if app.player_1.as_ref().unwrap().get_ai_type().unwrap()
                                    != AIType::QLearning
                                {
                                    let next_heuristic =
                                        app.player_1.as_ref().unwrap().get_heuristic().next();
                                    app.player_1.as_mut().unwrap().set_heuristic(next_heuristic);
                                } else {
                                    app.set_game_message(Some(
                                        "QLearning does not support heuristic change".to_string(),
                                    ));
                                }
                            }
                            Some(3) => {
                                if app.player_1.as_ref().unwrap().get_ai_type().unwrap()
                                    != AIType::QLearning
                                {
                                    if app.player_1.as_ref().unwrap().get_heuristic()
                                        == HeuristicType::Absolute
                                        || app.player_1.as_ref().unwrap().get_heuristic()
                                            == HeuristicType::Mobility
                                    {
                                        app.set_game_message(Some(
                                            "This heuristic do not support heuristic matrix change"
                                                .to_string(),
                                        ));
                                        continue;
                                    } else {
                                        let next_matrix = app
                                            .player_1
                                            .as_ref()
                                            .unwrap()
                                            .get_heuristic_matrix()
                                            .next();
                                        app.player_1
                                            .as_mut()
                                            .unwrap()
                                            .set_heuristic_matrix(next_matrix);
                                    }
                                } else {
                                    app.set_game_message(Some(
                                        "QLearning does not support heuristic matrix change"
                                            .to_string(),
                                    ));
                                }
                            }
                            Some(4) => {
                                if app.player_1.as_ref().unwrap().get_ai_type().unwrap()
                                    == AIType::MinMax
                                {
                                    let previous_double_threading =
                                        app.player_1.as_ref().unwrap().get_double_threading();
                                    app.player_1
                                        .as_mut()
                                        .unwrap()
                                        .set_double_threading(!previous_double_threading);
                                } else {
                                    app.set_game_message(Some(
                                        "Only MinMax AI can use double threading".to_string(),
                                    ));
                                }
                            }

                            Some(5) => {
                                // changer l'IA en gardant les mêmes paramètres, sauf Q learning
                                match app.player_2.as_mut().unwrap().get_ai_type() {
                                    Some(AIType::AlphaBeta) => {
                                        app.player_2 = Some(Box::new(QLearning::new(
                                            1000,
                                            app.player_2.as_ref().unwrap().get_heuristic(),
                                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                                            10000,
                                            Cell::White,
                                        )));
                                    }
                                    Some(AIType::MinMax) => {
                                        app.player_2 = Some(Box::new(AIAlphaBeta::new(
                                            app.player_2.as_ref().unwrap().get_depth(),
                                            app.player_2.as_ref().unwrap().get_heuristic(),
                                            Cell::White,
                                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                                        )));
                                    }
                                    Some(AIType::QLearning) => {
                                        app.player_2 = Some(Box::new(AIMinMax::new(
                                            app.player_2.as_ref().unwrap().get_depth(),
                                            app.player_2.as_ref().unwrap().get_heuristic(),
                                            Cell::White,
                                            app.player_2.as_ref().unwrap().get_heuristic_matrix(),
                                            false,
                                        )));
                                    }
                                    _ => {}
                                }
                            }
                            Some(6) => {
                                if app.player_2.as_ref().unwrap().get_ai_type().unwrap()
                                    != AIType::QLearning
                                {
                                    if app.player_2.as_ref().unwrap().get_depth() < MAX_DEPTH {
                                        let current_depth =
                                            app.player_2.as_ref().unwrap().get_depth();
                                        app.player_2.as_mut().unwrap().set_depth(current_depth + 1);
                                    } else {
                                        app.set_game_message(Some(
                                            "Maximum depth reached [see const MAX_DEPTH]"
                                                .to_string(),
                                        ));
                                    }
                                } else {
                                    app.set_game_message(Some(
                                        "QLearning does not support depth change".to_string(),
                                    ));
                                }
                            }
                            Some(7) => {
                                if app.player_2.as_ref().unwrap().get_ai_type().unwrap()
                                    != AIType::QLearning
                                {
                                    let next_heuristic =
                                        app.player_2.as_ref().unwrap().get_heuristic().next();
                                    app.player_2.as_mut().unwrap().set_heuristic(next_heuristic);
                                } else {
                                    app.set_game_message(Some(
                                        "QLearning does not support heuristic change".to_string(),
                                    ));
                                }
                            }
                            Some(8) => {
                                if app.player_2.as_ref().unwrap().get_ai_type().unwrap()
                                    != AIType::QLearning
                                {
                                    if app.player_2.as_ref().unwrap().get_heuristic()
                                        == HeuristicType::Absolute
                                        || app.player_2.as_ref().unwrap().get_heuristic()
                                            == HeuristicType::Mobility
                                    {
                                        app.set_game_message(Some(
                                            "This heuristic do not support heuristic matrix change"
                                                .to_string(),
                                        ));
                                        continue;
                                    } else {
                                        let next_matrix = app
                                            .player_2
                                            .as_ref()
                                            .unwrap()
                                            .get_heuristic_matrix()
                                            .next();
                                        app.player_2
                                            .as_mut()
                                            .unwrap()
                                            .set_heuristic_matrix(next_matrix);
                                    }
                                } else {
                                    app.set_game_message(Some(
                                        "QLearning does not support heuristic matrix change"
                                            .to_string(),
                                    ));
                                }
                            }
                            Some(9) => {
                                if app.player_2.as_ref().unwrap().get_ai_type().unwrap()
                                    == AIType::MinMax
                                {
                                    let previous_double_threading =
                                        app.player_2.as_ref().unwrap().get_double_threading();
                                    app.player_2
                                        .as_mut()
                                        .unwrap()
                                        .set_double_threading(!previous_double_threading);
                                } else {
                                    app.set_game_message(Some(
                                        "Only MinMax AI can use double threading".to_string(),
                                    ));
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    CurrentScreen::QLearningParameters => match key.code {
                        KeyCode::Char('q') => {
                            app.current_mode.select_first();
                            app.game_message = None;
                            app.current_screen = CurrentScreen::Main;
                        }
                        KeyCode::Up => {
                            app.current_mode.select_previous();
                        }
                        KeyCode::Down => {
                            app.current_mode.select_next();
                        }
                        KeyCode::Left => match app.current_mode.selected() {
                            Some(0) => {
                                if app.qlearning_parameters.as_ref().unwrap().get_epochs() > 500 {
                                    let current_epochs =
                                        app.qlearning_parameters.as_ref().unwrap().get_epochs();
                                    app.qlearning_parameters
                                        .as_mut()
                                        .unwrap()
                                        .set_epochs(current_epochs - 500);
                                }
                            }
                            Some(1) => {
                                if app.qlearning_parameters.as_ref().unwrap().get_max_step() > 1 {
                                    let current_max_step =
                                        app.qlearning_parameters.as_ref().unwrap().get_max_step();
                                    app.qlearning_parameters
                                        .as_mut()
                                        .unwrap()
                                        .set_max_step(current_max_step - 1);
                                }
                            }
                            Some(2) => {
                                let previous_heuristic = app
                                    .qlearning_parameters
                                    .as_ref()
                                    .unwrap()
                                    .get_heuristic()
                                    .previous();
                                app.qlearning_parameters
                                    .as_mut()
                                    .unwrap()
                                    .set_heuristic(previous_heuristic);
                            }
                            Some(3) => {
                                if app.qlearning_parameters.as_ref().unwrap().get_heuristic()
                                    == HeuristicType::Absolute
                                    || app.qlearning_parameters.as_ref().unwrap().get_heuristic()
                                        == HeuristicType::Mobility
                                {
                                    app.set_game_message(Some(
                                        "This heuristic do not support heuristic matrix change"
                                            .to_string(),
                                    ));
                                    continue;
                                } else {
                                    let previous_matrix = app
                                        .qlearning_parameters
                                        .as_ref()
                                        .unwrap()
                                        .get_heuristic_matrix()
                                        .previous();
                                    app.qlearning_parameters
                                        .as_mut()
                                        .unwrap()
                                        .set_heuristic_matrix(previous_matrix);
                                }
                            }
                            _ => {}
                        },
                        KeyCode::Right => match app.current_mode.selected() {
                            Some(0) => {
                                if app.qlearning_parameters.as_ref().unwrap().get_epochs()
                                    < QLEARNING_MAX_EPOCHS
                                {
                                    let current_epochs =
                                        app.qlearning_parameters.as_ref().unwrap().get_epochs();
                                    app.qlearning_parameters
                                        .as_mut()
                                        .unwrap()
                                        .set_epochs(current_epochs + 500);
                                }
                            }
                            Some(1) => {
                                if app.qlearning_parameters.as_ref().unwrap().get_max_step() < 64 {
                                    let current_max_step =
                                        app.qlearning_parameters.as_ref().unwrap().get_max_step();
                                    app.qlearning_parameters
                                        .as_mut()
                                        .unwrap()
                                        .set_max_step(current_max_step + 1);
                                }
                            }
                            Some(2) => {
                                let next_heuristic = app
                                    .qlearning_parameters
                                    .as_ref()
                                    .unwrap()
                                    .get_heuristic()
                                    .next();
                                app.qlearning_parameters
                                    .as_mut()
                                    .unwrap()
                                    .set_heuristic(next_heuristic);
                            }
                            Some(3) => {
                                if app.qlearning_parameters.as_ref().unwrap().get_heuristic()
                                    == HeuristicType::Absolute
                                    || app.qlearning_parameters.as_ref().unwrap().get_heuristic()
                                        == HeuristicType::Mobility
                                {
                                    app.set_game_message(Some(
                                        "This heuristic do not support heuristic matrix change"
                                            .to_string(),
                                    ));
                                } else {
                                    let next_matrix = app
                                        .qlearning_parameters
                                        .as_ref()
                                        .unwrap()
                                        .get_heuristic_matrix()
                                        .next();
                                    app.qlearning_parameters
                                        .as_mut()
                                        .unwrap()
                                        .set_heuristic_matrix(next_matrix);
                                }
                            }
                            _ => {}
                        },
                        KeyCode::Enter => match app.current_mode.selected() {
                            Some(4) => {
                                // lancer le training dans un thread séparé
                                // aller sur l'écran de training
                            }
                            _ => {}
                        },
                        _ => return Ok(()),
                    },
                }
            }
        }
    }
}
