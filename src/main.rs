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
mod game;
mod human;

use game::{board::Board, cell::Cell};
use human::Human;

use crate::ai::{common::HeuristicType, minmax::AIMinMax};

fn main() {
    let mut board = Board::new();
    let mut player_turn = Cell::Black;

    let player1 = Human::new(Cell::Black);
    // let player2 = Human::new(player1.get_color().get_opponent());
    let player2 = AIMinMax::new(
        6,                       // Depth of the search tree
        HeuristicType::Absolute, // Heuristic type to use
        Cell::White,
        None,
    );

    println!("Welcome to Othello!\n");
    println!("================");

    while !board.is_game_over() {
        match player_turn {
            Cell::Black => player1.play_turn(&mut board),
            Cell::White => player2.play_turn(&mut board),
            _ => unreachable!(),
        }

        println!(
            "\nCurrent leaderboard: {} {} discs, {} {} discs",
            Cell::Black,
            board.get_nb_discs(Cell::Black).unwrap(),
            Cell::White,
            board.get_nb_discs(Cell::White).unwrap(),
        );

        player_turn = player_turn.get_opponent();

        println!("\n================");
    }

    println!("\n Game over!\n");

    if board.get_nb_discs(Cell::Black).unwrap() > board.get_nb_discs(Cell::White).unwrap() {
        println!("{} wins!", Cell::Black);
    } else if board.get_nb_discs(Cell::Black).unwrap() < board.get_nb_discs(Cell::White).unwrap() {
        println!("{} wins!", Cell::White);
    } else {
        println!("It's a draw!");
    }
}
