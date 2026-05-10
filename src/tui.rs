use crate::{
    app::App,
    log_line::{LogLevel, LogLine},
    mode::Mode,
};
use ratatui::{prelude::*, widgets::*};

pub fn run_ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([Constraint::Min(1), Constraint::Length(3)]).split(frame.area());

    let search_matches = app.search().matches();
    let query = app.search().query().to_string();

    let total_lines = app.logs().len();
    let mut items = Vec::with_capacity(total_lines);

    for idx in 0..total_lines {
        if let Some(raw) = app.logs().get_line(idx) {
            // Skip empty lines to prevent ghost Debug/Trace entries at EOF
            if raw.trim().is_empty() {
                continue;
            }

            // Only process and push if parsing succeeds
            if let Some(log) = LogLine::new(raw) {
                let is_match = search_matches.contains(&idx);
                let line_style = if is_match {
                    Style::default().bg(Color::LightYellow)
                } else {
                    Style::default()
                };

                let level_style = get_style_log_level(log.log_level());

                items.push(
                    ListItem::new(Line::from(vec![
                        Span::styled(
                            format!("[{} ] ", log.time_stamp()),
                            Style::default().fg(Color::DarkGray),
                        ),
                        Span::styled(format!("{:?} ", log.log_level()), level_style),
                        // .to_string() solves the "borrowed value does not live long enough" error
                        Span::raw(log.content().to_string()),
                    ]))
                    .style(line_style),
                );
            }
        }
    }

    let list = List::new(items)
        .block(Block::default().title("Log Analyzer").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, chunks[0], &mut app.list_state());

    if app.mode() == Mode::Search {
        let search = Paragraph::new(query.clone())
            .block(Block::default().title("/Search").borders(Borders::ALL));
        frame.render_widget(search, chunks[1]);
        frame.set_cursor_position(Position {
            x: chunks[1].x + query.len() as u16 + 1,
            y: chunks[1].y + 1,
        });
    } else {
        // Render a clean status bar when not searching
        let status = Paragraph::new("Press '/' to search, 'q' to quit")
            .block(Block::default().title("Status").borders(Borders::ALL));
        frame.render_widget(status, chunks[1]);
    }
}

fn get_style_log_level(level: &LogLevel) -> Style {
    match level {
        LogLevel::Error => Style::default().fg(Color::Red),
        LogLevel::Warn => Style::default().fg(Color::Yellow),
        LogLevel::Info => Style::default().fg(Color::Blue),
        LogLevel::Debug => Style::default().fg(Color::Green),
        LogLevel::Trace => Style::default().fg(Color::Magenta),
    }
}
