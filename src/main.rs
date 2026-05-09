use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use log_analyzer::{
    app::App, errors::Errors, handle_input::handle_input, read_file::read_log_file, tui::run_ui,
};
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::io;
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
