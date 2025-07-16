use crate::{
    ai::{ai_type::AIType, heuristic::HeuristicType, heuristic_matrix::AIHeuristicMatrix},
    game::{board::Board, history_action::HistoryAction},
};

pub trait Player {
    fn play_turn(
        &self,
        board: &mut Board,
        cell: Option<(usize, usize)>,
    ) -> Result<HistoryAction, String>;

    fn import_q_table_file(&mut self, q_table: &str) -> Result<(), String> {
        Err("Importing Q-table is not supported for this player type".to_string())
    }

    fn is_human(&self) -> bool;

    fn get_ai_type(&self) -> Option<AIType> {
        None
    }
    fn get_double_threading(&self) -> bool {
        false
    }
    fn set_double_threading(&mut self, _double_threading: bool) {}

    fn get_heuristic_matrix(&self) -> AIHeuristicMatrix {
        AIHeuristicMatrix::A
    }

    fn set_heuristic_matrix(&mut self, _matrix: AIHeuristicMatrix) {}

    fn get_heuristic(&self) -> HeuristicType {
        HeuristicType::Absolute
    }

    fn set_heuristic(&mut self, _heuristic: HeuristicType) {}

    fn get_depth(&self) -> usize {
        1
    }

    fn set_depth(&mut self, _depth: usize) {}
}
