use crate::{
    app::App,
    log_line::{LogLevel, LogLine},
    mode::Mode,
};
use ratatui::{prelude::*, widgets::*};

pub fn run_ui(frame: &mut Frame, app: &mut App) {
    let chunks = match app.mode() {
        Mode::BookMark => {
            Layout::vertical([Constraint::Fill(1), Constraint::Percentage(30)]).split(frame.area())
        }
        _ => Layout::vertical([Constraint::Min(1), Constraint::Length(3)]).split(frame.area()),
    };

    let height = chunks[0].height as usize;
    let total_lines = app.logs().len();
    let selected = app.list_state().selected().unwrap_or(0);

    let start_idx = selected.saturating_sub(height / 2);
    let end_idx = (start_idx + height).min(total_lines);

    let search_matches = app.search().matches();
    let query = app.search().query();
    let bookmarks = app.bookmark().indices();

    let mut items = Vec::with_capacity(height);

    for idx in start_idx..end_idx {
        if let Some(raw) = app.logs().get_line(idx) {
            if raw.trim().is_empty() {
                continue;
            }

            if let Some(log) = LogLine::new(raw) {
                let is_match = search_matches.contains(&idx);
                let is_bookmarked = bookmarks.contains(&idx);

                let mut line_style = if is_match {
                    Style::default().bg(Color::LightYellow)
                } else {
                    Style::default()
                };

                let bookmark_symbol = if is_bookmarked { "* " } else { "  " };
                if is_bookmarked && !is_match {
                    line_style = line_style.fg(Color::Cyan);
                }

                let level_style = get_style_log_level(log.log_level());

                items.push(
                    ListItem::new(Line::from(vec![
                        Span::styled(bookmark_symbol, Style::default().fg(Color::Green)),
                        Span::styled(
                            format!("[ {} ] ", log.time_stamp()),
                            Style::default().fg(Color::DarkGray),
                        ),
                        Span::styled(format!("{:?} ", log.log_level()), level_style),
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

    let mut window_visible_state = ListState::default();
    window_visible_state.select(Some(selected.saturating_sub(start_idx)));
    frame.render_stateful_widget(list, chunks[0], &mut window_visible_state);

    match app.mode() {
        Mode::Search => {
            let search = Paragraph::new(query).block(
                Block::default()
                    .title("Search (Esc: Exit) ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            );
            frame.render_widget(search, chunks[1]);
            frame.set_cursor_position(Position {
                x: chunks[1].x + query.len() as u16 + 1,
                y: chunks[1].y + 1,
            });
        }
        Mode::BookMark => {
            let bookmark_indices = app.bookmark().indices();
            let mut bookmark_items = Vec::with_capacity(bookmark_indices.len());

            for &idx in bookmark_indices {
                if let Some(raw_line) = app.logs().get_line(idx) {
                    bookmark_items.push(ListItem::new(Line::from(vec![Span::raw(
                        raw_line.to_string(),
                    )])));
                }
            }

            let bookmark_list = List::new(bookmark_items)
                .block(
                    Block::default()
                        .title(" Bookmarks (Enter: Jump | Backspace: Remove | Esc: Exit) ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Cyan)),
                )
                .highlight_style(Style::default().bg(Color::LightYellow))
                .highlight_symbol(">> ");

            frame.render_stateful_widget(bookmark_list, chunks[1], app.bookmark_mut().state());
        }
        Mode::Normal => {
            let status = Paragraph::new(" [/] Search | [b/B] Bookmark | [q] Quit ")
                .block(Block::default().title("Log Analyzer").borders(Borders::ALL));
            frame.render_widget(status, chunks[1]);
        }
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
