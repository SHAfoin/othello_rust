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
        ai_type::AIType,
        algo::{alphabeta::AIAlphaBeta, minmax::AIMinMax, qlearning::QLearning},
        heuristic::HeuristicType,
        heuristic_matrix::AIHeuristicMatrix,
    },
    consts::{MAX_DEPTH, QLEARNING_MAX_EPOCHS},
    game::{board::Board, cell::Cell, player::Player},
    gui::{
        app::{App, CurrentScreen},
        control::{
            ai_vs_ai::{self, ai_vs_ai_control},
            exit::exit_control,
            game::game_control,
            human_vs_ai::human_vs_ai_control,
            main::main_control,
            q_learning::{self, q_learning_parameters_control},
            tutorial::tutorial_control,
        },
        screen::q_learning::q_learning_parameters_screen,
        ui::ui,
    },
};

use ratatui::crossterm::event;
use ratatui::crossterm::event::DisableMouseCapture;
use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
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
                    CurrentScreen::Game => game_control(app, key, its_a_human_player),
                    CurrentScreen::Tutorial => tutorial_control(app, key),
                    CurrentScreen::Exit => exit_control(app, key),
                    CurrentScreen::HumanVsAI => human_vs_ai_control(app, key),
                    CurrentScreen::AIVsAI => ai_vs_ai_control(app, key),
                    CurrentScreen::QLearningParameters => q_learning_parameters_control(app, key),
                }
            }
        }
    }
}
