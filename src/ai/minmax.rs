use std::{thread, vec};

use crate::{
    ai::common::{Action, HeuristicType},
    consts::{MAX_DEPTH, SIZE, ULTRA_THREADING},
    game::{
        board::{Board, HistoryAction, Player},
        cell::Cell,
    },
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

    pub fn init_tree(&self, board: &Board, depth: usize) -> isize {
        self.tree_step(board, depth)
    }

    pub fn tree_step(&self, board: &Board, depth: usize) -> isize {
        let comparation_function: fn(isize, isize) -> isize;
        let mut best_score;
        if depth % 2 == 0 {
            comparation_function = isize::max;
            best_score = isize::MIN;
        } else {
            comparation_function = isize::min;
            best_score = isize::MAX;
        }
        if depth == 1 || board.has_legal_moves(board.get_player_turn()) == None {
            let score = self
                .heuristic
                .evaluate(board, self.get_color(), self.matrix);
            return score;
        } else if depth == MAX_DEPTH && ULTRA_THREADING {
            let mut handles = vec![];
            for case in board.has_legal_moves(board.get_player_turn()).unwrap() {
                let mut new_board = board.clone();
                match new_board.try_play_move(case.0, case.1, board.get_player_turn()) {
                    Ok(_) => {
                        let ai_cloned = self.clone();
                        let handle =
                            thread::spawn(move || ai_cloned.tree_step(&new_board, depth - 1));
                        handles.push(handle);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }

            for handle in handles {
                match handle.join() {
                    Ok(score) => {
                        best_score = comparation_function(best_score, score);
                    }
                    Err(_) => {
                        println!("Thread panicked");
                    }
                }
            }
            best_score
        } else {
            for case in board.has_legal_moves(board.get_player_turn()).unwrap() {
                let mut new_board = board.clone();
                match new_board.try_play_move(case.0, case.1, board.get_player_turn()) {
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

impl Player for AIMinMax {
    fn is_human(&self) -> bool {
        false
    }
    fn play_turn(
        &self,
        board: &mut Board,
        cell: Option<(usize, usize)>,
    ) -> Result<(HistoryAction), String> {
        let mut best_action = Action {
            pos: (0, 0),
            score: isize::MIN,
        };
        let mut handles = vec![];

        for case in board.has_legal_moves(board.get_player_turn()).unwrap() {
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
                    return Err(format!("Error: {}", e));
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
                    return Err("Thread join error".to_string());
                }
            }
        }

        match board.try_play_move(best_action.pos.0, best_action.pos.1, self.get_color()) {
            Ok(gained_discs) => Ok(HistoryAction {
                coordinates: Some(Board::coordinates_to_input(
                    best_action.pos.0,
                    best_action.pos.1,
                )),
                gained_discs: Some(gained_discs),
                color: self.get_color(),
                player_turn: board.get_player_turn(),
                move_number: board.get_turn_number(),
            }),
            Err(e) => Err(format!("Error playing move: {}", e)),
        }
    }
}
