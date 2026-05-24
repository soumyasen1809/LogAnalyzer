use crate::{
    app::App,
    filter::FilterOp,
    log_line::{LogLevel, LogLine},
    mode::Mode,
};
use ratatui::{prelude::*, widgets::*};

const BOOKMARK_WIDTH: u16 = 30;

const CHECKED: &str = "[X]";
const UNCHECKED: &str = "[]";
const SEPARATOR: &str = " | ";

#[derive(Clone, Copy, Debug, PartialEq)]
enum FragmentType {
    Highlighted(usize),
    Normal,
}

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
                Layout::vertical([Constraint::Fill(1), Constraint::Percentage(BOOKMARK_WIDTH)])
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

fn render_footer(frame: &mut Frame, app: &mut App, area: Rect) {
    match app.mode() {
        Mode::Search => render_search(frame, app, area),
        Mode::BookMark => render_bookmark(frame, app, area),
        Mode::Filter => render_filter(frame, app, area),
        Mode::Highlight => render_highlight(frame, app, area),
        Mode::Normal => render_normal(frame, app, area),
    }
}

fn render_search(frame: &mut Frame, app: &mut App, area: Rect) {
    let query = app.search().query();
    let regex_indicator = add_regex_indicator(app.search().is_regex_search());
    let search_bar_title = Line::from(vec![
        Span::raw(" Search (Enter: Jump | Ctrl-R: Toggle Regex | Esc: Exit) "),
        regex_indicator,
    ]);
    let search = Paragraph::new(query).block(
        Block::default()
            .title(search_bar_title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    frame.render_widget(search, area);
}

fn add_regex_indicator<'regex>(is_regex_on: bool) -> Span<'regex> {
    if is_regex_on {
        Span::styled("[REGEX ON]", Style::default().fg(Color::Green))
    } else {
        Span::styled("[REGEX OFF]", Style::default().fg(Color::DarkGray))
    }
}

fn render_bookmark(frame: &mut Frame, app: &mut App, area: Rect) {
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

fn render_filter(frame: &mut Frame, app: &mut App, area: Rect) {
    let mut spans = Vec::new();
    let filters = app.filters();
    let selected_idx = app.filter_index();

    let filter_indicator = filters.get(selected_idx).map_or_else(
        || add_regex_indicator(false),
        |filter_state| add_regex_indicator(filter_state.is_regex_filter()),
    );

    for (idx, filter_state) in filters.iter().enumerate() {
        let style = if idx == selected_idx {
            Style::default().bg(Color::LightYellow)
        } else if filter_state.is_active() {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let status = if filter_state.is_active() {
            CHECKED
        } else {
            UNCHECKED
        };
        spans.push(Span::styled(
            format!(" {status} {} ", filter_state.query()),
            style,
        ));

        if idx < filters.len() - 1 {
            let op_str = match filter_state.op() {
                FilterOp::And => " AND ",
                FilterOp::Or => " OR ",
            };
            spans.push(Span::styled(op_str, style));
            spans.push(Span::styled(SEPARATOR, style));
        }
    }

    let filter_bar_title = Line::from(vec![
        Span::raw(
            " Filters (Tab: Move | Enter: Toggle | Up/Down: Op | BackSpace: Remove | +: New) ",
        ),
        filter_indicator,
    ]);
    let filter_bar = Paragraph::new(Line::from(spans)).block(
        Block::default()
            .title(filter_bar_title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    frame.render_widget(filter_bar, area);
}

fn render_highlight(frame: &mut Frame, app: &mut App, area: Rect) {
    let mut spans = Vec::new();
    let highlights = app.highlights();
    let selected_idx = app.highlight_index();

    for (idx, highlight_state) in highlights.iter().enumerate() {
        let style = if idx == selected_idx {
            Style::default().bg(Color::LightYellow)
        } else if highlight_state.is_active() {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let status = if highlight_state.is_active() {
            CHECKED
        } else {
            UNCHECKED
        };
        spans.push(Span::styled(
            format!(" {status} {} ", highlight_state.query()),
            style,
        ));

        if idx < highlights.len() - 1 {
            spans.push(Span::styled(
                SEPARATOR,
                Style::default().fg(Color::DarkGray),
            ));
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

fn render_normal(frame: &mut Frame, _app: &mut App, area: Rect) {
    let status =
        Paragraph::new(" [/] Search | [b/B] Bookmark | [f] Filter | [h] Highlight | [q] Quit ")
            .block(Block::default().borders(Borders::ALL));
    frame.render_widget(status, area);
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

fn build_span_from_log_line<'span>(
    log: LogLine,
    timestamp_style: Style,
    level_style: Style,
    bookmark_symbol: &str,
    active_highlights: &[&str],
    horizontal_scroll: usize,
) -> Vec<Span<'span>> {
    let mut spans = vec![Span::styled(
        format!("{bookmark_symbol} "),
        Style::default().fg(Color::Green),
    )];

    let timestamp_str = format!("[{}] ", log.time_stamp());
    let level_str = format!("{:?} ", log.log_level());

    if !active_highlights.is_empty() {
        build_multi_highlighted_spans(
            &timestamp_str,
            active_highlights,
            timestamp_style,
            &mut spans,
        );
        build_multi_highlighted_spans(&level_str, active_highlights, level_style, &mut spans);
        build_multi_highlighted_spans(
            log.content(),
            active_highlights,
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
            let sliced_content = content.chars().skip(offset).collect::<String>();
            scrolled_spans.push(Span::styled(sliced_content, span.style));
        }
        current_pos += len;
    }

    scrolled_spans
}

fn build_multi_highlighted_spans<'highlight>(
    content: &'highlight str,
    highlights: &[&str],
    base_style: Style,
    spans: &mut Vec<Span<'highlight>>,
) {
    let mut fragments: Vec<(&'highlight str, FragmentType)> = vec![(content, FragmentType::Normal)];

    for (term_idx, term) in highlights
        .iter()
        .enumerate()
        .filter(|(_, term)| !term.is_empty())
    {
        fragments = fragments
            .into_iter()
            .flat_map(|(text, frag_type)| match frag_type {
                FragmentType::Highlighted(_) => {
                    vec![(text, frag_type)]
                }
                FragmentType::Normal => split_fragment(text, term, term_idx),
            })
            .collect();
    }

    spans.extend(fragments.iter().map(|&(text, frag_type)| {
        let style = match frag_type {
            FragmentType::Highlighted(idx) => base_style.bg(get_highlight_color(idx)),
            FragmentType::Normal => base_style,
        };
        Span::styled(text, style)
    }));
}

fn split_fragment<'split>(
    text: &'split str,
    term: &str,
    term_idx: usize,
) -> Vec<(&'split str, FragmentType)> {
    if term.is_empty() {
        return vec![(text, FragmentType::Normal)];
    }

    let mut result = Vec::new();
    let mut prev_end = 0;

    for (start, _) in text.char_indices() {
        if start < prev_end {
            continue;
        }

        if text[start..].len() >= term.len()
            && text[start..start + term.len()].eq_ignore_ascii_case(term)
        {
            let end = start + term.len();

            if prev_end < start {
                result.push((&text[prev_end..start], FragmentType::Normal));
            }
            result.push((&text[start..end], FragmentType::Highlighted(term_idx)));
            prev_end = end;
        }
    }

    if prev_end < text.len() {
        result.push((&text[prev_end..], FragmentType::Normal));
    }

    result
}

fn get_all_highlight_colors<'color>() -> &'color [Color] {
    &[
        Color::LightGreen,
        Color::LightBlue,
        Color::LightCyan,
        Color::LightMagenta,
        Color::LightRed,
        Color::LightYellow,
    ]
}

fn get_highlight_color(index: usize) -> Color {
    let all_colors = get_all_highlight_colors();
    all_colors[index % all_colors.len()]
}
