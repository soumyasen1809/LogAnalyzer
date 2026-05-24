use crate::{app::App, errors::Errors, mode::Mode};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

pub fn handle_input(app: &mut App) -> Result<bool, Errors> {
    if event::poll(Duration::from_millis(10))?
        && let Event::Key(key) = event::read()?
    {
        return Ok(match app.mode() {
            Mode::Normal => handle_normal_mode_input(app, key.code),
            Mode::Search => handle_search_mode_input(app, key), // Need to pass the whole key here because of Control Modifier
            Mode::BookMark => handle_bookmark_mode_input(app, key.code),
            Mode::Filter => handle_filter_mode_input(app, key.code),
            Mode::Highlight => handle_highlight_mode_input(app, key.code),
        });
    };

    Ok(false)
}

fn handle_normal_mode_input(app: &mut App, key_code: KeyCode) -> bool {
    match key_code {
        KeyCode::Char('q') => {
            return true;
        }
        KeyCode::Char('/') => app.enter_search_mode(),
        KeyCode::Char('B') => app.enter_bookmark_mode(),
        KeyCode::Char('f') => app.enter_filter_mode(),
        KeyCode::Char('h') => app.enter_highlight_mode(),
        KeyCode::Down => app.select_next(),
        KeyCode::Up => app.select_previous(),
        KeyCode::Right => app.scroll_right(),
        KeyCode::Left => app.scroll_left(),
        KeyCode::PageDown => app.scroll_down(),
        KeyCode::PageUp => app.scroll_up(),
        KeyCode::Enter => app.next_search_match(),
        KeyCode::Char('b') => app.add_bookmark(),
        KeyCode::Esc => app.clear_search(),
        _ => {}
    }

    false
}

// How to make this function API same as others?
fn handle_search_mode_input(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Esc => app.exit_search_mode(),
        KeyCode::Enter => {
            app.run_search();
            app.exit_search_mode();
        }
        KeyCode::Backspace => app.handle_search_backspace(),
        KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.toggle_regex_search();
        }
        KeyCode::Char(c) => app.handle_search_char(c),
        _ => {}
    }
    false
}

fn handle_bookmark_mode_input(app: &mut App, code: KeyCode) -> bool {
    match code {
        KeyCode::Esc => app.exit_bookmark_mode(),
        KeyCode::Enter => {
            app.jump_to_bookmark();
            app.exit_bookmark_mode();
        }
        KeyCode::Backspace => app.remove_bookmark(),
        KeyCode::Up => app.select_previous_bookmark(),
        KeyCode::Down => app.select_next_bookmark(),
        _ => {}
    }
    false
}

fn handle_filter_mode_input(app: &mut App, code: KeyCode) -> bool {
    match code {
        KeyCode::Esc => app.exit_filter_mode(),
        KeyCode::Tab => app.next_filter(),
        KeyCode::Enter => app.toggle_filter(),
        KeyCode::Up | KeyCode::Down => app.change_filter_op(),
        KeyCode::Char('+') => app.add_filter(),
        KeyCode::Backspace => app.handle_filter_backspace(),
        KeyCode::Char(c) => app.handle_filter_char(c),
        _ => {}
    }
    false
}

fn handle_highlight_mode_input(app: &mut App, code: KeyCode) -> bool {
    match code {
        KeyCode::Esc => app.exit_highlight_mode(),
        KeyCode::Tab => app.next_highlight(),
        KeyCode::Enter => app.toggle_highlight(),
        KeyCode::Char('+') => app.add_highlight(),
        KeyCode::Backspace => app.handle_highlight_backspace(),
        KeyCode::Char(c) => app.handle_highlight_char(c),
        _ => {}
    }
    false
}
