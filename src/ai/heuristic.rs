use std::fmt::Display;

use crate::ai::heuristic_matrix::AIHeuristicMatrix;
use crate::consts::SIZE;
use crate::game::{board::Board, cell::Cell};

#[derive(Clone, Debug, PartialEq)]
pub enum HeuristicType {
    Absolute,
    Matrix,
    Mobility,
    Mixte,
    Global,
}

impl Display for HeuristicType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeuristicType::Absolute => write!(f, "Absolute"),
            HeuristicType::Matrix => write!(f, "Matrix"),
            HeuristicType::Mobility => write!(f, "Mobility"),
            HeuristicType::Mixte => write!(f, "Mixte"),
            HeuristicType::Global => write!(f, "Global"),
        }
    }
}

impl HeuristicType {
    pub fn evaluate(&self, board: &Board, player: Cell, matrix: AIHeuristicMatrix) -> isize {
        match self {
            HeuristicType::Absolute => heuristic_absolute(board, player),
            HeuristicType::Matrix => heuristic_matrix(board, player, &matrix),
            HeuristicType::Mobility => heuristic_mobility(board, player),
            HeuristicType::Mixte => heuristic_mixte(board, player, &matrix),
            HeuristicType::Global => {
                // Global heuristic combines all heuristics
                heuristic_absolute(board, player)
                    + heuristic_matrix(board, player, &matrix)
                    + heuristic_mobility(board, player)
            }
        }
    }

    pub fn next(&self) -> HeuristicType {
        match self {
            HeuristicType::Absolute => HeuristicType::Matrix,
            HeuristicType::Matrix => HeuristicType::Mobility,
            HeuristicType::Mobility => HeuristicType::Mixte,
            HeuristicType::Mixte => HeuristicType::Global,
            HeuristicType::Global => HeuristicType::Absolute,
        }
    }

    pub fn previous(&self) -> HeuristicType {
        match self {
            HeuristicType::Absolute => HeuristicType::Global,
            HeuristicType::Matrix => HeuristicType::Absolute,
            HeuristicType::Mobility => HeuristicType::Matrix,
            HeuristicType::Mixte => HeuristicType::Mobility,
            HeuristicType::Global => HeuristicType::Mixte,
        }
    }
}

fn heuristic_absolute(board: &Board, player: Cell) -> isize {
    board.get_nb_discs(player).unwrap() as isize
        - board.get_nb_discs(player.get_opponent()).unwrap() as isize
}

fn heuristic_matrix(board: &Board, player: Cell, matrix: &AIHeuristicMatrix) -> isize {
    let mut score = 0;
    for row in 0..SIZE {
        for col in 0..SIZE {
            if board.get_cell(row, col).unwrap() == player {
                score += matrix.value()[row][col];
            }
        }
    }
    score
}

fn heuristic_mobility(board: &Board, player: Cell) -> isize {
    let nb_moves_player = board.get_nb_legal_moves(player).unwrap().unwrap_or(0) as isize;
    let nb_moves_opponent = board
        .get_nb_legal_moves(player.get_opponent())
        .unwrap()
        .unwrap_or(0) as isize;
    nb_moves_player - nb_moves_opponent
}

fn heuristic_mixte(board: &Board, player: Cell, matrix: &AIHeuristicMatrix) -> isize {
    if board.get_turn_number() < 20 {
        heuristic_matrix(board, player, matrix)
    } else if board.get_turn_number() < 40 {
        heuristic_mobility(board, player)
    } else {
        heuristic_absolute(board, player)
    }
}
