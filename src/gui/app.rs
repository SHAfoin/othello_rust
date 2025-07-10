use ratatui::widgets::ListState;

use crate::game::board::Board;

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
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            current_mode: ListState::default().with_selected(Some(0)), // Sélectionner le premier élément par défaut
            board: None,
            game_message: None,
        }
    }
}
