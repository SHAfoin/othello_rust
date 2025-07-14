use ratatui::{crossterm::event::KeyCode, widgets::ListState};

use crate::{
    ai::qlearning::QLearning,
    consts::SIZE,
    game::{
        board::{Board, Player},
        cell::Cell,
        timer::Timer,
    },
};

pub enum CurrentScreen {
    Main,
    Game,
    Tutorial,
    HumanVsAI,
    AIVsAI,
    QLearningParameters,
    Exit,
}

pub struct App {
    pub current_screen: CurrentScreen, // quel écran est actuellement affiché à l'utilisateur.
    pub current_mode: ListState,       // quel mode de jeu est actuellement actif.
    pub board: Option<Board>,          // le plateau de jeu actuel, s'il y en a un.
    pub game_message: Option<String>,  // message d'erreur à afficher, s'il y en a un.
    pub player_1: Option<Box<dyn Player>>, // Joueur 1, peut être un humain ou une IA.
    pub player_2: Option<Box<dyn Player>>, // Joueur 2, peut
    pub selected_cell: Option<(usize, usize)>, // Cellule sélectionnée par l'utilisateur, si applicable.
    pub timer: Option<Timer>,                  // Timer pour le jeu, si applicable.
    pub qlearning_parameters: Option<QLearning>, // Paramètres pour l'entraînement QLearning.
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            current_mode: ListState::default().with_selected(Some(0)), // Sélectionner le premier élément par défaut
            board: None,
            game_message: None,
            player_1: None,             // Initialiser sans joueur
            player_2: None,             // Initialiser sans joueur
            selected_cell: None,        // Aucune cellule sélectionnée par défaut
            timer: None,                // Pas de timer initialement
            qlearning_parameters: None, // Pas de paramètres QLearning initialement
        }
    }

    pub fn start_game(&mut self) {
        self.current_screen = CurrentScreen::Game;
        self.board = Some(Board::new());
        self.game_message = Some(format!(
            "It's {} turn !",
            self.board.as_ref().unwrap().get_player_turn()
        ));
        self.timer = Some(Timer::new());
    }

    pub fn gui_play_turn(&mut self) {
        let mut play_turn_result = Err("Error in Enter".to_string());
        let mut new_message = None;
        if let Some(board) = &mut self.board {
            if !board.is_game_over() {
                match board.get_player_turn() {
                    Cell::Black => {
                        if let Some(player) = &self.player_1 {
                            play_turn_result = player.play_turn(board, self.selected_cell);
                        }
                    }
                    Cell::White => {
                        if let Some(player) = &self.player_2 {
                            play_turn_result = player.play_turn(board, self.selected_cell);
                        }
                    }
                    _ => {
                        play_turn_result = Err("Invalid player turn".to_string());
                    }
                }

                match play_turn_result {
                    Err(e) => {
                        self.set_game_message(Some(e));
                    }
                    Ok(history_action) => {
                        board.add_to_history(history_action);

                        if board.check_game_over() {
                            self.timer.as_mut().unwrap().stop();
                            if let Some(winner) = board.get_winner() {
                                new_message = Some(format!("Game over! {} is the WINNER!", winner));
                            } else {
                                new_message = Some("Game over! It's a draw!".to_string());
                            }
                            if let Some(message) = new_message {
                                self.set_game_message(Some(message));
                            }
                        } else {
                            board.next_turn();
                            new_message = Some(format!("It's {} turn !", board.get_player_turn()));
                            if let Some(message) = new_message {
                                self.set_game_message(Some(message));
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn quit_game(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.board = None;
        self.game_message = None;
        self.player_1 = None;
        self.player_2 = None;
        self.selected_cell = None;
        self.timer = None;
    }

    pub fn set_game_message(&mut self, message: Option<String>) {
        self.game_message = message;
    }

    pub fn select_cell_key(&mut self, key: KeyCode) {
        if self.selected_cell.is_none() {
            self.selected_cell = Some((0, 0)); // Initialiser la cellule sélectionnée si elle est None
        } else {
            let (row, col) = self.selected_cell.unwrap();
            match key {
                KeyCode::Up => {
                    if row > 0 {
                        self.selected_cell = Some((row - 1, col));
                    }
                }
                KeyCode::Down => {
                    if row < SIZE - 1 {
                        self.selected_cell = Some((row + 1, col));
                    }
                }
                KeyCode::Left => {
                    if col > 0 {
                        self.selected_cell = Some((row, col - 1));
                    }
                }
                KeyCode::Right => {
                    if col < SIZE - 1 {
                        self.selected_cell = Some((row, col + 1));
                    }
                }
                _ => {}
            }
        }
    }
}
