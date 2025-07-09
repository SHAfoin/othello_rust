use rand::{random_range, rng, Rng};
use std::{collections::HashMap, fs::File, hash::Hash, io::Write};

use crate::{
    ai::common::HeuristicType,
    consts::{EPSILON, GAMMA, LAMBDA_LEARN, MATRIX_A, SIZE},
    game::board::Board,
};

pub struct QLearning {
    max_step: usize,
    q_table: HashMap<String, HashMap<String, isize>>,
    heuristic: HeuristicType,
    epoch: usize,
    epsilon: f64,
}

impl QLearning {
    pub fn new(max_step: usize, heuristic: HeuristicType, epoch: usize) -> Self {
        Self {
            max_step: max_step,
            q_table: HashMap::new(),
            heuristic,
            epoch: epoch,
            epsilon: EPSILON,
        }
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
        while step < self.max_step && !board.is_game_over() {
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
                        .evaluate(&board, board.get_player_turn(), Some(MATRIX_A));

                if board.is_game_over() {
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

        (total_r, board.is_game_over())
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
        let json = serde_json::to_string_pretty(&self.get_q_table()).unwrap();
        let mut file = File::create("foo.txt").unwrap();
        file.write_all(json.as_bytes()).unwrap();
        // println!("Q-table after {} epochs:\n{}", self.epoch, json);
    }
}
