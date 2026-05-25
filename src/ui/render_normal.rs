use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn render_normal(frame: &mut Frame, _app: &mut App, area: Rect) {
    let status =
        Paragraph::new(" [/] Search | [b/B] Bookmark | [f] Filter | [h] Highlight | [q] Quit ")
            .block(Block::default().borders(Borders::ALL));
    frame.render_widget(status, area);
}
