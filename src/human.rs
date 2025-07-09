use crate::game::{board::Board, cell::Cell};

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

    pub fn play_turn(&self, board: &mut Board) {
        if let Some(_) = board.get_nb_legal_moves(self.get_color()).unwrap() {
            loop {
                if let Some((row, col)) = self.get_player_move() {
                    match board.try_play_move(row, col, self.get_color()) {
                        Ok(gained_discs) => {
                            println!(
                                "Move played successfully by {} in {}. +{} discs.",
                                self.get_color(),
                                Board::coordinates_to_input(row, col),
                                gained_discs
                            );
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
