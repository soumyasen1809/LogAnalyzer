use crate::{log_line::LogLine, ui::color_palette::Theme};
use ratatui::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum FragmentType {
    Highlighted(usize),
    Normal,
}

pub fn build_span_from_log_line<'span>(
    log: LogLine,
    timestamp_style: Style,
    level_style: Style,
    bookmark_symbol: &str,
    active_highlights: &[&str],
    horizontal_scroll: usize,
) -> Vec<Span<'span>> {
    let mut spans = vec![Span::styled(
        format!("{bookmark_symbol} "),
        Theme::BOOKMARK_MARKER,
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
            Theme::DEFAULT_THEME,
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

pub fn build_multi_highlighted_spans<'highlight>(
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
        Color::Indexed(150), // Matte Mint Green
        Color::Indexed(110), // Muted Steel Blue
        Color::Indexed(116), // Pastel Sage Teal
        Color::Indexed(182), // Muted Mauve
        Color::Indexed(167), // Terracotta
        Color::Indexed(222), // Parchment
        Color::Indexed(216), // Soft Peach
        Color::Indexed(146), // Lavender Blue
        Color::Indexed(174), // Dusty Rose
        Color::Indexed(245), // Slate Gray
    ]
}

fn get_highlight_color(index: usize) -> Color {
    let all_colors = get_all_highlight_colors();
    all_colors[index % all_colors.len()]
}
