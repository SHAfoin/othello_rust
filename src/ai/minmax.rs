use std::{thread, vec};

use crate::{
    ai::common::{Action, HeuristicType},
    game::{board::Board, cell::Cell, consts::SIZE},
};

#[derive(Clone, Debug)]
pub struct AIMinMax {
    depth: usize,
    heuristic: HeuristicType,
    color: Cell,
    matrix: Option<[[isize; SIZE]; SIZE]>, // Assuming a standard 8x8 Othello board
}

impl AIMinMax {
    pub fn new(
        depth: usize,
        heuristic: HeuristicType,
        color: Cell,
        matrix: Option<[[isize; SIZE]; SIZE]>,
    ) -> Self {
        Self {
            depth,
            heuristic,
            color,
            matrix,
        }
    }

    pub fn get_color(&self) -> Cell {
        self.color
    }

    pub fn play_turn(&self, board: &mut Board) {
        let mut best_action = Action {
            pos: (0, 0),
            score: isize::MIN,
        };
        let mut handles = vec![];

        if let Some(legal_moves) = board.has_legal_moves(self.get_color()) {
            for case in legal_moves {
                let mut new_board = board.clone();

                match new_board.try_play_move(case.0, case.1, self.get_color()) {
                    Ok(_) => {
                        let ai_cloned = self.clone();
                        let handle = thread::spawn(move || {
                            (case, ai_cloned.init_tree(&new_board, ai_cloned.depth))
                        });
                        handles.push(handle);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }

            for handle in handles {
                match handle.join() {
                    Ok((pos, score)) => {
                        if score > best_action.score {
                            best_action = Action { pos, score };
                        }
                    }
                    Err(_) => {
                        println!("Thread panicked");
                    }
                }
            }

            match board.try_play_move(best_action.pos.0, best_action.pos.1, self.get_color()) {
                Ok(gained_discs) => {
                    println!("Move played successfully. +{} discs.", gained_discs);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        } else {
            println!("\n{} : No legal moves available.", self.get_color());
        }
    }

    pub fn init_tree(&self, board: &Board, depth: usize) -> isize {
        self.tree_step(board, depth)
    }

    pub fn tree_step(&self, board: &Board, depth: usize) -> isize {
        let comparation_function: fn(isize, isize) -> isize;
        let mut best_score;
        let player_color;
        if depth % 2 == 0 {
            comparation_function = isize::max;
            best_score = isize::MIN;
            player_color = self.get_color();
        } else {
            comparation_function = isize::min;
            best_score = isize::MAX;
            player_color = self.get_color().get_opponent();
        }
        if depth == 1 || board.has_legal_moves(player_color) == None {
            let score = self
                .heuristic
                .evaluate(board, self.get_color(), self.matrix);
            return score;
        } else {
            for case in board.has_legal_moves(player_color).unwrap() {
                let mut new_board = board.clone();
                match new_board.try_play_move(case.0, case.1, player_color) {
                    Ok(_) => {
                        let score = self.tree_step(&new_board, depth - 1);
                        best_score = comparation_function(best_score, score);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            best_score
        }
    }
}
