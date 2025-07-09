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
mod human;

use crate::{
    ai::{alphabeta::AIAlphaBeta, common::HeuristicType, minmax::AIMinMax},
    consts::MAX_DEPTH,
    game::{board::Board, cell::Cell},
    human::Human,
};

fn main() {
    let mut board = Board::new();
    let mut player_turn = Cell::Black;

    // let player1 = Human::new(Cell::Black);
    let player1 = AIAlphaBeta::new(
        MAX_DEPTH,               // Depth of the search tree
        HeuristicType::Absolute, // Heuristic type to use
        Cell::Black,
        None,
    );
    // let player2 = Human::new(player1.get_color().get_opponent());
    let player2 = AIAlphaBeta::new(
        MAX_DEPTH,               // Depth of the search tree
        HeuristicType::Absolute, // Heuristic type to use
        Cell::White,
        None,
    );

    println!("Welcome to Othello!\n");
    println!("================\n");

    while !board.is_game_over() {
        println!("{}", board);

        match player_turn {
            Cell::Black => player1.play_turn(&mut board),
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

        player_turn = player_turn.get_opponent();

        println!("\n================\n");
    }

    println!("\nGame over!\n");

    if board.get_nb_discs(Cell::Black).unwrap() > board.get_nb_discs(Cell::White).unwrap() {
        println!("{} wins!", Cell::Black);
    } else if board.get_nb_discs(Cell::Black).unwrap() < board.get_nb_discs(Cell::White).unwrap() {
        println!("{} wins!", Cell::White);
    } else {
        println!("It's a draw!");
    }
}
