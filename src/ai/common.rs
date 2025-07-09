use crate::game::{board::Board, cell::Cell, consts::SIZE};

pub struct Action {
    pub pos: (usize, usize),
    pub score: isize,
}

pub enum HeuristicType {
    Absolute,
    Matrix,
    Mobility,
    Mixte,
}

impl HeuristicType {
    pub fn evaluate(
        &self,
        board: &Board,
        player: Cell,
        matrix: Option<[[isize; SIZE]; SIZE]>,
    ) -> isize {
        match self {
            HeuristicType::Absolute => heuristic_absolute(board, player),
            HeuristicType::Matrix => heuristic_matrix(board, player, &matrix.unwrap()),
            HeuristicType::Mobility => heuristic_mobility(board, player),
            HeuristicType::Mixte => heuristic_mixte(board, player, &matrix.unwrap()),
        }
    }
}

fn heuristic_absolute(board: &Board, player: Cell) -> isize {
    board.get_nb_discs(player).unwrap() as isize
        - board.get_nb_discs(player.get_opponent()).unwrap() as isize
}

fn heuristic_matrix(board: &Board, player: Cell, matrix: &[[isize; SIZE]; SIZE]) -> isize {
    let mut score = 0;
    for row in 0..SIZE {
        for col in 0..SIZE {
            if board.get_cell(row, col).unwrap() == player {
                score += matrix[row][col];
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

fn heuristic_mixte(board: &Board, player: Cell, matrix: &[[isize; SIZE]; SIZE]) -> isize {
    if board.get_turn_number() < 20 {
        heuristic_matrix(board, player, matrix)
    } else if board.get_turn_number() < 40 {
        heuristic_mobility(board, player)
    } else {
        heuristic_absolute(board, player)
    }
}
