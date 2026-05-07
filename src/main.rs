use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use log_analyzer::{app::App, errors::Errors, mode::Mode, read_file::read_log_file, tui::run_ui};
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::{io, time::Duration};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Errors> {
    let file_path = "src/sample_logs/simple.log";

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (log_sender, mut log_receiver) = mpsc::channel(1000);

    tokio::spawn(async move {
        if let Err(err) = read_log_file(file_path, log_sender).await {
            eprintln!("{err}");
        }
    });

    let mut app = App::new();

    loop {
        while let Ok(log_line) = log_receiver.try_recv() {
            app.push_logs(log_line);
        }

        terminal.draw(|frame| {
            run_ui(frame, &mut app);
        })?;

        if handle_input(&mut app)? {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn handle_input(app: &mut App) -> Result<bool, Errors> {
    if event::poll(Duration::from_millis(16))? {
        if let Event::Key(key) = event::read()? {
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
    }

    Ok(false)
}
