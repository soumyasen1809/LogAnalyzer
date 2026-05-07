use crate::{app::App, log_line::LogLevel};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn run_ui(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let height = area.height.saturating_sub(2) as usize;

    let logs = app.logs();
    let lines: Vec<Line> = logs
        .iter()
        .skip(app.scroll())
        .take(height)
        .map(|log| {
            let level_style = match log.log_level() {
                LogLevel::Error => Style::default().fg(Color::Red),
                LogLevel::Warn => Style::default().fg(Color::Yellow),
                LogLevel::Info => Style::default().fg(Color::Blue),
                LogLevel::Debug => Style::default().fg(Color::Green),
                LogLevel::Trace => Style::default().fg(Color::Gray),
            };

            Line::from(vec![
                Span::styled(
                    format!("[{}] ", log.time_stamp()),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(format!("{:?} ", log.log_level()), level_style),
                Span::raw(log.content()),
            ])
        })
        .collect();

    let paragraph =
        Paragraph::new(lines).block(Block::default().title("Log Analyzer").borders(Borders::ALL));

    frame.render_widget(paragraph, area);
}
