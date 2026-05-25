use crate::{
    app::App,
    log_line::{LogLevel, LogLine},
    ui::build_span_utils::build_span_from_log_line,
};
use ratatui::{prelude::*, widgets::*};

pub fn render_log_list(frame: &mut Frame, app: &mut App, area: Rect) {
    let search_matches = app.search().matches();
    let bookmarks = app.bookmark().indices();
    let horizontal_scroll = app.horizontal_scroll();
    let logs = app.logs();

    let active_highlights = app
        .highlights()
        .iter()
        .filter(|highlight_state| {
            highlight_state.is_active() && !highlight_state.query().is_empty()
        })
        .map(|highlight_state| highlight_state.query())
        .collect::<Vec<_>>();

    let items = app.visible_line_indices().into_iter().filter_map(|idx| {
        let raw = logs.get_line(idx)?;
        if raw.trim().is_empty() {
            return None;
        }

        let log = LogLine::new(raw)?;
        let is_match = search_matches.contains(&idx);
        let is_bookmarked = bookmarks.contains(&idx);

        let line_bg_color = if is_match {
            Style::default().bg(Color::LightYellow)
        } else if is_bookmarked {
            Style::default().bg(Color::Gray)
        } else {
            Style::default()
        };

        let bookmark_symbol = if is_bookmarked { "*" } else { " " };
        let level_style = get_style_log_level(log.log_level());
        let timestamp_style = Style::default().fg(Color::DarkGray);

        let spans = build_span_from_log_line(
            log,
            timestamp_style,
            level_style,
            bookmark_symbol,
            &active_highlights,
            horizontal_scroll,
        );
        Some(ListItem::new(Line::from(spans)).style(line_bg_color))
    });

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_symbol(">");

    frame.render_stateful_widget(list, area, app.list_state_mut());
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
