use crate::{
    app::App,
    ui::run_ui::{CHECKED, SEPARATOR, UNCHECKED},
};
use ratatui::{prelude::*, widgets::*};

pub fn render_highlight(frame: &mut Frame, app: &mut App, area: Rect) {
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
