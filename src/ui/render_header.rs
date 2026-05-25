use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn render_header(frame: &mut Frame, app: &App, area: Rect) {
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
