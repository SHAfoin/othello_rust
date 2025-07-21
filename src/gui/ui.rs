//! Main UI rendering and utility functions for the Othello game interface.
//!
//! This module provides the central UI coordination and common utility functions
//! for the Othello game's terminal user interface. It handles screen routing,
//! layout calculations, and reusable UI components that are shared across
//! different screens and application states.

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
        q_learning::q_learning_parameters_screen, q_learning_loading::q_learning_loading_screen,
        tutorial::tutorial_screen,
    },
};

/// Creates a centered rectangle within a given area for modal-style layouts.
///
/// This utility function calculates a centered rectangular area within a larger
/// rectangle, useful for creating popup dialogs, centered content, and modal
/// interfaces that need to be positioned in the middle of the screen.
///
/// # Layout Calculation
///
/// The function uses a two-stage layout process:
/// 1. **Vertical centering**: Creates 3-part vertical layout with specified height
/// 2. **Horizontal centering**: Creates 3-part horizontal layout with specified width
/// 3. **Returns center**: Returns the intersection (center rectangle) of both layouts
///
/// # Parameters
///
/// * `length_x` - Desired width of the centered rectangle in terminal units
/// * `length_y` - Desired height of the centered rectangle in terminal units  
/// * `r` - Parent rectangle area to center within
///
/// # Returns
///
/// A `Rect` representing the centered area with the specified dimensions.
/// The returned rectangle is guaranteed to be within the bounds of the parent area.
///
/// # Layout Structure
///
/// ```text
/// Parent Area:
/// ┌─────────────────────────┐
/// │        Fill(1)          │
/// │  ┌─────Length(x)─────┐  │ ← Length(y)
/// │  │   Centered Rect   │  │
/// │  └───────────────────┘  │
/// │        Fill(1)          │
/// └─────────────────────────┘
/// ```
///
/// # Use Cases
///
/// - **Modal dialogs**: Confirmation boxes, error messages
/// - **Centered content**: Menus, forms, configuration panels
/// - **Popups**: Tutorial overlays, help screens
/// - **Responsive design**: Content that adapts to terminal size
///
/// # Examples
///
/// ```rust
/// // Create a 60x15 centered rectangle in the full terminal area
/// let popup_area = centered_rect(60, 15, frame.area());
///
/// // Create a small confirmation dialog
/// let confirm_area = centered_rect(40, 6, frame.area());
/// ```
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

/// Main UI rendering function that coordinates all screen displays.
///
/// This function serves as the central UI dispatcher, determining which screen
/// to render based on the current application state and coordinating the
/// overall visual presentation. It handles the top-level frame setup and
/// routes to appropriate screen-specific rendering functions.
///
/// # Rendering Process
///
/// 1. **Frame setup**: Establishes main border and background
/// 2. **Screen routing**: Determines current screen from app state
/// 3. **Screen rendering**: Calls appropriate screen-specific function
/// 4. **Layout coordination**: Ensures consistent visual presentation
///
/// # Frame Configuration
///
/// - **Border**: Rounded border around the entire terminal area
/// - **Background**: Clear background for proper content display
/// - **Full area**: Utilizes complete terminal space
///
/// # Screen Routing
///
/// The function routes to different screens based on `app.current_screen`:
/// - **Main**: Primary menu and navigation
/// - **Game**: Active gameplay interface
/// - **Tutorial**: Help and rule explanation screen
/// - **HumanVsAI**: Human vs AI configuration
/// - **AIVsAI**: AI vs AI configuration  
/// - **QLearningParameters**: Q-Learning training setup
/// - **Exit**: Application termination confirmation
///
/// # State Management
///
/// - **Mutable access**: Allows screens to update application state
/// - **Centralized routing**: Single point for screen management
/// - **Consistent interface**: Uniform function signatures for all screens
///
/// # Visual Consistency
///
/// Ensures all screens maintain:
/// - **Consistent borders**: Rounded border styling throughout
/// - **Proper clearing**: Clean background for all content
/// - **Full coverage**: Complete terminal area utilization
///
/// # Arguments
///
/// * `frame` - Ratatui frame for rendering all UI components
/// * `app` - Mutable application state containing current screen and data
///
/// # Screen Function Requirements
///
/// All screen functions must accept:
/// - `frame: &mut Frame` - For rendering widgets
/// - `app: &mut App` - For accessing and updating state
///
/// # Examples
///
/// ```rust
/// // Main UI loop integration
/// loop {
///     terminal.draw(|frame| {
///         ui(frame, &mut app);
///     })?;
///     // ... handle input events ...
/// }
/// ```
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
        CurrentScreen::QLearningLoading => {
            q_learning_loading_screen(frame, app);
        }
    }
}

/// Renders a footer area with navigation instructions and controls.
///
/// This function creates a standardized footer display that shows users
/// the available navigation options and keyboard controls for the current
/// screen. It provides consistent styling and positioning for help text
/// across all application screens.
///
/// # Visual Design
///
/// - **Text color**: Red styling for high visibility and contrast
/// - **Centered alignment**: Text centered horizontally within the footer area
/// - **Minimal styling**: Clean, unbordered design for subtle presence
/// - **Full width**: Spans the entire allocated footer area
///
/// # Content Display
///
/// The footer displays context-sensitive help text:
/// - **Navigation keys**: Arrow key instructions for movement
/// - **Action keys**: Enter, specific letter keys for actions
/// - **Exit options**: Return to menu or quit instructions
/// - **Screen-specific**: Different text per screen for relevant actions
///
/// # Text Formatting
///
/// - **Custom text**: Provided by calling screen for context relevance
/// - **Red highlighting**: Makes instructions easily noticeable
/// - **Centered layout**: Professional, balanced appearance
/// - **Single line**: Concise, essential information only
///
/// # Common Footer Examples
///
/// - **Main menu**: "(↑↓) to choose / (ENTER) to validate / (q) to quit"
/// - **Game screen**: "(↑↓←→) to choose / (ENTER) to play / (t) for tutorial / (q) to quit"
/// - **Configuration**: "(↑↓←→) to choose / (ENTER) to validate / (q) to return to main menu"
///
/// # Arguments
///
/// * `frame` - Ratatui frame for rendering the footer widget
/// * `_app` - Application state (unused but kept for API consistency)
/// * `area` - Rectangle area allocated for the footer display
/// * `text` - Help text to display in the footer
///
/// # Lifetime Parameter
///
/// The `'a` lifetime ensures the text reference remains valid for the
/// duration of the rendering operation.
///
/// # Examples
///
/// ```rust
/// // Render footer for main menu
/// footer(frame, &app, footer_area, "(↑↓) to choose / (ENTER) to validate / (q) to quit");
///
/// // Render footer for game screen
/// footer(frame, &app, footer_area, "(↑↓←→) to move / (ENTER) to play / (q) to quit");
/// ```
pub fn footer<'a>(frame: &mut Frame, _app: &App, area: Rect, text: &'a str) {
    let current_navigation_text =
        Span::styled(text, Style::default().fg(Color::Red)).into_centered_line();
    let mode_footer = Paragraph::new(Line::from(current_navigation_text)).block(Block::default());
    frame.render_widget(mode_footer, area);
}

/// Renders the main application title using large, stylized text.
///
/// This function creates a prominent title display for the Othello application
/// using large-format text rendering. It provides consistent branding and
/// visual identity across different screens of the application.
///
/// # Visual Design
///
/// - **Large text**: Uses BigText crate for prominent, eye-catching display
/// - **Full pixels**: Maximum pixel density for crisp text appearance
/// - **Centered alignment**: Title centered horizontally within allocated area
/// - **"Othello" text**: Application name displayed as the title
///
/// # Text Rendering
///
/// - **BigText library**: Utilizes tui_big_text for large character rendering
/// - **Full pixel size**: PixelSize::Full for maximum detail and clarity
/// - **Vector lines**: Text built from vector line segments for scalability
/// - **Professional appearance**: Clean, readable large text formatting
///
/// # Layout Integration
///
/// The title is designed to integrate seamlessly with screen layouts:
/// - **Fixed height**: Predictable space requirements for layout planning
/// - **Responsive width**: Adapts to available horizontal space
/// - **Consistent positioning**: Same appearance across all screens
/// - **Proper spacing**: Works well with surrounding UI elements
///
/// # Branding Function
///
/// The title serves multiple purposes:
/// - **Application identification**: Clear application name display
/// - **Visual branding**: Consistent identity throughout the interface
/// - **Screen header**: Prominent top-of-screen element
/// - **Professional presentation**: Polished, game-like appearance
///
/// # Arguments
///
/// * `frame` - Ratatui frame for rendering the title widget
/// * `_app` - Application state (unused but kept for API consistency)
/// * `area` - Rectangle area allocated for the title display
///
/// # Technical Details
///
/// - **BigText builder**: Uses builder pattern for configuration
/// - **Single line**: Title is rendered as a single text line
/// - **No borders**: Clean presentation without additional decoration
///
/// # Examples
///
/// ```rust
/// // Render title at top of main menu
/// widget_title(frame, &app, title_area);
///
/// // Render title on configuration screens
/// widget_title(frame, &app, header_area);
/// ```
pub fn widget_title(frame: &mut Frame, _app: &App, area: Rect) {
    let big_text = BigText::builder()
        .alignment(Alignment::Center)
        .pixel_size(PixelSize::Full)
        .lines(vec!["Othello".into()])
        .build();

    frame.render_widget(big_text, area);
}
