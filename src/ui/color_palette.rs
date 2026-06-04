use ratatui::prelude::*;

const COLOR_MINT: Color = Color::Indexed(150); // Primary accent / Success
const COLOR_STEEL_BLUE: Color = Color::Indexed(110); // Secondary accent / Info
const COLOR_MAUVE: Color = Color::Indexed(182); // Highlight/Notice
const COLOR_TERRACOTTA: Color = Color::Indexed(167); // Error / Critical
const COLOR_PARCHMENT: Color = Color::Indexed(222); // Warning / Attention
const COLOR_SLATE: Color = Color::Indexed(245); // Muted background / Structural Borders
const COLOR_CHARCOAL: Color = Color::Indexed(237); // Dark block selection background
const COLOR_DEEP_BLUE: Color = Color::Indexed(24); // Deep Slate Blue (Bar backgrounds)
const COLOR_DEEP_TEAL: Color = Color::Indexed(30); // Deep Forest Teal
const COLOR_DEEP_PURPLE: Color = Color::Indexed(60); // Deep Plum / Amethyst
const COLOR_DEEP_GREEN: Color = Color::Indexed(65); // Deep Olive / Moss
const COLOR_DEEP_BRONZE: Color = Color::Indexed(130); // Muted Dark Orange / Ochre
const COLOR_LIGHT_GRAY: Color = Color::Indexed(250); // High contrast text foreground
const COLOR_DARK_GRAY: Color = Color::Indexed(236); // Soft Dark Gray
const COLOR_DEEP_RED: Color = Color::Indexed(124); // Rich Crimson / Error
const COLOR_DEEP_AMBER: Color = Color::Indexed(172); // Dark Amber / Warning
const COLOR_DEEP_STEEL: Color = Color::Indexed(67); // Muted Medium Steel Blue / Info
const COLOR_DEEP_SAGE: Color = Color::Indexed(107); // Matte Olive Green / Debug
const COLOR_DEEP_PLUM: Color = Color::Indexed(139); // Desaturated Deep Purple / Trace

/// Centralized application style definitions
pub struct Theme;

impl Theme {
    pub const DEFAULT_THEME: Style = Style::new();

    pub const HEADER_BORDER: Style = Style::new().fg(COLOR_MINT);
    pub const FOOTER_BORDER: Style = Style::new().fg(COLOR_STEEL_BLUE);
    pub const COMPONENT_TITLE: Style = Style::new().fg(COLOR_MINT).add_modifier(Modifier::BOLD);

    pub const TIMESTAMP_MATCH: Style = Style::new().fg(COLOR_DARK_GRAY);
    pub const LOG_LEVEL_ERROR: Style = Style::new().fg(COLOR_DEEP_RED).add_modifier(Modifier::BOLD);
    pub const LOG_LEVEL_WARN: Style = Style::new()
        .fg(COLOR_DEEP_AMBER)
        .add_modifier(Modifier::BOLD);
    pub const LOG_LEVEL_INFO: Style = Style::new()
        .fg(COLOR_DEEP_STEEL)
        .add_modifier(Modifier::BOLD);
    pub const LOG_LEVEL_DEBUG: Style = Style::new()
        .fg(COLOR_DEEP_SAGE)
        .add_modifier(Modifier::BOLD);
    pub const LOG_LEVEL_TRACE: Style = Style::new()
        .fg(COLOR_DEEP_PLUM)
        .add_modifier(Modifier::BOLD);

    pub const SELECTION_MATCH: Style = Style::new().bg(COLOR_PARCHMENT);
    pub const BOOKMARK_LINE: Style = Style::new()
        .bg(COLOR_LIGHT_GRAY)
        .add_modifier(Modifier::BOLD);
    pub const ACTIVE_STATE_HIGHLIGHT: Style = Style::new().bg(COLOR_MINT);
    pub const MATCH_STATE_HIGHLIGHT: Style = Style::new().bg(COLOR_MAUVE);
    pub const INACTIVE_STATE_HIGHLIGHT: Style = Style::new().bg(COLOR_SLATE);
    pub const ACTIVE_CURSOR: Style = Style::new()
        .fg(COLOR_TERRACOTTA)
        .add_modifier(Modifier::BOLD);
    pub const BOOKMARK_MARKER: Style = Style::new().fg(COLOR_DEEP_BRONZE);
    pub const INDICATOR_ON: Style = Style::new().fg(COLOR_MINT);
    pub const INDICATOR_OFF: Style = Style::new().fg(COLOR_SLATE);
    pub const SEPERATOR: Style = Style::new().fg(COLOR_CHARCOAL);

    pub const SEARCH_BAR: Style = Style::new().fg(COLOR_DEEP_BLUE);
    pub const FILTER_BAR: Style = Style::new().fg(COLOR_DEEP_TEAL);
    pub const HIGHLIGHT_BAR: Style = Style::new().fg(COLOR_DEEP_PURPLE);
    pub const BOOKMARK_BAR: Style = Style::new().fg(COLOR_DEEP_GREEN);
}
