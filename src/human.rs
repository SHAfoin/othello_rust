use crate::{
    game::{
        board::{Board, HistoryAction, Player},
        cell::Cell,
    },
    gui::app::App,
};

pub struct Human {
    color: Cell,
}

impl Human {
    pub fn new(color: Cell) -> Self {
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
}

impl Player for Human {
    fn is_human(&self) -> bool {
        true
    }
    fn play_turn(
        &self,
        board: &mut Board,
        cell: Option<(usize, usize)>,
    ) -> Result<(HistoryAction), String> {
        if let Some((row, col)) = cell {
            match board.try_play_move(row, col, self.get_color()) {
                Ok(gained_discs) => Ok(HistoryAction {
                    coordinates: Some(Board::coordinates_to_input(row, col)),
                    gained_discs: Some(gained_discs),
                    color: self.get_color(),
                    player_turn: board.get_player_turn(),
                    move_number: board.get_turn_number(),
                }),
                Err(e) => Err(format!("Error playing move: {}", e)),
            }
        } else {
            Err("No cell selected.".to_string())
        }
    }
}
