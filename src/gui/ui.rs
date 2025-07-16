use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use tui_big_text::{BigText, PixelSize};

use crate::gui::{
    app::{App, CurrentScreen},
    screen::{
        ai_vs_ai::ai_vs_ai_screen, exit::exit_screen, game::game_screen,
        human_vs_ai::human_vs_ai_screen, main::main_screen,
        q_learning::q_learning_parameters_screen, tutorial::tutorial_screen,
    },
};

// WIDGET TOUT FAIT POUR DES POPUPS CENTRÉES
pub fn centered_rect(length_x: u16, length_y: u16, r: Rect) -> Rect {
    // Coupe le rectangle verticalement en trois parties, avec la partie du milieu ayant la hauteur de `percent_y` centrée
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(length_y),
            Constraint::Fill(1),
        ])
        .split(r);

    // On prend la partie du milieu (dans le split) et on la centre horizontalement aussi, et on retourne que le milieu
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(length_x),
            Constraint::Fill(1),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

// Dessiner l'UI
pub fn ui(frame: &mut Frame, app: &mut App) {
    // Set background color for the entire frame
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
        frame.area(),
    );

    frame.render_widget(Clear, frame.area());

    match app.current_screen {
        CurrentScreen::Main => {
            main_screen(frame, app);
        }
        CurrentScreen::Game => {
            game_screen(frame, app);
        }
        CurrentScreen::Tutorial => {
            tutorial_screen(frame, app);
        }
        CurrentScreen::HumanVsAI => {
            human_vs_ai_screen(frame, app);
        }
        CurrentScreen::AIVsAI => {
            ai_vs_ai_screen(frame, app);
        }
        CurrentScreen::QLearningParameters => {
            q_learning_parameters_screen(frame, app);
        }
        CurrentScreen::Exit => {
            exit_screen(frame, app);
        }
    }
}

pub fn footer<'a>(frame: &mut Frame, app: &App, area: Rect, text: &'a str) {
    let current_navigation_text =
        Span::styled(text, Style::default().fg(Color::Red)).into_centered_line();
    let mode_footer = Paragraph::new(Line::from(current_navigation_text)).block(Block::default());
    frame.render_widget(mode_footer, area);
}

pub fn widget_title(frame: &mut Frame, app: &App, area: Rect) {
    let big_text = BigText::builder()
        .alignment(Alignment::Center)
        .pixel_size(PixelSize::Full)
        .lines(vec!["Othello".into()])
        .build();

    frame.render_widget(big_text, area);
}
