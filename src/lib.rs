pub mod app;
pub mod content;
pub mod db;
pub mod engine;
pub mod modes;
pub mod ui;

mod game;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

/// Entry point for the application. Sets up the terminal, runs the event loop,
/// then restores the terminal regardless of whether an error occurred.
pub fn run() -> anyhow::Result<()> {
    let conn = db::open_db()?;
    let mut app = app::App::new(&conn)?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = game::run_app(&mut terminal, &mut app, &conn);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}
