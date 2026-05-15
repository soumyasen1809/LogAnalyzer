use crate::{
    app::App,
    filter::{FilterOp, FilterState},
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
            let bookmark_layout =
                Layout::vertical([Constraint::Fill(1), Constraint::Percentage(30)])
                    .split(content_area);
            (bookmark_layout[0], bookmark_layout[1])
        }
        _ => (content_area, footer_area),
    };

    render_header(frame, app, header_area);
    render_log_list(frame, app, log_display_area);
    render_footer(frame, app, final_footer_area);
}

fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let file_info = format!(" File: {} ", app.logs().file_path.display());
    let header = Paragraph::new(file_info)
        .block(
            Block::default()
                .title(" Log Analyzer ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .alignment(Alignment::Left);
    frame.render_widget(header, area);
}

fn render_log_list(frame: &mut Frame, app: &mut App, area: Rect) {
    let total_lines = app.logs().len();
    let search_matches = app.search().matches();
    let bookmarks = app.bookmark().indices();
    let horizontal_scroll = app.horizontal_scroll();
    let logs = app.logs();
    let filters = app.filters();

    let active_filters: Vec<&FilterState> = filters.iter().filter(|f| f.is_active()).collect();

    let active_highlights: Vec<&str> = app
        .highlights()
        .iter()
        .filter(|h| h.is_active() && !h.query().is_empty())
        .map(|h| h.query())
        .collect();

    let mut items = Vec::with_capacity(total_lines);

    for idx in 0..total_lines {
        let Some(raw) = logs.get_line(idx) else {
            continue;
        };
        if raw.trim().is_empty() {
            continue;
        }

        if !active_filters.is_empty() {
            let mut line_matches = raw.contains(active_filters[0].query());
            for i in 0..active_filters.len().saturating_sub(1) {
                let current = active_filters[i];
                let next_match = raw.contains(active_filters[i + 1].query());
                match current.op() {
                    FilterOp::And => line_matches = line_matches && next_match,
                    FilterOp::Or => line_matches = line_matches || next_match,
                }
            }
            if !line_matches {
                continue;
            }
        }

        let Some(log) = LogLine::new(raw) else {
            continue;
        };
        let is_match = search_matches.contains(&idx);
        let is_bookmarked = bookmarks.contains(&idx);

        let line_bg_color = if is_match {
            Style::default().bg(Color::LightYellow)
        } else {
            Style::default()
        };

        let bookmark_symbol = if is_bookmarked { "*" } else { " " };
        let level_style = get_style_log_level(log.log_level());
        let timestamp_style = Style::default().fg(Color::DarkGray);

        let mut spans = vec![Span::styled(
            format!("{bookmark_symbol} "),
            Style::default().fg(Color::Green),
        )];

        let timestamp_str = format!("[{}] ", log.time_stamp());
        let level_str = format!("{:?} ", log.log_level());

        if !active_highlights.is_empty() {
            build_multi_highlighted_spans(
                &timestamp_str,
                &active_highlights,
                timestamp_style,
                &mut spans,
            );
            build_multi_highlighted_spans(&level_str, &active_highlights, level_style, &mut spans);
            build_multi_highlighted_spans(
                log.content(),
                &active_highlights,
                Style::default(),
                &mut spans,
            );
        } else {
            spans.push(Span::styled(timestamp_str, timestamp_style));
            spans.push(Span::styled(level_str, level_style));
            spans.push(Span::raw(log.content()));
        }

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

        items.push(ListItem::new(Line::from(scrolled_spans)).style(line_bg_color));
    }

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_symbol(">");

    frame.render_stateful_widget(list, area, app.list_state_mut());
}

fn render_footer(frame: &mut Frame, app: &mut App, area: Rect) {
    match app.mode() {
        Mode::Search => {
            let query = app.search().query();
            let search = Paragraph::new(query).block(
                Block::default()
                    .title(" Search (Esc: Exit) ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            );
            frame.render_widget(search, area);
            frame.set_cursor_position(Position {
                x: area.x + query.len() as u16 + 1,
                y: area.y + 1,
            });
        }
        Mode::BookMark => {
            let bookmark_indices = app.bookmark().indices();
            let logs = app.logs();
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
                .highlight_spacing(HighlightSpacing::Always)
                .highlight_symbol(">>");

            frame.render_stateful_widget(bookmark_list, area, app.bookmark_mut().state());
        }
        Mode::Filter => {
            let mut spans = Vec::new();
            let filters = app.filters();
            let selected_idx = app.filter_index();

            for (i, f) in filters.iter().enumerate() {
                let style = if i == selected_idx {
                    Style::default().bg(Color::LightYellow)
                } else if f.is_active() {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::DarkGray)
                };

                let status = if f.is_active() { "[X]" } else { "[]" };
                spans.push(Span::styled(format!(" {status} {} ", f.query()), style));

                if i < filters.len() - 1 {
                    let op_str = match f.op() {
                        FilterOp::And => " AND | ",
                        FilterOp::Or => " OR | ",
                    };
                    spans.push(Span::styled(op_str, style));
                }
            }
            let filter_bar = Paragraph::new(Line::from(spans)).block(
                Block::default()
                    .title(
                        " Filters (Tab: Move | Enter: Toggle | Up/Down: Op | BS: Remove | +: New) ",
                    )
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            );
            frame.render_widget(filter_bar, area);
        }
        Mode::Highlight => {
            let mut spans = Vec::new();
            let highlights = app.highlights();
            let selected_idx = app.highlight_index();

            for (i, h) in highlights.iter().enumerate() {
                let style = if i == selected_idx {
                    Style::default().bg(Color::LightYellow)
                } else if h.is_active() {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::DarkGray)
                };

                let status = if h.is_active() { "[X]" } else { "[]" };
                spans.push(Span::styled(format!(" {status} {} ", h.query()), style));

                if i < highlights.len() - 1 {
                    spans.push(Span::styled(" | ", Style::default().fg(Color::DarkGray)));
                }
            }
            let highlight_bar = Paragraph::new(Line::from(spans)).block(
                Block::default()
                    .title(" Highlights (Tab: Move | Enter: Toggle | BS: Remove | +: New) ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            );
            frame.render_widget(highlight_bar, area);
        }
        Mode::Normal => {
            let status = Paragraph::new(
                " [/] Search | [b/B] Bookmark | [f] Filter | [h] Highlight | [q] Quit ",
            )
            .block(Block::default().borders(Borders::ALL));
            frame.render_widget(status, area);
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

fn build_multi_highlighted_spans<'a>(
    content: &'a str,
    highlights: &[&str],
    base_style: Style,
    spans: &mut Vec<Span<'a>>,
) {
    let mut fragments = vec![(content, false)];

    for term in highlights {
        if term.is_empty() {
            continue;
        }
        let term_lower = term.to_lowercase();
        let mut next_fragments = Vec::with_capacity(fragments.len() * 2);

        for (text, is_highlighted) in fragments {
            if is_highlighted {
                next_fragments.push((text, true));
                continue;
            }

            let mut current_pos = 0;
            while let Some(start_offset) = text[current_pos..].to_lowercase().find(&term_lower) {
                let match_start = current_pos + start_offset;
                let match_end = match_start + term.len();

                if match_start > current_pos {
                    next_fragments.push((&text[current_pos..match_start], false));
                }
                next_fragments.push((&text[match_start..match_end], true));
                current_pos = match_end;
            }

            if current_pos < text.len() {
                next_fragments.push((&text[current_pos..], false));
            }
        }
        fragments = next_fragments;
    }

    for (text, is_highlighted) in fragments {
        if is_highlighted {
            spans.push(Span::styled(text, base_style.bg(Color::LightGreen)));
        } else {
            spans.push(Span::styled(text, base_style));
        }
    }
}
