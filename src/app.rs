use crate::{log_line::LogLine, mode::Mode, search::SearchState};
use ratatui::widgets::ListState;
use std::collections::VecDeque;

const LOG_STORAGE_CAPACITY: usize = 10_000;

#[derive(Debug, Default)]
pub struct App {
    logs: VecDeque<LogLine>,
    mode: Mode,
    search: SearchState,
    list_state: ListState,
}

impl App {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            logs: VecDeque::with_capacity(LOG_STORAGE_CAPACITY),
            list_state,
            ..Default::default()
        }
    }

    pub fn logs(&self) -> &VecDeque<LogLine> {
        &self.logs
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn search(&self) -> &SearchState {
        &self.search
    }

    pub fn list_state(&self) -> ListState {
        self.list_state
    }

    pub fn push_logs(&mut self, log: LogLine) {
        if self.logs.len() >= LOG_STORAGE_CAPACITY {
            self.logs.pop_front();
        }

        self.logs.push_back(log);

        if self.list_state.selected().is_none() {
            self.list_state.select(Some(0));
        }
    }

    pub fn enter_search_mode(&mut self) {
        self.mode = Mode::Search;
    }

    pub fn exit_search_mode(&mut self) {
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
        let query = self.search.query().to_lowercase();

        if query.is_empty() {
            self.search.clear_matches();
            return;
        }

        let matches: Vec<usize> = self
            .logs
            .iter()
            .enumerate()
            .filter_map(|(idx, log)| {
                if log.content().to_lowercase().contains(&query) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();

        self.search.set_matches(matches);

        if let Some(first) = self.search.current_match_index() {
            self.list_state.select(Some(first));
        }
    }

    pub fn next_match(&mut self) {
        self.search.next_match();

        if let Some(idx) = self.search.current_match_index() {
            self.list_state.select(Some(idx));
        }
    }

    pub fn select_next(&mut self) {
        if self.logs.is_empty() {
            return;
        }

        let current = self.list_state.selected().unwrap_or(0);
        let next = (current + 1).min(self.logs.len() - 1);
        self.list_state.select(Some(next));
    }

    pub fn select_previous(&mut self) {
        if self.logs.is_empty() {
            return;
        }

        let current = self.list_state.selected().unwrap_or(0);
        let prev = current.saturating_sub(1);
        self.list_state.select(Some(prev));
    }
}
