use ratatui::{crossterm::event::KeyCode, widgets::ListState};

use crate::{
    consts::SIZE,
    game::board::{Board, Player},
};

pub enum CurrentScreen {
    Main,
    Game,
    Tutorial,
    HumanVsAI,
    AIvsAI,
    QLearningParameters,
}

pub struct App {
    pub current_screen: CurrentScreen, // quel écran est actuellement affiché à l'utilisateur.
    pub current_mode: ListState,       // quel mode de jeu est actuellement actif.
    pub board: Option<Board>,          // le plateau de jeu actuel, s'il y en a un.
    pub game_message: Option<String>,  // message d'erreur à afficher, s'il y en a un.
    pub player_1: Option<Box<dyn Player>>, // Joueur 1, peut être un humain ou une IA.
    pub player_2: Option<Box<dyn Player>>, // Joueur 2, peut
    pub selected_cell: Option<(usize, usize)>, // Cellule sélectionnée par l'utilisateur, si applicable.
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            current_mode: ListState::default().with_selected(Some(0)), // Sélectionner le premier élément par défaut
            board: None,
            game_message: None,
            player_1: None,      // Initialiser sans joueur
            player_2: None,      // Initialiser sans joueur
            selected_cell: None, // Aucune cellule sélectionnée par défaut
        }
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
