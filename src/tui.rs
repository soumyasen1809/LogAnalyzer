use crate::{app::App, log_line::LogLevel, mode::Mode};
use ratatui::{prelude::*, widgets::*};

pub fn run_ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([Constraint::Min(1), Constraint::Length(3)]).split(frame.area());

    let logs = app.logs().iter().cloned().collect::<Vec<_>>();
    let matches = app.search().matches().to_vec();
    let query = app.search().query().to_string();
    let mode = app.mode();

    let items: Vec<ListItem> = logs
        .iter()
        .enumerate()
        .map(|(idx, log)| {
            let is_match = matches.contains(&idx);
            let line_style = if is_match {
                Style::default().bg(Color::LightYellow)
            } else {
                Style::default()
            };

            let level_style = match log.log_level() {
                LogLevel::Error => Style::default().fg(Color::Red),
                LogLevel::Warn => Style::default().fg(Color::Yellow),
                LogLevel::Info => Style::default().fg(Color::Blue),
                LogLevel::Debug => Style::default().fg(Color::Green),
                LogLevel::Trace => Style::default().fg(Color::Magenta),
            };

            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("[{}] ", log.time_stamp()),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(format!("{:?} ", log.log_level()), level_style),
                Span::raw(log.content()),
            ]))
            .style(line_style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Log Analyzer").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                // .bg(Color::Gray)
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, chunks[0], &mut app.list_state());

    if mode == Mode::Search {
        let search = Paragraph::new(query.clone())
            .block(Block::default().title("/Search").borders(Borders::ALL));
        frame.render_widget(search, chunks[1]);
        frame.set_cursor_position(Position {
            x: chunks[1].x + query.len() as u16 + 1,
            y: chunks[1].y + 1,
        });
    }
}
