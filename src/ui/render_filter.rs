use crate::{
    app::App,
    filter::FilterOp,
    ui::{
        color_palette::Theme,
        regex_utils::add_regex_indicator,
        run_ui::{CHECKED, SEPARATOR, UNCHECKED},
    },
};
use ratatui::{prelude::*, widgets::*};

pub fn render_filter(frame: &mut Frame, app: &mut App, area: Rect) {
    let mut spans = Vec::new();
    let filters = app.filters();
    let selected_idx = app.filter_index();

    let filter_indicator = filters.get(selected_idx).map_or_else(
        || add_regex_indicator(false),
        |filter_state| add_regex_indicator(filter_state.is_regex_filter()),
    );

    for (idx, filter_state) in filters.iter().enumerate() {
        let style = if idx == selected_idx {
            Theme::MATCH_STATE_HIGHLIGHT
        } else if filter_state.is_active() {
            Theme::ACTIVE_STATE_HIGHLIGHT
        } else {
            Theme::INACTIVE_STATE_HIGHLIGHT
        };

        let status = if filter_state.is_active() {
            CHECKED
        } else {
            UNCHECKED
        };
        spans.push(Span::styled(
            format!(" {status} {} ", filter_state.query()),
            style,
        ));

        if idx < filters.len() - 1 {
            let op_str = match filter_state.op() {
                FilterOp::And => " AND ",
                FilterOp::Or => " OR ",
            };
            spans.push(Span::styled(op_str, style));
            spans.push(Span::styled(SEPARATOR, style));
        }
    }

    let filter_bar_title = Line::from(vec![
        Span::raw(
            " Filters (Tab: Move | Enter: Toggle | Up/Down: Op | +: New | BackSpace: Remove | Ctrl-R: Toggle Regex) ",
        ),
        filter_indicator,
    ]);
    let filter_bar = Paragraph::new(Line::from(spans)).block(
        Block::default()
            .title(filter_bar_title)
            .borders(Borders::ALL)
            .border_style(Theme::FOOTER_BORDER),
    );
    frame.render_widget(filter_bar, area);
}
