use std::fmt::Display;

use crate::consts::SIZE;

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
