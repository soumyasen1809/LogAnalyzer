use crate::{
    app::App,
    mode::Mode,
    ui::{
        render_bookmark::render_bookmark, render_filter::render_filter,
        render_highlight::render_highlight, render_normal::render_normal,
        render_search::render_search,
    },
};
use ratatui::prelude::*;

pub fn render_footer(frame: &mut Frame, app: &mut App, area: Rect) {
    match app.mode() {
        Mode::Search => render_search(frame, app, area),
        Mode::BookMark => render_bookmark(frame, app, area),
        Mode::Filter => render_filter(frame, app, area),
        Mode::Highlight => render_highlight(frame, app, area),
        Mode::Normal => render_normal(frame, app, area),
    }
}
