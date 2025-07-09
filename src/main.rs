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
use game::{board::Board, cell::Cell};

struct Human {
    color: Cell,
}

impl Human {
    fn new(color: Cell) -> Self {
        Self { color }
    }

    fn get_color(&self) -> Cell {
        self.color
    }

    fn get_player_move(&self) -> Option<(usize, usize)> {
        loop {
            println!(
                "{} : Enter your move (row and column, e.g., '3D'): ",
                self.get_color()
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

    fn play_turn(&self, board: &mut Board) {
        board
            .set_nb_legal_moves(self.get_color(), board.has_legal_moves(self.get_color()))
            .unwrap();
        board
            .set_nb_legal_moves(
                self.get_color().get_opponent(),
                board.has_legal_moves(self.get_color().get_opponent()),
            )
            .unwrap();
        if let Some(_) = board.get_nb_legal_moves(self.get_color()).unwrap() {
            println!("{}", board);
            loop {
                if let Some((row, col)) = self.get_player_move() {
                    match board.try_play_move(row, col, self.get_color()) {
                        Ok(gained_discs) => {
                            println!("Move played successfully. +{} discs.", gained_discs);
                            break;
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }
                }
            }
        } else {
            println!("\n{} : No legal moves available.", self.get_color());
        }
    }
}

fn main() {
    let mut board = Board::new();
    let mut player_turn = Cell::Black;

    let player1 = Human::new(Cell::Black);
    let player2 = Human::new(player1.get_color().get_opponent());

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
