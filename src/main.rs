use std::fmt;

const SIZE: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty = 0,
    Black = 1,
    White = 2,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Cell::Black => "BLACK",
            Cell::White => "WHITE",
            _ => "*",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Board {
    cells: [[Cell; SIZE]; SIZE],
    nb_discs: [usize; 2],
}

impl Board {
    pub fn new() -> Self {
        let mut cells = [[Cell::Empty; SIZE]; SIZE];
        let center1 = SIZE / 2 - 1;
        let center2 = SIZE / 2;

        cells[center1][center1] = Cell::White;
        cells[center1][center2] = Cell::Black;
        cells[center2][center1] = Cell::Black;
        cells[center2][center2] = Cell::White;

        Board {
            cells,
            nb_discs: [2, 2],
        }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<Cell> {
        if row < SIZE && col < SIZE {
            Some(self.cells[row][col])
        } else {
            None
        }
    }

    fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
        self.cells[row][col] = cell;
    }

    pub fn get_nb_discs(&self, color: Cell) -> usize {
        match color {
            Cell::Black => self.nb_discs[0],
            Cell::White => self.nb_discs[1],
            _ => 0,
        }
    }

    pub fn get_cells(&self) -> &[[Cell; SIZE]; SIZE] {
        &self.cells
    }

    pub fn play_turn(&mut self, row: usize, col: usize, color: Cell) -> Result<(), String> {
        match self.can_play(row, col, color) {
            Ok(directions) => {
                self.set_cell(row, col, color);
                for direction in directions {
                    self.flip_discs(row, col, color, direction);
                }
            }
            Err(e) => return Err(e),
        }

        let index = match color {
            Cell::Black => 0,
            Cell::White => 1,
            _ => return Err("Invalid color".to_string()),
        };
        self.nb_discs[index] += 1;

        Ok(())
    }

    fn can_play(&self, row: usize, col: usize, color: Cell) -> Result<Vec<(isize, isize)>, String> {
        if row >= SIZE || col >= SIZE {
            Err("Index out of bounds".to_string())
        } else if self.cells[row][col] != Cell::Empty {
            Err("Cell is not empty".to_string())
        } else if color != Cell::Black && color != Cell::White {
            Err("Invalid color".to_string())
        } else {
            let directions = self.get_valid_directions(row, col, color);
            if directions.is_empty() {
                Err("Invalid move".to_string())
            } else {
                Ok(directions)
            }
        }
    }

    fn get_valid_directions(&self, row: usize, col: usize, color: Cell) -> Vec<(isize, isize)> {
        let mut valid_directions = Vec::new();

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue; // Skip the current cell
                }

                if self.is_move_valid_recursive(row as isize, col as isize, color, 1, (i, j)) {
                    valid_directions.push((i, j));
                }
            }
        }

        valid_directions
    }

    fn is_move_valid_recursive(
        &self,
        row: isize,
        col: isize,
        color: Cell,
        index: isize,
        direction: (isize, isize),
    ) -> bool {
        let next_row = row + direction.0 * index;
        let next_col = col + direction.1 * index;
        if next_row < 0 || next_row >= SIZE as isize || next_col < 0 || next_col >= SIZE as isize {
            return false; // Out of bounds
        } else if index == 1 && self.get_cell(next_row as usize, next_col as usize) == Some(color) {
            return false; // Found the same color right next to the move in the direction
        } else if self.get_cell(next_row as usize, next_col as usize) == Some(color) {
            return true; // Found a cell of the same color after some other color
        } else if self.get_cell(next_row as usize, next_col as usize) == Some(Cell::Empty) {
            return false; // Found an empty cell
        } else {
            return self.is_move_valid_recursive(row, col, color, index + 1, direction);
        }
    }

    fn flip_discs(&mut self, row: usize, col: usize, color: Cell, direction: (isize, isize)) {
        let mut next_row = row as isize + direction.0;
        let mut next_col = col as isize + direction.1;
        while self.get_cell(next_row as usize, next_col as usize) != Some(color) {
            self.set_cell(next_row as usize, next_col as usize, color);
            next_row += direction.0;
            next_col += direction.1;
        }
    }

    fn has_legal_moves(&self, color: Cell) -> Option<usize> {
        let mut count = 0;
        for row in 0..SIZE {
            for col in 0..SIZE {
                if self.can_play(row, col, color).is_ok() {
                    count += 1;
                }
            }
        }
        if count > 0 {
            Some(count)
        } else {
            None
        }
    }

    fn input_to_coordinates(input: &str) -> Option<(usize, usize)> {
        if input.len() != 2 {
            return None;
        }
        let row = input.chars().nth(0)?.to_digit(10)? as usize;
        let col = input.chars().nth(1)?.to_ascii_uppercase() as usize - 'A' as usize;
        if row < SIZE && col < SIZE {
            Some((row, col))
        } else {
            None
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display = String::new();
        display.push_str("\n   A B C D E F G H\n\n");
        for (index, row) in self.cells.iter().enumerate() {
            display.push_str(&format!("{}  ", index));
            for cell in row.iter() {
                let symbol = match cell {
                    Cell::Empty => "* ",
                    Cell::Black => "B ",
                    Cell::White => "W ",
                };
                display.push_str(symbol);
            }
            display.push_str("\n");
        }
        write!(f, "{}", display)
    }
}

fn main() {
    let mut board = Board::new();
    let mut input = String::new();
    let mut player_turn = Cell::Black;
    let mut success = false;
    println!("Welcome to Othello!\n");
    println!("================");

    loop {
        success = false;
        if let Some(nb_moves) = board.has_legal_moves(player_turn) {
            while !success {
                println!("{}", board);

                println!(
                    "{} : Enter your move (row and column, e.g., '3D'): ",
                    player_turn
                );
                input.clear();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let input = input.trim();
                if input == "exit" {
                    println!("Exiting the game.");
                    return;
                } else if input == "help" {
                    println!("Available commands:");
                    println!("  - Enter your move in 'rowColumn' format (e.g., '3D').");
                    println!("  - Type 'exit' to quit the game.");
                    continue;
                }

                if let Some((row, col)) = Board::input_to_coordinates(input) {
                    if let Err(e) = board.play_turn(row, col, player_turn) {
                        println!("Error : {}", e);
                    } else {
                        println!("Move played successfully.");
                        success = true;
                    }
                } else {
                    println!("Invalid input format. Please use 'rowColumn' format (e.g., '3D').");
                }
            }
        } else {
            println!("{} : No legal moves available.", player_turn);
            break; // No legal moves, skip to next player's turn
        }

        player_turn = match player_turn {
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
            _ => player_turn, // Should not happen
        };

        println!("\n================");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_even() {
        assert_eq!(SIZE % 2, 0);
    }

    #[test]
    fn get_out_of_bounds() {
        let board = Board::new();
        assert!(board.get_cell(SIZE, 0).is_none());
        assert!(board.get_cell(0, SIZE).is_none());
        assert!(board.get_cell(SIZE, SIZE).is_none());
    }

    #[test]
    fn get_cell() {
        let board = Board::new();
        assert_eq!(board.get_cell(0, 0), Some(Cell::Empty));
        assert_eq!(board.get_cell(3, 3), Some(Cell::White));
        assert_eq!(board.get_cell(3, 4), Some(Cell::Black));
        assert_eq!(board.get_cell(4, 3), Some(Cell::Black));
        assert_eq!(board.get_cell(4, 4), Some(Cell::White));
        assert_eq!(board.get_cell(7, 7), Some(Cell::Empty));
    }

    #[test]
    fn play_turn_valid_move() {
        let mut board = Board::new();
        // Black should be able to play at (2, 3)
        assert!(board.play_turn(2, 3, Cell::Black).is_ok());
        // Verify the cell was set to Black
        assert_eq!(board.get_cell(2, 3), Some(Cell::Black));
        // Verify the White disc at (3, 3) was flipped to Black
        assert_eq!(board.get_cell(3, 3), Some(Cell::Black));
    }

    #[test]
    fn play_turn_invalid_move() {
        let mut board = Board::new();
        // Should not be able to play at (0, 0) - no valid directions
        assert!(board.play_turn(0, 0, Cell::Black).is_err());
        // Should not be able to play on an occupied cell
        assert!(board.play_turn(3, 3, Cell::Black).is_err());
    }
}
