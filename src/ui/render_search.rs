use crate::{
    app::App,
    ui::{color_palette::Theme, regex_utils::add_regex_indicator},
};
use ratatui::{prelude::*, widgets::*};

pub fn render_search(frame: &mut Frame, app: &mut App, area: Rect) {
    let query = app.search().query();
    let regex_indicator = add_regex_indicator(app.search().is_regex_search());
    let search_bar_title = Line::from(vec![
        Span::raw(" Search (Enter: Jump | Ctrl-R: Toggle Regex | Esc: Exit) "),
        regex_indicator,
    ]);
    let search = Paragraph::new(query).block(
        Block::default()
            .title(search_bar_title)
            .borders(Borders::ALL)
            .border_style(Theme::SEARCH_BAR),
    );
    frame.render_widget(search, area);
}
