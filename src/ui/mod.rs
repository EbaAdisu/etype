pub mod finger_guide;
pub mod game;
pub mod heatmap;
pub mod help;
pub mod menu;
pub mod results;
pub mod stats;

use crate::app::{App, Screen};
use ratatui::Frame;

pub fn render(f: &mut Frame, app: &App) {
    match &app.screen {
        Screen::Menu => menu::render_menu(f, app),
        Screen::DifficultySelect {
            mode,
            selected_diff,
            selected_timer,
        } => menu::render_difficulty_select(f, app, mode, *selected_diff, *selected_timer),
        Screen::WordRush(state) => game::render_word_rush(f, state, app.show_finger_guide),
        Screen::Sentence(state) => game::render_sentence(f, state, app.show_finger_guide),
        Screen::Code(state) => game::render_code(f, state, app.show_finger_guide),
        Screen::Survival(state) => game::render_survival(f, state, app.show_finger_guide),
        Screen::Results(result) => results::render_results(f, app, result),
        Screen::Stats => stats::render_stats(f, app),
        Screen::Heatmap => heatmap::render_heatmap(f, app),
        Screen::Help => help::render_help(f),
    }
}
