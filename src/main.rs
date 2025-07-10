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
    game::{board::Board, cell::Cell},
    gui::{
        app::{App, CurrentScreen},
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
use std::error::Error;
use std::io;

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

    while !board.is_game_over() {
        println!("{}", board);

        match board.get_player_turn() {
            Cell::Black => qplayer.play_turn(&mut board),
            Cell::White => player2.play_turn(&mut board),
            _ => unreachable!(),
        }

        println!(
            "Current leaderboard: {} {} discs, {} {} discs",
            Cell::Black,
            board.get_nb_discs(Cell::Black).unwrap(),
            Cell::White,
            board.get_nb_discs(Cell::White).unwrap(),
        );

        println!("\n================\n");
    }

    println!("\nGame over!\n");

    println!("{}", board);

    println!(
        "Current leaderboard: {} {} discs, {} {} discs",
        Cell::Black,
        board.get_nb_discs(Cell::Black).unwrap(),
        Cell::White,
        board.get_nb_discs(Cell::White).unwrap(),
    );

    if board.get_nb_discs(Cell::Black).unwrap() > board.get_nb_discs(Cell::White).unwrap() {
        println!("{} wins!", Cell::Black);
    } else if board.get_nb_discs(Cell::Black).unwrap() < board.get_nb_discs(Cell::White).unwrap() {
        println!("{} wins!", Cell::White);
    } else {
        println!("It's a draw!");
    }
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

    // if let Ok(do_print) = res {
    //     if do_print {
    //         app.print_json()?;
    //     }
    // } else if let Err(err) = res {
    //     println!("{err:?}");
    // }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        // Dessiner en boucle sur le terminal
        terminal.draw(|f| ui(f, app))?;

        // Gérer les events
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
                            println!("Human vs Human selected");
                        }
                        Some(1) => {
                            app.current_screen = CurrentScreen::HumanVsAI;
                            println!("Human vs AI selected");
                        }
                        Some(2) => {
                            app.current_screen = CurrentScreen::AIvsAI;
                            println!("AI vs AI selected");
                        }
                        Some(3) => {
                            app.current_screen = CurrentScreen::QLearningParameters;
                            println!("Q-Learning Training selected");
                        }
                        _ => {}
                    },
                    _ => {}
                },
                // CurrentScreen::Game => match key.code {},
                // CurrentScreen::Tutorial => match key.code {},
                // CurrentScreen::HumanVsAI => match key.code {},
                // CurrentScreen::AIvsAI => match key.code {},
                // CurrentScreen::QLearningParameters => match key.code {},
                _ => return Ok(()),
            }
        }
    }
}
