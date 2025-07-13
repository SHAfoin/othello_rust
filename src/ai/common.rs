use std::fmt::Display;

use crate::consts::SIZE;
use crate::game::{board::Board, cell::Cell};

#[derive(Clone, Debug)]
pub enum AIHeuristicMatrix {
    A,
    B,
}

impl Display for AIHeuristicMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AIHeuristicMatrix::A => write!(f, "Matrix A"),
            AIHeuristicMatrix::B => write!(f, "Matrix B"),
        }
    }
}

impl AIHeuristicMatrix {
    pub fn value(&self) -> [[isize; SIZE]; SIZE] {
        match self {
            AIHeuristicMatrix::A => [
                [100, -20, 10, 5, 5, 10, -20, 100],
                [-20, -50, -2, -2, -2, -2, -50, -20],
                [10, -2, -1, -1, -1, -1, -2, 10],
                [5, -2, -1, -1, -1, -1, -2, 5],
                [5, -2, -1, -1, -1, -1, -2, 5],
                [10, -2, -1, -1, -1, -1, -2, 10],
                [-20, -50, -2, -2, -2, -2, -50, -20],
                [100, -20, 10, 5, 5, 10, -20, 100],
            ],
            AIHeuristicMatrix::B => [
                [500, -150, 30, 10, 10, 30, -150, 500],
                [-150, -250, 0, 0, 0, 0, -250, -150],
                [30, 0, 1, 2, 2, 1, 0, 30],
                [10, 0, 2, 16, 16, 2, 0, 10],
                [10, 0, 2, 16, 16, 2, 0, 10],
                [30, 0, 1, 2, 2, 1, 0, 30],
                [-150, -250, 0, 0, 0, 0, -250, -150],
                [500, -150, 30, 10, 10, 30, -150, 500],
            ],
        }
    }

    pub fn next(&self) -> AIHeuristicMatrix {
        match self {
            AIHeuristicMatrix::A => AIHeuristicMatrix::B,
            AIHeuristicMatrix::B => AIHeuristicMatrix::A,
        }
    }

    pub fn previous(&self) -> AIHeuristicMatrix {
        match self {
            AIHeuristicMatrix::A => AIHeuristicMatrix::B,
            AIHeuristicMatrix::B => AIHeuristicMatrix::A,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AIType {
    AlphaBeta,
    MinMax,
    QLearning,
}

impl std::fmt::Display for AIType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AIType::AlphaBeta => write!(f, "Alpha-Beta"),
            AIType::MinMax => write!(f, "Min-Max"),
            AIType::QLearning => write!(f, "Q-Learning"),
        }
    }
}

impl AIType {
    // return next AIType based on current AIType
    pub fn next(&self) -> AIType {
        match self {
            AIType::AlphaBeta => AIType::MinMax,
            AIType::MinMax => AIType::QLearning,
            AIType::QLearning => AIType::AlphaBeta,
        }
    }

    // return previous AIType based on current AIType
    pub fn previous(&self) -> AIType {
        match self {
            AIType::AlphaBeta => AIType::QLearning,
            AIType::MinMax => AIType::AlphaBeta,
            AIType::QLearning => AIType::MinMax,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Action {
    pub pos: (usize, usize),
    pub score: isize,
}

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
