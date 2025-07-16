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