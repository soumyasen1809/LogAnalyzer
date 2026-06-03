use crate::{
    app::App,
    log_line::{LogLevel, LogLine},
    ui::{build_span_utils::build_span_from_log_line, color_palette::Theme},
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
            Theme::SELECTION_MATCH
        } else if is_bookmarked {
            Theme::BOOKMARK_LINE
        } else {
            Theme::DEFAULT_THEME
        };

        let bookmark_symbol = if is_bookmarked { "*" } else { " " };
        let level_style = get_style_log_level(log.log_level());
        let timestamp_style = Theme::TIMESTAMP_MATCH;

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
        .highlight_style(Theme::ACTIVE_CURSOR)
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_symbol(">");

    frame.render_stateful_widget(list, area, app.list_state_mut());
}

fn get_style_log_level(level: &LogLevel) -> Style {
    match level {
        LogLevel::Error => Theme::LOG_LEVEL_ERROR,
        LogLevel::Warn => Theme::LOG_LEVEL_WARN,
        LogLevel::Info => Theme::LOG_LEVEL_INFO,
        LogLevel::Debug => Theme::LOG_LEVEL_DEBUG,
        LogLevel::Trace => Theme::LOG_LEVEL_TRACE,
    }
}
