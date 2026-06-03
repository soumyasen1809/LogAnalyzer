use ratatui::prelude::*;

// --- Palette Primitives (ANSI 256 Matte) ---
const COLOR_MINT: Color = Color::Indexed(150); // Primary accent / Success
const COLOR_STEEL_BLUE: Color = Color::Indexed(110); // Secondary accent / Info
const COLOR_MAUVE: Color = Color::Indexed(182); // Highlight/Notice
const COLOR_TERRACOTTA: Color = Color::Indexed(167); // Error / Critical
const COLOR_PARCHMENT: Color = Color::Indexed(222); // Warning / Attention
const COLOR_SAGE_TEAL: Color = Color::Indexed(116); // Normal interactive state
const COLOR_SLATE: Color = Color::Indexed(245); // Muted background / Structural Borders
const COLOR_CHARCOAL: Color = Color::Indexed(237); // Dark block selection background

/// Centralized application style definitions
pub struct Theme;

impl Theme {
    pub const DEFAULT_THEME: Style = Style::new();

    pub const HEADER_BORDER: Style = Style::new().fg(COLOR_MINT);
    pub const FOOTER_BORDER: Style = Style::new().fg(COLOR_STEEL_BLUE);
    pub const COMPONENT_TITLE: Style = Style::new().fg(COLOR_MINT).add_modifier(Modifier::BOLD);

    pub const TIMESTAMP_MATCH: Style = Style::new().fg(COLOR_CHARCOAL);
    pub const LOG_LEVEL_ERROR: Style = Style::new()
        .fg(COLOR_TERRACOTTA)
        .add_modifier(Modifier::BOLD);
    pub const LOG_LEVEL_WARN: Style = Style::new()
        .fg(COLOR_PARCHMENT)
        .add_modifier(Modifier::BOLD);
    pub const LOG_LEVEL_INFO: Style = Style::new()
        .fg(COLOR_STEEL_BLUE)
        .add_modifier(Modifier::BOLD);
    pub const LOG_LEVEL_DEBUG: Style = Style::new().fg(COLOR_MINT).add_modifier(Modifier::BOLD);
    pub const LOG_LEVEL_TRACE: Style = Style::new().fg(COLOR_MAUVE).add_modifier(Modifier::BOLD);

    pub const SELECTION_MATCH: Style = Style::new().bg(COLOR_PARCHMENT);
    pub const BOOKMARK_LINE: Style = Style::new().bg(COLOR_SLATE).add_modifier(Modifier::BOLD);
    pub const ACTIVE_STATE_HIGHLIGHT: Style = Style::new().bg(COLOR_MINT);
    pub const MATCH_STATE_HIGHLIGHT: Style = Style::new().bg(COLOR_MAUVE);
    pub const INACTIVE_STATE_HIGHLIGHT: Style = Style::new().bg(COLOR_SLATE);
    pub const ACTIVE_CURSOR: Style = Style::new()
        .fg(COLOR_TERRACOTTA)
        .add_modifier(Modifier::BOLD);
    pub const BOOKMARK_MARKER: Style = Style::new().fg(COLOR_MINT);
    pub const INDICATOR_ON: Style = Style::new().fg(COLOR_MINT);
    pub const INDICATOR_OFF: Style = Style::new().fg(COLOR_SLATE);
    pub const SEPERATOR: Style = Style::new().fg(COLOR_CHARCOAL);

    pub const SEARCH_BAR: Style = Style::new().fg(COLOR_STEEL_BLUE);
    pub const FILTER_BAR: Style = Style::new().fg(COLOR_SAGE_TEAL);
    pub const HIGHLIGHT_BAR: Style = Style::new().fg(COLOR_MAUVE);
    pub const BOOKMARK_BAR: Style = Style::new().fg(COLOR_MINT);
}
