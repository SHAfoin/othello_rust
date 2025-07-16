use rand::{rng, Rng};
use std::{collections::HashMap, fs::File, hash::Hash, io::Write};

use crate::{
    ai::{ai_type::AIType, heuristic::HeuristicType, heuristic_matrix::AIHeuristicMatrix},
    consts::{EPSILON, GAMMA, LAMBDA_LEARN},
    game::{board::Board, cell::Cell, history_action::HistoryAction, player::Player},
};

pub struct QLearning {
    max_step: usize,
    q_table: HashMap<String, HashMap<String, isize>>,
    heuristic: HeuristicType,
    matrix: AIHeuristicMatrix,
    epoch: usize,
    epsilon: f64,
    color: Cell,
}

impl QLearning {
    pub fn new(
        max_step: usize,
        heuristic: HeuristicType,
        matrix: AIHeuristicMatrix,
        epoch: usize,
        color: Cell,
    ) -> Self {
        Self {
            max_step: max_step,
            q_table: HashMap::new(),
            heuristic,
            matrix,
            epoch: epoch,
            epsilon: EPSILON,
            color: color, // Default color, can be changed later
        }
    }

    pub fn get_epochs(&self) -> usize {
        self.epoch
    }

    pub fn set_epochs(&mut self, epoch: usize) {
        self.epoch = epoch;
    }

    pub fn get_max_step(&self) -> usize {
        self.max_step
    }

    pub fn set_max_step(&mut self, max_step: usize) {
        self.max_step = max_step;
    }

    pub fn get_heuristic(&self) -> HeuristicType {
        self.heuristic.clone()
    }

    pub fn set_heuristic(&mut self, heuristic: HeuristicType) {
        self.heuristic = heuristic;
    }

    pub fn get_matrix(&self) -> AIHeuristicMatrix {
        self.matrix.clone()
    }

    pub fn set_matrix(&mut self, matrix: AIHeuristicMatrix) {
        self.matrix = matrix;
    }

    pub fn get_q_table(&self) -> &HashMap<String, HashMap<String, isize>> {
        &self.q_table
    }

    pub fn set_q_table(&mut self, state: String, action: (String, isize)) {
        self.q_table
            .entry(state)
            .or_insert_with(HashMap::new)
            .insert(action.0, action.1);
    }

    pub fn get_epsilon(&self) -> f64 {
        self.epsilon
    }

    pub fn set_epsilon(&mut self, epsilon: f64) {
        self.epsilon = epsilon;
    }

    pub fn q_learning(&mut self) -> (isize, bool) {
        // board vide
        let mut board = Board::new();
        let mut step = 0;
        let mut s = board.to_hash();
        let mut action: (usize, usize);
        let mut total_r = 0;

        // tant que step pas atteinte ou nombre d'itérations
        while step < self.max_step && !board.check_game_over() {
            step += 1;
            // choisir un état initial
            // si epsilon > random, choisir une action aléatoire
            // sinon choisir l'action avec la valeur q la plus élevée
            if let Some(actions) = board.has_legal_moves(board.get_player_turn()) {
                if rng().random::<f64>() < self.epsilon || self.get_q_table().get(&s).is_none() {
                    // choisir une action aléatoire
                    action = actions[rng().random_range(0..actions.len())];
                } else {
                    // choisir l'action avec la valeur q la plus élevée
                    let mut best_action = None;
                    let mut best_value = isize::MIN;
                    if let Some(q_values) = self.get_q_table().get(&s) {
                        for (action, value) in q_values {
                            if *value > best_value {
                                best_value = *value;
                                best_action = Some(action);
                            }
                        }
                    }
                    action = Board::input_to_coordinates(best_action.unwrap().as_str()).unwrap();
                }

                // jouer l'action et déterminer récompense, nouvel état
                board
                    .try_play_move(action.0, action.1, board.get_player_turn())
                    .unwrap();

                // mettre à jour la q_table avec la formule de Q-learning
                let mut r =
                    self.heuristic
                        .evaluate(&board, board.get_player_turn(), self.matrix.clone());

                if board.check_game_over() {
                    let winner = board.get_winner();

                    if let Some(w) = winner {
                        if w == board.get_player_turn() {
                            r += 1000;
                        } else if w == board.get_player_turn().get_opponent() {
                            r -= 1000;
                        }
                    } else {
                        r += 0;
                    }
                }
                // Q[s, a] = (1-lambda_learn)*Q[s, a] + lambda_learn*(r + gamma * np.max(Q[new_state, :]))

                let new_s = board.to_hash();

                let q_value = self
                    .get_q_table()
                    .get(&s)
                    .and_then(|q_values| {
                        q_values.get(Board::coordinates_to_input(action.0, action.1).as_str())
                    })
                    .cloned()
                    .unwrap_or(0);
                let new_q_value = (1.0 - LAMBDA_LEARN) * q_value as f64
                    + LAMBDA_LEARN
                        * (r as f64
                            + GAMMA
                                * self
                                    .get_q_table()
                                    .get(&new_s)
                                    .and_then(|q_values| q_values.values().cloned().max())
                                    .unwrap_or(0) as f64);

                self.set_q_table(
                    s.clone(),
                    (
                        Board::coordinates_to_input(action.0, action.1),
                        new_q_value as isize,
                    ),
                );

                // recommencer avec cet état, additionner la récompense au total,
                s = new_s;
                total_r += r;
                board.next_turn();

                // on ajoute pas l'action à la table des actions & l'état aux états réalisés car osef de la solution, on veut qu'il s'entraine
            }

            // retourner récompense totale, récompense de l'état final, si fini ou pas
        }

        (total_r, board.check_game_over())
    }

    pub fn try_q_learning(&mut self) {
        for i in 0..self.epoch {
            println!("Epoch {}/{}", i + 1, self.epoch);
            // récuperer total_r, r, states_list, actions_list, done via q_learning()
            let (total_r, done) = self.q_learning();
            // diminuer epsilon * 0.999
            self.set_epsilon(self.get_epsilon() * 0.999);
            // osef des solutions car objectif c'est l'entrainer pas de trouver une solution
        }
        // exporter la q_table dans un fichier
        self.export_q_table("q_table.json");
    }

    pub fn import_q_table(&mut self, file_path: &str) -> Result<(), String> {
        match File::open(file_path) {
            Ok(file) => match serde_json::from_reader(file) {
                Ok(q_table) => {
                    self.q_table = q_table;
                    Ok(())
                }
                Err(e) => Err(format!("Could not deserialize Q-table: {}", e)),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn get_color(&self) -> Cell {
        self.color
    }

    pub fn export_q_table(&self, file_path: &str) {
        let json =
            serde_json::to_string_pretty(&self.q_table).expect("Could not serialize Q-table");
        let mut file = File::create(file_path).expect("Could not create file");
        file.write_all(json.as_bytes())
            .expect("Could not write to file");
    }
}

impl Player for QLearning {
    fn is_human(&self) -> bool {
        false
    }
    fn import_q_table_file(&mut self, q_table: &str) -> Result<(), String> {
        match self.import_q_table(q_table) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to import Q-table {}: {}", q_table, &e)),
        }
    }
    fn get_ai_type(&self) -> Option<AIType> {
        Some(AIType::QLearning)
    }
    fn get_heuristic_matrix(&self) -> AIHeuristicMatrix {
        self.matrix.clone()
    }

    fn set_heuristic_matrix(&mut self, matrix: AIHeuristicMatrix) {
        self.matrix = matrix;
    }

    fn get_heuristic(&self) -> HeuristicType {
        self.heuristic.clone()
    }

    fn set_heuristic(&mut self, heuristic: HeuristicType) {
        self.heuristic = heuristic;
    }
    fn play_turn(
        &self,
        board: &mut Board,
        cell: Option<(usize, usize)>,
    ) -> Result<HistoryAction, String> {
        let mut actions = board.has_legal_moves(board.get_player_turn()).unwrap();
        // choisir l'action avec la valeur q la plus élevée
        let mut best_action = None;
        let mut best_value = isize::MIN;
        if let Some(q_values) = self.get_q_table().get(&board.to_hash()) {
            for (action, value) in q_values {
                if *value > best_value {
                    best_value = *value;
                    best_action = Some(action.clone());
                }
            }
        } else {
            // choisir une action aléatoire
            let random_index = rng().random_range(0..actions.len());
            let random_action =
                Board::coordinates_to_input(actions[random_index].0, actions[random_index].1);
            best_action = Some(random_action);
        }

        let action_coords = Board::input_to_coordinates(best_action.unwrap().as_str()).unwrap();

        match board.try_play_move(action_coords.0, action_coords.1, board.get_player_turn()) {
            Ok(gained_discs) => Ok(HistoryAction {
                coordinates: Some(Board::coordinates_to_input(
                    action_coords.0,
                    action_coords.1,
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
