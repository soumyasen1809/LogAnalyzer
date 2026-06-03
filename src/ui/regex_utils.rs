use crate::ui::color_palette::Theme;
use ratatui::prelude::*;

pub fn add_regex_indicator<'regex>(is_regex_on: bool) -> Span<'regex> {
    if is_regex_on {
        Span::styled("[REGEX ON]", Theme::INDICATOR_ON)
    } else {
        Span::styled("[REGEX OFF]", Theme::INDICATOR_OFF)
    }
}
