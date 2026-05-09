use crate::{app::App, errors::Errors, mode::Mode};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub fn handle_input(app: &mut App) -> Result<bool, Errors> {
    if event::poll(Duration::from_millis(16))?
        && let Event::Key(key) = event::read()?
    {
        match app.mode() {
            Mode::Normal => match key.code {
                KeyCode::Char('q') => {
                    return Ok(true);
                }
                KeyCode::Char('/') => {
                    app.enter_search_mode();
                }
                KeyCode::Down => {
                    app.select_next();
                }
                KeyCode::Up => {
                    app.select_previous();
                }
                KeyCode::Enter => {
                    app.next_match();
                }
                _ => {}
            },

            Mode::Search => match key.code {
                KeyCode::Esc => {
                    app.clear_search();
                    app.exit_search_mode();
                }
                KeyCode::Enter => {
                    app.run_search();
                    app.exit_search_mode();
                }
                KeyCode::Backspace => {
                    app.handle_search_backspace();
                }
                KeyCode::Char(c) => {
                    app.handle_search_char(c);
                }
                _ => {}
            },
        }
    }

    Ok(false)
}
