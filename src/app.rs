use crate::{bookmark::BookMark, log_store::LogStore, mode::Mode, search::SearchState};
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

    pub fn handle_search_char(&mut self, c: char) {
        self.search.push_char(c);
    }

    pub fn handle_search_backspace(&mut self) {
        self.search.pop_char();
    }

    pub fn clear_search(&mut self) {
        self.search.clear();
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
                self.list_state.select(Some(first));
            }
            self.rx = None;
        }
    }

    pub fn next_search_match(&mut self) {
        self.search.next_match();

        if let Some(idx) = self.search.current_match_index() {
            self.list_state.select(Some(idx));
        }
    }

    pub fn select_next(&mut self) {
        let current = self.list_state.selected().unwrap_or(0);
        let next = (current + 1).min(self.logs().len().saturating_sub(1));
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
}
