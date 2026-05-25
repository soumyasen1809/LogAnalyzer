use ratatui::prelude::*;

pub fn add_regex_indicator<'regex>(is_regex_on: bool) -> Span<'regex> {
    if is_regex_on {
        Span::styled("[REGEX ON]", Style::default().fg(Color::Green))
    } else {
        Span::styled("[REGEX OFF]", Style::default().fg(Color::DarkGray))
    }
}
