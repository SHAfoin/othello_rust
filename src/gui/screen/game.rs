//! Main game screen implementation for active Othello gameplay.
//!
//! This module provides the primary game interface where players interact
//! with the Othello board, view game state, track history, and monitor
//! scores. It combines multiple UI components to create a comprehensive
//! gaming experience with real-time feedback and information display.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{Block, BorderType, List, ListItem, ListState, Padding, Paragraph},
    Frame,
};
use tui_big_text::{BigText, PixelSize};

use crate::{
    consts::SIZE,
    game::cell::Cell,
    gui::{app::App, ui::footer},
};

/// Renders the main game screen with board, history, scores, and controls.
///
/// This function orchestrates the complete game interface, combining multiple
/// UI components to provide players with all necessary information and controls
/// for playing Othello. It manages layout, coordinates widget rendering, and
/// ensures proper information display during active gameplay.
///
/// # Screen Layout
///
/// The game screen uses a sophisticated multi-panel layout:
/// - **Main horizontal split**: Game board (left) and information panel (right)
/// - **Right panel sections**: History, timer, messages, and player scores
/// - **Footer area**: Navigation and control instructions
///
/// # UI Components
///
/// The screen integrates several specialized widgets:
/// - **Game board**: Interactive 8x8 grid showing disc positions and selection
/// - **Move history**: Scrollable list of all moves played with details
/// - **Timer display**: Real-time game duration tracking
/// - **Message area**: Current game status and notifications
/// - **Score displays**: Large, prominent score counters for both players
///
/// # Visual Design
///
/// - **Responsive layout**: Adapts to different terminal sizes
/// - **Color coding**: Blue for black discs/player, yellow for white discs/player
/// - **Interactive feedback**: Selected cell highlighted with double border
/// - **Information hierarchy**: Important information prominently displayed
///
/// # Real-time Information
///
/// The screen provides live updates of:
/// - **Current board state**: All disc positions and available moves
/// - **Player scores**: Dynamic disc count for each player
/// - **Game progress**: Move history with coordinates and captured discs
/// - **Game timing**: Elapsed time since game start
/// - **Status messages**: Current player turn, game events, errors
///
/// # User Interaction
///
/// Players can interact through:
/// - **Arrow keys**: Navigate board cell selection
/// - **Enter**: Place disc at selected position
/// - **'t'**: Access tutorial overlay
/// - **'q'**: Quit current game
///
/// # Arguments
///
/// * `frame` - Ratatui frame for rendering all game widgets
/// * `app` - Mutable application state containing game board and UI state
///
/// # Layout Calculations
///
/// The function uses complex constraint-based layouts:
/// - Main area: Minimum 66 units for board, 40% for information panel
/// - Right panel: Flexible history, fixed timer/message areas, 30% for scores
/// - Score area: Equal 50% split for both player score displays
///
/// # Error Handling
///
/// Gracefully handles missing game components:
/// - Displays appropriate messages when board is unavailable
/// - Safely accesses optional application state components
/// - Provides fallback displays for incomplete game state
///
/// # Examples
///
/// ```rust
/// // Called by main UI loop during active gameplay
/// match app.current_screen {
///     CurrentScreen::Game => game_screen(&mut frame, &mut app),
///     // ... other screen handlers
/// }
/// ```
pub fn game_screen(frame: &mut Frame, app: &mut App) {
    // Ecran découpé en zone de base + footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(1)])
        .flex(Flex::Center)
        .split(frame.area());

    // Zone de base découpée en zone gauche / droite
    let main_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(66), Constraint::Percentage(40)])
        .split(chunks[0]);

    // Zone droite découpée en deux : historique et score
    let right_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Percentage(30),
        ])
        .split(main_area[1]);

    // Zone des scores
    let score_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(right_area[3]);

    widget_grid(frame, app, main_area[0]);

    widget_history(frame, app, right_area[0]);

    widget_timer(frame, app, right_area[1]);

    widget_message(frame, app, right_area[2]);

    // Récupérer les scores des joueurs
    let mut black_score = String::new();
    let mut white_score = String::new();

    if let Some(board) = &app.board {
        black_score = board.get_nb_discs(Cell::Black).unwrap().to_string();
        white_score = board.get_nb_discs(Cell::White).unwrap().to_string();
    }

    widget_score(frame, app, score_area[0], black_score, Color::Blue, "BLACK");

    widget_score(
        frame,
        app,
        score_area[1],
        white_score,
        Color::Yellow,
        "WHITE",
    );

    // Footer
    footer(
        frame,
        app,
        chunks[1],
        " (↑↓←→) to choose / (ENTER) to play / (t) for tutorial / (q) to quit ",
    );
}

/// Renders the interactive game board grid with discs and selection highlight.
///
/// This function creates and displays the main 8x8 Othello game board where
/// players can see disc positions, make moves, and navigate with cursor
/// selection. It handles the complex task of rendering a grid layout with
/// proper spacing, labels, and visual feedback for user interaction.
///
/// # Board Representation
///
/// The grid displays:
/// - **Column labels**: A-H letters across the top for column identification
/// - **Row numbers**: 0-7 digits along the left side for row identification
/// - **Disc positions**: Color-coded cells showing black (blue) and white (yellow) discs
/// - **Empty cells**: Bordered cells indicating available board positions
/// - **Selection highlight**: Double-bordered cell showing current cursor position
///
/// # Visual Layout
///
/// The board uses a sophisticated layout system:
/// - **Container**: Rounded border with "Game Board" title
/// - **Centering**: Horizontal and vertical centering within allocated area
/// - **Grid structure**: 9x9 layout (8x8 board + labels) with proper spacing
/// - **Cell sizing**: Fixed 4-unit width, 2-unit height per cell
/// - **Spacing**: 2-unit horizontal, 1-unit vertical spacing between cells
///
/// # Color Scheme
///
/// - **Black discs**: Blue background (Cell::Black → Color::Blue)
/// - **White discs**: Yellow background (Cell::White → Color::Yellow)
/// - **Empty cells**: Default bordered style
/// - **Selected cell**: Reversed style with double border
///
/// # Interactive Features
///
/// - **Cursor navigation**: Responds to app.selected_cell state
/// - **Visual feedback**: Selected position clearly highlighted
/// - **Move indication**: Shows where player can potentially place disc
///
/// # Layout Calculations
///
/// The function performs complex layout calculations:
/// 1. Creates bordered container for the entire board area
/// 2. Centers the board both horizontally (58 units wide) and vertically (27 units tall)
/// 3. Generates 9x9 grid constraints for board cells plus labels
/// 4. Maps grid positions to cell types (label, disc, empty, selected)
///
/// # Cell Rendering Logic
///
/// Each grid position is rendered based on its type:
/// - **Position 0**: Empty (top-left corner)
/// - **Top row (1-8)**: Column labels (A-H)
/// - **Left column (multiples of 9)**: Row numbers (0-7)
/// - **Board cells**: Disc state with potential selection highlight
///
/// # Error Handling
///
/// - **Missing board**: Displays error message when board is unavailable
/// - **Safe cell access**: Uses proper bounds checking for board queries
/// - **Graceful degradation**: Continues rendering even with partial data
///
/// # Arguments
///
/// * `frame` - Ratatui frame for rendering the board widgets
/// * `app` - Mutable application state containing board and selection state
/// * `area` - Screen rectangle allocated for the game board display
///
/// # Performance Considerations
///
/// - **Grid generation**: Efficiently creates layout constraints
/// - **Cell iteration**: Processes all 81 grid positions systematically
/// - **Conditional rendering**: Only applies selection styling when appropriate
///
/// # Examples
///
/// ```rust
/// // Called internally by game_screen() to render the board
/// widget_grid(&mut frame, &mut app, board_area);
/// ```
fn widget_grid(frame: &mut Frame, app: &mut App, area: Rect) {
    // Zone de jeu
    let game_board = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(" Game Board ")
        .title_alignment(Alignment::Center)
        .padding(Padding::uniform(0));

    frame.render_widget(&game_board, area);

    // Centrer la grille horizontalement
    let game_board_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(58),
            Constraint::Fill(1),
        ])
        .split(area);

    // Centrer la grille verticalement
    let game_board_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(27),
            Constraint::Fill(1),
        ])
        .split(game_board_horizontal[1]);

    // Grille de jeu
    if let Some(board) = &app.board {
        let col_constraints = (0..SIZE + 1).map(|_| Constraint::Length(4));
        let row_constraints = (0..SIZE + 1).map(|_| Constraint::Length(2));
        let horizontal = Layout::horizontal(col_constraints).spacing(2);
        let vertical = Layout::vertical(row_constraints).spacing(1);

        let rows = vertical.split(game_board_vertical[1]);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            if i != 0 {
                if i % 9 == 0 && i != 0 {
                    frame.render_widget(
                        Paragraph::new(Span::raw((i / 9).to_string()).into_centered_line())
                            .block(Block::default()),
                        cell,
                    );
                } else if i < 9 {
                    frame.render_widget(
                        Paragraph::new(
                            Span::raw(char::from_u32(i as u32 + 64).unwrap().to_string())
                                .into_centered_line(),
                        )
                        .block(Block::default()),
                        cell,
                    );
                } else {
                    match board.get_cell(i / 9 - 1, i % 9 - 1) {
                        Ok(Cell::Black) => {
                            frame.render_widget(
                                Block::default().style(Style::default().bg(Color::Blue)),
                                cell,
                            );
                        }
                        Ok(Cell::White) => {
                            frame.render_widget(
                                Block::default().style(Style::default().bg(Color::Yellow)),
                                cell,
                            );
                        }
                        _ => {
                            frame.render_widget(Block::bordered().style(Style::default()), cell);
                        }
                    }
                    if app.selected_cell == Some((i / 9 - 1, i % 9 - 1)) {
                        frame.render_widget(
                            Block::new()
                                .border_type(BorderType::Double)
                                .style(Style::default().reversed()),
                            cell,
                        );
                    }
                }
            }
        }
    } else {
        app.game_message = Some("No game board available !".to_string());
    }
}

/// Renders the game message display area for status and notifications.
///
/// This function creates a simple message display widget that shows current
/// game status, error messages, player notifications, and other important
/// information to keep players informed about game state and events.
///
/// # Message Types
///
/// The widget displays various types of messages:
/// - **Status updates**: Current player turn, game phase information
/// - **Error messages**: Invalid move attempts, system errors
/// - **Game events**: Move confirmations, game state changes
/// - **Notifications**: Important alerts and instructions
///
/// # Visual Design
///
/// - **Bordered container**: Rounded border for visual separation
/// - **Centered text**: Message content centered for readability
/// - **Uniform padding**: Consistent spacing around message text
/// - **Fallback content**: "No message" displayed when no message is available
///
/// # Message Source
///
/// Messages are retrieved from `app.game_message`:
/// - **Dynamic content**: Updates based on game events and user actions
/// - **Optional handling**: Gracefully handles None values with fallback
/// - **State management**: Messages persist until explicitly updated
///
/// # Arguments
///
/// * `frame` - Ratatui frame for rendering the message widget
/// * `app` - Application state containing current game message
/// * `area` - Screen rectangle allocated for message display
///
/// # Examples
///
/// ```rust
/// // Called internally by game_screen() to show current status
/// widget_message(&mut frame, &app, message_area);
/// ```
fn widget_message(frame: &mut Frame, app: &App, area: Rect) {
    // Zone message de jeu
    let game_message = Paragraph::new(app.game_message.clone().unwrap_or("No message".into()))
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .padding(Padding::uniform(1)),
        )
        .alignment(Alignment::Center);

    frame.render_widget(game_message, area);
}

/// Renders the game move history display with chronological move list.
///
/// This function creates a scrollable list widget showing the complete
/// history of moves played in the current game. It provides detailed
/// information about each move including player, position, and captured discs.
///
/// # History Information
///
/// For each move, the history displays:
/// - **Move number**: Sequential numbering of all moves
/// - **Player identification**: Which player (Black/White) made the move
/// - **Move coordinates**: Board position in human-readable format
/// - **Captured discs**: Number of opponent discs flipped
/// - **Special moves**: Pass moves when no legal moves available
///
/// # Visual Features
///
/// - **Color coding**: Black moves in blue, white moves in yellow
/// - **Reverse order**: Most recent moves appear at the top
/// - **Detailed formatting**: Clear, consistent move descriptions
/// - **Bordered container**: "Game History" titled container
/// - **Scrollable content**: Handles long game histories
///
/// # Move Display Format
///
/// Regular moves: "Move X: [Player] played at [Position]. +[Discs] discs."
/// Pass moves: "Move X: [Player] passed (no legal move)."
///
/// # Data Processing
///
/// The function processes move history by:
/// 1. Retrieving move history from the game board
/// 2. Converting each move to formatted list item with appropriate styling
/// 3. Reversing order to show recent moves first
/// 4. Applying color coding based on player
///
/// # Arguments
///
/// * `frame` - Ratatui frame for rendering the history widget
/// * `app` - Application state containing game board with move history
/// * `area` - Screen rectangle allocated for history display
///
/// # Error Handling
///
/// - **Missing board**: Gracefully handles when no board is available
/// - **Empty history**: Displays empty list for new games
/// - **Data validation**: Safely handles optional move data
///
/// # Examples
///
/// ```rust
/// // Called internally by game_screen() to show move history
/// widget_history(&mut frame, &app, history_area);
/// ```
fn widget_history(frame: &mut Frame, app: &App, area: Rect) {
    // Zone historique du jeu
    let history_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title("Game History")
        .title_alignment(Alignment::Center)
        .padding(Padding::uniform(1));

    // Générer l'historique du jeu
    let mut game_history = List::default();
    if let Some(board) = &app.board {
        let mut history_items: Vec<ListItem> = board
            .get_history()
            .iter()
            .enumerate()
            .map(|(_, action)| {
                if action.coordinates.is_none() {
                    return ListItem::new(format!(
                        "Move {}: {} passed (no legal move).",
                        action.move_number, action.player_turn
                    ))
                    .style(Style::default().fg(
                        if action.color == Cell::Black {
                            Color::Blue
                        } else {
                            Color::Yellow
                        },
                    ));
                } else {
                    ListItem::new(format!(
                        "Move {}: {} played at {}. +{} discs.",
                        action.move_number,
                        action.player_turn,
                        action.coordinates.clone().unwrap_or("0".into()),
                        action.gained_discs.unwrap_or(0)
                    ))
                    .style(Style::default().fg(
                        if action.color == Cell::Black {
                            Color::Blue
                        } else {
                            Color::Yellow
                        },
                    ))
                }
            })
            .collect();
        history_items.reverse();
        game_history = List::new(history_items);
    }

    frame.render_stateful_widget(
        game_history.block(history_block),
        area,
        &mut ListState::default(),
    );
}

/// Renders a player score display with large, prominent number presentation.
///
/// This function creates an individual score display widget for one player,
/// featuring large-format numbers that are easily readable during gameplay.
/// It provides a visually prominent way to track each player's current
/// disc count throughout the game.
///
/// # Visual Design
///
/// - **Bordered container**: Rounded border with player name as title
/// - **Color coding**: Border and text colored to match player (blue/yellow)
/// - **Large text display**: Big, easy-to-read score numbers using BigText
/// - **Centered layout**: Score prominently centered within allocated area
/// - **Vertical spacing**: Proper padding above and below the score number
///
/// # Score Presentation
///
/// - **Font style**: Large pixel-based font for maximum readability
/// - **Color consistency**: Score color matches player's disc color
/// - **Dynamic updates**: Score changes automatically as game progresses
/// - **Professional appearance**: Clean, sports-scoreboard-like presentation
///
/// # Layout Structure
///
/// The widget uses a three-section vertical layout:
/// 1. **Top spacer**: Flexible spacing for vertical centering
/// 2. **Score display**: Fixed minimum height (4 units) for the large number
/// 3. **Bottom spacer**: Flexible spacing for balance
///
/// # Player Identification
///
/// - **Title display**: Player name shown in container title
/// - **Color coding**: Consistent with game's color scheme
/// - **Clear labeling**: "BLACK" and "WHITE" labels for easy identification
///
/// # Arguments
///
/// * `frame` - Ratatui frame for rendering the score widget
/// * `_app` - Application state (unused but kept for API consistency)
/// * `area` - Screen rectangle allocated for this score display
/// * `score` - Current score value as a string
/// * `color` - Display color for borders and text (Blue for Black, Yellow for White)
/// * `name` - Player name for title display ("BLACK" or "WHITE")
///
/// # Technical Features
///
/// - **BigText integration**: Uses tui_big_text crate for large number display
/// - **Quadrant pixels**: Uses PixelSize::Quadrant for optimal terminal rendering
/// - **Responsive sizing**: Adapts to allocated area while maintaining proportions
///
/// # Examples
///
/// ```rust
/// // Called by game_screen() to display both player scores
/// widget_score(&mut frame, &app, black_area, "15", Color::Blue, "BLACK");
/// widget_score(&mut frame, &app, white_area, "12", Color::Yellow, "WHITE");
/// ```
fn widget_score(
    frame: &mut Frame,
    _app: &App,
    area: Rect,
    score: String,
    color: Color,
    name: &str,
) {
    let player_score_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(color))
        .title(name)
        .title_alignment(Alignment::Center);

    frame.render_widget(player_score_block, area);

    let player_score_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Min(4), Constraint::Fill(1)])
        .split(area);

    let player_score = BigText::builder()
        .alignment(Alignment::Center)
        .pixel_size(PixelSize::Quadrant)
        .style(Style::default().fg(color))
        .lines(vec![format!("{}", score).into()])
        .build();

    frame.render_widget(player_score, player_score_layout[1]);
}

/// Renders the game timer display showing elapsed time since game start.
///
/// This function creates a timer widget that displays the total elapsed time
/// for the current game session in a clear, digital clock format. It provides
/// players with awareness of how long they've been playing and can be useful
/// for time management and game analysis.
///
/// # Time Display Format
///
/// - **Digital format**: MM:SS (minutes:seconds) with zero-padding
/// - **Real-time updates**: Continuously updates as game progresses
/// - **Precise tracking**: Accurate to the second for detailed timing
/// - **Clean presentation**: Clear, easy-to-read digital display
///
/// # Visual Design
///
/// - **Bordered container**: Rounded border with "Timer" title
/// - **Centered layout**: Time display centered both horizontally and vertically
/// - **Vertical padding**: Proper spacing above and below the time text
/// - **Consistent styling**: Matches overall game interface aesthetic
///
/// # Timer Integration
///
/// The widget integrates with the application's timer system:
/// - **Optional timer**: Gracefully handles cases where timer is not available
/// - **Live updates**: Displays current elapsed time from timer state
/// - **Automatic formatting**: Converts Duration to readable time format
///
/// # Time Calculation
///
/// The function performs time formatting by:
/// 1. Retrieving elapsed duration from the application timer
/// 2. Converting total seconds to minutes and remaining seconds
/// 3. Applying zero-padding for consistent two-digit display
/// 4. Formatting as "MM:SS" string for display
///
/// # Use Cases
///
/// The timer serves multiple purposes:
/// - **Game duration tracking**: Monitor total time spent on current game
/// - **Performance analysis**: Evaluate decision-making speed
/// - **Session management**: Awareness of play time for breaks
/// - **Competitive play**: Time tracking for tournament or timed games
///
/// # Error Handling
///
/// - **Missing timer**: No display when timer is unavailable (graceful degradation)
/// - **Safe calculations**: Handles duration conversion safely
/// - **Continuous operation**: Maintains display throughout game session
///
/// # Arguments
///
/// * `frame` - Ratatui frame for rendering the timer widget
/// * `app` - Application state containing optional timer instance
/// * `area` - Screen rectangle allocated for timer display
///
/// # Display Range
///
/// - **Format capacity**: Supports up to 99:59 (nearly 2 hours)
/// - **Overflow handling**: Will show higher minutes if game exceeds expected duration
/// - **Second precision**: Updates every second for accurate tracking
///
/// # Examples
///
/// ```rust
/// // Called internally by game_screen() to show elapsed time
/// widget_timer(&mut frame, &app, timer_area);
/// ```
fn widget_timer(frame: &mut Frame, app: &App, area: Rect) {
    if let Some(timer) = &app.timer {
        let elapsed = timer.elapsed();
        let minutes = elapsed.as_secs() / 60;
        let seconds = elapsed.as_secs() % 60;

        let timer_text = format!("{:02}:{:02}", minutes, seconds);
        let timer_paragraph = Paragraph::new(timer_text)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("Timer")
                    .title_alignment(Alignment::Center)
                    .padding(Padding::vertical(1)),
            )
            .alignment(Alignment::Center);

        frame.render_widget(timer_paragraph, area);
    }
}
