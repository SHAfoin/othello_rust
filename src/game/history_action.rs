use crate::game::cell::Cell;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryAction {
    pub coordinates: Option<String>,
    pub gained_discs: Option<usize>,
    pub color: Cell,
    pub move_number: usize,
    pub player_turn: Cell,
}
