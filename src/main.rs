use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use log_analyzer::{
    app::App, errors::Errors, handle_input::handle_input, log_store::LogStore, tui::run_ui,
};
use ratatui::{Terminal, prelude::CrosstermBackend};

const DEFAULT_FILE_PATH: &str = "src/sample_logs/simple.log";

#[tokio::main]
async fn main() -> Result<(), Errors> {
    let file_path = DEFAULT_FILE_PATH;
    let log_store = LogStore::new(file_path)?;
    let mut app = App::new(log_store);

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|frame| {
            run_ui(frame, &mut app);
        })?;

        app.receive_search_result();

        if handle_input(&mut app)? {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
