use crate::{app::App, ui::color_palette::Theme};
use ratatui::{prelude::*, widgets::*};

pub fn render_bookmark(frame: &mut Frame, app: &mut App, area: Rect) {
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
                .border_style(Theme::BOOKMARK_BAR),
        )
        .highlight_style(Theme::ACTIVE_STATE_HIGHLIGHT)
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_symbol(">>");

    frame.render_stateful_widget(bookmark_list, area, app.bookmark_mut().state());
}
