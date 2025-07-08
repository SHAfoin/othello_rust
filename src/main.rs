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

mod game;
use game::board::Board;
use game::cell::Cell;

fn get_player_move(player_turn: Cell) -> Option<(usize, usize)> {
    loop {
        println!(
            "{} : Enter your move (row and column, e.g., '3D'): ",
            player_turn
        );

        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if input == "exit" {
                    println!("Exiting the game.");
                    std::process::exit(0);
                } else if input == "help" {
                    println!("Available commands:");
                    println!("  - Enter your move in 'rowColumn' format (e.g., '3D').");
                    println!("  - Type 'exit' to quit the game.");
                    continue;
                }

                match Board::input_to_coordinates(input) {
                    Some(coords) => return Some(coords),
                    None => println!(
                        "Invalid input format. Please use 'rowColumn' format (e.g., '3D')."
                    ),
                }
            }
            Err(e) => {
                println!("Error reading input: {}", e);
                continue;
            }
        }
    }
}

fn play_turn_human(board: &mut Board, player_turn: Cell) {
    if let Some(nb_moves) = board.has_legal_moves(player_turn) {
        board.set_nb_legal_moves(player_turn, nb_moves).unwrap();

        loop {
            println!("{}", board);

            if let Some((row, col)) = get_player_move(player_turn) {
                match board.try_play_move(row, col, player_turn) {
                    Ok(_) => {
                        println!("Move played successfully.");
                        break; // Sort de la boucle d'input
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        // Continue la boucle pour redemander l'input
                    }
                }
            }
        }
    } else {
        println!("{} : No legal moves available.", player_turn);
    }
}

fn main() {
    let mut board = Board::new();
    let mut player_turn = Cell::Black;

    println!("Welcome to Othello!\n");
    println!("================");

    loop {
        play_turn_human(&mut board, player_turn);

        player_turn = match player_turn {
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
            _ => player_turn,
        };

        println!("\n================");
    }
}
