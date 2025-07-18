use std::thread;

use crate::{
    ai::common::{Action, HeuristicType},
    consts::{MAX_DEPTH, SIZE, ULTRA_THREADING},
    game::{
        board::{Board, HistoryAction, Player},
        cell::Cell,
    },
};

#[derive(Clone, Debug)]
pub struct AIAlphaBeta {
    depth: usize,
    heuristic: HeuristicType,
    color: Cell,
    matrix: Option<[[isize; SIZE]; SIZE]>, // Assuming a standard 8x8 Othello board
}

impl AIAlphaBeta {
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
        self.tree_step(board, depth, &isize::MIN, &isize::MAX)
    }

    pub fn tree_step(&self, board: &Board, depth: usize, alpha: &isize, beta: &isize) -> isize {
        let mut alpha_mut = alpha.clone();
        let mut beta_mut = beta.clone();

        if depth == 1 || board.has_legal_moves(board.get_player_turn()) == None {
            let score = self
                .heuristic
                .evaluate(board, self.get_color(), self.matrix);
            return score;
        } else {
            for case in board.has_legal_moves(board.get_player_turn()).unwrap() {
                let mut new_board = board.clone();
                match new_board.try_play_move(case.0, case.1, board.get_player_turn()) {
                    Ok(_) => {
                        let score = self.tree_step(&new_board, depth - 1, &alpha_mut, &beta_mut);
                        if depth % 2 == 1 {
                            // je suis sur min
                            if score < beta_mut {
                                beta_mut = score
                            }
                            if alpha_mut >= beta_mut {
                                return beta_mut;
                            }
                        } else {
                            // je suis sur max
                            if score > alpha_mut {
                                alpha_mut = score
                            }
                            if alpha_mut >= beta_mut {
                                return alpha_mut;
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            if depth % 2 == 1 {
                // je suis sur min
                return beta_mut;
            } else {
                // je suis sur max
                return alpha_mut;
            }
        }
    }
}

impl Player for AIAlphaBeta {
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
                Err(e) => return Err(format!("Error: {}", e)),
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
