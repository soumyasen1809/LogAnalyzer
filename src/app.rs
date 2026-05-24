use crate::{
    bookmark::BookMark,
    filter::{FilterOp, FilterState},
    highlight::HighlightState,
    log_line::LogLine,
    log_store::LogStore,
    mode::Mode,
    search::SearchState,
};
use ratatui::widgets::ListState;
use std::sync::Arc;
use tokio::sync::oneshot;

const SCROLL_AMOUNT: usize = 10;

#[derive(Debug)]
pub struct App {
    logs: Arc<LogStore>,
    mode: Mode,
    search: SearchState,
    bookmark: BookMark,
    filters: Vec<FilterState>,
    filter_index: usize,
    highlights: Vec<HighlightState>,
    highlight_index: usize,
    list_state: ListState,
    horizontal_scroll: usize,
    rx: Option<oneshot::Receiver<Vec<usize>>>,
}

impl App {
    pub fn new(logs: Arc<LogStore>) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            logs,
            mode: Mode::default(),
            search: SearchState::new(),
            bookmark: BookMark::new(),
            filters: Vec::new(),
            filter_index: 0,
            highlights: Vec::new(),
            highlight_index: 0,
            list_state,
            horizontal_scroll: 0,
            rx: None,
        }
    }

    pub fn logs(&self) -> Arc<LogStore> {
        Arc::clone(&self.logs)
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn search(&self) -> &SearchState {
        &self.search
    }

    pub fn search_mut(&mut self) -> &mut SearchState {
        &mut self.search
    }

    pub fn bookmark(&self) -> &BookMark {
        &self.bookmark
    }

    pub fn bookmark_mut(&mut self) -> &mut BookMark {
        &mut self.bookmark
    }

    pub fn filters(&self) -> &[FilterState] {
        &self.filters
    }

    pub fn filter_index(&self) -> usize {
        self.filter_index
    }

    pub fn highlights(&self) -> &[HighlightState] {
        &self.highlights
    }

    pub fn highlight_index(&self) -> usize {
        self.highlight_index
    }

    pub fn list_state(&self) -> ListState {
        self.list_state
    }

    pub fn list_state_mut(&mut self) -> &mut ListState {
        &mut self.list_state
    }

    pub fn horizontal_scroll(&self) -> usize {
        self.horizontal_scroll
    }

    pub fn enter_search_mode(&mut self) {
        self.mode = Mode::Search;
    }

    pub fn exit_search_mode(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn enter_bookmark_mode(&mut self) {
        self.mode = Mode::BookMark;
    }

    pub fn exit_bookmark_mode(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn enter_filter_mode(&mut self) {
        if self.filters.is_empty() {
            self.filters.push(FilterState::new());
        }
        self.mode = Mode::Filter;
    }

    pub fn exit_filter_mode(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn enter_highlight_mode(&mut self) {
        if self.highlights.is_empty() {
            self.highlights.push(HighlightState::new());
        }
        self.mode = Mode::Highlight;
    }

    pub fn exit_highlight_mode(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn handle_search_char(&mut self, c: char) {
        self.search.push_char(c);
    }

    pub fn handle_search_backspace(&mut self) {
        self.search.pop_char();
    }

    pub fn clear_search(&mut self) {
        self.search.clear();
    }

    pub fn toggle_regex_search(&mut self) {
        let current_state = self.search().is_regex_search();
        self.search_mut().set_is_regex_search(!current_state);

        self.run_search();
    }

    pub fn run_search(&mut self) {
        let (tx, rx) = oneshot::channel();
        self.rx = Some(rx);

        let logs = Arc::clone(&self.logs);
        self.search_mut().run_search(logs, tx);
    }

    pub fn receive_search_result(&mut self) {
        if let Some(matches) = self.rx.as_mut().and_then(|r| r.try_recv().ok()) {
            self.search.set_matches(matches);
            if let Some(first) = self.search.current_match_index() {
                let visible = self.visible_line_indices();
                if let Some(idx) = visible.iter().position(|&idx| idx == first) {
                    self.list_state.select(Some(idx));
                } else {
                    self.next_search_match();
                }
            }
            self.rx = None;
        }
    }

    pub fn visible_line_indices(&self) -> Vec<usize> {
        let active_filters = self
            .filters
            .iter()
            .filter(|filter_state| filter_state.is_active())
            .collect::<Vec<_>>();
        let bookmarks = self.bookmark.indices();
        let total_lines = self.logs.len();

        (0..total_lines)
            .filter(|&idx| {
                let Some(raw) = self.logs.get_line(idx) else {
                    return false;
                };
                if raw.trim().is_empty() || LogLine::new(raw).is_none() {
                    return false;
                }
                if bookmarks.contains(&idx) || active_filters.is_empty() {
                    // Bookmarked lines will remain visible inspite of the filters
                    return true;
                }

                let mut line_matches = raw
                    .to_lowercase()
                    .contains(&active_filters[0].query().to_lowercase());

                for i in 0..active_filters.len().saturating_sub(1) {
                    let current = active_filters[i];
                    let next_match = raw
                        .to_lowercase()
                        .contains(&active_filters[i + 1].query().to_lowercase());
                    line_matches = match current.op() {
                        FilterOp::And => line_matches && next_match,
                        FilterOp::Or => line_matches || next_match,
                    };
                }
                line_matches
            })
            .collect()
    }

    pub fn next_search_match(&mut self) {
        let match_count = self.search.matches().len();
        if match_count == 0 {
            return;
        }

        let visible = self.visible_line_indices();
        if visible.is_empty() {
            return;
        }

        for _ in 0..match_count {
            self.search.next_match();
            if let Some(match_idx) = self.search.current_match_index()
                && let Some(idx) = visible.iter().position(|&idx| idx == match_idx)
            {
                self.list_state.select(Some(idx));
                return;
            }
        }
    }

    pub fn select_next(&mut self) {
        let current = self.list_state.selected().unwrap_or(0);
        let next = (current + 1).min(self.visible_line_indices().len().saturating_sub(1));
        self.list_state.select(Some(next));
    }

    pub fn select_previous(&mut self) {
        let current = self.list_state.selected().unwrap_or(0);
        self.list_state.select(Some(current.saturating_sub(1)));
    }

    pub fn scroll_right(&mut self) {
        self.horizontal_scroll = self.horizontal_scroll.saturating_add(SCROLL_AMOUNT);
    }

    pub fn scroll_left(&mut self) {
        self.horizontal_scroll = self.horizontal_scroll.saturating_sub(SCROLL_AMOUNT);
    }

    pub fn scroll_up(&mut self) {
        self.list_state.scroll_up_by(SCROLL_AMOUNT as u16);
    }

    pub fn scroll_down(&mut self) {
        self.list_state.scroll_down_by(SCROLL_AMOUNT as u16);
    }

    pub fn select_next_bookmark(&mut self) {
        self.bookmark.select_next_bookmark();
    }

    pub fn select_previous_bookmark(&mut self) {
        self.bookmark.select_previous_bookmark();
    }

    pub fn add_bookmark(&mut self) {
        if let Some(log_idx) = self.list_state.selected() {
            self.bookmark.add_bookmark(log_idx);
        }
    }

    pub fn remove_bookmark(&mut self) {
        if let Some(state_idx) = self.bookmark.state().selected()
            && let Some(&log_idx) = self.bookmark.indices().get(state_idx)
        {
            self.bookmark.remove_bookmark(log_idx);
        }
    }

    pub fn jump_to_bookmark(&mut self) {
        if let Some(state_idx) = self.bookmark.state().selected()
            && let Some(&log_idx) = self.bookmark.indices().get(state_idx)
        {
            self.list_state.select(Some(log_idx));
        }
    }

    pub fn next_filter(&mut self) {
        if !self.filters.is_empty() {
            self.filter_index = (self.filter_index + 1) % self.filters.len();

            if let Some(filter_state) = self.filters.get_mut(self.filter_index) {
                filter_state.set_is_editing(false);
            }
        }
    }

    pub fn toggle_filter(&mut self) {
        if let Some(filter_state) = self.filters.get_mut(self.filter_index) {
            let active = filter_state.is_active();
            filter_state.set_active(!active);
            filter_state.set_is_editing(false);
        }
    }

    pub fn change_filter_op(&mut self) {
        if let Some(filter_state) = self.filters.get_mut(self.filter_index) {
            filter_state.change_op();
        }
    }

    pub fn add_filter(&mut self) {
        self.filters.push(FilterState::new());
        self.filter_index = self.filters.len() - 1;
        if let Some(filter_state) = self.filters.get_mut(self.filter_index) {
            filter_state.set_is_editing(false);
        }
    }

    pub fn handle_filter_char(&mut self, c: char) {
        if let Some(filter_state) = self.filters.get_mut(self.filter_index) {
            filter_state.set_is_editing(true);
            filter_state.push_char(c);
        }
    }

    pub fn handle_filter_backspace(&mut self) {
        if let Some(filter_state) = self.filters.get_mut(self.filter_index) {
            if !filter_state.is_editing() {
                self.filters.remove(self.filter_index);
                if self.filter_index >= self.filters.len() && !self.filters.is_empty() {
                    self.filter_index = self.filters.len() - 1;
                }
            } else {
                filter_state.pop_char();
            }
        }
    }

    pub fn next_highlight(&mut self) {
        if !self.highlights.is_empty() {
            self.highlight_index = (self.highlight_index + 1) % self.highlights.len();

            if let Some(highlight_state) = self.highlights.get_mut(self.highlight_index) {
                highlight_state.set_is_editing(false);
            }
        }
    }

    pub fn toggle_highlight(&mut self) {
        if let Some(highlight_state) = self.highlights.get_mut(self.highlight_index) {
            let active = highlight_state.is_active();
            highlight_state.set_active(!active);
            highlight_state.set_is_editing(false);
        }
    }

    pub fn add_highlight(&mut self) {
        self.highlights.push(HighlightState::new());
        self.highlight_index = self.highlights.len() - 1;
        if let Some(highlight_state) = self.highlights.get_mut(self.highlight_index) {
            highlight_state.set_is_editing(false);
        }
    }

    pub fn handle_highlight_char(&mut self, c: char) {
        if let Some(highlight_state) = self.highlights.get_mut(self.highlight_index) {
            highlight_state.set_is_editing(true);
            highlight_state.push_char(c);
        }
    }

    pub fn handle_highlight_backspace(&mut self) {
        if let Some(highlight_state) = self.highlights.get_mut(self.highlight_index) {
            if !highlight_state.is_editing() {
                self.highlights.remove(self.highlight_index);
                if self.highlight_index >= self.highlights.len() && !self.highlights.is_empty() {
                    self.highlight_index = self.highlights.len() - 1;
                }
            } else {
                highlight_state.pop_char();
            }
        }
    }
}
