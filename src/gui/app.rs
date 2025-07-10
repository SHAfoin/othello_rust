use ratatui::widgets::ListState;

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
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            current_mode: ListState::default().with_selected(Some(0)), // Sélectionner le premier élément par défaut
        }
    }
}
