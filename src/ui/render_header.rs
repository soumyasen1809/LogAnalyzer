use crate::{app::App, ui::color_palette::Theme};
use ratatui::{prelude::*, widgets::*};

pub fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let file_info = format!(" File: {} ", app.logs().file_path.display());
    let header = Paragraph::new(file_info)
        .block(
            Block::default()
                .title(" Log Analyzer ")
                .borders(Borders::ALL)
                .border_style(Theme::HEADER_BORDER),
        )
        .alignment(Alignment::Left);
    frame.render_widget(header, area);
}
