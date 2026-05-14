use crate::{
    app::App,
    log_line::{LogLevel, LogLine},
    mode::Mode,
};
use ratatui::{prelude::*, widgets::*};

pub fn run_ui(frame: &mut Frame, app: &mut App) {
    let main_layout = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    let header_area = main_layout[0];
    let content_area = main_layout[1];
    let footer_area = main_layout[2];

    let (log_display_area, final_footer_area) = match app.mode() {
        Mode::BookMark => {
            let book_layout = Layout::vertical([Constraint::Fill(1), Constraint::Percentage(30)])
                .split(content_area);
            (book_layout[0], book_layout[1])
        }
        _ => (content_area, footer_area),
    };

    let file_info = format!(" File: {} ", app.logs().file_path.display());
    let header = Paragraph::new(file_info)
        .block(
            Block::default()
                .title(" Log Analyzer ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .alignment(Alignment::Left);
    frame.render_widget(header, header_area);

    let total_lines = app.logs().len();
    let query = app.search().query().to_string();
    let query_len = query.len();
    let search_matches = app.search().matches().to_vec();
    let bookmarks = app.bookmark().indices().to_vec();
    let horizontal_scroll = app.horizontal_scroll();
    let logs = app.logs();

    let mut items = Vec::with_capacity(total_lines);

    for idx in 0..total_lines {
        if let Some(raw) = logs.get_line(idx) {
            if raw.trim().is_empty() {
                continue;
            }

            if let Some(log) = LogLine::new(raw) {
                let is_match = search_matches.contains(&idx);
                let is_bookmarked = bookmarks.contains(&idx);

                let line_bg = if is_match {
                    Style::default().bg(Color::LightYellow)
                } else {
                    Style::default()
                };

                let bookmark_symbol = if is_bookmarked { "*" } else { " " };
                let level_style = get_style_log_level(log.log_level());

                let spans = vec![
                    Span::styled(
                        format!("{bookmark_symbol} "),
                        Style::default().fg(Color::Green),
                    ),
                    Span::styled(
                        format!("[{}] ", log.time_stamp()),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::styled(format!("{:?} ", log.log_level()), level_style),
                    Span::raw(log.content()),
                ];

                let mut current_pos = 0;
                let mut scrolled_spans = Vec::new();

                for span in spans {
                    let content = span.content.as_ref();
                    let len = content.chars().count();

                    if current_pos + len > horizontal_scroll {
                        let offset = horizontal_scroll.saturating_sub(current_pos);
                        let sliced_content: String = content.chars().skip(offset).collect();
                        scrolled_spans.push(Span::styled(sliced_content, span.style));
                    }
                    current_pos += len;
                }

                items.push(ListItem::new(Line::from(scrolled_spans)).style(line_bg));
            }
        }
    }

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, log_display_area, app.list_state_mut());

    match app.mode() {
        Mode::Search => {
            let search = Paragraph::new(query).block(
                Block::default()
                    .title(" Search (Esc: Exit) ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            );
            frame.render_widget(search, final_footer_area);
            frame.set_cursor_position(Position {
                x: final_footer_area.x + query_len as u16 + 1,
                y: final_footer_area.y + 1,
            });
        }
        Mode::BookMark => {
            let bookmark_indices = app.bookmark().indices();
            let mut bookmark_items = Vec::with_capacity(bookmark_indices.len());

            for &idx in bookmark_indices {
                if let Some(raw_line) = logs.get_line(idx) {
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

            frame.render_stateful_widget(
                bookmark_list,
                final_footer_area,
                app.bookmark_mut().state(),
            );
        }
        Mode::Normal => {
            let status = Paragraph::new(" [/] Search | [b/B] Bookmark | [q] Quit ")
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(status, final_footer_area);
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
