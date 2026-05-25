use crate::{
    app::App,
    mode::Mode,
    ui::{
        render_footer::render_footer, render_header::render_header,
        render_log_list::render_log_list,
    },
};
use ratatui::prelude::*;

const BOOKMARK_WIDTH: u16 = 30;

pub const CHECKED: &str = "[X]";
pub const UNCHECKED: &str = "[]";
pub const SEPARATOR: &str = " | ";

pub fn run_ui(frame: &mut Frame, app: &mut App) {
    let main_layout = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    let header_area = main_layout[0];
    let content_area = main_layout[1];
    let footer_area = main_layout[2];

    let (log_display_area, final_footer_area) = match app.mode() {
        Mode::BookMark => {
            let bookmark_layout =
                Layout::vertical([Constraint::Fill(1), Constraint::Percentage(BOOKMARK_WIDTH)])
                    .split(content_area);
            (bookmark_layout[0], bookmark_layout[1])
        }
        _ => (content_area, footer_area),
    };

    render_header(frame, app, header_area);
    render_log_list(frame, app, log_display_area);
    render_footer(frame, app, final_footer_area);
}
