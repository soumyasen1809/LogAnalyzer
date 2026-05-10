use crate::{app::App, errors::Errors, mode::Mode};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub fn handle_input(app: &mut App) -> Result<bool, Errors> {
    if event::poll(Duration::from_millis(10))?
        && let Event::Key(key) = event::read()?
    {
        return Ok(match app.mode() {
            Mode::Normal => handle_normal_mode_input(app, key.code),
            Mode::Search => handle_search_mode_input(app, key.code),
        });
    };

    Ok(false)
}

fn handle_normal_mode_input(app: &mut App, key_code: KeyCode) -> bool {
    match key_code {
        KeyCode::Char('q') => {
            return true;
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
            app.next_search_match();
        }
        _ => {}
    }

    false
}

fn handle_search_mode_input(app: &mut App, key_code: KeyCode) -> bool {
    match key_code {
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
    }

    false
}
